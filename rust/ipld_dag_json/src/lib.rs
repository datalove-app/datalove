//!

#![feature(specialization)]
#![warn(missing_docs)]
#![recursion_limit = "512"]

#[macro_use]
extern crate nom;

mod encoder;
mod tokenizer;

pub use encoder::Encoder;
