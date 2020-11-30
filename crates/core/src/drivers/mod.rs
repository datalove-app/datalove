//! network (yggdrasil, libp2p), fs (secondary port adapters)
//!
//! pattern:
//!     - manages a series of connections
//!     (incoming (external/from OS))
//!         - subs to a `Stream` from
//!         - publishes msgs to a subscribing `Sink` defined in src/protocol/
//! ...
//!     (outgoing (external/to OS)) directly call dependencies
//!
//! i.e. perform ops, could also set up queue services for ops
//!     - defines a config store service
//!     - defines a block service
//!     - defines an yggdrasil (and/or packet) service
//!     - defines a signing/verification service
//!     - defines an encryption/decryption service

mod keystore;

pub use keystore::{Error as KeystoreError, Keystore};

use crate::prelude::*;

pub enum DriverType {
    Configstore,
    Datastore,
    Eventstore,
    Keystore,
    Network,
    Nix,
}

///
/// TODO subtraits **SHOULD** be tailored to **GENERAL** core/service needs
///     **SHOULD** be implemented per-platform
///     **SHOULD**
///     **SHOULD NOT** handle events/effects/queries specific to only one use case/service
///     **SHOULD NOT** be the one to decide to ignore external events
///     - i.e. should intelligently manage caching, GC, refcounts
///     - e.g. resource management specific to the device
///         - any effects that exceed resources should either be automatically handled or be translated to another Core event
/// TODO **SHOULD NOT** contain
pub trait Driver<C: Core>: Service<C> {
    const TYPE: DriverType;

    /// an external event, like:
    ///     - STDIN
    ///     - receiving a client command/query
    ///     - ? a detected file/directory change
    ///     - ?? receiving a packet
    ///     - ?? receiving a new build artifact
    ///     - ?? a db insert, or table creation/deletion
    type Event: Event<C>;

    /// an effect to perform, like:
    ///     - writing to STDOUT
    ///     - sending a client response
    ///     - writing to a file, watch a file
    ///     - sending a packet, connect to an IP
    ///     - write/indexing a build artifact
    ///     - performing a db insert
    type Effect: Effect;

    // /// a query against the external environment, like:
    // ///     - ?? awaiting STDIN
    // ///     - ?? awaiting input from a client
    // ///     - reading a file's contents
    // ///     - state of the network's connections, connected/listening IPs
    // ///     - ??
    // ///     - querying the db
    // type Query: From<C::EnvQuery>;

    /// Adapts an external event for a particular [`crate::service::Service`].
    fn handle_event(&mut self, event: Self::Event);

    /// Applies a [`crate::Core`]'s effect to the external environment.
    fn apply_effect(&mut self, effect: Self::Effect);

    // ///
    // /// Called by [`crate::Core`] to query against the external environment.
    // fn handle_query(&self, query: Self::Query);
}
