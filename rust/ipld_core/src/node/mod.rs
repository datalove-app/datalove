mod float;
mod int;
mod key;

pub use self::{float::Float, int::Int, key::Key};
use crate::{cid::CID, format::Token};
use indexmap::map::{Iter as MapIter, IterMut as MapIterMut};
use serde::ser::Serialize;
use std::{borrow, slice};

///
pub enum Kind {
    Null,
    Bool,
    Integer,
    Float,
    Str,
    Bytes,
    List(Option<usize>),
    Map(Option<usize>),
    Link,
}

///
pub trait Node<'a>: Serialize {
    ///
    type Key: 'a + Into<Key> = &'static str;

    ///
    type Child: 'a + Node<'a> = ();

    ///
    type ListIter: Iterator<Item = &'a Self::Child> = slice::Iter<'a, Self::Child>;

    ///
    type ListIterMut: Iterator<Item = &'a mut Self::Child> = slice::IterMut<'a, Self::Child>;

    ///
    type MapIter: Iterator<Item = (&'a Self::Key, &'a Self::Child)> =
        slice::Iter<'a, (Self::Key, Self::Child)>;

    ///
    type MapIterMut: Iterator<Item = (&'a Self::Key, &'a mut Self::Child)> =
        slice::IterMut<'a, (Self::Key, Self::Child)>;

    ///
    fn kind(&self) -> Kind;

    ///
    #[inline]
    fn len(&self) -> Option<usize> {
        None
    }

    ///
    #[inline]
    fn is_null(&self) -> bool {
        false
    }

    ///
    #[inline]
    fn as_bool(&self) -> Option<bool> {
        None
    }

    ///
    #[inline]
    fn as_int(&self) -> Option<Int> {
        None
    }

    ///
    #[inline]
    fn as_float(&self) -> Option<Float> {
        None
    }

    ///
    #[inline]
    fn as_str(&self) -> Option<&str> {
        None
    }

    ///
    #[inline]
    fn as_bytes(&self) -> Option<&[u8]> {
        None
    }

    ///
    #[inline]
    fn as_link(&self) -> Option<CID> {
        None
    }

    ///
    #[inline]
    fn list_iter(&'a self) -> Option<Self::ListIter> {
        None
    }

    ///
    #[inline]
    fn list_iter_mut(&'a mut self) -> Option<Self::ListIterMut> {
        None
    }

    ///
    #[inline]
    fn map_iter(&'a self) -> Option<Self::MapIter> {
        None
    }

    ///
    #[inline]
    fn map_iter_mut(&'a mut self) -> Option<Self::MapIterMut> {
        None
    }

    ///
    #[inline]
    fn traverse_index(&self, _index: usize) -> Option<&Self::Child> {
        None
    }

    ///
    #[inline]
    fn traverse_index_mut(&mut self, _index: usize) -> Option<&mut Self::Child> {
        None
    }

    ///
    #[inline]
    fn traverse_field(&self, _key: &Self::Key) -> Option<&Self::Child> {
        None
    }

    ///
    #[inline]
    fn traverse_field_mut(&mut self, _key: &Self::Key) -> Option<&mut Self::Child> {
        None
    }
}

impl<'a> Node<'a> for () {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Null
    }

    #[inline]
    fn is_null(&self) -> bool {
        true
    }
}

impl<'a> Node<'a> for bool {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Bool
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

macro_rules! for_integer {
    ($($ty:ident)*) => {
        $(
            impl<'a> Node<'a> for $ty {
                #[inline]
                fn kind(&self) -> Kind {
                    Kind::Integer
                }

                #[inline]
                fn as_int(&self) -> Option<Int> {
                    Some((*self).into())
                }
            }
        )*
    };
}

macro_rules! for_float {
    ($($ty:ident)*) => {
        $(
            impl<'a> Node<'a> for $ty {
                #[inline]
                fn kind(&self) -> Kind {
                    Kind::Float
                }

                #[inline]
                fn as_float(&self) -> Option<Float> {
                    Some((*self).into())
                }
            }
        )*
    };
}

for_integer! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 }
for_float! { f32 f64 }

impl<'a> Node<'a> for String {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Str
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(&self)
    }
}

impl<'a> Node<'a> for &'a str {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Str
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> Node<'a> for borrow::Cow<'a, str> {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Str
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> Node<'a> for &'a [u8] {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::Bytes
    }

    #[inline]
    fn as_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
}

macro_rules! match_option {
    ($node:ident, $($variant:pat => $opt:expr)*) => {{
        match $node {
            $($variant => $opt)*,
            _ => None,
        }
    }}
}

impl<'a, T> Node<'a> for Option<T>
where
    T: Node<'a>,
{
    type Key = T::Key;
    type Child = T::Child;
    type ListIter = T::ListIter;
    type ListIterMut = T::ListIterMut;
    type MapIter = T::MapIter;
    type MapIterMut = T::MapIterMut;

    #[inline]
    fn kind(&self) -> Kind {
        match self {
            None => Kind::Null,
            Some(node) => node.kind(),
        }
    }

    fn len(&self) -> Option<usize> {
        match_option!(self, Some(node) => Node::len(node))
    }

    #[inline]
    fn is_null(&self) -> bool {
        match self {
            None => true,
            Some(node) => node.is_null(),
        }
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        match_option!(self, Some(node) => node.as_bool())
    }

    #[inline]
    fn as_int(&self) -> Option<Int> {
        match_option!(self, Some(node) => node.as_int())
    }

    #[inline]
    fn as_float(&self) -> Option<Float> {
        match_option!(self, Some(node) => node.as_float())
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        match_option!(self, Some(node) => node.as_str())
    }

    #[inline]
    fn as_bytes(&self) -> Option<&[u8]> {
        match_option!(self, Some(node) => node.as_bytes())
    }

    #[inline]
    fn as_link(&self) -> Option<CID> {
        match_option!(self, Some(node) => node.as_link())
    }

    #[inline]
    fn list_iter(&'a self) -> Option<Self::ListIter> {
        match_option!(self, Some(node) => node.list_iter())
    }

    #[inline]
    fn list_iter_mut(&'a mut self) -> Option<Self::ListIterMut> {
        match_option!(self, Some(node) => node.list_iter_mut())
    }

    #[inline]
    fn map_iter(&'a self) -> Option<Self::MapIter> {
        match_option!(self, Some(node) => node.map_iter())
    }

    #[inline]
    fn map_iter_mut(&'a mut self) -> Option<Self::MapIterMut> {
        match_option!(self, Some(node) => node.map_iter_mut())
    }

    #[inline]
    fn traverse_index(&self, index: usize) -> Option<&Self::Child> {
        match_option!(self, Some(node) => node.traverse_index(index))
    }

    #[inline]
    fn traverse_index_mut(&mut self, index: usize) -> Option<&mut Self::Child> {
        match_option!(self, Some(node) => node.traverse_index_mut(index))
    }

    #[inline]
    fn traverse_field(&self, key: &Self::Key) -> Option<&Self::Child> {
        match_option!(self, Some(node) => node.traverse_field(key))
    }

    #[inline]
    fn traverse_field_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Child> {
        match_option!(self, Some(node) => node.traverse_field_mut(key))
    }
}

impl<'a, T> Node<'a> for Vec<T>
where
    T: 'a + Node<'a>,
{
    type Key = &'static str;
    type Child = T;
    type ListIter = slice::Iter<'a, Self::Child>;
    type ListIterMut = slice::IterMut<'a, Self::Child>;
    type MapIter = MapIter<'a, Self::Key, Self::Child>;
    type MapIterMut = MapIterMut<'a, Self::Key, Self::Child>;

    #[inline]
    fn kind(&self) -> Kind {
        Kind::List(Node::len(self))
    }

    #[inline]
    fn len(&self) -> Option<usize> {
        Some(self.len())
    }

    #[inline]
    fn is_null(&self) -> bool {
        false
    }

    #[inline]
    fn as_bool(&self) -> Option<bool> {
        None
    }

    #[inline]
    fn as_int(&self) -> Option<Int> {
        None
    }

    #[inline]
    fn as_float(&self) -> Option<Float> {
        None
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        None
    }

    #[inline]
    fn as_bytes(&self) -> Option<&[u8]> {
        None
    }

    #[inline]
    fn as_link(&self) -> Option<CID> {
        None
    }

    #[inline]
    fn list_iter(&'a self) -> Option<Self::ListIter> {
        Some(self.iter())
    }

    #[inline]
    fn list_iter_mut(&'a mut self) -> Option<Self::ListIterMut> {
        Some(self.iter_mut())
    }

    #[inline]
    fn map_iter(&'a self) -> Option<Self::MapIter> {
        None
    }

    #[inline]
    fn map_iter_mut(&'a mut self) -> Option<Self::MapIterMut> {
        None
    }

    #[inline]
    fn traverse_index(&self, index: usize) -> Option<&Self::Child> {
        self.get(index)
    }

    #[inline]
    fn traverse_index_mut(&mut self, index: usize) -> Option<&mut Self::Child> {
        self.get_mut(index)
    }

    #[inline]
    fn traverse_field(&self, _key: &Self::Key) -> Option<&Self::Child> {
        None
    }

    #[inline]
    fn traverse_field_mut(&mut self, _key: &Self::Key) -> Option<&mut Self::Child> {
        None
    }
}
