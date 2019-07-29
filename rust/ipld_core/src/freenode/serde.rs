use crate::{format::Encoder, multibase::Base, FreeNode};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for FreeNode {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            // encoder-specific
            FreeNode::ByteBuf(buf, base) => serializer.encode_bytes(buf, *base),
            FreeNode::Link(cid, _) => serializer.encode_link(cid),

            // standard
            FreeNode::Null => serializer.serialize_none(),
            FreeNode::Bool(b) => serializer.serialize_bool(*b),
            FreeNode::Integer(int) => int.serialize(serializer),
            FreeNode::Float(float) => float.serialize(serializer),
            FreeNode::String(s) => serializer.serialize_str(s),
            FreeNode::List(seq) => serializer.collect_seq(seq),
            FreeNode::Map(map) => serializer.collect_map(map),
        }
    }
}

// impl<'de, T> From<T> for Dag where T: Deserialize<'de> {
//     fn from(t: T) -> Self {
//     }
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
