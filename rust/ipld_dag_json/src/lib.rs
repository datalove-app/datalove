#![recursion_limit = "512"]

mod error;

use crate::error::{j2i_de_err, j2i_ser_err, key_must_be_a_string};
// use base64::{display::Base64Display, encode as b64Encode};
use delegate::delegate;
use ipld_dag::{
    format,
    indexmap::indexmap,
    multibase::{Base, Encodable},
    Dag, Error, Token, CID,
};
use serde::{
    de,
    ser::{self, Serialize, Serializer},
};
use serde_json::{ser as json_ser, Error as JsonError, Serializer as JsonSerializer};

// ***** TODO: newest idea: *****
// ***** TODO *****
// ***** TODO *****
//  - each format exposes a Serializer / Deserializer for that format
//      - allows for any user-defined type (implementing the Dag trait) to be auto-serialized according to the Format's rules easily
//      - allows us to more easily/performantly implement selectors (by directly transcoding from a block to a token stream
//          - and eventually to a FormatDag/RawDag (or a Link?)
//  - each format [optionally] exposes an abstract Dag struct
//  --- it implements Serialize/Deserialize custom to it's serializer
//  --- it implements From<Dag> for FormatDag (so it can be translated between Serialize/Deserialize impls)
// pub struct JsonDag(RawDag<JsonDag>);

// impl<I, O> From<I> for O where I: Dag<I>, O: Dag<JsonDag> {
//     fn from(dag: I) -> Self {

//     }
// }

// impl<'a> From<RawDag<'a, JsonDag<'a>>> for JsonDag<'a> {
//     fn from(dag: RawDag<'a, JsonDag<'a>>) -> Self {
//     }
// }

// impl<'a, T: Dag + Into<JsonDag<'a>>> From<RawDag<'a, T>> for JsonDag<'a> {
//     fn from(dag: RawDag<'a, T>) -> Self {
//         JsonDag(dag)
//     }
// }

pub struct Encoder<W: std::io::Write>(JsonSerializer<W>);

///
impl<'a, W> format::Encoder for &'a mut Encoder<W>
where
    W: std::io::Write,
{
    ///
    type EncodeList = ListEncoder;

    ///
    type EncodeMap = MapEncoder;

    /// Encodes CID bytes as `{"/": String}`.
    fn encode_link(self, v: &CID) -> Result<Self::Ok, Self::Error> {
        self.serialize_newtype_variant("", 0, "/", &v.encode(Base::Base64))
    }

    ///
    fn encode_list(self, len: Option<usize>) -> Result<Self::EncodeList, Self::Error> {
        match self.serialize_seq(len) {
            Ok(json_ser::Compound::Map { ser: _, state }) => Ok(CompoundEncoder::new(self, state)),
            _ => Err(Error::Serialization("".to_string())),
        }
    }

    ///
    fn encode_map(self, len: Option<usize>) -> Result<Self::EncodeMap, Self::Error> {
        Ok(MapEncoder)
    }
}

pub struct ListEncoder;

impl format::EncodeList for ListEncoder {
    ///
    type Ok = ();

    ///
    type Error = serde_json::error::Error;

    ///
    fn encode_element<T>(&mut self, element: &T) -> Result<(), Self::Error>
    where
        T: format::Encode,
    {

    }

    ///
    fn end(self) -> Result<Self::Ok, Self::Error> {}
}

pub struct MapEncoder;

impl format::EncodeMap for MapEncoder {
    ///
    type Ok = ();

    ///
    type Error = serde_json::error::Error;

    ///
    fn encode_key(&mut self, key: &Key) -> Result<(), Self::Error> {}

    ///
    fn encode_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: format::Encode,
    {

    }

    ///
    fn end(self) -> Result<Self::Ok, Self::Error> {}
}

///
/// `impl Serializer` by delegating to the raw `serde_json::Serializer`.
impl<'a, W> Serializer for &'a mut Encoder<W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    type SerializeSeq = <&'a mut JsonSerializer<W> as Serializer>::SerializeSeq;
    type SerializeTuple = <&'a mut JsonSerializer<W> as Serializer>::SerializeTuple;
    type SerializeTupleStruct = <&'a mut JsonSerializer<W> as Serializer>::SerializeTupleStruct;
    type SerializeTupleVariant = <&'a mut JsonSerializer<W> as Serializer>::SerializeTupleVariant;
    type SerializeMap = <&'a mut JsonSerializer<W> as Serializer>::SerializeMap;
    type SerializeStruct = <&'a mut JsonSerializer<W> as Serializer>::SerializeStruct;
    type SerializeStructVariant = <&'a mut JsonSerializer<W> as Serializer>::SerializeStructVariant;

    /// Serializes bytes as `{"/": { "base64": String }}`.
    fn serialize_bytes(self, v: &[u8]) -> Result<(), JsonError> {
        use ser::SerializeStructVariant as SV;

        let mut sv_ser = self.serialize_struct_variant("", 0, "/", 1)?;
        SV::serialize_field(&mut sv_ser, "base64", &v.encode(Base::Base64))?;
        SV::end(sv_ser)
    }

    delegate! {
        target self.0 {
            fn serialize_bool(self, v: bool) -> Result<(), JsonError>;
            fn serialize_i8(self, v: i8) -> Result<(), JsonError>;
            fn serialize_i16(self, v: i16) -> Result<(), JsonError>;
            fn serialize_i32(self, v: i32) -> Result<(), JsonError>;
            fn serialize_i64(self, v: i64) -> Result<(), JsonError>;
            fn serialize_i128(self, v: i128) -> Result<(), JsonError>;
            fn serialize_u8(self, v: u8) -> Result<(), JsonError>;
            fn serialize_u16(self, v: u16) -> Result<(), JsonError>;
            fn serialize_u32(self, v: u32) -> Result<(), JsonError>;
            fn serialize_u64(self, v: u64) -> Result<(), JsonError>;
            fn serialize_u128(self, v: u128) -> Result<(), JsonError>;
            fn serialize_f32(self, v: f32) -> Result<(), JsonError>;
            fn serialize_f64(self, v: f64) -> Result<(), JsonError>;
            fn serialize_char(self, v: char) -> Result<(), JsonError>;
            fn serialize_str(self, v: &str) -> Result<(), JsonError>;

            // TODO: override this above this macro block
            fn serialize_unit(self) -> Result<(), JsonError>;
            fn serialize_unit_struct(self, _name: &'static str) -> Result<(), JsonError>;
            fn serialize_unit_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str
            ) -> Result<(), JsonError>;
            fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<(), JsonError>
            where
                T: Serialize;
            fn serialize_newtype_variant<T: ?Sized>(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                value: &T
            ) -> Result<(), JsonError>
            where
                T: Serialize;

            fn serialize_none(self) -> Result<(), JsonError>;

            fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), JsonError>
            where
                T: Serialize;

            fn serialize_seq(self, len: Option<usize>) -> Result<<Encoder<W> as Serializer>::SerializeSeq, JsonError>;

            fn serialize_tuple(self, len: usize) -> Result<<Encoder<W> as Serializer>::SerializeTuple, JsonError>;

            fn serialize_tuple_struct(
                self,
                _name: &'static str,
                len: usize
            ) -> Result<<Encoder<W> as Serializer>::SerializeTupleStruct, JsonError>;

            fn serialize_tuple_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                len: usize
            ) -> Result<<Encoder<W> as Serializer>::SerializeTupleVariant, JsonError>;

            fn serialize_map(self, len: Option<usize>) -> Result<<Encoder<W> as Serializer>::SerializeMap, JsonError>;

            fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Encoder<W> as Serializer>::SerializeStruct, JsonError>;

            fn serialize_struct_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                len: usize
            ) -> Result<<Encoder<W> as Serializer>::SerializeStructVariant, JsonError>;

            fn collect_str<T: ?Sized>(self, value: &T) -> Result<<Encoder<W> as Serializer>::Ok, JsonError>
            where
                T: std::fmt::Display;
        }
    }
}

// impl Dag for JsonDag {
//     fn get_type(&self) -> Token {
//         Token::Null
//     }
// }

/// Serialization behaviour is almost identical to the standard JSON format, with a few exceptions:
///     - encodes bytes as `{"/": { "base64": String }}`
///     - encodes a CID as `{"/": String}`
// impl Serialize for JsonDag {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match &self.0 {
//             // Serialization unique to the DagJSON format
//             // Encodes bytes as `{"/": { "base64": String }}`
//             RawDag::ByteBuf(bytes, _base) => {
//                 let mut ser = serializer.serialize_struct_variant("", 0, "/", 1)?;
//                 // todo: should this be configurable?
//                 let (base, base_key) = (&Base::Base64, "base64");
//                 ser.serialize_field(base_key, &bytes.encode(*base))?;
//                 ser.end()
//             }
//             // Encodes CID bytes as `{"/": String}`
//             RawDag::Link(link) => match link {
//                 Link::Dag(dag) => (*dag).serialize(serializer),
//                 Link::CID(cid) => {
//                     let cid_str = cid.to_string(Some(Base::Base64));
//                     serializer.serialize_newtype_variant("CID", 0, "/", &cid_str)
//                 }
//             },

//             // Serialization identical to the default format
//             _ => Dag::serialize(self, serializer),
//         }
//     }
// }

// TODO: ?? use serde_test methods to test the cases (instead of manually making json??)
#[cfg(test)]
mod tests {
    // use crate::JsonDag;
    // use ipld_dag::{
    //     multibase::{encode as mb_encode, Base},
    //     Link, RawDag, CID,
    // };
    // use serde_json::to_string;

    // const CID_STR: &'static str = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";

    // #[test]
    // fn test_bytes() {
    //     let bytes: Vec<u8> = vec![0, 1, 2, 3];
    //     let byte_str = mb_encode(Base::Base64, &bytes);
    //     let dag: JsonDag = JsonDag(RawDag::ByteBuf(bytes, None));

    //     let expected = format!(
    //         r#"{{"/":{{"base64":"{bytes}"}}}}"#,
    //         bytes = byte_str.to_string(),
    //     );
    //     let actual = to_string(&dag).unwrap();
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // fn test_cid() {
    //     let cid: CID = CID_STR.parse().unwrap();
    //     let dag = JsonDag(RawDag::Link(Link::CID(cid)));

    //     let expected = make_cid(CID_STR);
    //     let actual = to_string(&dag).unwrap();
    //     assert_eq!(expected, actual);
    // }

    // fn make_cid(cid_str: &str) -> String {
    //     format!(
    //         r#"{{"{key}":"{cid}"}}"#,
    //         key = r#"/"#,
    //         cid = cid_str.to_string(),
    //     )
    // }

    // #[test]
    // fn test_vec() {
    //     let cid: CID = CID_STR.parse().unwrap();
    //     let dag: JsonDag = JsonDag(RawDag::List(vec![
    //         JsonDag(RawDag::Link(Link::CID(cid.clone()))),
    //         JsonDag(RawDag::Link(Link::CID(cid))),
    //     ]));

    //     let link = make_cid(CID_STR);
    //     let expected = format!(r#"[{},{}]"#, &link, &link);
    //     let actual = to_string(&dag).unwrap();
    //     assert_eq!(expected, actual)
    // }
}
