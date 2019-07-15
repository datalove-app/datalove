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

mod de;
pub mod float;
pub mod int;
pub mod key;
mod ser;
pub mod token;

pub use crate::dag::{float::Float, int::Int, key::Key, token::Token};
use crate::{base::Base, cid::CID};
use indexmap::IndexMap;
use std::{borrow, iter};

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

    /// Represents some bytes, and an optional desired [`multibase::Base`] if intended to be encoded as a string.
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

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Dag {
                fn from(n: $ty) -> Self {
                    Dag::Integer(n.into())
                }
            }
        )*
    };
}

macro_rules! from_float {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Dag {
                fn from(n: $ty) -> Self {
                    Dag::Float(n.into())
                }
            }
        )*
    };
}

from_integer! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 }
from_float! { f32 f64 }

impl<'a> From<&'a str> for Dag {
    fn from(v: &str) -> Self {
        Dag::String(v.to_string())
    }
}

impl<'a> From<borrow::Cow<'a, str>> for Dag {
    fn from(v: borrow::Cow<'a, str>) -> Self {
        Dag::String(v.into_owned())
    }
}

impl<T> From<Option<T>> for Dag
where
    T: Into<Dag>,
{
    fn from(o: Option<T>) -> Self {
        match o {
            None => Dag::Null,
            Some(t) => t.into(),
        }
    }
}

impl<'a, T> From<&'a [T]> for Dag
where
    T: Clone + Into<Dag>,
{
    /// Convert a slice to a `Dag`.
    fn from(v: &'a [T]) -> Self {
        Dag::List(v.iter().cloned().map(Into::into).collect())
    }
}

impl<T> iter::FromIterator<T> for Dag
where
    T: Into<Dag>,
{
    /// Convert an iteratable type to a `Dag`.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Dag::List(iter.into_iter().map(Into::into).collect())
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
