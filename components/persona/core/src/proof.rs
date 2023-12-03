use crate::{
    maybestd::io,
    util::{risc0::TypedJournal, Sha256Digest, Sha256Pipe},
    Error,
};
use borsh::{BorshDeserialize, BorshSerialize};
use risc0_zkvm::sha::rust_crypto::Sha256;
use signature::Verifier;

#[cfg(target_os = "zkvm")]
pub fn exec<I, O, Op, S>() -> Result<(), Error>
where
    State<I, O>: BorshDeserialize + BorshSerialize + Default,
    Transition<Op, S>: BorshDeserialize + BorshSerialize,
    I: Verifier<S>,
    O: BorshSerialize,
    Op: Operation<I, O>,
{
    use risc0_zkvm::guest::env;

    let mut stdin = env::stdin();
    let transition = Transition::<Op, S>::deserialize_reader(&mut stdin)?;
    let state = <Option<State<I, O>>>::deserialize_reader(&mut stdin)?.unwrap_or_default();

    state.validate(&transition)?;
    let outputs = state.apply(transition.op)?;

    let mut journal = env::journal();
    outputs.serialize(&mut journal)?;
    Ok(())
}

///
#[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
pub struct State<I, O> {
    private_inputs: I,
    prev_outputs: TypedJournal<O>,
}

impl<I, O> State<I, O> {
    fn validate<Op, S>(&self, transition: &Transition<Op, S>) -> Result<(), Error>
    where
        I: Verifier<S>,
        Op: Operation<I, O>,
    {
        // assert operation applies to current state
        if self.prev_outputs.digest() != transition.op.prev_outputs_digest() {
            return Err(Error::InvalidOperation("unexpected state"));
        }

        // verify signature validaty, applicability to operation
        self.private_inputs
            .verify(transition.digest.as_ref(), &transition.signature)?;

        // verify operation is valid against current state
        transition
            .op
            .validate(&self.prev_outputs.as_ref(), &self.private_inputs)?;

        Ok(())
    }

    fn apply<Op>(mut self, op: Op) -> Result<O, Error>
    where
        Op: Operation<I, O>,
    {
        op.apply(self.prev_outputs.into(), self.private_inputs)
    }
}

/// A state transition, as provided to the prover.
#[derive(Clone, Debug, BorshSerialize)]
pub struct Transition<Op, S> {
    /// Digest of the borsh-encoded operation.
    #[borsh(skip)]
    digest: Sha256Digest,
    op: Op,
    signature: S,
}

/// Prover operations generate the operation digest when deserialized.
impl<Op, S> BorshDeserialize for Transition<Op, S>
where
    Op: BorshDeserialize,
    S: BorshDeserialize,
{
    fn deserialize_reader<R: io::Read>(mut reader: &mut R) -> io::Result<Self> {
        let (digest, op) = Sha256Pipe::decode_from_reader(&mut reader)?;
        let signature = S::deserialize_reader(reader)?;

        Ok(Self {
            digest: digest.into(),
            op,
            signature,
        })
    }
}

///
pub trait Operation<I, O> {
    fn prev_outputs_digest(&self) -> &Sha256Digest;

    fn validate(&self, outputs: &O, inputs: &I) -> Result<(), Error>;

    fn apply(self, outputs: O, inputs: I) -> Result<O, Error>;
}
