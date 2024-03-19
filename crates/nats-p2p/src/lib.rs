//! listen for client connections
//!     each has subs
//!     each sends commands
//! to iroh actor
//! route iroh doc/blob/msg events to clients

mod error;
mod iroh;

pub mod cluster;
pub mod config;
pub mod core;
pub mod flow;
pub mod jetstream;
pub mod server;

pub use crate::{
    config::Config,
    core::{
        codec::Codec,
        session::{NetworkSplit, Split},
        ClientOp, ServerOp, Subject,
    },
    error::Error,
    server::Server,
};

#[doc(hidden)]
pub use server::server::{run_basic_server, run_server, run_server_with_port};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Notes and other design docs.
/// include_mmd!("docs/ACTORS.mmd")
/// include_mmd!("docs/FLOW.mmd")
pub mod docs {}
