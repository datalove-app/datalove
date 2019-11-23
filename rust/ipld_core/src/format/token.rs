use crate::{
    cid::CID,
    multibase::Base,
    node::{Float, Int},
};
use nom::{
    error::{ErrorKind, ParseError},
    IResult, InputIter, InputLength, InputTake, InputTakeAtPosition, Slice,
};
use std::{iter::Enumerate, slice::Iter};

// TODO: ideas:
//  - create a Parser trait
//  - replace Tokenizer with a Selector-related struct
//      - it is generic over a Parser, which uses this tokenizer internally
//      - Selector exposes:
//          - a push-type method to add more bytes
//          - a pull-type method to parse n tokens and attempt to resolve the selector

///
#[derive(Clone, Copy, Debug, From, PartialEq)]
pub enum Token<'a> {
    ///
    EOF,

    ///
    Invalid,

    ///
    Null,

    ///
    Bool(bool),

    ///
    IntegerStr(&'a str),

    ///
    IntegerBytes(&'a [u8]),

    ///
    FloatStr(&'a str),

    ///
    FloatBytes(&'a [u8]),

    /// A UTF-8 string.
    StrRaw(&'a str),

    /// A UTF-8 string, as bytes.
    StrBytes(&'a [u8]),

    /// Raw bytes.
    BytesRaw(&'a [u8]),

    /// A [`multibase`]-encoded byte `str`.
    BytesStr(&'a str),

    ///
    ListStart(Option<usize>),

    ///
    ListEnd,

    ///
    MapStart(Option<usize>),

    ///
    MapEnd,

    ///
    LinkBytes(&'a [u8]),

    ///
    LinkStr(&'a str),
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Initial,
    ListStart,
    ListElement,
    MapStart,
    MapValue,
    Finished,
}

#[derive(Clone, Copy)]
pub enum LexResult<'a> {
    ///
    Ok(&'a [u8], Token<'a>),

    ///
    Incomplete(usize, usize),

    ///
    Err,
}

pub struct Tokenizer<'a, S, E>
where
    S: Fn(&'a [u8]) -> IResult<&'a [u8], Token<'a>>,
    E: Fn(&'a [u8]) -> IResult<&'a [u8], Token<'a>>,
{
    ///
    ///
    /// TODO: look into slice_deque or buf_reduxe6
    input: &'a [u8],
    /// Position (in bytes) of the input byte (stream).
    position: usize,
    // length: usize,
    states: Vec<State>,

    lex_start: S,
    lex_end: E,
}

macro_rules! lex_start {
    ($lex:expr, $input:expr) => {
        match ($lex)($input) {
            Ok(res) => res,
            // Err(Incomplete) => return Some(LexResult::Incomplete(_))
            Err(_) => return None,
        }
    };
}

impl<'a, S, E> Tokenizer<'a, S, E>
where
    S: Fn(&'a [u8]) -> IResult<&'a [u8], Token<'a>>,
    E: Fn(&'a [u8]) -> IResult<&'a [u8], Token<'a>>,
{
    pub fn new(input: &'a [u8], lex_start: S, lex_end: E) -> Self {
        Tokenizer {
            input,
            position: 0,
            states: vec![State::Initial],
            lex_start,
            lex_end,
        }
    }

    // fn extend(&mut self, input: &[u8]) {
    //     self.input.copy_from_slice(input)
    // }

    ///
    /// lex_start -> token
    /// if State::Finished => None
    /// if State::Initial
    ///         if list/map start
    ///             push State::ListStart/MapStart
    ///         if list/map end
    ///
    ///             return Token::Finished
    ///             // pop current state, assert that it is State::ListStart/MapStart
    ///                 // if not,
    ///     return token
    /// if State::ListStart     (expecting elem)
    ///     get_token token
    ///         if list/map start
    ///         if list/map end
    /// if State::ListElement   (expecting ...)
    ///     if can eat a list_separator
    ///         get_token
    ///     else
    ///         expect Token::ListEnd, return error if not found
    ///
    /// if State::MapStart      (expecting a key and semicolon)
    ///     get_token a key token and eat a key_value_separator
    ///     get_token token
    /// if State::MapValue      (expecting ...)
    ///     if can eat a map_separator
    ///         get_token
    ///     else
    ///         expect Token::MapEnd, return error if not found
    ///
    fn peek(&self) -> Option<LexResult<'a>> {
        let state = self.states.last().unwrap();
        if state.eq(&State::Finished) {
            return None;
        }

        if state.eq(&State::ListElement) {
            // match
        }

        let (input, token) = lex_start!(self.lex_start, self.input);

        match state {
            State::Initial => match token {
                Token::ListEnd | Token::MapEnd => Some(LexResult::Err),
                _ => Some(LexResult::Ok(input, token)),
            },
            State::ListStart => None,
            State::MapStart => None,
            State::ListElement => None,
            State::MapValue => None,
            State::Finished => return None,
        }
    }

    // fn eat(&mut self, (input, token): (&'a [u8], Token<'a>)) {
    //     match self.states.last() {
    //         State::Initial => match token {
    //             Token::ListEnd | Token::MapEnd => Some(LexResult::Failure),
    //             _ => Some(LexResult::Ok(input, token)),
    //         },
    //         State::ListStart => None,
    //         State::MapStart => None,
    //         State::ListElement => None,
    //         State::MapValue => None,
    //         State::Finished => return None,
    //     }
    // }
}

impl<'a, S, E> Iterator for Tokenizer<'a, S, E>
where
    S: Fn(&'a [u8]) -> IResult<&'a [u8], Token<'a>>,
    E: Fn(&'a [u8]) -> IResult<&'a [u8], Token<'a>>,
{
    type Item = LexResult<'a>;

    /// Produces the next `Token`.
    ///
    /// Parses the next token, then performs any tokenizer state changes.
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.peek();
        let (input, token) = match res {
            None => return None,
            Some(LexResult::Incomplete(_, _)) => return res,
            Some(LexResult::Err) => {
                *self.states.last_mut().unwrap() = State::Finished;
                return None;
            }
            Some(LexResult::Ok(input, token)) => (input, token),
        };

        match token {
            Token::ListStart(_) => self.states.push(State::ListStart),
            Token::MapStart(_) => self.states.push(State::MapStart),
            _ => {}
        };

        None
    }
}

// /
// /
// / borrows heavily from [`monkey-rust`]
// / [`monkey-rust`]: https://github.com/Rydgel/monkey-rust/blob/master/lib/lexer/token.rs
// /
// / TODO: needs to keep track of depth and container state (so it can produce )
// / ...
// / ...
// / create a TokenProducer (generator/iterator/stream) for parsers:
// /     - impls Parser input traits
// /     - takes in a binary (or Read?)
// / write manual parser combinator macros that:
// /     tag_token
// /     parse_ident
// /     parse_literal
// / Selectors:
// /     - main method (generic over TokenProducer):
// /         receives a TokenProducer and a selector
// /             - is selector a string, bytes, maybe a data structure ??
// /             - internally, it might produce a parser from the selector
// /         returns a Result<Dag | Cont> (?? or Future??)
// /         uses our combinator macros to tag tokens from TokenProducer
// /             then it could pipe the do_parse!(TokenProducer, selector_parser)
// / In Elixir:
// /

// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct Lexer<'a, P>
// where
//     P: Fn(&'a [u8]) -> IResult<&'a [u8], Token, ParseError>,
// {
//     parser: P,
//     // tokens: &'a [Token<'a>],
//     start: usize,
//     end: usize,
// }

// impl<'a, P> Lexer<'a, P> {
//     #[inline]
//     fn new(parser: P) -> Self {
//         Lexer {
//             tokens,
//             start: 0,
//             end: tokens.len(),
//         }
//     }

//     // #[inline]
//     // fn extend(&mut self, tokens: &'a Vec<Token<'a>>) -> Self {
//     //     Lexer {
//     //         tokens: &
//     //     }
//     // }

//     #[inline]
//     fn parse(&mut self, bytes: &'a [u8]) -> IResult<&'a [u8], (), ParseError> {

//         // Lexer {
//         //     tokens: &[],
//         //     start: 0,
//         //     end: 0,
//         // }
//     }
// }

// impl<'a> InputIter for Lexer<'a> {
//     type Item = &'a Token<'a>;
//     type Iter = Enumerate<Self::IterElem>;
//     type IterElem = Iter<'a, Token<'a>>;

//     #[inline]
//     fn iter_elements(&self) -> Self::IterElem {
//         self.tokens.iter()
//     }

//     #[inline]
//     fn iter_indices(&self) -> Self::Iter {
//         self.iter_elements().enumerate()
//     }

//     #[inline]
//     fn position<P>(&self, predicate: P) -> Option<usize>
//     where
//         P: Fn(Self::Item) -> bool,
//     {
//         self.iter_elements().position(predicate)
//     }

//     #[inline]
//     fn slice_index(&self, count: usize) -> Option<usize> {
//         if self.tokens.len() >= count {
//             Some(count)
//         } else {
//             None
//         }
//     }
// }

// impl<'a> InputLength for Lexer<'a> {
//     #[inline]
//     fn input_len(&self) -> usize {
//         self.tokens.len()
//     }
// }

// impl<'a> InputTake for Lexer<'a> {
//     #[inline]
//     fn take(&self, count: usize) -> Self {
//         Lexer {
//             tokens: &self.tokens[0..count],
//             start: 0,
//             end: count,
//         }
//     }

//     #[inline]
//     fn take_split(&self, count: usize) -> (Self, Self) {
//         let (prefix, suffix) = self.tokens.split_at(count);
//         let first = Lexer {
//             tokens: prefix,
//             start: 0,
//             end: prefix.len(),
//         };
//         let second = Lexer {
//             tokens: suffix,
//             start: 0,
//             end: suffix.len(),
//         };
//         (second, first)
//     }
// }

// // impl<'a> InputTakeAtPosition for Lexer<'a> {
// //     type Item = Token<'a>;

// //     fn split_at_position<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
// //     where
// //         E: ParseError<Self>,
// //         P: Fn(Self::Item) -> bool,
// //     {
// //     }

// //     fn split_at_position1<P, E>(&self, predicate: P, e: ErrorKind) -> IResult<Self, Self, E>
// //     where
// //         E: ParseError<Self>,
// //         P: Fn(Self::Item) -> bool,
// //     {
// //     }

// //     fn split_at_position_complete<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
// //     where
// //         E: ParseError<Self>,
// //         P: Fn(Self::Item) -> bool,
// //     {
// //     }

// //     fn split_at_position1_complete<P, E>(
// //         &self,
// //         predicate: P,
// //         e: ErrorKind,
// //     ) -> IResult<Self, Self, E>
// //     where
// //         E: ParseError<Self>,
// //         P: Fn(Self::Item) -> bool,
// //     {
// //     }
// // }

// impl<'a> Slice<Range<usize>> for Lexer<'a> {
//     #[inline]
//     fn slice(&self, range: Range<usize>) -> Self {
//         Lexer {
//             tokens: self.tokens.slice(range),
//             start: self.start + range.start,
//             end: self.start + range.end,
//         }
//     }
// }

// impl<'a> Slice<RangeTo<usize>> for Lexer<'a> {
//     #[inline]
//     fn slice(&self, range: RangeTo<usize>) -> Self {
//         self.slice(0..range.end)
//     }
// }

// impl<'a> Slice<RangeFrom<usize>> for Lexer<'a> {
//     #[inline]
//     fn slice(&self, range: RangeFrom<usize>) -> Self {
//         self.slice(range.start..self.end - self.start)
//     }
// }

// impl<'a> Slice<RangeFull> for Lexer<'a> {
//     #[inline]
//     fn slice(&self, _: RangeFull) -> Self {
//         self.slice(..)
//     }
// }
