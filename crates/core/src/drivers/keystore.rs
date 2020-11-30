use crate::dev::*;
use libp2p_core::identity::error::DecodingError;
use thiserror::Error;

/// Manages loading, decryption and access to the stored key(s).
pub trait Keystore<C: Core>: Driver<C> {
    /// Unlocks the `Keystore`, using according to whatever authentication
    /// the `Keystore` is configured to use. returning the device-specific
    /// [`libp2p::identity::Keypair`].
    fn unlock(&mut self) -> Result<Keypair, Error>;
}

///
#[derive(Debug, Error)]
pub enum Error {
    #[error("decoding error: {0}")]
    Decode(#[from] DecodingError),

    #[error("failed to unlock keystore: {0}")]
    UnlockFailure(String),
}
