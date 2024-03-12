//! listen for client connections
//!     each has subs
//!     each sends commands
//! to iroh actor
//! route iroh doc/blob/msg events to clients

pub(crate) mod cluster;
pub(crate) mod config;
pub(crate) mod core;
mod error;
mod iroh;
// pub(crate) mod jetstream;
mod server;

pub use crate::{config::Config, core::Subject, error::Error, server::Server};

#[doc(hidden)]
pub use server::server::{run_basic_server, run_server, run_server_with_port};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Notes and other design docs.
/// include_mmd!("docs/ACTORS.mmd")
/// include_mmd!("docs/FLOW.mmd")
pub mod docs {}
