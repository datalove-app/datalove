#![feature(specialization)]
#![recursion_limit = "1024"]

mod error;

use delegate::delegate;
use ipld_dag::{
    base::{to_name, Base, Encodable},
    format, Dag, Error, Token, CID,
};
use serde::{
    de,
    ser::{self, Serialize, Serializer},
};
use serde_json::{
    ser::{self as json_ser, Formatter, MapKeySerializer, State as CompoundState},
    Error as JsonError, Serializer as JsonSerializer,
};

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

    fn writer_mut(&mut self) -> &mut W {
        &mut self.0.writer
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

        let base = base.or(Some(Base::Base64)).unwrap();
        let base_name = to_name(base);

        let mut sv_ser = self.serialize_struct_variant("", 0, "/", 1)?;
        SV::serialize_field(&mut sv_ser, base_name, &bytes.encode(base))?;
        SV::end(sv_ser)
    }

    /// Serialize link as `{"/": <<base64_string>> }`.
    #[inline]
    fn encode_link(self, cid: &CID) -> Result<Self::Ok, Self::Error> {
        self.serialize_newtype_variant("", 0, "/", &cid.encode(Base::Base64))
    }
}

/// Implement `Serializer` by delegating to the raw `serde_json::Serializer`, overriding `serialize_bytes`.
impl<'a, W> Serializer for &'a mut Encoder<W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    type SerializeSeq = CompoundEncoder<'a, W>;
    type SerializeTuple = CompoundEncoder<'a, W>;
    type SerializeTupleStruct = CompoundEncoder<'a, W>;
    type SerializeTupleVariant = CompoundEncoder<'a, W>;
    type SerializeMap = CompoundEncoder<'a, W>;
    type SerializeStruct = CompoundEncoder<'a, W>;
    type SerializeStructVariant = CompoundEncoder<'a, W>;

    /// JSON is human readable
    fn is_human_readable(&self) -> bool {
        true
    }

    /// Serializes *all* byte sequences as `{"/": { "base64": String }}`.
    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        format::Encoder::encode_bytes(self, v, None)
    }

    delegate! {
        target self.0 {
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

            fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
            where
                T: std::fmt::Display;
        }
    }

    /**
     * The following are directly ripped and adapted from `serde_json`.
     *
     * Duplicated here b/c we need to recursively send our `Encoder` (rather than `serde_json::Serializer`).
     * This will be necessary until we can override trait method impls on types from external crates.
     */

    #[inline]
    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let iter = iter.into_iter();
        let mut seq_enc = self.serialize_seq(None)?;
        for elem in iter {
            ser::SerializeSeq::serialize_element(&mut seq_enc, &elem)?;
        }
        ser::SerializeSeq::end(seq_enc)
    }

    #[inline]
    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let iter = iter.into_iter();
        let mut map_enc = self.serialize_map(None)?;
        for (key, value) in iter {
            ser::SerializeMap::serialize_key(&mut map_enc, &key)?;
            ser::SerializeMap::serialize_value(&mut map_enc, &value)?;
        }
        ser::SerializeMap::end(map_enc)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        if len == Some(0) {
            formatter
                .begin_array(self.writer_mut())
                .map_err(JsonError::io)?;
            formatter
                .end_array(self.writer_mut())
                .map_err(JsonError::io)?;
            Ok(CompoundEncoder {
                enc: self,
                state: CompoundState::Empty,
            })
        } else {
            formatter
                .begin_array(self.writer_mut())
                .map_err(JsonError::io)?;
            Ok(CompoundEncoder {
                enc: self,
                state: CompoundState::First,
            })
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        formatter
            .begin_object(self.writer_mut())
            .map_err(JsonError::io)?;
        formatter
            .begin_object_key(self.writer_mut(), true)
            .map_err(JsonError::io)?;
        self.serialize_str(variant)?;

        formatter
            .end_object_key(self.writer_mut())
            .map_err(JsonError::io)?;
        formatter
            .begin_object_value(self.writer_mut())
            .map_err(JsonError::io)?;
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        if len == Some(0) {
            formatter
                .begin_object(self.writer_mut())
                .map_err(JsonError::io)?;
            formatter
                .end_object(self.writer_mut())
                .map_err(JsonError::io)?;
            Ok(CompoundEncoder {
                enc: self,
                state: CompoundState::Empty,
            })
        } else {
            formatter
                .begin_object(self.writer_mut())
                .map_err(JsonError::io)?;
            Ok(CompoundEncoder {
                enc: self,
                state: CompoundState::First,
            })
        }
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        formatter
            .begin_object(self.writer_mut())
            .map_err(JsonError::io)?;
        formatter
            .begin_object_key(self.writer_mut(), true)
            .map_err(JsonError::io)?;
        self.serialize_str(variant)?;

        formatter
            .end_object_key(self.writer_mut())
            .map_err(JsonError::io)?;
        formatter
            .begin_object_value(self.writer_mut())
            .map_err(JsonError::io)?;
        self.serialize_map(Some(len))
    }
}

pub struct CompoundEncoder<'a, W: 'a>
where
    W: std::io::Write,
{
    enc: &'a mut Encoder<W>,
    state: json_ser::State,
}

impl<'a, W> ser::SerializeSeq for CompoundEncoder<'a, W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, element: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut formatter = json_ser::CompactFormatter;

        formatter
            .begin_array_value(self.enc.writer_mut(), self.state == CompoundState::First)
            .map_err(JsonError::io)?;

        self.state = CompoundState::Rest;
        element.serialize(&mut *self.enc)?;

        formatter
            .end_array_value(self.enc.writer_mut())
            .map_err(JsonError::io)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        match self.state {
            CompoundState::Empty => {}
            _ => formatter
                .end_array(self.enc.writer_mut())
                .map_err(JsonError::io)?,
        }
        Ok(())
    }
}

impl<'a, W> ser::SerializeMap for CompoundEncoder<'a, W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut formatter = json_ser::CompactFormatter;

        formatter
            .begin_object_key(self.enc.writer_mut(), self.state == CompoundState::First)
            .map_err(JsonError::io)?;

        self.state = CompoundState::Rest;
        key.serialize(MapKeySerializer {
            ser: &mut self.enc.0,
        })?;

        formatter
            .end_object_key(self.enc.writer_mut())
            .map_err(JsonError::io)?;
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut formatter = json_ser::CompactFormatter;

        formatter
            .begin_object_value(self.enc.writer_mut())
            .map_err(JsonError::io)?;

        value.serialize(&mut *self.enc)?;

        formatter
            .end_object_value(self.enc.writer_mut())
            .map_err(JsonError::io)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        match self.state {
            CompoundState::Empty => {}
            _ => formatter
                .end_object(self.enc.writer_mut())
                .map_err(JsonError::io)?,
        }
        Ok(())
    }
}

// extraneous

impl<'a, W> ser::SerializeTuple for CompoundEncoder<'a, W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleStruct for CompoundEncoder<'a, W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for CompoundEncoder<'a, W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        match self.state {
            CompoundState::Empty => {}
            _ => formatter
                .end_array(self.enc.writer_mut())
                .map_err(JsonError::io)?,
        };
        formatter
            .end_object_value(self.enc.writer_mut())
            .map_err(JsonError::io)?;
        formatter
            .end_object(self.enc.writer_mut())
            .map_err(JsonError::io)?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for CompoundEncoder<'a, W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeMap::serialize_key(self, key)?;
        ser::SerializeMap::serialize_value(self, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        ser::SerializeMap::end(self)
    }
}

impl<'a, W> ser::SerializeStructVariant for CompoundEncoder<'a, W>
where
    W: std::io::Write,
{
    type Ok = <&'a mut JsonSerializer<W> as Serializer>::Ok;
    type Error = <&'a mut JsonSerializer<W> as Serializer>::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        let mut formatter = json_ser::CompactFormatter;

        match self.state {
            CompoundState::Empty => {}
            _ => formatter
                .end_object(self.enc.writer_mut())
                .map_err(JsonError::io)?,
        };
        formatter
            .end_object_value(self.enc.writer_mut())
            .map_err(JsonError::io)?;
        formatter
            .end_object(self.enc.writer_mut())
            .map_err(JsonError::io)?;
        Ok(())
    }
}

// TODO: ?? use serde_test methods to test the cases (instead of manually making json??)
#[cfg(test)]
mod tests {
    use crate::to_string;
    use ipld_dag::{
        base::{Base, Encodable},
        indexmap::IndexMap,
        Dag, CID,
    };
    use serde::Serialize;

    const EXAMPLE_CID_STR: &'static str = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";

    #[test]
    fn test_bytes() {
        let bytes: Vec<u8> = vec![0, 1, 2, 3];
        let expected = make_bytes_json(&bytes);

        let dag = Dag::ByteBuf(bytes, None);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cid() {
        let expected = make_cid_json(EXAMPLE_CID_STR);

        let dag = Dag::Link(example_cid(), None);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bytes_vec() {
        let bytes: Vec<u8> = vec![0, 1, 2, 3];
        let expected = make_vec2_json(&make_bytes_json(&bytes));

        let map_elem = Dag::ByteBuf(bytes, None);
        let dag = Dag::List(vec![map_elem.clone(), map_elem]);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_cid_vec() {
        let expected = make_vec2_json(&make_cid_json(EXAMPLE_CID_STR));

        let map_elem = Dag::Link(example_cid(), None);
        let dag = Dag::List(vec![map_elem.clone(), map_elem]);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Example<'a> {
            #[serde(with = "serde_bytes")]
            some_bytes: &'a [u8],
            a_link: CID,
        }

        let bytes = vec![0, 1, 2, 3];
        let expected = format!(
            r#"{{"some_bytes":{bytes},"a_link":{cid}}}"#,
            bytes = make_bytes_json(&bytes),
            cid = make_cid_json(EXAMPLE_CID_STR)
        );

        let example = Example {
            some_bytes: &bytes,
            a_link: example_cid(),
        };
        let actual = to_string(&example).unwrap();
        assert_eq!(expected, actual)
    }

    fn example_cid() -> CID {
        EXAMPLE_CID_STR.parse().unwrap()
    }

    fn make_bytes_json(bytes: &[u8]) -> String {
        let byte_str = bytes.encode(Base::Base64);
        format!(r#"{{"/":{{"base64":"{bytes}"}}}}"#, bytes = byte_str)
    }

    fn make_cid_json(cid_str: &str) -> String {
        format!(r#"{{"/":"{cid}"}}"#, cid = cid_str)
    }

    fn make_vec2_json(s: &str) -> String {
        format!(r#"[{elem1},{elem2}]"#, elem1 = s, elem2 = s)
    }
}
