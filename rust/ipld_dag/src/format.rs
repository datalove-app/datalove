use crate::{Dag, Link, Prefix, Token, CID};
use futures::{Sink, Stream};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

///
#[derive(Deserialize, Serialize)]
pub enum ResolvedDag<'a, D: Dag> {
    Final(D),
    Link {
        // link: Link<D>,
        remaining_path: &'a str,
    },
}

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

    /// Retrieves a `Dag` value from within a `Read`, either returning the value or a `Link` and the remaining path.
    fn resolve<D, R>(blob: R, path: &str) -> Result<ResolverResult<D>, Self::Error>
    where
        D: Dag,
        R: Read;
}

///
pub trait Encode {
    ///
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
        <E as serde::Serializer>::Error: Into<Error>;
}

impl<T> Encode for T
where
    T: serde::Serialize,
{
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
        <E as serde::Serializer>::Error: Into<Error>,
    {
        self.serialize(encoder)
    }
}

///
pub trait Encoder: Sized + serde::Serializer
where
    <Self as serde::Serializer>::Error: Into<Error>,
{
    ///
    type EncodeList: EncodeList;

    ///
    type EncodeMap: EncodeMap;

    ///
    fn encode_bytes(self, bytes: &[u8], base: Option<Base>) -> Result<Self::Ok, Self::Error> {
        match base {
            None => self.serialize_bytes(bytes),
            Some(base) => self.serialize_str(&Encodable::encode(bytes, base)),
        }
    }

    /// Encodes a `CID` as bytes if `multibase::Base` is missing, otherwise as a string.
    fn encode_link(self, cid: &CID) -> Result<Self::Ok, Self::Error> {
        match cid.base() {
            None => self.serialize_bytes(&cid.to_vec()),
            Some(base) => self.serialize_str(&cid.to_string(None)),
        }
    }

    ///
    fn encode_list(self, len: Option<usize>) -> Result<Self::EncodeList, Self::Error>;

    ///
    fn encode_map(self, len: Option<usize>) -> Result<Self::EncodeMap, Self::Error>;

    // fn encode_linked_dag(self, )
}

///
pub trait EncodeList {
    ///
    type Ok;

    ///
    type Error: Into<Error>;

    ///
    fn encode_element<T>(&mut self, element: &T) -> Result<(), Self::Error>
    where
        T: Encode;

    ///
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

///
pub trait EncodeMap {
    ///
    type Ok;

    ///
    type Error: Into<Error>;

    ///
    fn encode_key(&mut self, key: &Key) -> Result<(), Self::Error>;

    ///
    fn encode_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Encode;

    ///
    fn end(self) -> Result<Self::Ok, Self::Error>;
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
