use crate::{base::Base, cid::CID, node::Node, Float, Int, Key};
use indexmap::IndexMap;
use serde::{Serialize, Serializer};
use std::{borrow::Cow, marker::PhantomData};

pub struct CowT<'a, I, T>
where
    I: ToOwned,
    T: Node<'a>,
    // T: Node<'a> + From<I>,
{
    is_dirty: bool,
    cow: Cow<'a, I>,
    t: Option<T>,
}

// impl<'a, I, T> Serialize for CowT<'a, I, T>
// where
//     I: ToOwned,
//     T: Node<'a>,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {

//     }
// }

/// An abstract Dag type that borrows all of its values from the underlying binary.
pub enum CowNode<'a, I>
where
    I: ToOwned, // + something like &[u8] or &str
{
    Null,

    Bool(CowT<'a, I, bool>),

    Integer(CowT<'a, I, Int>),

    Float(CowT<'a, I, Float>),

    Str(CowT<'a, I, &'a str>),

    Bytes(CowT<'a, I, &'a [u8]>),
    // List(CowT<'a, I, Vec<CowNode<'a, I>>>),

    // Map(CowT<'a, I, IndexMap<CowT<'a, I, Key>, CowNode<'a, I>>>),

    // Link(CowT<'a, I, CID>, Option<CowNode<'a, I>>),
}

// impl<'a, I> Serialize for CowNode<'a, I> {
//     #[inline]
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             // encoder-specific
//             CowNode::Bytes(buf, base) => serializer.encode_bytes(buf, *base),
//             CowNode::Link(cid, _) => serializer.encode_link(cid),

//             // standard
//             CowNode::Null => serializer.serialize_none(),
//             CowNode::Bool(b) => serializer.serialize_bool(*b),
//             CowNode::Integer(int) => int.serialize(serializer),
//             CowNode::Float(float) => float.serialize(serializer),
//             CowNode::String(s) => serializer.serialize_str(s),
//             CowNode::List(seq) => serializer.collect_seq(seq),
//             CowNode::Map(map) => serializer.collect_map(map),
//         }
//     }
// }
