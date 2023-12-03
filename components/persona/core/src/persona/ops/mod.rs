mod init;
mod swap;

use super::{Group, Member, MemberSignature, Persona};
use crate::{
    maybestd::{io, vec::Vec},
    proof::Operation as IOperation,
    util::{DigestPipe, Sha256Digest},
    Error,
};
use borsh::{BorshDeserialize, BorshSerialize};
// use ed25519_dalek::{Signature, Signer};

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
    fn prev_outputs_digest(&self) -> &Sha256Digest {
        match self {
            Self::Init(op) => op.prev_outputs_digest(),
            Self::Swap(op) => op.prev_outputs_digest(),
            // Self::Sign(op) => op.prev_outputs_digest(),
        }
    }

    #[inline]
    fn validate(&self, persona: &Persona, group: &Group) -> Result<(), Error> {
        match self {
            Self::Init(op) => op.validate(persona, group),
            Self::Swap(op) => op.validate(persona, group),
            // Self::Sign(op) => op.validate(prev_persona),
        }
    }

    #[inline]
    fn apply(self, persona: Persona, group: Group) -> Result<Persona, Error> {
        match self {
            Self::Init(op) => op.apply(persona, group),
            Self::Swap(op) => op.apply(persona, group),
            // Self::Sign(op) => op.validate(prev_persona),
        }
    }
}

/// All operations besides must carry additional info to be verifiably applied.
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
#[repr(align(4))]
pub struct GenericOperation<T> {
    /// The previous state of the persona to which this operation will be applied.
    /// Prevents replay attacks.
    prev_state_digest: Sha256Digest,

    /// The new metadata to be associated with the [`Persona`].
    new_metadata: Sha256Digest,

    /// The inner operation payload.
    payload: T,
}

#[cfg(test)]
mod tests {
    // #[test]
}
