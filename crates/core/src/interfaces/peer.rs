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

// ! NOTE: may not be necessary if a user is just a bunch of signing keys,
// where at least one is hooked into a local device signer

use crate::dev::*;

/// Represents a network-enabled, cryptographic peer on the network.
///
/// This can be a computer, a user, or a group of other `Peer`s.
pub trait Peer: AsPeerId + ToMultiaddrs {
    type Signature;

    ///
    /// TODO? should this be &Self::SecretKey?
    fn sign(&self, msg: &[u8]) -> Result<Self::Signature, Error>;

    ///
    /// TODO? should this be &Self::PublicKey?
    fn verify(&self, msg: &[u8], sig: &Self::Signature) -> Result<bool, Error>;
}

/// Represents a swarm of `Peer`s.
///
/// TODO? look at libp2p::NetworkBehaviour, hbbft::ConsensusProtocol
/// todo? adding/removing peers?
pub trait PeerGroup
where
    Self: AsPeerId,
    Self: ToMultiaddrs,
    Self: NetworkBehaviour,
{
    ///
    type Peer: Peer;

    fn did(&self) -> Did;

    fn peers(&self) -> &[PeerId];
}

/// Extension trait for `PeerGroup`s, providing cryptographic and transactional
/// capabilities.
pub trait PeerGroupExt: Peer + PeerGroup {
    ///
    type Clock: PeerClock;
}
