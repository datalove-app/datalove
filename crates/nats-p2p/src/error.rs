use std::io;

use ractor::ActorProcessingErr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Configuration error: {0}")]
    Config(&'static str),

    #[error("SSH key error: {0}")]
    Key(#[from] ssh_key::Error),

    #[error("Message codec error: {0}")]
    Codec(anyhow::Error),

    #[error("Subject error: {0}")]
    Subject(&'static str),

    #[error("{0}")]
    Server(anyhow::Error),

    #[error("General iroh error: {0}")]
    Iroh(anyhow::Error),
}

impl Error {
    pub fn codec(e: impl Into<anyhow::Error>) -> Self {
        Self::Codec(e.into())
    }

    pub fn server(msg: &str, e: impl Into<anyhow::Error>) -> Self {
        let err = e.into();
        tracing::error!("{}: {:?}", msg, err);
        Self::Server(err)
    }

    pub fn actor(err: &str) -> Self {
        Self::Server(anyhow::anyhow!(err.to_string()))
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        match e {
            Error::Io(e) => e,
            e => io::Error::other(e),
        }
    }
}

impl From<ractor::ActorProcessingErr> for Error {
    fn from(e: ractor::ActorProcessingErr) -> Self {
        Self::server("actor lifecycle method err:", anyhow::anyhow!(e))
    }
}
impl From<ractor::SpawnErr> for Error {
    fn from(e: ractor::SpawnErr) -> Self {
        Self::server("actor spawn err:", e)
    }
}
impl<T: Send + Sync + 'static> From<ractor::MessagingErr<T>> for Error {
    fn from(e: ractor::MessagingErr<T>) -> Self {
        Self::server("actor send/recv err:", e)
    }
}
