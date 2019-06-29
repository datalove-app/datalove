use crate::dag::Int;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An IPLD Dag map key.
#[derive(Eq, From, Hash, PartialEq)]
pub enum Key {
    Integer(Int),
    String(String),
}

impl Serialize for Key {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Key::Integer(int) => int.serialize(serializer),
            Key::String(s) => serializer.serialize_str(s),
        }
    }
}
