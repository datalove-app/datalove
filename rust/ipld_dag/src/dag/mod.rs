mod float;
mod int;
mod link;

pub use crate::dag::{float::Float, int::Int, link::Link};

use crate::error::Error;
use multibase::Base;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, SerializeMap, SerializeSeq, Serializer},
};
use std::collections::BTreeMap;

/**
 * Notes:
 *
 * dag needs to:
 *  - define a FormatEncoder, FormatDecoder, and FormatResolver trait
 *      - Encoder and Decoder methods are hard-coded to take in each Dag and Link variant
 *      -
 *
 * each format needs:
 *  - define a FormatEncoder
 *
 *  - to implement a Serializer and Deserializer that matches the Dag, Link and CID Serialize and Deserialize behaviour
 *      - custom impl Serialize and Deserialize for Dag, Link and CID
 *      - with a custom
 *      - deref (or whatever) impls to access the underlying Dag and Link
 *      - these can be aided with serde attributes and (our own) macros
 *  - expose functions that pair these new types with their format's specific Serializer and Deserializer impls
 */

///
// pub trait Dag: From<DagNode<Self>> + Serialize {}
pub trait Dag: Serialize {
    // fn serialize_dag<S: Serializer, T: Dag>(dag: T, serializer: S) -> Result<S::Ok, S::Error>;
}

// TODO: possibly add/repalce with a fully-owned alternative
// TODO: ?? (just so we can implement conversion from DagNode<T> -> DagNode<U>) impl a Serializer + Deserializer, and macros for auto deriving them for the DagNode wrapper struct
/// Represents an abtract IPLD Dag. Useful if decoding unknown IPLD.
///
/// An `indexmap` is used as the map implementation in order to preserve key order.
pub enum DagNode<'a, T: Dag> {
    /// Represents an IPLD null value.
    Null,

    /// Represents an IPLD boolean.
    Bool(bool),

    /// Represents an IPLD integer.
    Integer(Int),

    /// Represents an IPLD float.
    Float(Float),

    /// Represents an IPLD string.
    // Str(String),
    Str(&'a str),

    /// Represents IPLD bytes.
    // Bytes(Vec<u8>, Option<Base>),
    Bytes(&'a [u8], Option<Base>),

    /// Represents an IPLD list.
    List(Vec<T>),

    /// Represents an IPLD map.
    /// Uses a BTreeMap to preserve key order.
    Map(BTreeMap<T, T>),

    /// Represents an IPLD link.
    Link(Link<T>),
}

// impl<'a, T: Dag, U: Dag> From<DagNode<'a, T>> for DagNode<'a, U> {}

impl<'a, T: Dag> Dag for DagNode<'a, T> {}

// impl<'a, T: Serialize> From<Dag<'a, T>> for T {
//     fn from(dag: Dag<'a, T>) -> T {
//         match dag {
//             Dag::Link(link) => {
//                 let new_link = match link {
//                     Link::CID(cid) => Link::CID(cid),
//                     Link::Dag(dag) => Link::Dag(Box::new((*dag).into())),
//                 };

//                 Dag::Link(new_link).into()
//             }
//             Dag::List(seq) => JsonDag(Dag::List(seq.into())),
//             Dag::Map(map) => JsonDag(Dag::Map(map.into())),
//             _ => dag.into(),
//         }
//     }
// }

impl<'a, T: Dag> Serialize for DagNode<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DagNode::Null => serializer.serialize_none(),
            DagNode::Bool(b) => serializer.serialize_bool(*b),
            DagNode::Integer(int) => int.serialize(serializer),
            DagNode::Float(float) => float.serialize(serializer),
            DagNode::Str(s) => serializer.serialize_str(s),
            DagNode::Bytes(bytes, _) => serializer.serialize_bytes(bytes),
            DagNode::Link(link) => link.serialize(serializer),
            DagNode::List(seq) => {
                let mut seq_enc = serializer.serialize_seq(Some(seq.len()))?;
                for dag in seq {
                    seq_enc.serialize_element(&dag)?;
                }
                seq_enc.end()
            }
            DagNode::Map(map) => {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
