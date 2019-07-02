// TODO: fix this to support CID errors

use serde::{de, ser};
use std::{error::Error as StdError, fmt};

///
#[derive(Debug)]
pub enum Error {
    ExpectedCID,
    ExpectedLinkedDag,
    ExpectedList,
    ExpectedMap,
    InvalidType,
    CID(String),
    Multibase(String),
    Serialization(String),
    Deserialization(String),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidType => "invalid Dag type",
            Error::ExpectedCID => "expected CID",
            Error::ExpectedLinkedDag => "expected linked dag",
            Error::ExpectedList => "expected list",
            Error::ExpectedMap => "expected map",
            Error::CID(ref s) => s,
            Error::Multibase(ref s) => s,
            Error::Serialization(ref s) => s,
            Error::Deserialization(ref s) => s,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Serialization(ref s) => write!(f, "{}", s),
            Error::Deserialization(ref s) => write!(f, "{}", s),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Serialization(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Deserialization(msg.to_string())
    }
}

impl From<multibase::Error> for Error {
    fn from(err: multibase::Error) -> Self {
        Error::Multibase(err.description().into())
    }
}

impl From<::cid::Error> for Error {
    fn from(err: ::cid::Error) -> Self {
        Error::CID(err.description().into())
    }
}

// impl<W, F> From<<ser::Serializer<W, F> as ser::Serializer>::Error> for Error {
//     fn from(err: ser::Error) -> Error {
//         Error::Encode(err.description())
//     }
// }
