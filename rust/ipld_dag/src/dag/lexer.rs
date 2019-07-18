use super::{Float, Int};
use crate::{base::Base, cid::CID};
use nom::{
    error::{ErrorKind, ParseError},
    IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Slice
};
use std::{iter::Enumerate, slice::Iter};

// TODO: ideas:
//  - implement Serialize for Stream of Lexer
//  - implement

///
#[derive(Clone, Debug, From, PartialEq)]
pub enum Token<'a> {
    ///
    EOF,

    ///
    Null,

    ///
    Bool(bool),

    ///
    Integer(Int),

    ///
    Float(Float),

    ///
    Str(&'a str),

    /// Raw bytes.
    Bytes(&'a [u8]),

    /// [`multibase`]-encoded `str`.
    ByteStr(&'a str),

    ///
    List(Option<usize>),

    ///
    ListEnd,

    ///
    Map(Option<usize>),

    ///
    MapEnd,

    ///
    Link(CID),
    // RawValue?

    // TODO: a link to another `Dag`, possibly of another format
    // LinkedData(Option<Prefix>),

    // TODO:
    // LinkedDataEnd,
}

impl<'a> InputLength for Token<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

///
///
/// borrows heavily from [`monkey-rust`]
/// [`monkey-rust`]: https://github.com/Rydgel/monkey-rust/blob/master/lib/lexer/token.rs
///
/// TODO: needs to keep track of depth and container state (so it can produce )
/// ...
/// ...
/// create a TokenProducer (generator/iterator/stream) for parsers:
///     - impls Parser input traits
///     - takes in a binary (or Read?)
/// write manual parser combinator macros that:
///     tag_token
///     parse_ident
///     parse_literal
/// Selectors:
///     - main method (generic over TokenProducer):
///         receives a TokenProducer and a selector
///             - is selector a string, bytes, maybe a data structure ??
///             - internally, it might produce a parser from the selector
///         returns a Result<Dag | Cont> (?? or Future??)
///         uses our combinator macros to tag tokens from TokenProducer
///             then it could pipe the do_parse!(TokenProducer, selector_parser)
/// In Elixir:
///
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lexer<'a, P> {
    parser: P,
    // tokens: &'a [Token<'a>],
    start: usize,
    end: usize,
}

impl<'a, P> Lexer<'a, P> {
    #[inline]
    fn new(parser: P) -> Self {
        Lexer {
            tokens,
            start: 0,
            end: tokens.len(),
        }
    }

    // #[inline]
    // fn extend(&mut self, tokens: &'a Vec<Token<'a>>) -> Self {
    //     Lexer {
    //         tokens: &
    //     }
    // }

    #[inline]
    fn parse(&mut self, bytes: &'a [u8]) -> IResult<&'a [u8], (), ParseError> {
        // Lexer {
        //     tokens: &[],
        //     start: 0,
        //     end: 0,
        // }
    }
}

impl<'a> InputIter for Lexer<'a> {
    type Item = &'a Token<'a>;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Iter<'a, Token<'a>>;

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.iter_elements().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        if self.tokens.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}

impl<'a> InputLength for Lexer<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputTake for Lexer<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Lexer {
            tokens: &self.tokens[0..count],
            start: 0,
            end: count,
        }
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        let first = Lexer {
            tokens: prefix,
            start: 0,
            end: prefix.len(),
        };
        let second = Lexer {
            tokens: suffix,
            start: 0,
            end: suffix.len(),
        };
        (second, first)
    }
}

// impl<'a> InputTakeAtPosition for Lexer<'a> {
//     type Item = Token<'a>;

//     fn split_at_position<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
//     where
//         E: ParseError<Self>,
//         P: Fn(Self::Item) -> bool,
//     {
//     }

//     fn split_at_position1<P, E>(&self, predicate: P, e: ErrorKind) -> IResult<Self, Self, E>
//     where
//         E: ParseError<Self>,
//         P: Fn(Self::Item) -> bool,
//     {
//     }

//     fn split_at_position_complete<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
//     where
//         E: ParseError<Self>,
//         P: Fn(Self::Item) -> bool,
//     {
//     }

//     fn split_at_position1_complete<P, E>(
//         &self,
//         predicate: P,
//         e: ErrorKind,
//     ) -> IResult<Self, Self, E>
//     where
//         E: ParseError<Self>,
//         P: Fn(Self::Item) -> bool,
//     {
//     }
// }

impl<'a> Slice<Range<usize>> for Lexer<'a> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Lexer {
            tokens: self.tokens.slice(range),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Lexer<'a> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Lexer<'a> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Lexer<'a> {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        self.slice(..)
    }
}
