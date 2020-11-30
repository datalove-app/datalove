use crate::dev::*;
use std::{convert::TryInto, fmt::Debug, vec::IntoIter};

/// Trait for types that have `PeerId`s.
pub trait ToPeerId {
    fn to_peer_id(&self) -> PeerId;
}

impl ToPeerId for PeerId {
    #[inline(always)]
    fn to_peer_id(&self) -> PeerId {
        self.clone()
    }
}
impl ToPeerId for Keypair {
    #[inline(always)]
    fn to_peer_id(&self) -> PeerId {
        self.public().into_peer_id()
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
/// This can be a single device, a collection of devices representing a user,
/// or a group of users.
/// TODO? encrypt + decrypt?
/// TODO? derive + verify_derived_sig?
/// TODO? multi-party generation/sign/verify?
/// TODO? transform/re-encryption?
pub trait Peer: ToPeerId + ToMultiaddrs + Signer<Signature> + Verifier<Signature> {
    /// Provides the `Peer`'s [`libp2p_core::identity::PublicKey`].
    #[inline(always)]
    fn public(&self) -> PublicKey;

    /// Gets the underlying [`libp2p_core::identity::Keypair`].
    #[inline(always)]
    fn keypair(&self) -> Keypair;
}

/// Represents a swarm of `Peer`s.
///
/// TODO? look at libp2p::NetworkBehaviour, hbbft::ConsensusProtocol
/// todo? adding/removing peers?
/// TODO? provide cryptographic and transactional capabilities.
pub trait PeerGroup: Peer + NetworkBehaviour {
    fn did(&self) -> Did;

    fn peers(&self) -> &[PeerId];
}

