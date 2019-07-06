#![feature(specialization)]
#![recursion_limit = "1024"]

mod error;

use delegate::delegate;
use ipld_dag::{
    base::{to_name, Base, Encodable},
    format, Error, Token, CID,
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

pub struct Encoder<W: std::io::Write>(JsonSerializer<W>);

#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, JsonError>
where
    T: Serialize,
{
    let mut writer = Vec::new();
    let mut ser = Encoder::new(&mut writer);
    value.serialize(&mut ser)?;
    Ok(writer)
}

#[inline]
pub fn to_string<T>(value: &T) -> Result<String, JsonError>
where
    T: Serialize,
{
    let writer = to_vec(value)?;
    let string = unsafe { String::from_utf8_unchecked(writer) };
    Ok(string)
}

impl<W> Encoder<W>
where
    W: std::io::Write,
{
    fn new(writer: W) -> Self {
        Encoder(JsonSerializer::new(writer))
    }
}

///
impl<'a, W> format::Encoder for &'a mut Encoder<W>
where
    W: std::io::Write,
{
    /// Serialize bytes as `{"/": { "base64": <<base64_string>> }}`.
    #[inline]
    fn encode_bytes(self, bytes: &[u8], base: Option<Base>) -> Result<Self::Ok, Self::Error> {
        use ser::SerializeStructVariant as SV;
        println!("\n|||||> JSON bytes: {:?} {:?}", bytes, base);

        let base = base.or(Some(Base::Base64)).unwrap();
        let base_name_str = to_name(base);
        let byte_str = bytes.encode(base);

        let mut sv_ser = self.serialize_struct_variant("", 0, "/", 1)?;
        SV::serialize_field(&mut sv_ser, base_name_str, &byte_str)?;
        SV::end(sv_ser)
    }

    /// Serialize link as `{"/": <<base64_string>> }`.
    #[inline]
    fn encode_link(self, cid: &CID) -> Result<Self::Ok, Self::Error> {
        println!("\n|||||> JSON link: {:?}", cid);
        self.serialize_newtype_variant("", 0, "/", &cid.encode(Base::Base64))
    }
}

/// `impl Serializer` by delegating to the raw `serde_json::Serializer`, overriding `serialize_bytes`.
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
    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        format::Encoder::encode_bytes(self, v, None)
    }

    delegate! {
        target self.0 {
            // fn is_human_readable(&self) -> bool;

            fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
            fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
            fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
            fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
            fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
            fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error>;
            fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
            fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
            fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
            fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
            fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error>;
            fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
            fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
            fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
            fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
            fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
            fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error>;

            fn serialize_unit_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str
            ) -> Result<Self::Ok, Self::Error>;

            fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize;

            fn serialize_newtype_variant<T: ?Sized>(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                value: &T
            ) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize;

            fn serialize_none(self) -> Result<Self::Ok, Self::Error>;

            fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
            where
                T: Serialize;

            fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;

            fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;

            fn serialize_tuple_struct(
                self,
                _name: &'static str,
                len: usize
            ) -> Result<Self::SerializeTupleStruct, Self::Error>;

            fn serialize_tuple_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                len: usize
            ) -> Result<Self::SerializeTupleVariant, Self::Error>;

            fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;

            fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error>;

            fn serialize_struct_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                len: usize
            ) -> Result<Self::SerializeStructVariant, Self::Error>;

            fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
            where
                I: IntoIterator,
                <I as IntoIterator>::Item: Serialize;

            fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
            where
                K: Serialize,
                V: Serialize,
                I: IntoIterator<Item = (K, V)>;

            fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
            where
                T: std::fmt::Display;
        }
    }
}

// TODO: ?? use serde_test methods to test the cases (instead of manually making json??)
#[cfg(test)]
mod tests {
    use crate::to_string;
    use ipld_dag::{
        base::{Base, Encodable},
        Dag, CID,
    };
    // use serde_json::to_string;

    const CID_STR: &'static str = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";

    #[test]
    fn test_bytes() {
        let bytes: Vec<u8> = vec![0, 1, 2, 3];
        let byte_str = &bytes.encode(Base::Base58btc);
        let expected = make_bytes_json(byte_str);

        let dag = Dag::ByteBuf(bytes, None);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cid() {
        let expected = make_cid_json(CID_STR);

        let cid: CID = CID_STR.parse().unwrap();
        let dag = Dag::Link(cid, None);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_vec() {
        let link = make_cid_json(CID_STR);
        let expected = format!(r#"[{},{}]"#, &link, &link);

        let cid: CID = CID_STR.parse().unwrap();
        let link = Dag::Link(cid, None);
        let dag = Dag::List(vec![link.clone(), link]);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual)
    }

    fn make_bytes_json(byte_str: &str) -> String {
        format!(r#"{{"/":{{"base64":"{bytes}"}}}}"#, bytes = byte_str)
    }

    fn make_cid_json(cid_str: &str) -> String {
        format!(r#"{{"/":"{cid}"}}"#, cid = cid_str)
    }
}
