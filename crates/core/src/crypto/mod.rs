//!
//! TODO: (along these lines...)
//! - hd from/conversion to/from libp2p keys
//! - combination of libp2p pubkeys into a "user"/group key
//! - hd from seed to libp2p keys
//! - ? creation of a symmetric key

pub type PeerKeypair = libp2p_core::identity::ed25519::Keypair;
pub type PeerSecretKey = libp2p_core::identity::ed25519::SecretKey;
pub type PeerPublicKey = libp2p_core::identity::ed25519::PublicKey;

// pub trait MultiSigner<S> {
//     async fn try_sign(&self, message: &[u8]) -> Result<S, ()>;
// }
