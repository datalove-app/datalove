use thiserror::Error;

///
#[derive(Debug, Error)]
pub enum Error {
    #[error("Root error: {0}")]
    Root(#[from] RootError),
}

///
#[derive(Debug, Error)]
pub enum RootError {}
