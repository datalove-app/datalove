use quick_error::quick_error;
use serde::ser;
use std::fmt::Display;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Invalid {
            description("Invalid")
        }
    }
}

impl ser::Error for Error {
    // #[cold]
    fn custom<T: Display>(_msg: T) -> Error {
        Error::Invalid
    }
}
