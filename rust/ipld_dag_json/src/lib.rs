//!

#![feature(specialization)]
#![warn(missing_docs)]
#![recursion_limit = "512"]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_trace;

mod encoder;
mod tokenizer;

pub use encoder::Encoder;
