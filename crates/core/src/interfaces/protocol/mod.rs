//! WASI adapters; defines services that we can provide to WASI runtime and
//! other nodes
//!
//! e.g.
//!     - given a std WASI config
//!         - produce a WASI instance (wrapped in a core type)
//!         - uses WASI/std syscall
//! interface
//!     - block service
//!         (incoming) ...
//!         (outgoing) ...
//!     - did / path resolution
//!         (incoming) ...
//!         (outgoing) ...
