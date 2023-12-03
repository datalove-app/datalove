mod group;
mod ops;

pub use crate::device::Device;
pub use group::{Group, GroupSignature, Member, MemberSignature};
pub use ops::Operation;

use crate::util::Sha256Digest;
use borsh::{BorshDeserialize, BorshSerialize};

/// The decentralized identifier of a [`Persona`].
pub type Did = Sha256Digest;

/// Each device associated with a [`Persona`] has a configurable weight that
/// governs its share in signing for persona state change operations.
pub type Weight = u8;

/// Publicly committed state of the persona.
#[derive(Clone, Debug, Default, BorshDeserialize, BorshSerialize)]
pub struct Persona {
    /// The sequence number (i.e. age by number of proofs generated).
    seqno: u32,

    /// The DID of the persona.
    did: Did,

    /// The SHA256 digest of the proof-specific message.
    msg_digest: Sha256Digest,

    /// The SHA256 digest of persona-related metadata to be committed.
    metadata: Sha256Digest,
}

#[cfg(target_os = "zkvm")]
pub fn exec() -> crate::maybestd::io::Result<()> {
    crate::proof::exec::<Group, Persona, Operation, GroupSignature>()?;
    Ok(())
}
