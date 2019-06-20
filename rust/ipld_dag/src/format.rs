use crate::{Dag, DagFloat, DagInt, Error, Link};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

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
