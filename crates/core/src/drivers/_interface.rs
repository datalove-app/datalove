//!
//! "driving" adapters
//!     - wraps a port/driver exposed by core
//!     - translates commands into core method calls
//!
//! in our case:
//!     - converts API (shell, web) events into core service events
//!     - pass them to core (? or services) directly
//!

use crate::prelude::*;

///
pub trait Interface<C: Core>: Driver<C> {}
