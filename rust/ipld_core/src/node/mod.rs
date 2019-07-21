use crate::{
    cid::CID,
    freenode::{Float, Int, Key},
    lexer::Token,
};
use serde::Serialize;
use std::borrow;

pub trait Node: Serialize {
    fn kind(&self) -> Token;

    fn len(&self) -> Option<usize> {
        None
    }

    fn is_null(&self) -> bool {
        false
    }

    fn as_bool(&self) -> Option<bool> {
        None
    }

    fn as_int(&self) -> Option<Int> {
        None
    }

    fn as_float(&self) -> Option<Float> {
        None
    }

    fn as_str(&self) -> Option<&str> {
        None
    }

    fn as_bytes(&self) -> Option<&[u8]> {
        None
    }

    fn as_link(&self) -> Option<CID> {
        None
    }

    fn list_iter<I, N>(&self) -> Option<I>
    where
        I: Iterator<Item = N>,
        N: Node,
    {
        None
    }

    fn list_iter_mut<I, N>(&mut self) -> Option<I>
    where
        I: Iterator<Item = N>,
        N: Node,
    {
        None
    }

    fn map_iter<I, K, N>(&self) -> Option<I>
    where
        I: Iterator<Item = (K, N)>,
        K: Into<Key>,
        N: Node,
    {
        None
    }

    fn map_iter_mut<I, K, N>(&mut self) -> Option<I>
    where
        I: Iterator<Item = (K, N)>,
        K: Into<Key>,
        N: Node,
    {
        None
    }

    fn traverse_index<N>(&self, index: usize) -> Option<&N>
    where
        N: Node,
    {
        None
    }

    fn traverse_index_mut<N>(&mut self, index: usize) -> Option<&mut N>
    where
        N: Node,
    {
        None
    }

    fn traverse_field<K, N>(&self, key: &K) -> Option<&N>
    where
        K: Into<Key>,
        N: Node,
    {
        None
    }

    fn traverse_field_mut<K, N>(&mut self, key: &K) -> Option<&mut N>
    where
        K: Into<Key>,
        N: Node,
    {
        None
    }
}

impl Node for bool {
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
            impl Node for $ty {
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
            impl Node for $ty {
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

impl Node for String {
    #[inline]
    fn kind(&self) -> Token {
        Token::Str(&self)
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(&self)
    }
}

impl<'a> Node for &'a str {
    #[inline]
    fn kind(&self) -> Token {
        Token::Str(self)
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> Node for borrow::Cow<'a, str> {
    #[inline]
    fn kind(&self) -> Token {
        Token::Str(self)
    }

    #[inline]
    fn as_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl<'a> Node for &'a [u8] {
    #[inline]
    fn kind(&self) -> Token {
        Token::Bytes(self)
    }

    #[inline]
    fn as_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
}

impl<T> Node for Option<T>
where
    T: Node,
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

impl Node for CID {
    #[inline]
    fn kind(&self) -> Token {
        Token::Link(*self)
    }

    #[inline]
    fn as_link(&self) -> Option<CID> {
        Some(*self)
    }
}

impl<T> Node for T {
    fn kind(&self) -> Token {
        Token::Invalid
    }

    // ListIterator() ListIterator
    default fn list_iter<I, N>(&self) -> Option<I>
    where
        I: Iterator<Item = N>,
        N: Node,
    {
        None
    }

    // ListIterator() ListIterator
    default fn list_iter_mut<I, N>(&mut self) -> Option<I>
    where
        I: Iterator<Item = N>,
        N: Node,
    {
        None
    }

    // MapIterator() MapIterator
    default fn map_iter<I, K, N>(&self) -> Option<I>
    where
        I: Iterator<Item = (K, N)>,
        K: Into<Key>,
        N: Node,
    {
        None
    }

    // MapIterator() MapIterator
    default fn map_iter_mut<I, K, N>(&mut self) -> Option<I>
    where
        I: Iterator<Item = (K, N)>,
        K: Into<Key>,
        N: Node,
    {
        None
    }

    // TraverseIndex(idx int) Node
    default fn traverse_index<N>(&self, index: usize) -> Option<&N>
    where
        N: Node,
    {
        None
    }

    // TraverseIndex(idx int) Node
    default fn traverse_index_mut<N>(&mut self, index: usize) -> Option<&mut N>
    where
        N: Node,
    {
        None
    }

    // TraverseField(path string) Node
    default fn traverse_field<K, N>(&self, key: &K) -> Option<&N>
    where
        K: Into<Key>,
        N: Node,
    {
        None
    }

    // TraverseField(path string) Node
    default fn traverse_field_mut<K, N>(&mut self, key: &K) -> Option<&mut N>
    where
        K: Into<Key>,
        N: Node,
    {
        None
    }
}
