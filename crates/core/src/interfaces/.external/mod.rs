//! network (yggdrasil, libp2p), fs (secondary port adapters)
//!
//! pattern:
//!     - manages a series of connections
//!     (incoming)
//!         - subs to a `Stream` from
//!         - publishes msgs to a subscribing `Sink` defined in src/protocol/
//! ...
//!     (outgoing) directly call dependencies
//!
//! i.e. perform ops, could also set up queue services for ops
//!     - defines a config store service
//!     - defines a block service
//!     - defines an yggdrasil (and/or packet) service
//!     - defines a signing/verification service
//!     - defines an encryption/decryption service
