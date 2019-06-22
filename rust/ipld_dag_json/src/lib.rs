mod error;

// use crate::error::{j2i_de_err, j2i_ser_err, key_must_be_a_string};
// use base64::{display::Base64Display, encode as b64Encode};
use ipld_dag::{
    cid::CID,
    dag::{Dag, DagNode, Link},
};
use multibase::{encode, Base};
use serde::{
    de,
    ser::{self, Serialize, SerializeStructVariant, Serializer},
};

// TODO: === two problems to contend with ===
// 1. Dag variants dont map one-to-one to serde's data model
// 2. serialize recurses - for encode to recurse, we'll have to hijack the recursive step

// new idea - attach a context to the encoder when starting to serialize a compound, which allows any serializer_method to know something about it's parent
// thus within serialize_bytes, we know if it is a regular binary or a cid binary
// then make sure to reset the context in serialize_(list, map) after ending the compound (or in Serialize(List, Map).end())?

// TODO: newest idea:
//  - each format exposes functions tailored to it's specific struct and Serializer/Deserializer
//  - each format exposes a Dag struct
//  --- it implements Serialize/Deserialize custom to it's serializer
//  --- it implements From<Dag> for FormatDag (so it can be translated between Serialize/Deserialize impls)
pub struct JsonDag<'a>(DagNode<'a, JsonDag<'a>>);

impl<'a> From<DagNode<'a, JsonDag<'a>>> for JsonDag<'a> {
    fn from(dag: DagNode<'a, JsonDag<'a>>) -> Self {
        JsonDag(dag)
    }
}

// impl<'a, T: Dag> From<DagNode<'a, T>> for JsonDag<'a> {
//     fn from(dag: DagNode<'a, T>) -> Self {
//         match dag {
//             DagNode::Null => dag.into(),
//             DagNode::Bool(_) => dag.into(),
//             DagNode::Integer(int) => dag.into(),
//             DagNode::Float(float) => dag.into(),
//             DagNode::Str(s) => dag.into(),
//             DagNode::Bytes(b, t) => dag.into(),
//             DagNode::Link(link) => match link {
//                 Link::CID(cid) => dag.into(),
//                 Link::Dag(dag) => {
//                     let new_dag = Box::new(JsonDag::from(*dag));
//                     JsonDag(DagNode::Link(Link::Dag(new_dag))
//                 }
//             }
//         }
//     }
// }

impl<'a> Dag for JsonDag<'a> {}

// impl<'a> Serialize for Link<JsonDag<'a>> {

// }

impl<'a> Serialize for JsonDag<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            // Serialization identical to the default format
            DagNode::Null => self.0.serialize(serializer),
            DagNode::Bool(_) => self.0.serialize(serializer),
            DagNode::Integer(_) => self.0.serialize(serializer),
            DagNode::Float(_) => self.0.serialize(serializer),
            DagNode::Str(_) => self.0.serialize(serializer),
            DagNode::List(_) => self.0.serialize(serializer),
            DagNode::Map(_) => self.0.serialize(serializer),

            // Serialization unique to the DagJSON format
            // Encodes bytes as `{"/": { "base64": String }}`
            DagNode::Bytes(bytes, _base) => {
                // todo: should this be configurable?
                let (base, base_key) = (&Base::Base64, "base64");
                let bytes_str = &encode(*base, bytes);
                let mut ser = serializer.serialize_struct_variant("", 0, "/", 1)?;
                ser.serialize_field(base_key, bytes_str)?;
                ser.end()
            }
            // Encodes CID bytes as `{"/": String}`
            DagNode::Link(link) => match link {
                Link::Dag(dag) => (*dag).serialize(serializer),
                Link::CID(cid) => {
                    serializer.serialize_newtype_variant("CID", 0, "/", &cid.to_string())
                }
            },
        }
    }
}

///
/// ```edition2018
/// use ipld_dag::{Dag, Link};
/// use ipld_dag_json::{Deserializer, Serializer};
///
/// let dag: Dag = Deserializer::deserialize()
///
///
/// ```
// pub struct Decoder<R>(json_de::Deserializer<R>);
// pub enum EncoderContext {
//     Bytes,
//     CID,
//     Dag,
// }

///
///
// pub struct Encoder<W> {
//     ser: json_ser::Serializer<W>,
// }

// // TODO: impl From for Read supported types
// impl<'de, R: json_de::Read<'de>> Decoder<R> {
//     fn from_reader(reader: R) -> Self {
//         Decoder(json_de::Deserializer::new(reader))
//     }
// }

// // TODO: impl From for Write supported types
// impl<W: Write> Encoder<W> {
//     fn from_writer(writer: W) -> Self {
//         let ser = json_ser::Serializer::with_formatter(writer, json_ser::CompactFormatter);
//         Encoder { ser: ser }
//     }

//     fn writer_mut(&mut self) -> &mut W {
//         &mut self.ser.writer
//     }
// }

// impl<'a, W> FormatEncoder for &'a mut Encoder<W>
// where
//     W: Write,
// {
//     type Ok = ();
//     type Error = Error;

//     // type EncodeLink = LinkEncoder;
//     type EncodeList = CompoundEncoder<'a, W>;
//     type EncodeMap = CompoundEncoder<'a, W>;

//     fn encode_null(self) -> Result<Self::Ok, Self::Error> {
//         self.ser.serialize_unit().map_err(j2i_ser_err)
//     }

//     fn encode_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
//         self.ser.serialize_bool(v).map_err(j2i_ser_err)
//     }

//     fn encode_int(self, v: &DagInt) -> Result<Self::Ok, Self::Error> {
//         let res = match v {
//             DagInt::U8(num) => self.ser.serialize_u8(*num),
//             DagInt::U16(num) => self.ser.serialize_u16(*num),
//             DagInt::U32(num) => self.ser.serialize_u32(*num),
//             DagInt::U64(num) => self.ser.serialize_u64(*num),
//             DagInt::U128(num) => self.ser.serialize_u128(*num),
//             DagInt::I8(num) => self.ser.serialize_i8(*num),
//             DagInt::I16(num) => self.ser.serialize_i16(*num),
//             DagInt::I32(num) => self.ser.serialize_i32(*num),
//             DagInt::I64(num) => self.ser.serialize_i64(*num),
//             DagInt::I128(num) => self.ser.serialize_i128(*num),
//         };

//         res.map_err(j2i_ser_err)
//     }

//     fn encode_float(self, v: &DagFloat) -> Result<Self::Ok, Self::Error> {
//         let res = match v {
//             DagFloat::F32(num) => self.ser.serialize_f32(*num),
//             DagFloat::F64(num) => self.ser.serialize_f64(*num),
//         };

//         res.map_err(j2i_ser_err)
//     }

//     fn encode_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
//         self.ser.serialize_str(v).map_err(j2i_ser_err)
//     }

//     /// Encodes bytes as `{"/": { "base64": String }}`.
//     fn encode_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
//         use ser::SerializeStructVariant as SV;

//         let byte_string = &encode(Base::Base64, v);
//         let mut sv_ser = self
//             .ser
//             .serialize_struct_variant("", 0, "/", 1)
//             .map_err(j2i_ser_err)?;
//         SV::serialize_field(&mut sv_ser, "base64", byte_string).map_err(j2i_ser_err)?;
//         SV::end(sv_ser).map_err(j2i_ser_err)
//     }

//     /// Encodes CID bytes as `{"/": String}`.
//     fn encode_link<'b>(self, v: &Link<'b>) -> Result<Self::Ok, Self::Error> {
//         match v {
//             Link::Dag(dag) => (*dag).encode(self),
//             Link::CID(cid) => {
//                 use ser::SerializeMap as Map;

//                 let byte_string = &encode(Base::Base64, &cid.to_vec());
//                 let mut map_ser = self.ser.serialize_map(Some(1)).map_err(j2i_ser_err)?;
//                 Map::serialize_key(&mut map_ser, "/").map_err(j2i_ser_err)?;
//                 Map::serialize_value(&mut map_ser, byte_string).map_err(j2i_ser_err)?;
//                 Map::end(map_ser).map_err(j2i_ser_err)
//             }
//         }
//     }

//     fn encode_list(self, len: Option<usize>) -> Result<Self::EncodeList, Self::Error> {
//         match self.ser.serialize_seq(len).map_err(j2i_ser_err) {
//             Ok(json_ser::Compound::Map { ser: _, state }) => Ok(CompoundEncoder::new(self, state)),
//             _ => Err(Error::Serialization("".to_string())),
//         }
//     }

//     fn encode_map(self, len: Option<usize>) -> Result<Self::EncodeMap, Self::Error> {
//         match self.ser.serialize_map(len).map_err(j2i_ser_err) {
//             Ok(json_ser::Compound::Map { ser: _, state }) => Ok(CompoundEncoder::new(self, state)),
//             _ => Err(Error::Serialization("".to_string())),
//         }
//     }
// }

// pub struct CompoundEncoder<'a, W: 'a> {
//     encoder: &'a mut Encoder<W>,
//     formatter: json_ser::CompactFormatter,
//     state: json_ser::State,
// }

// impl<'a, W> CompoundEncoder<'a, W>
// where
//     W: Write,
// {
//     fn new(encoder: &'a mut Encoder<W>, state: json_ser::State) -> Self {
//         CompoundEncoder {
//             encoder: encoder,
//             formatter: json_ser::CompactFormatter,
//             state: state,
//         }
//     }
// }

// // Adapted directly from `serde_json::ser::Compound`
// impl<'a, W> EncodeList for CompoundEncoder<'a, W>
// where
//     W: Write,
// {
//     type Ok = ();
//     type Error = Error;

//     #[inline]
//     fn encode_element(&mut self, element: &Dag) -> Result<(), Self::Error> {
//         self.formatter
//             .begin_array_value(
//                 self.encoder.writer_mut(),
//                 self.state == json_ser::State::First,
//             )
//             .map_err(serde_json::Error::io)
//             .map_err(j2i_ser_err)?;

//         self.state = json_ser::State::Rest;
//         element.encode(&mut *self.encoder)?;

//         self.formatter
//             .end_array_value(self.encoder.writer_mut())
//             .map_err(serde_json::Error::io)
//             .map_err(j2i_ser_err)?;
//         Ok(())
//     }

//     #[inline]
//     fn end(self) -> Result<(), Self::Error> {
//         let mut formatter = json_ser::CompactFormatter;
//         match self.state {
//             json_ser::State::Empty => {}
//             _ => formatter
//                 .end_array(self.encoder.writer_mut())
//                 .map_err(serde_json::Error::io)
//                 .map_err(j2i_ser_err)?,
//         }
//         Ok(())
//     }
// }

// impl<'a, W> EncodeMap for CompoundEncoder<'a, W>
// where
//     W: Write,
// {
//     type Ok = ();
//     type Error = Error;

//     // TODO: refactor this to take in a Dag, rather than an E
//     // todo: this way, we can match directly against the internal data of the dag, and then let json_ser call serialize on it (since we dont need recursion here, we can delegate directly to the wrapped impl)
//     // key can be:
//     // char, int, uint, str
//     // unit variant (as str)
//     // newtype struct (as ...?)
//     #[inline]
//     fn encode_key(&mut self, key: &Dag) -> Result<(), Self::Error> {
//         self.formatter
//             .begin_object_key(
//                 self.encoder.writer_mut(),
//                 self.state == json_ser::State::First,
//             )
//             .map_err(serde_json::Error::io)
//             .map_err(j2i_ser_err)?;

//         self.state = json_ser::State::Rest;
//         match *key {
//             Dag::Str(s) => self.encoder.encode_str(s),
//             Dag::Integer(ref int) => {
//                 self.formatter
//                     .begin_string(self.encoder.writer_mut())
//                     .map_err(serde_json::Error::io)
//                     .map_err(j2i_ser_err)?;

//                 self.encoder.encode_int(int)?;

//                 self.formatter
//                     .end_string(self.encoder.writer_mut())
//                     .map_err(serde_json::Error::io)
//                     .map_err(j2i_ser_err)?;

//                 Ok(())
//             }
//             _ => Err(key_must_be_a_string()),
//         }?;

//         self.formatter
//             .end_object_key(self.encoder.writer_mut())
//             .map_err(serde_json::Error::io)
//             .map_err(j2i_ser_err)?;

//         Ok(())
//     }

//     #[inline]
//     fn encode_value(&mut self, value: &Dag) -> Result<(), Self::Error> {
//         self.formatter
//             .begin_object_value(self.encoder.writer_mut())
//             .map_err(serde_json::Error::io)
//             .map_err(j2i_ser_err)?;

//         value.encode(&mut *self.encoder)?;

//         self.formatter
//             .end_object_value(self.encoder.writer_mut())
//             .map_err(serde_json::Error::io)
//             .map_err(j2i_ser_err)?;
//         Ok(())
//     }

//     #[inline]
//     fn end(self) -> Result<(), Self::Error> {
//         let mut formatter = json_ser::CompactFormatter;
//         match self.state {
//             json_ser::State::Empty => {}
//             _ => formatter
//                 .end_object(self.encoder.writer_mut())
//                 .map_err(serde_json::Error::io)
//                 .map_err(j2i_ser_err)?,
//         }
//         Ok(())
//     }
// }

// impl<R> FormatDecoder for Deserializer<R> {
//     fn decode<'de, D>(self, block: &[u8]) -> Result<Dag, Error>
//     where
//         D: Deserializer<'de>,
//     {
//         Ok(Dag::Null)
//     }
// }

// impl<'a, W, F> ser::Serializer for &'a mut Encoder<'a, W, F>
// where
//     W: Write,
//     F: json_ser::Formatter,
// {
//     type Ok = ();
//     type Error = Error;

//     type SerializeSeq = Compound<'a, W, F>;
//     type SerializeTuple = Compound<'a, W, F>;
//     type SerializeTupleStruct = Compound<'a, W, F>;
//     type SerializeTupleVariant = Compound<'a, W, F>;
//     type SerializeMap = Compound<'a, W, F>;
//     type SerializeStruct = Compound<'a, W, F>;
//     type SerializeStructVariant = Compound<'a, W, F>;

//     #[inline]
//     fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_bool(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_i16(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_i32(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_i64(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_u8(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_u16(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_u32(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_u64(v).map_err(j2i_ser_err)
//     }

//     serde_if_integer128! {
//         fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
//             self.0.serialize_i128(v).map_err(j2i_ser_err)
//         }

//         fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
//             self.0.serialize_u128(v).map_err(j2i_ser_err)
//         }
//     }

//     #[inline]
//     fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_f32(v).map_err(j2i_ser_err)
//     }

//     fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_f64(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_char(v).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_str(v).map_err(j2i_ser_err)
//     }

//     // TODO: write to a writer, avoid the intermediary string
//     /// Serialize bytes as `{"/": { "base64": String }}`.
//     /// Serialize CID bytes as `{"/": String}`.
//     #[inline]
//     fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
//         let current_context = self.1;
//         let byte_string = &encode(Base::Base64, v);

//         match current_context {
//             EncoderContext::Bytes => {
//                 use ser::SerializeStructVariant as SV;

//                 self.1 = EncoderContext::Bytes;
//                 let mut sv_ser: Compound<W, F> = self
//                     .0
//                     .serialize_struct_variant("", 0, "/", 1)
//                     .map_err(j2i_ser_err)?;
//                 SV::serialize_field(&mut sv_ser, "base64", byte_string).map_err(j2i_ser_err)?;
//                 self.1 = current_context;
//                 SV::end(sv_ser).map_err(j2i_ser_err)
//             }
//             EncoderContext::CID => {
//                 use ser::SerializeMap as Map;

//                 let mut map_ser: Compound<W, F> =
//                     self.0.serialize_map(Some(1)).map_err(j2i_ser_err)?;
//                 Map::serialize_key(&mut map_ser, "/").map_err(j2i_ser_err)?;
//                 Map::serialize_value(&mut map_ser, byte_string).map_err(j2i_ser_err)?;
//                 Map::end(map_ser).map_err(j2i_ser_err)
//             }
//         }
//     }

//     #[inline]
//     fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_unit().map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_unit_struct(name).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_unit_variant(
//         self,
//         name: &'static str,
//         v_index: u32,
//         v: &'static str,
//     ) -> Result<Self::Ok, Self::Error> {
//         self.0
//             .serialize_unit_variant(name, v_index, v)
//             .map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_newtype_struct<T: ?Sized>(
//         self,
//         name: &'static str,
//         value: &T,
//     ) -> Result<Self::Ok, Self::Error>
//     where
//         T: Serialize,
//     {
//         value.serialize(self)
//     }

//     #[inline]
//     fn serialize_newtype_variant<T: ?Sized>(
//         self,
//         name: &'static str,
//         variant_index: u32,
//         variant: &'static str,
//         value: &T,
//     ) -> Result<Self::Ok, Self::Error>
//     where
//         T: Serialize,
//     {
//         self.0
//             .serialize_newtype_variant(name, variant_index, variant, value)
//             .map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
//         self.0.serialize_none().map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
//     where
//         T: Serialize,
//     {
//         value.serialize(self)
//     }

//     #[inline]
//     fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
//         self.0.serialize_seq(len).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
//         self.0.serialize_tuple(len).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_tuple_struct(
//         self,
//         name: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeTupleStruct, Self::Error> {
//         self.0
//             .serialize_tuple_struct(name, len)
//             .map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_tuple_variant(
//         self,
//         name: &'static str,
//         variant_index: u32,
//         variant: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeTupleVariant, Self::Error> {
//         self.0
//             .serialize_tuple_variant(name, variant_index, variant, len)
//             .map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
//         self.0.serialize_map(len).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_struct(
//         self,
//         name: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeStruct, Self::Error> {
//         self.0.serialize_struct(name, len).map_err(j2i_ser_err)
//     }

//     #[inline]
//     fn serialize_struct_variant(
//         self,
//         name: &'static str,
//         variant_index: u32,
//         variant: &'static str,
//         len: usize,
//     ) -> Result<Self::SerializeStructVariant, Self::Error> {
//         self.0
//             .serialize_struct_variant(name, variant_index, variant, len)
//             .map_err(j2i_ser_err)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::JsonDag;
    use ipld_dag::{
        cid::CID,
        dag::{DagNode, Link},
    };
    use serde_json::to_string;

    const CID_STR: &'static str = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";

    #[test]
    fn test_vec() {
        let cid: CID = CID_STR.parse().unwrap();
        let dag: JsonDag = DagNode::List(vec![
            JsonDag(DagNode::Link(Link::CID(cid.clone()))),
            JsonDag(DagNode::Link(Link::CID(cid))),
        ])
        .into();

        let link = make_cid(CID_STR);
        let expected = format!(r#"[{},{}]"#, &link, &link);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_cid() {
        let cid: CID = CID_STR.parse().unwrap();
        let dag: JsonDag = DagNode::Link(Link::CID(cid)).into();

        let expected = make_cid(CID_STR);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual);
    }

    fn make_cid(cid_str: &str) -> String {
        format!(
            r#"{{"{key}":"{cid}"}}"#,
            key = r#"/"#,
            cid = cid_str.to_string(),
        )
    }
}
