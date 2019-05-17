use quick_error::quick_error;
use serde::{de, ser};
use std::fmt::Display;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DeserializationError(err: String) {
            description(err)
        }

        SerializationError(err: String) {
            description(err)
        }
        InvalidVariant {
            description("Failed to serialize variant to atom or string")
        }
        InvalidStructName {
            description("Failed to serialize struct name to atom or string")
        }
        InvalidMap {
            description("Failed to serialize map to NIF map")
        }
        InvalidStruct {
            description("Failed to serialize struct to NIF struct")
        }
    }
}

impl ser::Error for Error {
    // #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error::SerializationError(msg.to_string())
    }
}

impl de::Error for Error {
    // #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        Error::DeserializationError(msg.to_string())
    }
}
