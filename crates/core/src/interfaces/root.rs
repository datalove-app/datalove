//! The `Root`

use crate::dev::*;

///
pub trait Root {
    type InitParams;
    type LoadParams;

    /// Initializes a new `Root`.
    fn init(params: Self::InitParams) -> Result<Self, RootError>;

    /// Loads an already established `Root`.
    fn load(params: Self::LoadParams) -> Result<Self, RootError>;

    ///
    fn destroy(self) -> Result<(), RootError>;
}
