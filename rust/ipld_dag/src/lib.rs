//! adsf

#![feature(specialization)]
#![warn(missing_docs)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;

pub mod base;
mod cid;
mod dag;
mod error;
pub mod format;

pub use crate::cid::CID;
pub use dag::{Dag, Float, Int, Key, Token};
pub use error::Error;

pub use ::cid::{Codec, Prefix, Version};
pub use indexmap;
pub use multibase;
pub use multihash;
