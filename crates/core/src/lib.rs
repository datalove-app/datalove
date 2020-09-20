//!
//!

mod error;
mod interfaces;
mod types;

/// Exports of all core types and interfaces for convenience.
pub mod prelude {
    pub use crate::error::*;
    pub use crate::interfaces::*;
    pub use crate::types::*;
}

/// Dependency re-exports for convenience.
pub mod dev {
    pub use crate::prelude::*;

    // dependency re-exports
    pub use async_trait;
    pub use async_trait_ext;
    pub use cid::Cid;
    pub use crdts::CvRDT;
    pub use did_common::did::Did;
    pub use libp2p_core::{Multiaddr, PeerId};
    pub use libp2p_swarm::NetworkBehaviour;
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
}
