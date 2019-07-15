use crate::error::Error;
use serde::{Serialize, Serializer};

/// Float wrapper
#[derive(Clone, Debug, From, PartialEq)]
pub enum Float {
    /// `f32`
    F32(f32),

    /// `f64`
    F64(f64),
}

impl Serialize for Float {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Float::F32(num) => serializer.serialize_f32(num),
            Float::F64(num) => serializer.serialize_f64(num),
        }
    }
}

impl std::str::FromStr for Float {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(Error::Custom("".into()))
    }
}
