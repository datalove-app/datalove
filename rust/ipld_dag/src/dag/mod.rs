//! Two ways of using the Encoding/Decoding traits
//!     - with your own types
//!         - derive `Serialize` for your types
//!         - annotate bytes you include in your types (if you dont want the custom Encoder behaviour)
//!             - needs special consideration due to base encoding
//!         - annotate CIDs you include in your types
//!             - needs special consideration b/c:
//!                 - it doesnt map to the data model
//!                 - serialize behaviour is dictated by serializer, not type
//!             - ... define your own CID type
//!             - ... redirect CID serialize behaviour to an Encoder specific function
//!     - with the provided abstract Dag enum
//!         - each variant already configured to

pub mod float;
pub mod int;
pub mod key;
// pub mod link;
pub mod token;

pub use crate::{
    base::Base,
    cid::CID,
    dag::{float::Float, int::Int, key::Key, token::Token},
    error::Error,
    format::{Encoder, Format},
};
use indexmap::IndexMap;
use serde::{
    de::{Deserialize, Deserializer, Visitor},
    ser::{self, Serialize, SerializeMap, SerializeSeq, Serializer},
};
use serde_bytes::ByteBuf;

// fn resolve<I: Dag, O: Dag>(dag: &I) -> Result<&O, Error> {
//     Err(Error::ExpectedLinkedDag)
// }

// fn from_str() {}

// fn from_reader() {}

// fn from_tokens() {}

// pub enum Dag2<R: std::io::Read, D: Dag> {
//     Block(R),
//     Link(CID),
//     Dag(D),
// }

// TODO: latest updates:
// get rid of Link and Dag trait
// instead:
//  - keep Encode(r) traits
//  - write abstract `Dag` enum (flattens link variants into it)

// ?? impl Serialize for Dag
// ?? - would allow impl Dag to use it's own overridden serialize methods
//
// ?? what if Dag was instead generic over a Dag trait object...??
/// ... currently (will) define behaviour that `Format`s can use to dig into `Dag`s.
///
/// Represents an abtract IPLD [Dag](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md). Useful if decoding unknown IPLD.
///
/// An `indexmap` is used as the map implementation in order to preserve key order.
#[derive(Clone, Debug, From)]
pub enum Dag {
    /// Represents a null value.
    Null,

    /// Represents a boolean.
    Bool(bool),

    /// Represents an integer.
    Integer(Int),

    /// Represents a float.
    Float(Float),

    /// Represents a string.
    String(String),

    /// Represents some bytes, and an optional desired [`multibase::Base`] if encoded as a string.
    ///
    /// [`multibase::Base`]: https://docs.rs/multibase/0.6.0/multibase/enum.Base.html
    ByteBuf(Vec<u8>, Option<Base>),

    /// Represents a list of `Dag` nodes.
    List(Vec<Self>),

    /// Represents a map of `Dag` nodes.
    /// Uses an [`IndexMap`] to preserve key order.
    ///
    /// [`IndexMap`]: https://docs.rs/indexmap/1.0.2/indexmap/map/struct.IndexMap.html
    Map(IndexMap<Key, Self>),

    /// Represents an IPLD [`CID`] [`Link`].
    ///
    /// [`CID`]
    /// [`Link`]: https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind
    Link(CID, Option<Box<Dag>>),
}

impl Serialize for Dag {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Dag::Null => serializer.serialize_none(),
            Dag::Bool(b) => serializer.serialize_bool(*b),
            Dag::Integer(int) => int.serialize(serializer),
            Dag::Float(float) => float.serialize(serializer),
            Dag::String(s) => serializer.serialize_str(s),

            Dag::ByteBuf(buf, base) => <S as Encoder>::encode_bytes(serializer, buf, *base),
            // Dag::Link(cid, _) => <S as Encoder>::encode_link(serializer, cid),
            Dag::Link(cid, _) => cid.serialize(serializer),
            // Dag::ByteBuf(buf, base) => serializer.encode_bytes(buf, *base),
            // Dag::Link(cid, _) => serializer.encode_link(cid),
            Dag::List(seq) => {
                let mut seq_enc = serializer.serialize_seq(Some(seq.len()))?;
                for dag in seq {
                    seq_enc.serialize_element(dag)?;
                }
                seq_enc.end()
            }
            Dag::Map(map) => {
                let mut map_enc = serializer.serialize_map(Some(map.len()))?;
                for (key, value) in map.iter() {
                    map_enc.serialize_key(key)?;
                    map_enc.serialize_value(value)?;
                }
                map_enc.end()
            }
        }
    }
}

// impl<'de> Deserialize<'de> for CID {
//     fn deserialize<D>(deserializer: D) -> Result<CID, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         Ok(CID::new())
//     }
// }

// pub struct DagVisitor;

// impl<'de> Visitor<'de> for DagVisitor {
//     type Value = Dag;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("an IPLD dag node")
//     }

//     fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Bool(v))
//     }

//     fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::I8(v)))
//     }

//     fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::I16(v)))
//     }

//     fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::I32(v)))
//     }

//     fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::I64(v)))
//     }

//     fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::I128(v)))
//     }

//     fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::U8(v)))
//     }

//     fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::U16(v)))
//     }

//     fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::U32(v)))
//     }

//     fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::U64(v)))
//     }

//     fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Integer(Int::U128(v)))
//     }

//     fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Float(Float::F32(v)))
//     }

//     fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Float(Float::F64(v)))
//     }

//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::String(v.into()))
//     }

//     fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::String(v.into()))
//     }

//     fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::String(v.into()))
//     }

//     fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::ByteBuf(v.into(), None))
//     }

//     fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::ByteBuf(v.into(), None))
//     }

//     fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::ByteBuf(v.into(), None))
//     }

//     fn visit_none<E>(self) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Null)
//     }

//     fn visit_unit<E>(self) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(Dag::Null)
//     }

//     fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         Ok(Dag::Null)
//     }

//     //     fn visit_seq<A>(self, seq: A) -> Result<Self::Value, std::error::Error>
//     // where
//     //     A: SeqAccess<'de>{}

//     //     fn visit_map<A>(self, map: A) -> Result<Self::Value, std::error::Error>
//     // where
//     //     A: MapAccess<'de>{}

//     //     fn visit_enum<A>(self, data: A) -> Result<Self::Value, std::error::Error>
//     // where
//     //     A: EnumAccess<'de>{}
// }

// TODO: use serde_test methods to test the cases
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
