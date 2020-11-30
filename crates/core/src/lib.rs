//!
//!

#[macro_use]
extern crate lazy_static;

mod drivers;
mod error;
mod group;
mod peer;
mod service;
mod time;
mod user;

/// Exports of all core types and interfaces.
pub mod prelude {
    pub use crate::Core;
    pub use crate::drivers::*;
    pub use crate::error::*;
    pub use crate::group::*;
    pub use crate::peer::*;
    pub use crate::service::*;
    pub use crate::time::*;
    pub use crate::user::*;
}

/// Core dependency re-exports for convenience.
pub mod dev {
    pub use crate::prelude::*;

    pub use cid::Cid;
    pub use ::ed25519::{
        signature::{Signer, Verifier},
        Signature,
    };
    pub use futures::prelude::*;
    pub use libp2p_core::{
        identity::{ed25519, Keypair, PublicKey},
        Multiaddr, PeerId,
    };
    pub use libp2p_swarm::NetworkBehaviour;
    pub use num_traits::{One, Unsigned};
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub use uuid::Uuid;
}

use crate::prelude::*;

///
pub trait Core: Service<Self>
{
    // drivers
    type Keystore: Keystore<Self>;

    /// A persistable, use-case event.
    type Event: Event<Self>;

    /// A use-case query to perform against a running `Core`.
    type Query: Query<Self>;

    fn handle_event<E: Into<Self::Event>>(
        &mut self,
        event: E,
    ) -> AsyncResponse<Option<<Self::Event as Event<Self>>::Effect>>;

    fn handle_api_query(
        &self,
        query: Self::Query,
    ) -> AsyncResponse<<Self::Query as Query<Self>>::Response>;
}

