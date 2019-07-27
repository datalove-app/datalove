//!

use serde::{de, ser};
use std::{error::Error as StdError, fmt};

///
#[derive(Debug)]
pub enum Error {
    /// Invalid `CID` `str`.
    InvalidCIDStr,

    ///
    CID(::cid::Error),

    ///
    Multibase(multibase::Error),

    ///
    Custom(String),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidCIDStr => "invalid CID str",
            Error::CID(ref err) => err.description(),
            Error::Multibase(ref err) => err.description(),
            Error::Custom(ref msg) => msg,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(ref msg) => write!(f, "{}", msg),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Custom(msg.to_string())
    }
}

impl From<::cid::Error> for Error {
    fn from(err: ::cid::Error) -> Self {
        Error::CID(err)
    }
}

impl From<multibase::Error> for Error {
    fn from(err: multibase::Error) -> Self {
        Error::Multibase(err)
    }
}
