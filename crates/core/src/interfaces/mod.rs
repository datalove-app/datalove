mod clock;
mod database;
mod peer;
// mod runtime;

pub use clock::*;
pub use database::*;
pub use peer::*;
// pub use runtime::*;

use crate::dev::*;

/// Trait for types that have `PeerId`s.
pub trait AsPeerId {
    fn as_peer_id(&self) -> PeerId;
}

/// Trait for types that can be synchronously resolved to one or more `Multiaddr`s.
pub trait ToMultiaddrs {
    type Iter: Iterator<Item = Multiaddr>;
    fn to_multiaddrs(&self) -> Result<Self::Iter, ()>;
}
