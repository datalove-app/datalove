use crate::*;
use snafu::prelude::*;

#[derive(core::fmt::Debug, Snafu)]
pub enum Error {
    #[cfg(feature = "file")]
    #[snafu(display(""))]
    Io {
        #[snafu(source(from(std::io::Error, Into::into)))]
        source: std::io::Error,
    },

    #[snafu(display(""))]
    MalformedData,

    #[snafu(display(""))]
    InvalidSigningKeypair,

    #[snafu(display(""))]
    InvalidEncryptionKeypair,

    #[snafu(display(""))]
    InvalidSignature,

    #[snafu(display(""))]
    InvalidDerivationPath,

    #[snafu(display(""))]
    EncryptionError,

    #[snafu(display(""))]
    DecryptionError,

    #[snafu(display(""))]
    TransformError,
}

impl Error {
    pub fn io(source: std::io::Error) -> Self {
        Self::Io { source }
    }
}
