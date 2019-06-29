//!

#![warn(missing_docs)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate serde;

mod cid;
mod dag;
mod error;
mod format;

pub use crate::cid::CID;
pub use dag::{Dag, Float, Int, Key, Link, RawDag, Token};
pub use error::Error;

pub use ::cid::{Codec, Prefix, Version};
pub use indexmap;
pub use multibase;
pub use multihash;
