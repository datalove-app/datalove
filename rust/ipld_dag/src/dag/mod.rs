//!

pub mod float;
pub mod int;
pub mod key;
pub mod link;
pub mod token;

pub use crate::{
    cid::CID,
    dag::{float::Float, int::Int, key::Key, link::Link, token::Token},
    error::Error,
};
use indexmap::IndexMap;
use multibase::{Base, Encodable};
use erased_serde::serialize_trait_object;
use serde::{
    de::{Deserialize, Deserializer, Visitor},
    ser::{self, Serialize, SerializeMap, SerializeSeq, Serializer},
};
use std::marker::PhantomData;

fn resolve<I: Dag, O: Dag>(dag: &I) -> Result<&O, Error> {
    Err(Error::ExpectedLinkedDag)
}

pub type DagListIterator<T: Dag> = Iterator<Item = T>;
pub type DagMapIterator<T: Dag> = Iterator<Item = (Key, T)>;

// ?? impl Serialize for Dag
// ?? - would allow impl Dag to use it's own overridden serialize methods
//
// ?? what if RawDag was instead generic over a Dag trait object...??
/// ... currently (will) define behaviour that `Format`s can use to dig into `Dag`s.
pub trait Dag: erased_serde::Serialize {
    /// Returns a token representation of `self`.
    fn get_type(&self) -> Token;

    /// Unwraps the `Dag` as a `CID`.
    fn as_linked_cid(&self) -> Result<CID, Error> {
        Err(Error::ExpectedCID)
    }

    /// Unwraps the `Dag` as a linked `Dag`.
    fn as_linked_dag(&self) -> Result<Box<Dag>, Error> {
        Err(Error::ExpectedLinkedDag)
    }

    /// Unwraps the `Dag` as a List [`Iterator`].
    ///
    /// [`Iterator`]
    fn as_seq_iter(&self) -> Result<Box<DagListIterator<Self>>, Error>
    where
        Self: Sized,
    {
        Err(Error::ExpectedList)
    }

    /// Unwraps the `Dag` as a Map [`Iterator`].
    ///
    /// [`Iterator`]
    fn as_map_iter(&self) -> Result<Box<DagMapIterator<Self>>, Error>
    where
        Self: Sized,
    {
        Err(Error::ExpectedMap)
    }
}

///
// impl Serialize for Dag {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self.get_type() {
//             Token::Null => serializer.serialize_none(),
//             Token::Bool(b) => serializer.serialize_bool(b),
//             Token::Integer(int) => int.serialize(serializer),
//             Token::Float(float) => float.serialize(serializer),
//             Token::Str(s) => serializer.serialize_str(s),
//             Token::Bytes(bytes, base) => match base {
//                 None => serializer.serialize_bytes(bytes),
//                 Some(base) => serializer.serialize_str(&bytes.encode(base)),
//             },
//             Token::Link(cid, base) => match base {
//                 None => cid.serialize(serializer),
//                 Some(base) => serializer.serialize_str(&cid.encode(base)),
//             },
//             // TODO: is this right?
//             Token::LinkedData => {
//                 let boxed_dag = self.as_linked_dag().map_err(ser::Error::custom)?;
//                 boxed_dag.serialize(serializer)
//             }
//             Token::List(len) => {
//                 let mut seq_enc = serializer.serialize_seq(len)?;
//                 let iter = self.as_seq_iter().map_err(ser::Error::custom)?;
//                 for ref dag in iter {
//                     seq_enc.serialize_element(dag)?;
//                 }
//                 seq_enc.end()
//             }
//             Token::Map(len) => {
//                 let mut map_enc = serializer.serialize_map(len)?;
//                 let iter = self.as_map_iter().map_err(ser::Error::custom)?;
//                 for (ref key, ref value) in iter {
//                     map_enc.serialize_entry(key, value)?;
//                 }
//                 map_enc.end()
//             }
//             _ => Err(ser::Error::custom("invalid dag type")),
//         }
//     }
// }

serialize_trait_object!(Dag);

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

    /// Represents a list.
    List(Vec<Self>),

    /// Represents a map.
    /// Uses an [`IndexMap`] to preserve key order.
    ///
    /// [`IndexMap`]: https://docs.rs/indexmap/1.0.2/indexmap/map/struct.IndexMap.html
    Map(IndexMap<Key, Self>),

    /// Represents an IPLD [`Link`].
    ///
    /// [`Link`]: https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind
    Link(Link<Dag>),
}

impl Dag for RawDag {
    ///
    fn get_type(&self) -> Token {
        match self {
            RawDag::Null => Token::Null,
            RawDag::Bool(b) => Token::Bool(*b),
            RawDag::Integer(int) => Token::Integer(*int),
            RawDag::Float(float) => Token::Float(*float),
            RawDag::String(s) => Token::Str(&s),
            RawDag::ByteBuf(bytes, base) => Token::Bytes(bytes, *base),
            RawDag::Link(link) => match link {
                Link::CID(cid) => Token::Link(*cid, None),
                Link::Dag(dag) => Token::LinkedData,
            },
            RawDag::List(seq) => Token::List(Some(seq.len())),
            RawDag::Map(map) => Token::Map(Some(map.len())),
        }
    }

    ///
    fn as_seq_iter(&self) -> Result<Box<Iterator<Item = Box<RawDag>>>, Error> {
        match self {
            RawDag::List(seq) => {
                let iter = Box::new(seq.iter().map(Box::new));
                Ok(iter)
            }
            _ => Err(Error::Serialization("".to_string())),
        }
    }

    ///
    fn as_map_iter(&self) -> Result<Box<Iterator<Item = (&Key, Box<RawDag>)>>, Error> {
        match self {
            RawDag::Map(map) => {
                let iter = Box::new(map.iter().map(|dag| Box::new(dag)));
                Ok(iter)
            }
            _ => Err(Error::Serialization("".to_string())),
        }
    }
}

impl Serialize for RawDag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RawDag::Null => serializer.serialize_none(),
            RawDag::Bool(b) => serializer.serialize_bool(*b),
            RawDag::Integer(int) => int.serialize(serializer),
            RawDag::Float(float) => float.serialize(serializer),
            RawDag::String(s) => serializer.serialize_str(s),
            RawDag::ByteBuf(bytes, _) => serializer.serialize_bytes(bytes),
            RawDag::Link(link) => link.serialize(serializer),
            RawDag::List(seq) => {
                let mut seq_enc = serializer.serialize_seq(Some(seq.len()))?;
                for dag in seq {
                    seq_enc.serialize_element(&dag)?;
                }
                seq_enc.end()
            }
            RawDag::Map(map) => {
                let mut map_enc = serializer.serialize_map(Some(map.len()))?;
                for (key, value) in map.iter() {
                    map_enc.serialize_entry(key, value)?;
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

pub struct DagVisitor;

impl<'de> Visitor<'de> for DagVisitor {
    type Value = RawDag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an IPLD dag node")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::I8(v)))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::I16(v)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::I32(v)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::I64(v)))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::I128(v)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::U8(v)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::U16(v)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::U32(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::U64(v)))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Integer(Int::U128(v)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Float(Float::F32(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Float(Float::F64(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::String(v.into()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::String(v.into()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::String(v.into()))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::ByteBuf(v.into(), None))
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::ByteBuf(v.into(), None))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::ByteBuf(v.into(), None))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(RawDag::Null)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(RawDag::Null)
    }

    //     fn visit_seq<A>(self, seq: A) -> Result<Self::Value, std::error::Error>
    // where
    //     A: SeqAccess<'de>{}

    //     fn visit_map<A>(self, map: A) -> Result<Self::Value, std::error::Error>
    // where
    //     A: MapAccess<'de>{}

    //     fn visit_enum<A>(self, data: A) -> Result<Self::Value, std::error::Error>
    // where
    //     A: EnumAccess<'de>{}
}

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
