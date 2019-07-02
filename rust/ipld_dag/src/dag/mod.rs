//!

pub mod float;
pub mod int;
pub mod key;
// pub mod link;
pub mod token;

pub use crate::{
    cid::CID,
    dag::{float::Float, int::Int, key::Key, token::Token},
    error::Error,
    format::{Encode, EncodeList, EncodeMap, Encoder, Format},
};
use erased_serde::serialize_trait_object;
use indexmap::IndexMap;
use multibase::{Base, Encodable};
use serde::{
    de::{Deserialize, Deserializer, Visitor},
    ser::{self, Serialize, SerializeMap, SerializeSeq, Serializer},
};

// fn resolve<I: Dag, O: Dag>(dag: &I) -> Result<&O, Error> {
//     Err(Error::ExpectedLinkedDag)
// }

// fn from_str() {}

// fn from_reader() {}

// fn from_tokens() {}

// TODO: latest updates:
// get rid of Link and Dag trait
// instead:
//  - keep Encode(r) traits
//  - write abstract `Dag` enum (flattens link variants into it)

// ?? impl Serialize for Dag
// ?? - would allow impl Dag to use it's own overridden serialize methods
//
// ?? what if RawDag was instead generic over a Dag trait object...??
/// ... currently (will) define behaviour that `Format`s can use to dig into `Dag`s.
pub trait Dag: Encode + Serialize {
    /// Returns a token representation of `self`.
    fn get_type(&self) -> Token;

    // ///
    // fn as_list_iter(&self) -> Result<Box<Iterator<Item = Self>>, Error>;

    // ///
    // fn as_map_iter(&self) -> Result<Box<Iterator<Item = (Key, Self)>>, Error>;

    // ///
    // fn get_cid(&self) -> Result<CID, Error>;

    // ///
    // fn resolve(&self, path: &str) -> Result<&Self, Error>;
}

// pub enum Dag2<R: std::io::Read, D: Dag> {
//     Block(R),
//     Link(CID),
//     Dag(D),
// }

// pub type DagListIterator<T: Dag> = Iterator<Item = T>;
// pub type DagMapIterator<T: Dag> = Iterator<Item = (Key, T)>;

// impl Encode for RawDag {
//     ///
//     fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
//     where
//         E: Encoder,
//         <E as serde::Serializer>::Error: Into<Error>,
//     {
//         match self.get_type() {
//             Token::Null => encoder.serialize_none(),
//             Token::Bool(b) => encoder.serialize_bool(b),
//             Token::Integer(ref int) => int.serialize(encoder),
//             Token::Float(ref float) => float.serialize(encoder),
//             Token::Str(s) => encoder.serialize_str(s),
//             Token::Bytes(bytes, _base) => encoder.serialize_bytes(bytes),
//             Token::Link(ref cid, base) => encoder.encode_link_cid(cid, base),
//             Token::List(len) => {
//                 let mut list_enc = encoder.encode_list(len)?;
//                 for dag in self.as_list_iter()? {
//                     list_enc.encode_element(&dag)?;
//                 }
//                 list_enc.end()
//             }
//             Token::Map(len) => {
//                 let mut map_enc = encoder.encode_map(len)?;
//                 for (key, value) in self.as_map_iter()? {
//                     map_enc.encode_key(&key)?;
//                     map_enc.encode_value(&value)?;
//                 }
//                 map_enc.end()
//             }
//             _ => Err(serde::ser::Error::custom("")),
//         }
//     }
// }

/// Represents an abtract IPLD [Dag](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md). Useful if decoding unknown IPLD.
///
/// An `indexmap` is used as the map implementation in order to preserve key order.
#[derive(From)]
pub enum RawDag {
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

    /// Represents a list of `RawDag` nodes.
    List(Vec<Self>),

    /// Represents a map of `RawDag` nodes.
    /// Uses an [`IndexMap`] to preserve key order.
    ///
    /// [`IndexMap`]: https://docs.rs/indexmap/1.0.2/indexmap/map/struct.IndexMap.html
    Map(IndexMap<Key, Self>),

    /// Represents an IPLD [`CID`] [`Link`].
    ///
    /// [`CID`]
    /// [`Link`]: https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind
    Link(CID),

    /// Represents a linked IPLD [`Dag`].
    ///
    LinkedDag(Box<Self>),
}

impl Encode for RawDag {
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
        <E as serde::Serializer>::Error: Into<Error>,
    {
        match self {
            RawDag::Null => encoder.serialize_none(),
            RawDag::Bool(b) => encoder.serialize_bool(*b),
            RawDag::Integer(int) => int.serialize(encoder),
            RawDag::Float(float) => float.serialize(encoder),
            RawDag::String(s) => encoder.serialize_str(s),
            RawDag::ByteBuf(bytes, base) => encoder.encode_bytes(bytes, *base),
            RawDag::Link(cid) => Encode::encode(cid, encoder),
            RawDag::List(seq) => {
                let mut seq_enc = encoder.encode_list(Some(seq.len()))?;
                for dag in seq {
                    seq_enc.encode_element(&dag)?;
                }
                seq_enc.end()
            }
            RawDag::Map(map) => {
                let mut map_enc = encoder.encode_map(Some(map.len()))?;
                for (key, value) in map.iter() {
                    map_enc.encode_entry(key, value)?;
                }
                map_enc.end()
            }
        }
    }
}

// impl Dag for RawDag {
//     ///
//     fn get_type(&self) -> Token {
//         match self {
//             RawDag::Null => Token::Null,
//             RawDag::Bool(b) => Token::Bool(*b),
//             RawDag::Integer(int) => Token::Integer(*int),
//             RawDag::Float(float) => Token::Float(*float),
//             RawDag::String(s) => Token::Str(&s),
//             RawDag::ByteBuf(bytes, base) => Token::Bytes(bytes, *base),
//             RawDag::Link(cid, base) => Token::Link(*cid, *base),
//             RawDag::List(list) => Token::List(Some(list.len())),
//             RawDag::Map(map) => Token::Map(Some(map.len())),
//         }
//     }

//     ///
//     fn as_list_iter(&self) -> Result<Box<DagListIterator<Self>>, Error> {}

//     ///
//     fn as_map_iter(&self) -> Result<Box<DagMapIterator<Self>>, Error> {}
// }

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
//     type Value = RawDag;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("an IPLD dag node")
//     }

//     fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Bool(v))
//     }

//     fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::I8(v)))
//     }

//     fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::I16(v)))
//     }

//     fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::I32(v)))
//     }

//     fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::I64(v)))
//     }

//     fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::I128(v)))
//     }

//     fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::U8(v)))
//     }

//     fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::U16(v)))
//     }

//     fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::U32(v)))
//     }

//     fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::U64(v)))
//     }

//     fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Integer(Int::U128(v)))
//     }

//     fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Float(Float::F32(v)))
//     }

//     fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Float(Float::F64(v)))
//     }

//     fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::String(v.into()))
//     }

//     fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::String(v.into()))
//     }

//     fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::String(v.into()))
//     }

//     fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::ByteBuf(v.into(), None))
//     }

//     fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::ByteBuf(v.into(), None))
//     }

//     fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::ByteBuf(v.into(), None))
//     }

//     fn visit_none<E>(self) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Null)
//     }

//     fn visit_unit<E>(self) -> Result<Self::Value, E>
//     where
//         E: std::error::Error,
//     {
//         Ok(RawDag::Null)
//     }

//     fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         Ok(RawDag::Null)
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

// impl<'a, D: IpldDag> Encode for Dag<'a, D> {
//     fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
//     where
//         E: FormatEncoder,
//     {
//         match self {
//             Dag::Null => encoder.encode_null(),
//             Dag::Bool(b) => encoder.encode_bool(*b),
//             Dag::Integer(ref int) => encoder.encode_int(int),
//             Dag::Float(ref float) => encoder.encode_float(float),
//             Dag::Str(s) => encoder.encode_str(s),
//             Dag::Bytes(bytes) => encoder.encode_bytes(bytes),
//             Dag::Link(ref link) => encoder.encode_link(link),
//             Dag::List(ref list) => {
//                 let mut list_enc = encoder.encode_list(Some(list.len()))?;
//                 for dag in list {
//                     list_enc.encode_element(dag)?;
//                 }
//                 list_enc.end()
//             }
//             Dag::Map(ref map) => {
//                 let mut map_enc = encoder.encode_map(Some(map.len()))?;
//                 for (key, value) in map.iter() {
//                     map_enc.encode_key(key)?;
//                     map_enc.encode_value(value)?;
//                 }
//                 map_enc.end()
//             }
//         }
//     }
// }

// impl<'de, 'a: 'de, D: IpldDag> Decode<'de> for Dag<'a, D> {
//     fn decode<D>(decoder: D) -> Result<Self, D::Error>
//     where
//         D: FormatDecoder<'de>,
//     {
//         Ok(Dag::Null)
//     }
// }

// TODO: use serde_test methods to test the cases
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
