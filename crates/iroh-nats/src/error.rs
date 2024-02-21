use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("SSH key error: {0}")]
    Key(#[from] ssh_key::Error),

    #[error("General iroh error: {0}")]
    Iroh(anyhow::Error),

    #[error("Decode error: {0}")]
    Decode(#[from] tokio_util::codec::AnyDelimiterCodecError),
    // #[error("Iroh error: {0}")]
    // Iroh(#[from] iroh::Error),
}

impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        match e {
            Error::Io(e) => e,
            e => io::Error::other(e),
        }
    }
}
