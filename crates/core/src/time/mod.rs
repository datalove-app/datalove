//!
//! defines a distributed vector clock

mod event;
mod query;
mod types;

pub use types::*;
pub mod service {
    pub use super::event::*;
    pub use super::query::*;
}

use crate::dev::*;

///
pub trait TimeService<C: Core> {
    type Clock: Clock;
}
