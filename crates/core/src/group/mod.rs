mod event;
mod query;
mod types;

pub use types::*;
pub mod service {
    pub use super::event::*;
    pub use super::query::*;
}

use crate::dev::*;

pub trait GroupService<C: Core>
where
    Self: Service<C>,
{
}
