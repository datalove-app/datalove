// ! ```
// ! advanced!(type Device struct {
// !     address: String,
// !         // IPv6?
// !     network_pub_key: HostNetworkPubKey,
// !         // PublicKey?
// ! } representation
// !     Signed(key = HostDevicePubKey),
// !         // check this at compile-time
// !         // this creates a wrapper type that impl Signer<algo>
// !             // ? then separately impl Representation for all signers?
// !         // ? since this is datalove-specific, it reads itself from envvar
// !     Signed(from = "network_pub_key"));
// !         // check at compile-time that the type impl Signer
// !         // then do the same as above
// ! ```
// !
// ! Can prove (passively, interactively?):
// !     - that the hash of `network_pub_key` is the IPv6
// !     - that there is also

use crate::dev::*;
use std::{convert::TryInto, fmt::Debug, vec::IntoIter};

/// Trait for types that have `PeerId`s.
pub trait ToPeerId {
    fn to_peer_id(&self) -> PeerId;

    /// Provides the `Peer`'s [`PublicKey`].
    ///
    /// [`PublicKey`]: libp2p_core::identity::ed25519::PublicKey;
    fn public(&self) -> PeerPublicKey;
}

impl ToPeerId for PeerKeypair {
    #[inline(always)]
    fn to_peer_id(&self) -> PeerId {
        libp2p_identity::PublicKey::Ed25519(self.public()).into_peer_id()
    }

    #[inline(always)]
    fn public(&self) -> PeerPublicKey {
        self.public()
    }
}

/// Trait for types that can be resolved to one or more `Multiaddr`s.
pub trait ToMultiaddrs {
    type Iter: Iterator<Item = Multiaddr>;

    fn to_multiaddrs(&self) -> Self::Iter;
}

impl ToMultiaddrs for Vec<Multiaddr> {
    type Iter = IntoIter<Multiaddr>;
    fn to_multiaddrs(&self) -> Self::Iter {
        self.clone().into_iter()
    }
}

/// Represents a network-capable, cryptographic peer on the network.
///
/// While its simplest implementation represents a single device, the trait encapsulates the minimally required behaviours of any number of entities operating in unison.
/// TODO? encrypt + decrypt?
/// TODO? derive + verify_derived_sig?
/// TODO? multi-party generation/sign/verify?
/// TODO? transform/re-encryption?
pub trait Peer: ToPeerId + ToMultiaddrs + Verifier<Signature> {
    // /// Signs some bytes using the `Peer`'s underlying keypair.
    // ///
    // /// TODO? should this be &Self::SecretKey?
    // fn sign(&self, msg: &[u8]) -> Result<Signature, Error>;

    // /// Verifies some bytes and a signature against the `Peer`'s underlying
    // /// keypair.
    // ///
    // /// TODO? should this be &Self::PublicKey?
    // fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<bool, Error>;
}

pub trait ManagedPeer: Peer + Signer<Signature> {
    /// Gets the underlying [`libp2p_core::identity::Keypair`].
    #[inline(always)]
    fn keypair(&self) -> PeerKeypair;
}

/// Represents a swarm of `Peer`s.
///
/// This can be a collection of devices representing a user, or a collection of
/// `Peer`s representing a group of users, or even a collection of `PeerGroup`s
/// representing a consortium of groups.
///
/// TODO? look at libp2p::NetworkBehaviour, hbbft::ConsensusProtocol
/// todo? adding/removing peers?
/// TODO? provide cryptographic and transactional capabilities.
pub trait PeerGroup: Peer + NetworkBehaviour // + Verifier<MultiSignature>
{
    // type Peer: Peer;s

    // fn did(&self) -> Did;

    // fn add_peer(&mut self, peer: )
}

pub trait ManagedPeerGroup: PeerGroup // + Signer<Signature> + Signer<MultiSignature>
{
    fn peers(&self) -> &[PeerId];
}

// ///
// ///
// #[derive(Debug)]
// pub struct MemoryPeer {
//     pub(crate) keypair: ed25519::Keypair,
//     pub multiaddrs: Vec<Multiaddr>,
// }

// impl MemoryPeer {
//     /// Creates a random `MemoryPeer`.
//     #[inline(always)]
//     pub fn random() -> Self {
//         Self {
//             keypair: ed25519::Keypair::generate(),
//             multiaddrs: Vec::new(),
//         }
//     }

//     /// Tries to decode a `MemoryPeer` from bytes.
//     #[inline(always)]
//     pub fn from_bytes(sk_bytes: impl AsMut<[u8]>) -> Result<Self, RootError> {
//         Ok(Self {
//             keypair: ed25519::SecretKey::from_bytes(sk_bytes)?.into(),
//             multiaddrs: Vec::new(),
//         })
//     }
// }

// impl ToPeerId for MemoryPeer {
//     #[inline(always)]
//     fn to_peer_id(&self) -> PeerId {
//         Peer::public_key(self).into_peer_id()
//     }
// }

// impl ToMultiaddrs for MemoryPeer {
//     type Iter = IntoIter<Multiaddr>;

//     #[inline(always)]
//     fn to_multiaddrs(&self) -> Self::Iter {
//         self.multiaddrs.clone().into_iter()
//     }
// }

// impl Peer for MemoryPeer {
//     // type Signature = Vec<u8>;

//     #[inline(always)]
//     fn public_key(&self) -> PublicKey {
//         PublicKey::Ed25519(self.keypair.public())
//     }

//     // #[inline(always)]
//     // fn sign(&self, msg: &[u8]) -> Result<Signature, Error> {
//     //     Ok(self.keypair.sign(msg).try_into().expect(""))
//     // }

//     // #[inline(always)]
//     // fn verify(&self, msg: &[u8], sig: &[u8]) -> Result<bool, Error> {
//     //     Ok((&self.keypair.public()).verify(msg, sig))
//     // }
// }

// impl Serialize for MemoryPeer {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         unimplemented!()
//     }
// }

// impl<'de> Deserialize<'de> for MemoryPeer {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         unimplemented!()
//     }
// }
