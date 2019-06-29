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

///
pub trait Format {
    ///
    type Dag: Dag;

    ///
    type Error;

    /// Derives a `CID` from a `Read` and an optional `Prefix`.
    fn cid<R>(blob: R, prefix: Option<Prefix>) -> Result<CID, Self::Error>
    where
        R: Read;

    /// Deserializes a `Read` into a `Dag`.
    fn decode<R>(blob: R) -> Result<Self::Dag, Self::Error>
    where
        R: Read;

    /// Serializes a `Dag` into a `Write`.
    fn encode<W>(dag: Self::Dag) -> Result<W, Self::Error>
    where
        W: Write;

    /// Deserializes a `Read` into a `Sink` of `Tokens`.
    fn decode_tokens<'a, R, S>(blob: R, sink: S) -> Result<(), Self::Error>
    where
        R: Read,
        S: Sink<SinkItem = Token<'a>>;

    /// Retrieves a `Dag` value from within a `Read`, either returning the value or a `Link` and the remaining path.
    fn resolve<R>(blob: R, path: &str) -> Result<ResolvedDag<Self::Dag>, Self::Error>
    where
        R: Read;
}

// pub trait Decode<'de>: Deserialize<'de> + Sized {
// pub trait Decode<'de>: Sized {
//     fn decode<D>(decoder: D) -> Result<Self, D::Error>
//     where
//         D: FormatDecoder<'de>;
// }

// pub trait Encode: Serialize {
// pub trait Encode {
//     fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
//     where
//         E: FormatEncoder;
// }

// pub trait FormatDecoder<'de> {
//     type Ok: IpldDag;
//     type Error: std::error::Error;
//     fn decode<D>(self, block: &[u8]) -> Result<Self::Ok, Error>
//     where
//         D: Deserializer<'de>;
// }

// pub trait FormatEncoder {
//     type Ok;
//     type Error: std::error::Error;

//     // type EncodeLink: EncodeLink<Ok = Self::Ok, Error = Self::Error>;
//     type EncodeList: EncodeList<Ok = Self::Ok, Error = Self::Error>;
//     type EncodeMap: EncodeMap<Ok = Self::Ok, Error = Self::Error>;

//     fn encode_null(self) -> Result<Self::Ok, Self::Error>;

//     fn encode_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;

//     fn encode_int(self, v: &DagInt) -> Result<Self::Ok, Self::Error>;

//     fn encode_float(self, v: &DagFloat) -> Result<Self::Ok, Self::Error>;

//     fn encode_str(self, v: &str) -> Result<Self::Ok, Self::Error>;

//     fn encode_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;

//     fn encode_link<'a>(self, v: &Link<'a>) -> Result<Self::Ok, Self::Error>;

//     fn encode_list(self, len: Option<usize>) -> Result<Self::EncodeList, Self::Error>;

//     fn encode_map(self, len: Option<usize>) -> Result<Self::EncodeMap, Self::Error>;
// }

// pub trait EncodeList {
//     type Ok;
//     type Error: std::error::Error;

//     fn encode_element(&mut self, element: &Dag) -> Result<(), Self::Error>;

//     fn end(self) -> Result<Self::Ok, Self::Error>;
// }

// pub trait EncodeMap {
//     type Ok;
//     type Error: std::error::Error;

//     fn encode_key(&mut self, key: &Dag) -> Result<(), Self::Error>;

//     fn encode_value(&mut self, value: &Dag) -> Result<(), Self::Error>;

//     fn end(self) -> Result<Self::Ok, Self::Error>;
// }
