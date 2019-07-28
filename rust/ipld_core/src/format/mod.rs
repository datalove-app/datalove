//!

mod encoder;
mod token;

pub use self::{encoder::Encoder, token::Token};
use crate::{base::Base, Error, Node, CID};
use futures::{Sink, Stream};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::{Read, Write};

// ///
// #[derive(Deserialize, Serialize)]
// pub enum ResolverResult<'a, D: Dag> {
//     Final(D),
//     Link {
//         // link: Link<D>,
//         remaining_path: &'a str,
//     },
// }

/// Represents an IPLD Dag Format
///
/// [`IPLD`]
/// [`Dag`]
/// [`Format`]
pub trait Format<'de> {
    // type Encoder: Encoder;
    // type Decoder: Decoder<'de>;

    ///
    type Error;

    // /// Derives a `CID` from a `Read` and an optional `Prefix`.
    // fn get_cid<R>(blob: R, prefix: Option<Prefix>) -> Result<CID, Self::Error>
    // where
    //     R: Read;

    // /// Serializes a `Dag` into a `Write`.
    // fn encode<D, W>(dag: D) -> Result<W, Self::Error>
    // where
    //     D: Dag,
    //     W: Write;

    // /// Deserializes a `Read` into a `Dag`.
    // fn decode<D, R>(blob: R) -> Result<D, Self::Error>
    // where
    //     D: Dag,
    //     R: Read;

    // /// Deserializes a `Read` into a `Sink` of `Tokens`.
    // fn decode_tokens<'a, R, S>(blob: R, sink: S) -> Result<(), Self::Error>
    // where
    //     R: Read,
    //     S: Sink<SinkItem = Token<'a>>;

    // /// Retrieves a `Dag` value from within a `Read`, either returning the value or a `Link` and the remaining path.
    // fn resolve<D, R>(blob: R, path: &str) -> Result<ResolverResult<D>, Self::Error>
    // where
    //     D: Dag,
    //     R: Read;
}

///
pub trait Decode<'de>: Sized {
    ///
    fn decode<D>(&self, decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder<'de>;

    ///
    fn decode_tokens<D, S>(&self, decoder: D, sink: S) -> Result<(), D::Error>
    where
        D: Decoder<'de>,
        S: Sink<SinkItem = Token<'de>>;
}

///
pub trait Decoder<'de>: Sized {
    ///
    type Ok;

    ///
    type Error: From<Error>;

    ///
    fn decode<R>(self, blob: R) -> Result<(), Self::Error>
    where
        R: Read;
}
