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
//!     - with the provided abstract FreeNode enum
//!         - each variant already configured to

mod de;
pub mod float;
pub mod int;
pub mod key;
mod ser;

pub use crate::freenode::{float::Float, int::Int, key::Key};
use crate::{base::Base, cid::CID, lexer::Token, node::Node};
use indexmap::{
    map::{Iter as MapIter, IterMut as MapIterMut},
    IndexMap,
};
use std::{borrow, iter, slice};

// fn resolve<I: FreeNode, O: FreeNode>(dag: &I) -> Result<&O, Error> {
//     Err(Error::ExpectedLinkedDag)
// }

// fn from_str() {}

// fn from_reader() {}

// fn from_tokens() {}

// ?? impl Serialize for FreeNode
// ?? - would allow impl FreeNode to use it's own overridden serialize methods
//
// ?? what if FreeNode was instead generic over a FreeNode trait object...??
/// ... currently (will) define behaviour that `Format`s can use to dig into `FreeNode`s.
///
/// Represents an abtract IPLD [FreeNode](https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md). Useful if decoding unknown IPLD.
///
/// An `indexmap` is used as the map implementation in order to preserve key order.
#[derive(Clone, Debug, From)]
pub enum FreeNode {
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

    /// Represents a list of `FreeNode` nodes.
    List(Vec<Self>),

    /// Represents a map of `FreeNode` nodes.
    /// Uses an [`IndexMap`] to preserve key order.
    ///
    /// [`IndexMap`]: https://docs.rs/indexmap/1.0.2/indexmap/map/struct.IndexMap.html
    Map(IndexMap<Key, Self>),

    /// Represents an IPLD [`CID`] [`Link`].
    ///
    /// [`CID`]
    /// [`Link`]: https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind
    Link(CID, Option<Box<FreeNode>>),
}

macro_rules! match_freenode {
    ($node:ident, $($variant:pat => $opt:expr)*) => {{
        match $node {
            $($variant => $opt)*,
            _ => None,
        }
    }}
}

impl Node for FreeNode {
    #[inline]
    fn kind(&self) -> Token {
        match self {
            FreeNode::Null => Token::Null,
            FreeNode::Bool(b) => Token::Bool(*b),
            FreeNode::Integer(i) => Token::Integer(*i),
            FreeNode::Float(f) => Token::Float(*f),
            FreeNode::String(s) => Token::Str(&s),
            FreeNode::ByteBuf(bytes, _base) => Token::Bytes(&bytes),
            FreeNode::List(vec) => Token::List(Some(vec.len())),
            FreeNode::Map(map) => Token::Map(Some(map.len())),
            FreeNode::Link(cid, _) => Token::Link(*cid),
        }
    }

    #[inline]
    fn len(&self) -> Option<usize> {
        match self {
            FreeNode::List(vec) => Some(vec.len()),
            FreeNode::Map(map) => Some(map.len()),
            _ => None,
        }
    }

    #[inline]
    fn is_null(&self) -> bool {
        match self {
            FreeNode::Null => true,
            _ => false,
        }
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        match_freenode!(self, FreeNode::Bool(b) => Some(*b))
    }

    #[inline]
    fn as_int(&self) -> Option<Int> {
        match_freenode!(self, FreeNode::Integer(i) => Some(*i))
    }

    #[inline]
    fn as_float(&self) -> Option<Float> {
        match_freenode!(self, FreeNode::Float(f) => Some(*f))
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        match_freenode!(self, FreeNode::String(s) => Some(&s))
    }

    #[inline]
    fn as_bytes(&self) -> Option<&[u8]> {
        match_freenode!(self, FreeNode::ByteBuf(bytes, _) => Some(&bytes))
    }

    #[inline]
    fn as_link(&self) -> Option<CID> {
        match_freenode!(self, FreeNode::Link(cid, _) => Some(*cid))
    }

    #[inline]
    fn list_iter(&self) -> Option<slice::Iter<FreeNode>> {
        match_freenode!(self, FreeNode::List(vec) => Some(vec.iter()))
    }

    #[inline]
    fn list_iter_mut(&mut self) -> Option<slice::Iter<FreeNode>> {
        match_freenode!(self, FreeNode::List(vec) => Some(vec.iter()))
    }

    #[inline]
    fn map_iter(&self) -> Option<MapIter<Key, FreeNode>> {
        match_freenode!(self, FreeNode::Map(map) => Some(map.iter()))
    }

    #[inline]
    fn map_iter_mut(&mut self) -> Option<MapIterMut<Key, FreeNode>> {
        match_freenode!(self, FreeNode::Map(map) => Some(map.iter_mut()))
    }

    #[inline]
    fn traverse_index(&self, index: usize) -> Option<&FreeNode> {
        match_freenode!(self, FreeNode::List(vec) => vec.get(index))
    }

    #[inline]
    fn traverse_index_mut(&mut self, index: usize) -> Option<&mut FreeNode> {
        match_freenode!(self, FreeNode::List(vec) => vec.get_mut(index))
    }

    #[inline]
    fn traverse_field<K>(&self, key: &K) -> Option<&FreeNode>
    where
        K: Into<Key>,
    {
        match_freenode!(self, FreeNode::Map(map) => map.get(&(*key).into()))
    }

    #[inline]
    fn traverse_field_mut<K>(&mut self, key: &K) -> Option<&mut FreeNode>
    where
        K: Into<Key>,
    {
        match_freenode!(self, FreeNode::Map(map) => map.get_mut(&(*key).into()))
    }
}

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for FreeNode {
                #[inline]
                fn from(n: $ty) -> Self {
                    FreeNode::Integer(n.into())
                }
            }
        )*
    };
}

macro_rules! from_float {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for FreeNode {
                #[inline]
                fn from(n: $ty) -> Self {
                    FreeNode::Float(n.into())
                }
            }
        )*
    };
}

from_integer! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 }
from_float! { f32 f64 }

impl<'a> From<&'a str> for FreeNode {
    #[inline]
    fn from(v: &str) -> Self {
        FreeNode::String(v.to_string())
    }
}

impl<'a> From<borrow::Cow<'a, str>> for FreeNode {
    #[inline]
    fn from(v: borrow::Cow<'a, str>) -> Self {
        FreeNode::String(v.into_owned())
    }
}

impl<T> From<Option<T>> for FreeNode
where
    T: Into<FreeNode>,
{
    #[inline]
    fn from(o: Option<T>) -> Self {
        match o {
            None => FreeNode::Null,
            Some(t) => t.into(),
        }
    }
}

impl<T> iter::FromIterator<T> for FreeNode
where
    T: Into<FreeNode>,
{
    /// Convert an iteratable type to a `FreeNode`.
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        FreeNode::List(iter.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
