use crate::{
    maybestd::{cell::Ref, io, ops::Deref},
    util::{
        risc0::{BlockData, TypedJournal},
        Sha256Digest,
    },
    Error,
};
use borsh::{BorshDeserialize, BorshSerialize};

// pub type PublicState<V> = TypedJournal<(Sha256Digest, V)>;

///
pub trait ProverState: Default + BorshSerialize + BorshDeserialize {
    // type Signature: BorshDeserialize;
    // fn verify(&self, msg: &[u8], signature: &Self::Signature) -> Result<(), Error>;
}
// impl<P: Default + BorshSerialize + BorshDeserialize> ProverState for P {}

///
pub trait VerifierState: Default + BorshSerialize + BorshDeserialize {}
// impl<V: Default + BorshSerialize + BorshDeserialize> VerifierState for V {}

///
pub trait Operation<P: ProverState, V>: BorshSerialize + BorshDeserialize {
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
#[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
pub struct StateMachine<P, V> {
    /// Public verifier state
    /// Commits to prover state, assuring consistency.
    verifier: TypedJournal<(Sha256Digest, V)>,
    /// Prover state; verifies authorization of operations.
    prover: BlockData<P>,
}

impl<P: ProverState, V: VerifierState> StateMachine<P, V> {}

impl<P: ProverState, V: VerifierState> StateMachine<P, V> {
    pub fn exec<Op>(
        mut stdin: impl io::Read,
        mut stdout: impl io::Write,
        mut journal: impl io::Write,
    ) -> Result<(), Error>
    where
        Op: Operation<P, V>,
    {
        let transition = Transition::deserialize_reader(&mut stdin)?;
        let mut sm = <Option<Self>>::try_from_reader(&mut stdin)?.unwrap_or_default();

        sm.validate::<Op>(&transition)?;
        sm.apply::<Op>(transition)?;

        sm.prover.serialize(&mut stdout)?;
        sm.verifier.as_inner_mut().0 = sm.prover.digest().clone();
        sm.verifier.serialize(&mut journal)?;
        Ok(())
    }

    fn validate<Op>(&self, transition: &Transition<Op>) -> Result<(), Error>
    where
        Op: Operation<P, V>,
    {
        // assert operation expects verifier state
        if transition.op_commitment() != self.verifier.digest().deref() {
            Err(Error::InvalidOperation(
                "operation expected different prover state",
            ))?;
        }

        // assert verifier expects prover state
        if self.verifier_commitment() != self.prover.digest().deref() {
            Err(Error::InvalidOperation(
                "verifier expected different prover state",
            ))?;
        }

        // verify operation is valid against current state
        let (prover, verifier) = self.as_ref();
        let (op_digest, op) = transition.as_ref();
        op.validate(op_digest, verifier, prover)?;

        Ok(())
    }

    fn apply<Op>(&mut self, transition: Transition<Op>) -> Result<(), Error>
    where
        Op: Operation<P, V>,
    {
        let (prover, verifier) = self.as_mut();
        let (op_digest, op) = transition.into_inner();
        op.apply(op_digest, verifier, prover)?;

        Ok(())
    }

    fn verifier_commitment(&self) -> &Sha256Digest {
        &self.verifier.as_inner().0
    }

    fn as_ref(&self) -> (&P, &V) {
        (&self.prover.as_inner(), &self.verifier.as_inner().1)
    }

    fn as_mut(&mut self) -> (&mut P, &mut V) {
        (
            self.prover.as_inner_mut(),
            &mut self.verifier.as_inner_mut().1,
        )
    }
}

/// A state transition, as provided to the guest.
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Transition<Op> {
    /// Commitment to verifier state; plus the operation which will be applied.
    /// Prevents replay attacks.
    op: BlockData<(Sha256Digest, Op)>,
    // /// Signature of the entire operation.
    // signature: S,
}

impl<Op> Transition<Op> {
    fn as_ref(&self) -> &(Sha256Digest, Op) {
        self.op.as_inner()
    }

    fn into_inner(self) -> (Sha256Digest, Op) {
        self.op.into_inner()
    }

    fn op_commitment(&self) -> &Sha256Digest {
        &self.op.as_inner().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
    struct ProverState(u32);

    #[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
    struct VerifierState(u32);

    enum Op {}

    #[test]
    fn test_exec() {
        // TODO
    }
}
