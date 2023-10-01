use crate::*;
use borsh::io;

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    #[cfg_attr(feature = "std", error("I/O error: {0}"))]
    Io(#[cfg_attr(feature = "std", from)] io::Error),

    #[cfg_attr(feature = "std", error("Signature error: {0}"))]
    SignatureError(#[cfg_attr(feature = "std", from)] ed25519_dalek::SignatureError),

    #[cfg_attr(feature = "std", error("MerkleLog error: {0}"))]
    MerkleLogError(#[cfg_attr(feature = "std", from)] merkle_log::Error),

    // #[error("high threshold must be greater than low threshold")]
    // InvalidRelativeThresholds,
    #[cfg_attr(feature = "std", error("total weight sum must be between 0 and 255"))]
    InvalidUserWeights,

    // #[error("high threshold must be greater than low threshold")]
    // InvalidThresholds,

    // #[error("exceeds max threshold ({})", u16::MAX)]
    // ExceedsMaxThreshold,
    // #[error("number of peers must be between 0 and max ({})", User::MAX_DEVICES)]
    // InvalidGuardianCount,
    // #[error("guardian threshold sum must be greater than high threshold")]
    // InsufficientGuardianThreshold,
    #[cfg_attr(feature = "std", error("unauthorized operation"))]
    Unauthorized,
}
