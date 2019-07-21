use ipld_core::Error;
use serde_json::Error as JsonError;
use std::{error::Error as StdError, fmt::Display};

pub fn j2i_ser_err(err: JsonError) -> Error {
    Error::Serialization(format!("{}", err))
}

pub fn j2i_de_err(err: JsonError) -> Error {
    Error::Deserialization(format!("{}", err))
}

pub fn key_must_be_a_string() -> Error {
    Error::Serialization("key must be a string".to_string())
}

// pub fn s2i_ser_err(err: serde::ser::Error) -> Error {

// }
