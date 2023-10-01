//! datalove-persona-core
//!
//! Each persona is managed by an evolving set of devices, each of which has a
//! keypair used to secure network communications and sign operations on behalf of the persona.
//!
//! ## Components:
//!
//! ### Device:
//! An [`ed25519`] keypair and a [`MerkleLog`] of signed messages, the first of which
//! is the signed public key itself.
//!
//! ## Stages:
//!
//! ### Genesis:
//! A device
//!
//! ### Continued operation:
//!

#![cfg_attr(not(feature = "std"), no_std)]
mod device;
mod error;
mod persona;
pub(crate) mod util;

pub use device::Device;
pub use error::Error;
pub use persona::{Persona, SignedOperation, Weight};

#[doc(hidden)]
pub use persona::State;
