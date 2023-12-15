use super::{Group, Member, Persona, Threshold, Weight};
use crate::{
    util::{risc0::Sha256, Sha256Digest},
    zksm::Operation as IOperation,
    Error, GroupSignature,
};
use borsh::{BorshDeserialize, BorshSerialize};
use digest::Digest;

///
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SignedOperation {
    /// The operation payload.
    op: Operation,

    /// The signature of the operation.
    signature: GroupSignature,
    // #[borsh(skip)]
    // weight: Option<Threshold>,
}

impl SignedOperation {
    // /// Verifies a group signature against the group's state.
    // pub fn verify_signature(
    //     &self,
    //     op_digest: &Sha256Digest,
    //     sig: &GroupSignature,
    // ) -> Result<Threshold, Error> {
    //     // assert participant count
    //     let num_participants = sig.len();
    //     if !(num_participants > 0 && num_participants <= self.len()) {
    //         return Err(signature::Error::new().into());
    //     }

    //     let mut sig_weight = 0u16;

    //     // verify each member sig
    //     // TODO: group and batch device sig verifies
    //     for (member_idx, member_sig) in sig.idx_iter() {
    //         let member = &self.members[member_idx];
    //         member.verify_signature(op_digest, &member_sig)?;

    //         sig_weight += member.weight() as Threshold;
    //     }

    //     Ok(sig_weight)
    // }
}

impl IOperation<Group, Persona> for SignedOperation {
    #[inline]
    fn validate(
        &self,
        op_digest: &Sha256Digest,
        persona: &Persona,
        group: &Group,
    ) -> Result<(), Error> {
        let sig_group = if group.len() == 0 {
            self.op
                .try_as_init()
                .ok_or_else(|| {
                    Error::InvalidOperation("only Init can be applied to an empty group")
                })?
                .as_ref()
        } else {
            group
        };

        let sig_weight = sig_group.verify_signature(op_digest, &self.signature)?;
        // self.operation.verify_weight(sig_weight, sig_group)?;
        self.op.validate(op_digest, persona, group)?;

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
    /// The initialization operation, that creates a new [`Persona`] from the provided [`MemberState`].
    Init(init::Init),
    // Bump(bump::Bump),
    // Swap(swap::Swap),
    // Sign(sign::Sign),
    // Freeze,
    // Thaw,
}

impl Operation {
    fn try_as_init(&self) -> Option<&init::Init> {
        match self {
            Self::Init(op) => Some(op),
            _ => None,
        }
    }
}

impl Operation {}

impl IOperation<Group, Persona> for Operation {
    #[inline]
    fn validate(
        &self,
        self_digest: &Sha256Digest,
        persona: &Persona,
        group: &Group,
    ) -> Result<(), Error> {
        match self {
            // group weight must be greater than or equal to the signature weight
            Self::Init(op) => op.validate(self_digest, persona, group),
            // use case(s):
            //  - bump: only updates seqno + group member clocks (no threshold change)
            //      - can be self-signed (i.e. by only members being updated)
            // Self::Bump(op)

            // use case(s):
            //  - addition:
            //  - removal:
            //  - (device) key rotation: 1:1 swap && old signs new(head), no threshold change
            //      - self-solo-signed iff 1:1 swap && old signs new(head), no threshold change
            //  - ???
            // CANNOT be ONLY self-signed personas
            // Self::Swap(op) => op.validate(self_digest, persona, group),

            // use case(s):
            //  - sign: ... ? something that requires consensus?
            // CANNOT be ONLY self-signed personas
            // Self::Sign(op) => op.validate(self_digest, persona, group),
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
            // Self::Swap(op) => op.apply(self_digest, persona, group),
            // Self::Sign(op) => op.apply(self_digest, persona, group),
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

impl<T, U> AsRef<U> for GenericOperation<T>
where
    T: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.payload.as_ref()
    }
}

impl<T, U> From<(Sha256Digest, U)> for GenericOperation<T>
where
    T: From<U>,
{
    fn from((new_metadata, payload): (Sha256Digest, U)) -> Self {
        Self {
            new_metadata,
            payload: payload.into(),
        }
    }
}

mod init {
    use super::*;

    /// The initialization operation, that creates a new [`Persona`] from the provided [`MemberState`].
    pub type Init = GenericOperation<InitInner>;

    #[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
    pub struct InitInner {
        // msg: Sha256Digest,
        group: Group,
    }

    impl Init {
        pub fn new(metadata: Sha256Digest, group: Group) -> Self {
            Self {
                new_metadata: metadata,
                payload: InitInner { group },
            }
        }
    }

    impl AsRef<Group> for InitInner {
        fn as_ref(&self) -> &Group {
            &self.group
        }
    }

    // impl From<Group> for InitInner {
    //     fn from(group: Group) -> Self {
    //         Self { group }
    //     }
    // }

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
            // let Self { payload, .. } = self;

            *group = self.payload.group;
            persona.seqno = 1;
            persona.metadata = self.new_metadata;
            // persona.msg = self.payload.msg;
            persona.did = Sha256::default()
                .chain_update(&self_digest)
                .chain_update(&persona.metadata)
                // .chain_update(&persona.msg)
                .finalize()
                .into();

            Ok(())
        }
    }
}

mod bump {
    use super::*;

    /// The bump operation, which increments the [`Persona`]'s sequence number.
    pub type Bump = GenericOperation<BumpInner>;

    pub struct BumpInner {}
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
    use super::*;

    // fn random_device_member() -> Member {}

    #[test]
    fn can_init() {}
}
