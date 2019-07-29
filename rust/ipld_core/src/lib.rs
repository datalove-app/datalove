//! adsf

#![feature(associated_type_defaults)]
#![feature(specialization)]
#![warn(missing_docs)]

#[macro_use]
extern crate derive_more;

mod cid;
// mod cownode;
mod error;
pub mod format;
mod freenode;
pub mod multibase;
mod node;

pub use crate::cid::CID;
pub use crate::multibase::Base;
// pub use cownode::CowNode;
pub use error::Error;
pub use format::Token;
pub use freenode::FreeNode;
pub use node::{Float, Int, Key, Kind, Node};

pub use ::cid::{Codec, Prefix, Version};
pub use indexmap;
pub use multihash;
