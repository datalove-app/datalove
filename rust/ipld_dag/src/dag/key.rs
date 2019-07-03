use crate::{dag::Int, Error};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An IPLD Dag map key.
#[derive(Eq, From, Hash, PartialEq)]
pub enum Key {
    Integer(Int),
    String(String),
}

// impl Encode for Key {
//     #[inline]
//     fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
//     where
//         E: Encoder,
//         <E as serde::Serializer>::Error: From<Error>,
//     {
//         match self {
//             Key::Integer(int) => int.encode(encoder),
//             Key::String(s) => s.encode(encoder),
//         }
//     }
// }

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
