use crate::drivers::KeystoreError;
use thiserror::Error;

///
#[derive(Debug, Error)]
pub enum Error {
    #[error("Keystore error: {0}")]
    Keystore(#[from] KeystoreError),
}
