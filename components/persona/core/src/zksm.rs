use crate::{
    maybestd::{cell::Ref, fmt::Debug, io, ops::Deref},
    util::{
        risc0::{self, BlockData, ImageId, TypedJournal},
        Empty, Sha256Digest,
    },
    Error,
};
use borsh::{BorshDeserialize, BorshSerialize};

///
pub trait ProverState: Debug + Default + BorshSerialize + BorshDeserialize {
    // type Signature: BorshDeserialize;
    // fn verify(&self, msg: &[u8], signature: &Self::Signature) -> Result<(), Error>;
}

///
pub trait VerifierState: Debug + Default + BorshSerialize + BorshDeserialize {
    // /// Verifier's commitment to the prover state.
    // fn prover_commitment(&self) -> &Sha256Digest;
}

///
pub trait Operation<P: ProverState, V: VerifierState>:
    Debug + BorshSerialize + BorshDeserialize
{
    fn validate(
        &self,
        self_digest: &Sha256Digest,
        verifier_state: &V,
        prover_state: &P,
    ) -> Result<(), Error>;

    fn apply(
        self,
        self_digest: Sha256Digest,
        verifier_state: &mut V,
        prover_state: &mut P,
    ) -> Result<(), Error>;
}

///
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct StateMachine<P, V> {
    /// Prover state; private state of the state machine.
    prover: BlockData<P>,
    /// Commitment to prover state, plus public verifier state (read from the journal bytes and seal).
    /// Verifies the verifier's state journal is valid.
    #[cfg_attr(
        target_os = "zkvm",
        borsh(deserialize_with = "TypedJournal::deserialize_verify_self")
    )]
    verifier: TypedJournal<VState<V>>,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct VState<V> {
    commitment: Sha256Digest,
    state: V,
}

impl<P: ProverState, V: VerifierState> StateMachine<P, V> {
    /// Produces the default state machine (with appropriate commitments) given the image_id of the guest that is executing it.
    pub fn new(self_image_id: Sha256Digest) -> Self {
        let prover = BlockData::default();
        let verifier = TypedJournal::new(
            self_image_id,
            VState {
                commitment: prover.digest().clone(),
                state: Default::default(),
            },
        );
        Self { prover, verifier }
    }

    pub fn load(mut stdin: impl io::Read) -> Result<Self, Error> {
        Ok(Self::try_from_reader(&mut stdin)?)
    }

    ///
    pub fn run<Op: Operation<P, V>>(self, transition: Transition<Op>) -> Result<Self, Error> {
        self.run_with_writer(transition, Empty, Empty)
    }

    pub fn new_transition<Op: Operation<P, V>>(&self, op: Op) -> Transition<Op> {
        let commitment = self.verifier.digest().clone();
        Transition::new(TransitionOp { commitment, op })
    }

    pub fn as_ref(&self) -> (&P, &V) {
        (&self.prover.as_inner(), &self.verifier.as_inner().state)
    }

    pub fn into_inner(self) -> (P, V) {
        (self.prover.into_inner(), self.verifier.into_inner().state)
    }

    pub fn verifier_digest(&self) -> Sha256Digest {
        self.verifier.digest().clone()
    }

    pub fn verifier_state_ref(&self) -> &V {
        &self.verifier.as_inner().state
    }

    pub fn verifier_commitment(&self) -> &Sha256Digest {
        &self.verifier.as_inner().commitment
    }

    pub fn prover_digest(&self) -> Sha256Digest {
        self.prover.digest().clone()
    }

    pub fn prover_state_ref(&self) -> &P {
        &self.prover.as_inner()
    }
}

impl<P: ProverState, V: VerifierState> StateMachine<P, V> {
    ///
    pub fn run_io<Op>(
        mut stdin: impl io::Read,
        stdout: impl io::Write,
        journal: impl io::Write,
        // trace: impl FnMut(&Self),
    ) -> Result<(), Error>
    where
        Op: Operation<P, V>,
    {
        #[cfg(not(target_os = "zkvm"))]
        let self_image_id = ImageId::default();
        #[cfg(target_os = "zkvm")]
        let self_image_id: ImageId = risc0::self_image_id();

        Self::run_io_with_image_id::<Op>(self_image_id, stdin, stdout, journal)
    }

    ///
    pub fn run_io_with_image_id<Op>(
        self_image_id: ImageId,
        mut stdin: impl io::Read,
        stdout: impl io::Write,
        journal: impl io::Write,
        // trace: impl FnMut(&Self),
    ) -> Result<(), Error>
    where
        Op: Operation<P, V>,
    {
        let mut cc = risc0::trace(format_args!("run_io"), None);

        let transition = Transition::<Op>::deserialize_reader(&mut stdin)?;
        cc = risc0::trace(format_args!("deserialized transition",), Some(cc));

        let start = Option::<Self>::try_from_reader(&mut stdin)?;
        cc = risc0::trace(format_args!("read sm"), Some(cc));

        let _ = start
            .unwrap_or_else(|| Self::new(self_image_id))
            .run_with_writer(transition, stdout, journal)?;
        risc0::trace(format_args!("end sm run"), Some(cc));

        Ok(())
    }

    /// Execute the state machine with the given transition.
    fn run_with_writer<Op: Operation<P, V>>(
        mut self,
        transition: Transition<Op>,
        mut stdout: impl io::Write,
        mut journal: impl io::Write,
    ) -> Result<Self, Error> {
        let mut cc = risc0::trace(format_args!("starting sm validate"), None);

        self.validate(&transition)?;
        cc = risc0::trace(
            format_args!(
                "validated transition:\n\top_digest {:?}\n\top_commitment {:?}\n\tprover ({:?})\n\tverifier ({:?})",
                &transition.as_op().0,
                transition.op_commitment(),
                &self.prover_digest(),
                &self.verifier_digest(),
            ),
            Some(cc),
        );

        self.apply(transition)?;
        cc = risc0::trace(format_args!("applied transition"), Some(cc));

        self.prover.serialize(&mut stdout)?;
        cc = risc0::trace(
            format_args!("prover state -> stdout ({:?})", &self.prover_digest()),
            Some(cc),
        );

        // re-assign the verifier commitment
        *self.verifier_commitment_mut() = self.prover_digest();
        self.verifier.serialize(&mut journal)?;
        risc0::trace(
            format_args!(
                "verifier state -> journal ({:?})\n\timage_id {:?}",
                &self.verifier_digest(),
                &self.verifier.image_id(),
            ),
            Some(cc),
        );

        Ok(self)
    }

    fn validate<Op>(&self, transition: &Transition<Op>) -> Result<(), Error>
    where
        Op: Operation<P, V>,
    {
        // assert operation expects verifier state
        if transition.op_commitment() != self.verifier.digest().deref() {
            Err(Error::InvalidOperation(
                "operation expected different verifier state",
            ))?;
        }

        // assert verifier expects prover state
        if self.verifier_commitment() != self.prover.digest().deref() {
            Err(Error::InvalidOperation(
                "verifier expected different prover state",
            ))?;
        }

        // validate op against current state
        let (prover, verifier) = self.as_ref();
        let (op_digest, op) = transition.as_op();
        op.validate(op_digest.deref(), verifier, prover)?;

        Ok(())
    }

    fn apply<Op>(&mut self, transition: Transition<Op>) -> Result<(), Error>
    where
        Op: Operation<P, V>,
    {
        let (prover, verifier) = self.as_mut_state();
        let (op_digest, op) = transition.into_op_parts();
        op.apply(op_digest, verifier, prover)?;

        Ok(())
    }

    fn verifier_commitment_mut(&mut self) -> &mut Sha256Digest {
        &mut self.verifier.as_inner_mut().commitment
    }

    fn as_mut_state(&mut self) -> (&mut P, &mut V) {
        (
            self.prover.as_inner_mut(),
            &mut self.verifier.as_inner_mut().state,
        )
    }
}

impl<P: ProverState, V: VerifierState> Default for StateMachine<P, V> {
    fn default() -> Self {
        Self::new(Sha256Digest::ZERO)
    }
}

///
pub type Transition<Op> = BlockData<TransitionOp<Op>>;

/// A state transition, as provided to the guest.
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct TransitionOp<Op> {
    /// Private operation to apply to prover + verifier state, plus commitment to
    /// expected verifier state.
    /// Prevents replay attacks.
    commitment: Sha256Digest,
    op: Op,
}

impl<Op> Transition<Op> {
    fn as_op(&self) -> (Ref<Sha256Digest>, &Op) {
        (self.digest.borrow(), &self.t.op)
    }

    fn into_op_parts(self) -> (Sha256Digest, Op) {
        (self.digest.into_inner(), self.t.op)
    }

    fn op_commitment(&self) -> &Sha256Digest {
        &self.t.commitment
    }
}

#[doc(hidden)]
#[cfg(any(test, target_os = "zkvm", feature = "test"))]
pub mod tests {
    use super::*;

    pub type Mod7SM = StateMachine<PState, VState>;

    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
    pub struct PState(pub u32);
    impl ProverState for PState {}

    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
    pub struct VState(pub u32);
    impl VerifierState for VState {}

    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
    pub enum Op {
        // (privately) init to n, (publicly) commit to n % 7
        Init(u8),
        // (privately) inc by n, (publicly) commit to n % 7
        Inc(u16),
    }

    impl Operation<PState, VState> for Op {
        fn validate(
            &self,
            _self_digest: &Sha256Digest,
            verifier_state: &VState,
            prover_state: &PState,
        ) -> Result<(), Error> {
            match self {
                Op::Init(_) if !(prover_state.0 | verifier_state.0 == 0) => {
                    Err(Error::InvalidOperation("init called twice"))?
                }
                Op::Inc(_) if prover_state.0 | verifier_state.0 == 0 => {
                    Err(Error::InvalidOperation("inc called before init"))?
                }
                _ => Ok(()),
            }
        }

        fn apply(
            self,
            _self_digest: Sha256Digest,
            verifier_state: &mut VState,
            prover_state: &mut PState,
        ) -> Result<(), Error> {
            match self {
                Op::Init(n) => {
                    *prover_state = PState(n as u32);
                    *verifier_state = VState(n as u32 % 7);
                }
                Op::Inc(n) => {
                    prover_state.0 += n as u32;
                    verifier_state.0 = prover_state.0 % 7;
                }
            };

            Ok(())
        }
    }

    #[test]
    fn can_run() -> Result<(), Error> {
        // init op
        // assert commitments are zeroes, digests of default state machine inputs are zeroes
        let sm = Mod7SM::new(Default::default());
        let transition = sm.new_transition(Op::Init(43));
        assert_eq!(sm.verifier_commitment(), &sm.prover_digest());
        // assert_eq!(sm.verifier_commitment(), &Sha256Digest::ZERO);
        // assert_eq!(&sm.prover_digest(), &Sha256Digest::ZERO);
        // assert_eq!(&sm.verifier_digest(), &Sha256Digest::ZERO);
        // assert_eq!(transition.op_commitment(), &Sha256Digest::ZERO);

        // assert prover and verifier output states, and verifier commits to prover state
        let sm = sm.run(transition)?;
        assert_eq!(sm.prover_state_ref().0, 43);
        assert_eq!(sm.verifier_state_ref().0, 1);
        assert_eq!(sm.verifier_commitment(), &sm.prover_digest());

        // inc op
        // assert prover and verifier output states, assert transition commits to verifier state
        let transition = sm.new_transition(Op::Inc(5));
        assert_eq!(transition.op_commitment(), &sm.verifier_digest());
        let sm = sm.run(transition)?;
        assert_eq!(sm.prover_state_ref().0, 48);
        assert_eq!(sm.verifier_state_ref().0, 6);
        assert_eq!(sm.verifier_commitment(), &sm.prover_digest());

        // another inc op
        let transition = sm.new_transition(Op::Inc(13));
        assert_eq!(transition.op_commitment(), &sm.verifier_digest());
        let sm = sm.run(transition)?;
        assert_eq!(sm.prover_state_ref().0, 61);
        assert_eq!(sm.verifier_state_ref().0, 5);
        assert_eq!(sm.verifier_commitment(), &sm.prover_digest());

        Ok(())
    }

    fn test_run_io(
        transition: Transition<Op>,
        sm: Option<Mod7SM>,
        pstate: u32,
        vstate: u32,
    ) -> Result<Mod7SM, Error> {
        let mut input_bytes = Vec::new();
        transition.serialize(&mut input_bytes)?;
        sm.serialize(&mut input_bytes)?;

        let mut output_prover_bytes = Vec::new();
        let mut output_verifier_bytes = Vec::new();

        Mod7SM::run_io::<Op>(
            &input_bytes[..],
            &mut output_prover_bytes,
            &mut output_verifier_bytes,
        )?;

        // TODO: reconstruct state machine from output bytes

        // let prover = PState::deserialize_reader(&mut output_prover_bytes.as_slice())?;
        // let (prover_digest, verifier) =
        //     <(Sha256Digest, VState)>::deserialize_reader(&mut output_verifier_bytes.as_slice())?;
        // let sm = SM { prover, }

        // assert_eq!(sm.prover_digest(), prover_digest);
        // assert_eq!(sm.prover_state_ref().0, pstate);
        // assert_eq!(sm.verifier_state_ref().0, vstate);

        todo!()
    }

    #[test]
    fn can_run_io() -> Result<(), Error> {
        Ok(())
    }

    pub fn exec(
        mut stdin: impl io::Read,
        mut stdout: impl io::Write,
        mut journal: impl io::Write,
        // mut pause: impl FnMut() -> bool,
    ) -> Result<(), Error> {
        StateMachine::<PState, VState>::run_io::<Op>(&mut stdin, &mut stdout, &mut journal)
    }
}
