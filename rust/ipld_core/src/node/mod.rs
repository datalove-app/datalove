mod float;
mod int;
mod key;

pub use self::{float::Float, int::Int, key::Key};
use crate::{cid::CID, lexer::Token};
use serde::Serialize;
use std::{borrow, slice};

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
    fn kind(&self) -> Token;

    ///
    fn len(&self) -> Option<usize> {
        None
    }

    ///
    fn is_null(&self) -> bool {
        false
    }

    ///
    fn as_bool(&self) -> Option<bool> {
        None
    }

    ///
    fn as_int(&self) -> Option<Int> {
        None
    }

    ///
    fn as_float(&self) -> Option<Float> {
        None
    }

    ///
    fn as_str(&self) -> Option<&str> {
        None
    }

    ///
    fn as_bytes(&self) -> Option<&[u8]> {
        None
    }

    ///
    fn as_link(&self) -> Option<CID> {
        None
    }

    ///
    fn list_iter(&'a self) -> Option<Self::ListIter> {
        None
    }

    ///
    fn list_iter_mut(&'a mut self) -> Option<Self::ListIterMut> {
        None
    }

    ///
    fn map_iter(&'a self) -> Option<Self::MapIter> {
        None
    }

    ///
    fn map_iter_mut(&'a mut self) -> Option<Self::MapIterMut> {
        None
    }

    ///
    fn traverse_index(&self, _index: usize) -> Option<&Self::Child> {
        None
    }

    ///
    fn traverse_index_mut(&mut self, _index: usize) -> Option<&mut Self::Child> {
        None
    }

    ///
    fn traverse_field(&self, _key: &Self::Key) -> Option<&Self::Child> {
        None
    }

    ///
    fn traverse_field_mut(&mut self, _key: &Self::Key) -> Option<&mut Self::Child> {
        None
    }
}

impl<'a> Node<'a> for () {
    #[inline]
    fn kind(&self) -> Token {
        Token::Null
    }

    #[inline]
    fn is_null(&self) -> bool {
        true
    }
}

impl<'a> Node<'a> for bool {
    #[inline]
    fn kind(&self) -> Token {
        Token::Bool(*self)
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
                fn kind(&self) -> Token {
                    Token::Integer((*self).into())
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
                fn kind(&self) -> Token {
                    Token::Float((*self).into())
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
    fn kind(&self) -> Token {
        Token::Str(&self)
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(&self)
    }
}

impl<'a> Node<'a> for &'a str {
    #[inline]
    fn kind(&self) -> Token {
        Token::Str(self)
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> Node<'a> for borrow::Cow<'a, str> {
    #[inline]
    fn kind(&self) -> Token {
        Token::Str(self)
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> Node<'a> for &'a [u8] {
    #[inline]
    fn kind(&self) -> Token {
        Token::Bytes(self)
    }

    #[inline]
    fn as_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
}

impl<'a, T> Node<'a> for Option<T>
where
    T: Node<'a>,
{
    #[inline]
    fn kind(&self) -> Token {
        match self {
            None => Token::Null,
            Some(t) => t.kind(),
        }
    }

    #[inline]
    fn is_null(&self) -> bool {
        self.is_none()
    }
}

impl<'a> Node<'a> for CID {
    #[inline]
    fn kind(&self) -> Token {
        Token::Link(self.clone())
    }

    #[inline]
    fn as_link(&self) -> Option<CID> {
        Some(self.clone())
    }
}

impl<'a, T> Node<'a> for T
where
    T: Serialize,
{
    default fn kind(&self) -> Token {
        Token::Invalid
    }

    default fn len(&self) -> Option<usize> {
        None
    }

    default fn is_null(&self) -> bool {
        false
    }

    default fn as_bool(&self) -> Option<bool> {
        None
    }

    default fn as_int(&self) -> Option<Int> {
        None
    }

    default fn as_float(&self) -> Option<Float> {
        None
    }

    default fn as_str(&self) -> Option<&str> {
        None
    }

    default fn as_bytes(&self) -> Option<&[u8]> {
        None
    }

    default fn as_link(&self) -> Option<CID> {
        None
    }

    // ListIter() ListIter
    default fn list_iter(&'a self) -> Option<Self::ListIter> {
        None
    }

    // ListIter() ListIter
    default fn list_iter_mut(&'a mut self) -> Option<Self::ListIterMut> {
        None
    }

    // MapIterMut() MapIterMut
    default fn map_iter(&'a self) -> Option<Self::MapIter> {
        None
    }

    // MapIterMut() MapIterMut
    default fn map_iter_mut(&'a mut self) -> Option<Self::MapIterMut> {
        None
    }

    // TraverseIndex(idx int) Node
    default fn traverse_index(&self, _index: usize) -> Option<&Self::Child> {
        None
    }

    // TraverseIndex(idx int) Node
    default fn traverse_index_mut(&mut self, _index: usize) -> Option<&mut Self::Child> {
        None
    }

    // TraverseField(path string) Node
    default fn traverse_field(&self, _key: &Self::Key) -> Option<&Self::Child> {
        None
    }

    // TraverseField(path string) Node
    default fn traverse_field_mut(&mut self, _key: &Self::Key) -> Option<&mut Self::Child> {
        None
    }
}
