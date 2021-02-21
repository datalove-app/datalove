use crate::dev::*;
// use libp2p_core::identity::error::DecodingError;
use std::fmt::Debug;
use thiserror::Error;

/// Manages loading, decryption and access to the stored key(s).
pub trait Keystore<C: Core>: Driver<C> {
    type KeyParams: Debug + Default;

    // fn lock(&mut self, lock_params: Self::LockParams);

    // /// Unlocks the `Keystore`, delegating to whatever authentication system
    // /// the `Keystore` is configured to use, returning the stored, device-
    // /// specific [`libp2p::identity::Keypair`].
    // fn unlock(&mut self, lock_params: Self::LockParams) -> Result<Keypair, Error>;

    /// Retrieves the device's root ed25519 keypair, used for securing network
    /// communications and signing entries in the device's log.
    fn peer_keypair(&self) -> PeerKeypair;

    // /// Retrieves/derives a keypair,
    // fn group_keypair(&self, params: Self::KeyParams) -> ed25519::Keypair;
}

///
#[derive(Debug, Error)]
pub enum Error {
    // #[error("decoding error: {0}")]
    // Decode(#[from] DecodingError),
    #[error("failed to unlock keystore: {0}")]
    UnlockFailure(String),
}
