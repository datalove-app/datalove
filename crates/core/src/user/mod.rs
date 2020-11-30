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
