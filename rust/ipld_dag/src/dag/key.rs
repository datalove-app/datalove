use crate::{dag::Int, Error};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An IPLD Dag map key.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Key {
    /// Integer key
    Int(Int),

    /// String key
    String(String),
}

impl Serialize for Key {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Key::Int(int) => int.serialize(serializer),
            Key::String(s) => serializer.serialize_str(s),
        }
    }
}

impl std::str::FromStr for Key {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(s.into())
    }
}

impl From<String> for Key {
    #[inline]
    fn from(v: String) -> Self {
        Key::String(v)
    }
}

impl From<&str> for Key {
    #[inline]
    fn from(v: &str) -> Self {
        Key::String(v.into())
    }
}

impl<T: Into<Int>> From<T> for Key {
    #[inline]
    fn from(v: T) -> Key {
        Key::Int(v.into())
    }
}
