// TODO: fix this to support CID errors

use serde::{de, ser};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Serialization(String),
    Deserialization(String),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Serialization(ref string) => string,
            Error::Deserialization(ref string) => string,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Serialization(ref string) => write!(f, "{}", string),
            Error::Deserialization(ref string) => write!(f, "{}", string),
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
        Error::Serialization(msg.to_string())
    }
}

// impl<W, F> From<<ser::Serializer<W, F> as ser::Serializer>::Error> for Error {
//     fn from(err: ser::Error) -> Error {
//         Error::Encode(err.description())
//     }
// }
