use super::{Group, Member, Persona};
use crate::{proof::Operation as IOperation, util::Sha256Digest, Error, GroupSignature};
use borsh::{BorshDeserialize, BorshSerialize};

///
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SignedOperation {
    /// The operation payload.
    payload: Operation,

    /// The signature of the operation.
    signature: GroupSignature,
}

impl IOperation<Group, Persona> for SignedOperation {
    #[inline]
    fn validate(
        &self,
        self_digest: &Sha256Digest,
        persona: &Persona,
        group: &Group,
    ) -> Result<(), Error> {
        group.verify(self_digest.as_ref(), &self.signature)?;
        Ok(())
    }

    #[inline]
    fn apply(
        self,
        self_digest: Sha256Digest,
        persona: &mut Persona,
        group: &mut Group,
    ) -> Result<(), Error> {
        todo!()
    }
}

/// An operation to be applied to a persona's [`State`].
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[non_exhaustive]
pub enum Operation {
    Init(init::Init),
    Swap(swap::Swap),
    // Sign(sign::Sign),
}

impl IOperation<Group, Persona> for Operation {
    #[inline]
    fn validate(
        &self,
        self_digest: &Sha256Digest,
        persona: &Persona,
        group: &Group,
    ) -> Result<(), Error> {
        match self {
            Self::Init(op) => op.validate(self_digest, persona, group),
            Self::Swap(op) => op.validate(self_digest, persona, group),
            // Self::Sign(op) => op.validate(prev_persona),
        }
    }

    #[inline]
    fn apply(
        self,
        self_digest: Sha256Digest,
        persona: &mut Persona,
        group: &mut Group,
    ) -> Result<(), Error> {
        match self {
            Self::Init(op) => op.apply(self_digest, persona, group),
            Self::Swap(op) => op.apply(self_digest, persona, group),
            // Self::Sign(op) => op.validate(prev_persona),
        }
    }
}

/// All operations besides must carry additional info to be verifiably applied.
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
#[repr(align(4))]
pub struct GenericOperation<T> {
    /// The new metadata to be associated with the [`Persona`].
    new_metadata: Sha256Digest,

    /// The inner operation payload.
    payload: T,
}

mod init {
    use super::*;

    /// The initialization operation, that creates a new [`Persona`] from the provided [`MemberState`].
    pub type Init = GenericOperation<InitInner>;

    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
    pub struct InitInner {
        members: Vec<Member>,
    }

    impl IOperation<Group, Persona> for Init {
        fn validate(
            &self,
            self_digest: &Sha256Digest,
            persona: &Persona,
            group: &Group,
        ) -> Result<(), Error> {
            // state should be default/uninitialized
            if !(persona == &Persona::DEFAULT && group == &Group::DEFAULT) {
                return Err(Error::InvalidOperation("persona already initialized"));
            }

            Ok(())
        }

        fn apply(
            self,
            self_digest: Sha256Digest,
            persona: &mut Persona,
            group: &mut Group,
        ) -> Result<(), Error> {
            let Self { payload, .. } = self;

            // *group = payload;
            persona.seqno = 1;
            persona.metadata = self.new_metadata;
            persona.msg = Sha256Digest::ZERO;
            // persona.did =

            Ok(())
        }
    }
}

mod swap {
    use super::*;

    /// The member swap operation, which adds, removes or replaces members of the [`Persona`].
    pub type Swap = GenericOperation<SwapInner>;

    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
    pub struct SwapInner {
        members_to_remove: Vec<Member>,
        members_to_add: Vec<Member>,
    }

    impl IOperation<Group, Persona> for Swap {
        fn validate(
            &self,
            self_digest: &Sha256Digest,
            persona: &Persona,
            group: &Group,
        ) -> Result<(), Error> {
            todo!()
        }

        fn apply(
            mut self,
            self_digest: Sha256Digest,
            persona: &mut Persona,
            group: &mut Group,
        ) -> Result<(), Error> {
            todo!()
        }
    }
}

mod sign {

    use super::*;

    /// ...
    pub const DOMAIN_SEPARATOR: &[u8] = b"datalove::persona::sign";

    ///
    pub type SignPayload = [u8; 64];

    /// The `sign` operation, which produces a signature on behalf of a [`Persona`].
    ///
    ///
    pub type Sign = GenericOperation<SignPayload>;

    impl IOperation<Group, Persona> for Sign {
        fn validate(
            &self,
            self_digest: &Sha256Digest,
            persona: &Persona,
            group: &Group,
        ) -> Result<(), Error> {
            todo!()
        }

        fn apply(
            mut self,
            self_digest: Sha256Digest,
            persona: &mut Persona,
            group: &mut Group,
        ) -> Result<(), Error> {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    // #[test]
}
