//!
//! TODO: define service-specific traits, use case-specific Events and Queries
//! - each Event (and Query) should have a marker trait
//!     - should convert to/from all other related services for that use case
//!     - C::Event should convert into "use case entry" Event, from "use case exit" Effect
//!
//! TODO: begin simple implementation
//! - start with Keystore driver, CLI interface
//! - start with Root (? and basic IPFS?) service
//! - Core should:
//!     - ? represent the external API of the entire app
//!     - ? each event should map to an API use case
//!

#[macro_use]
extern crate lazy_static;

// mod admin;
// mod appvm;
mod crypto;
mod drivers;
mod error;
mod group;
// mod ipfs;
// mod ledger;
mod peer;
mod service;
mod time;
mod user;

/// Exports of all core types and interfaces.
pub mod prelude {
    pub use crate::Core;
    // pub use crate::admin::*;
    // pub use crate::appvm::*;
    pub use crate::crypto::*;
    pub use crate::drivers::*;
    pub use crate::error::*;
    pub use crate::group::*;
    // pub use crate::ipfs::*;
    // pub use crate::ledger::*;
    pub use crate::peer::*;
    pub use crate::service::*;
    pub use crate::time::*;
    pub use crate::user::*;
}

/// Core dependency re-exports for convenience.
pub mod dev {
    pub use crate::prelude::*;

    pub use cid::Cid;
    // pub use crdts::FunkyCvRDT;
    // pub use did_common::did::Did;
    pub use ::ed25519::{
        signature::{Signer, Verifier},
        Signature,
    };
    pub use futures::prelude::*;
    pub use libp2p_core::{identity as libp2p_identity, Multiaddr, PeerId};
    pub use libp2p_swarm::NetworkBehaviour;
    pub use num_traits::{One, Unsigned};
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub use uuid::Uuid;
}

use crate::prelude::*;

///
/// Used as the go-between for Drivers and Services:
/// - Services use the `Core` to:
///     - directly access and call other services
///     - abstract away platform-/dependency-specific details
///     - ? register event hooks (so the core can auto-route events to particular services)
///
/// - Drivers use the `Core` to:
///     - abstract away knowledge of which services require which events
///     - ? register query/effect hooks?
///
/// APIs (embedded, shell, ?web)
/// - hold a handle to the Core
/// - pass messages to/from Core
///
/// **implementation requirements**
/// -
pub trait Core: Service<Self>
// where
//     Self: EventHandler<Self, CoreEvent>,
//     Self: QueryHandler<Self, ApiQuery>,
//     Self: QueryHandler<Self, EnvQuery>,
{
    // drivers
    type Keystore: Keystore<Self>;

    // services
    // type Ledger: LedgerService<Self>;
    // type User: UserService<Self>;
    // type Time: TimeService<Self>;

    /// A persistable, use-case event.
    type Event: Event<Self>;
    // + From<<Self::Keystore as Driver<Self>>::Event>;

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

// #[derive(Debug, Deserialize, Serialize)]
// pub enum CoreEvent {}

// impl<C: Core> service::Event<C> for CoreEvent {
//     type Effect = ();

//     fn id(&self) -> &Uuid {
//         &NIL_UUID
//     }
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub enum ApiQuery {}

// impl<C: Core> service::Query<C> for ApiQuery {
//     type Response = ();
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub enum EnvQuery {}

// impl<C: Core> service::Query<C> for EnvQuery {
//     type Response = ();
// }
