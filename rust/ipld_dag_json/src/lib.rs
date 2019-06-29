mod error;

// use crate::error::{j2i_de_err, j2i_ser_err, key_must_be_a_string};
// use base64::{display::Base64Display, encode as b64Encode};
use ipld_dag::{
    indexmap::indexmap,
    multibase::{Base, Encodable},
    Dag, Link, RawDag, Token, CID,
};
use serde::{
    de,
    ser::{self, Serialize, SerializeStructVariant, Serializer},
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
pub struct JsonDag(RawDag<JsonDag>);

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

impl Dag for JsonDag {
    fn get_type(&self) -> Token {
        Token::Null
    }
}

/// Serialization behaviour is almost identical to the standard JSON format, with a few exceptions:
///     - encodes bytes as `{"/": { "base64": String }}`
///     - encodes CID bytes as `{"/": String}`
impl Serialize for JsonDag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            // Serialization unique to the DagJSON format
            // Encodes bytes as `{"/": { "base64": String }}`
            RawDag::ByteBuf(bytes, _base) => {
                let mut ser = serializer.serialize_struct_variant("", 0, "/", 1)?;
                // todo: should this be configurable?
                let (base, base_key) = (&Base::Base64, "base64");
                ser.serialize_field(base_key, &bytes.encode(*base))?;
                ser.end()
            }
            // Encodes CID bytes as `{"/": String}`
            RawDag::Link(link) => match link {
                Link::Dag(dag) => (*dag).serialize(serializer),
                Link::CID(cid) => {
                    let cid_str = cid.to_string(Some(Base::Base64));
                    serializer.serialize_newtype_variant("CID", 0, "/", &cid_str)
                }
            },

            // Serialization identical to the default format
            _ => Dag::serialize(self, serializer),
        }
    }
}

// TODO: ?? use serde_test methods to test the cases (instead of manually making json??)
#[cfg(test)]
mod tests {
    use crate::JsonDag;
    use ipld_dag::{
        multibase::{encode as mb_encode, Base},
        Link, RawDag, CID,
    };
    use serde_json::to_string;

    const CID_STR: &'static str = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";

    #[test]
    fn test_bytes() {
        let bytes: Vec<u8> = vec![0, 1, 2, 3];
        let byte_str = mb_encode(Base::Base64, &bytes);
        let dag: JsonDag = JsonDag(RawDag::ByteBuf(bytes, None));

        let expected = format!(
            r#"{{"/":{{"base64":"{bytes}"}}}}"#,
            bytes = byte_str.to_string(),
        );
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cid() {
        let cid: CID = CID_STR.parse().unwrap();
        let dag = JsonDag(RawDag::Link(Link::CID(cid)));

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

    #[test]
    fn test_vec() {
        let cid: CID = CID_STR.parse().unwrap();
        let dag: JsonDag = JsonDag(RawDag::List(vec![
            JsonDag(RawDag::Link(Link::CID(cid.clone()))),
            JsonDag(RawDag::Link(Link::CID(cid))),
        ]));

        let link = make_cid(CID_STR);
        let expected = format!(r#"[{},{}]"#, &link, &link);
        let actual = to_string(&dag).unwrap();
        assert_eq!(expected, actual)
    }
}
