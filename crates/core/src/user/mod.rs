//!
//! ```text
//! schema!(type DeviceLink &Device);
//!
//! advanced!(type Devices ...Device... representation
//!     Group());
//!     // b/c this is CRDT+Set, it has set mutations, meaning
//!         // internal representations keep around unapplied mutations
//!         // by default, all ops are applied (if valid)
//!     // b/c this is a Signer(all)
//!         // it requires a signature from all members
//!         // e.g. Signer(m_of_n(m=3,n=5))
//!     // b/c this is a Signer and a Set, it is a Group
//!
//! advanced!(type Guardians ...Device + User... representation
//!     ...);
//!
//! advanced!(type User struct {
//!     vclock: [{&Device: u64}]    // clock of clocks?
//!     devices: Devices,
//!     guardians: Guardians
//! } representation ...);
//! ```
//!

mod event;
mod query;
mod types;

pub use types::*;
pub mod service {
    pub use super::event::*;
    pub use super::query::*;
}

use crate::dev::*;

/// The runtime service responsible for managing the `Core`'s user identity,
/// which is itself a distributed collection of [`crate::peer::Peer`]s.
pub trait UserService<C: Core>
where
    Self: Service<C>,
{
    type Root: Peer;
    type User: PeerGroup;

    /// Our local, persistent, network-aware cryptographic identity.
    fn root(&self) -> Self::Root;
}

#[cfg(test)]
mod test {

    #[test]
    fn it_works() -> Result<(), ()> {
        println!("running test");
        assert_eq!(true, true);
        Ok(())
    }
}
