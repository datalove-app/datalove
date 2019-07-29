use ipld_core::Token;
use nom;
use util::{
    parse_string, semicolon, string_bytes, tag_bytes_end, tag_bytes_start, tag_link_start,
    tag_map_end,
};

/******************************************************************************
 * Main single token parser
 *****************************************************************************/

// TODO: wrap in ws!?
named!(lex<&[u8], Token>, alt!(
    primitive
    | compound_start
    | compound_end
));

named!(primitive<&[u8], Token>, alt!(
   bytes
   | link
   | boolean
   | null
   | number
   | string
));

named!(compound_start<&[u8], Token>, alt!(list_start | map_start));
named!(compound_end<&[u8], Token>, alt!(list_end | map_end));

/******************************************************************************
 * Raw token parsers
 *****************************************************************************/

/*
 * Null
 */
named!(null<&[u8], Token>, value!(Token::Null, util::tag_null));

/*
 * Boolean
 */
named!(boolean<&[u8], Token>, alt!(
    value!(Token::Bool(false), util::tag_false) |
    value!(Token::Bool(true), util::tag_true)
));

/*
 * Number
 */
named!(number<&[u8], Token>, alt!(float | integer));
named!(float<&[u8], Token>, map!(util::float_as_str, Token::FloatStr));
named!(integer<&[u8], Token>, alt!(util::signed | util::unsigned));

/*
 * String
 */
named!(string<&[u8], Token>, map!(util::parse_string, Token::Str));

/*
 * Bytes
 */
named!(bytes<&[u8], Token>, do_parse!(
            tag_bytes_start >>
    _base:  string_bytes >>
            semicolon >>
    bytes:  parse_string >>
            tag_bytes_end >>
            (Token::ByteStr(&bytes))
));

/*
 * List
 */
named!(list_start<&[u8], Token>, value!(Token::List(None), util::tag_list_start));
named!(list_end<&[u8], Token>, value!(Token::ListEnd, util::tag_list_end));

/*
 * Map
 */
named!(map_start<&[u8], Token>, value!(Token::Map(None), util::tag_map_start));
named!(map_end<&[u8], Token>, value!(Token::MapEnd, util::tag_map_end));
named!(map_key<&[u8], Token>, alt!(integer | string));
// named!(map_pair<&[u8], (Token, Token)>, do_parse!(
//     k:  map_key >>
//         eat_separator!(b":") >>
//     v:  value >>
//         opt!(eat_separator!(b",")) >>
//         (k, v)
// ));

/*
 * Link
 */
named!(link<&[u8], Token>, do_parse!(
                tag_link_start >>
    cid_str:    parse_string >>
                tag_map_end >>
                (Token::LinkStr(cid_str))
));

#[allow(dead_code)]
mod util {
    use ipld_core::Token;
    use nom::{
        character::streaming::{alphanumeric1, digit1},
        number::streaming::recognize_float,
    };

    /****************************************/
    // string-related
    /****************************************/

    // TODO: support unicode characters
    fn is_string_char(c: u8) -> bool {
        c != b'"' && c != b'\\'
    }

    named_attr!(#[inline], pub(crate) string_bytes<&[u8], &[u8]>, do_parse!(
            tag!(b"\"") >>
        // s:  take_until!("\"") >>
        s:  escaped!(alphanumeric1, '\\', one_of!("\"n\\")) >>
            tag!(b"\"") >>
            (s)
    ));

    named_attr!(#[inline], pub(crate) parse_string<&[u8], &str>, map!(
        string_bytes,
        |string| unsafe { std::str::from_utf8_unchecked(string) }
    ));

    /****************************************/
    // number-related
    /****************************************/

    named_attr!(#[inline], pub(crate) signed<&[u8], Token>, do_parse!(
                peek!(tag!(b"-")) >>
        token:  map!(int_as_str, Token::IntegerStr) >>
                (token)
    ));
    named_attr!(#[inline], pub(crate) unsigned<&[u8], Token>, do_parse!(
                opt!(tag!(b"+")) >>
        token:  map!(uint_as_str, Token::IntegerStr) >>
                (token)
    ));

    named_attr!(#[inline], pub(crate) float_as_str<&[u8], &str>, map!(
        recognize_float,
        |string| unsafe { std::str::from_utf8_unchecked(string) }
    ));

    named_attr!(#[inline], int_as_str<&[u8], &str>, map!(
        recognize!(preceded!(opt!(tag!(b"-")), digit1)),
        |string| unsafe { std::str::from_utf8_unchecked(string) }
    ));
    named_attr!(#[inline], uint_as_str<&[u8], &str>, map!(
        digit1,
        |string| unsafe { std::str::from_utf8_unchecked(string) }
    ));

    /****************************************/
    // whitespace, punctuation, tags
    /****************************************/

    named_attr!(#[inline], pub(crate) tag_null<&[u8], &[u8]>, tag!(b"null"));
    named_attr!(#[inline], pub(crate) tag_true<&[u8], &[u8]>, tag!(b"true"));
    named_attr!(#[inline], pub(crate) tag_false<&[u8], &[u8]>, tag!(b"false"));
    named_attr!(#[inline], pub(crate) tag_bytes_start<&[u8], &[u8]>, tag!(b"{\"/\":{"));
    named_attr!(#[inline], pub(crate) tag_bytes_end<&[u8], &[u8]>, tag!(b"}}"));
    named_attr!(#[inline], pub(crate) tag_list_start<&[u8], &[u8]>, tag!(b"["));
    named_attr!(#[inline], pub(crate) tag_list_end<&[u8], &[u8]>, tag!(b"]"));
    named_attr!(#[inline], pub(crate) tag_map_start<&[u8], &[u8]>, tag!(b"{"));
    named_attr!(#[inline], pub(crate) tag_map_end<&[u8], &[u8]>, tag!(b"}"));
    named_attr!(#[inline], pub(crate) tag_link_start<&[u8], &[u8]>, tag!(b"{\"/\":"));

    named_attr!(#[inline], pub(crate) comma<&[u8], Option<&[u8]>>, opt!(eat_separator!(b",")));
    named_attr!(#[inline], pub(crate) semicolon<&[u8], &[u8]>, eat_separator!(b":"));

    // named!(whitespace<&[u8], Token>, ws!)
    // named!(esc_quote<&[u8]>, escaped!(b"\\\"", '\\', |_| "\""));
}

#[cfg(test)]
mod tests {
    use crate::{encoder::to_vec, tokenizer};
    use ipld_core::{
        multibase::{Base, Encodable},
        Token,
    };
    use serde::Serialize;
    use serde_bytes::ByteBuf;
    use std::io::Write;

    #[test]
    fn test_null() {
        let json = to_newlined_json(None as Option<()>);
        let (_, actual) = tokenizer::null(&json).unwrap();
        assert_eq!(Token::Null, actual);
    }

    #[test]
    fn test_boolean() {
        let json = to_newlined_json(true);
        let (_, actual) = tokenizer::boolean(&json).unwrap();
        assert_eq!(Token::Bool(true), actual);

        let json = to_newlined_json(false);
        let (_, actual) = tokenizer::boolean(&json).unwrap();
        assert_eq!(Token::Bool(false), actual);
    }

    #[test]
    fn test_integer() {
        let num: i128 = std::i128::MIN;
        let json = to_newlined_json(&num);
        let (_, actual) = tokenizer::integer(&json).unwrap();
        assert_eq!(Token::IntegerStr(&format(num)), actual);

        let num: u128 = std::u128::MAX;
        let json = to_newlined_json(&num);
        let (_, actual) = tokenizer::integer(&json).unwrap();
        assert_eq!(Token::IntegerStr(&format(num)), actual);
    }

    #[test]
    fn test_float() {
        let pi: f64 = 3.14159265358979323846264338327950288;
        let json = to_newlined_json(&pi);
        let (_, actual) = tokenizer::float(&json).unwrap();
        assert_eq!(Token::FloatStr(&format(pi)), actual);
    }

    #[test]
    fn test_string() {
        let string = "hello world";
        let json = to_newlined_json(&string);
        println!("{:?}\n{:?}", format_vec(&json), &json);

        let (_, actual) = tokenizer::string(&json).unwrap();
        assert_eq!(Token::StrBytes(&string.as_bytes()), actual);

        let string = r#"hello "double-quoted world""#;
        let json = to_newlined_json(&string);
        println!("{:?}\n", &json);
        // println!("{:?}\n{:?}", format_vec(&json), &json);

        let (_, actual) = tokenizer::string(&json).unwrap();
        assert_eq!(Token::StrBytes(&string.as_bytes()), actual);
    }

    #[test]
    fn test_bytes() {
        let bytes = ByteBuf::from(vec![0, 1, 2, 3]);
        let byte_str = bytes.encode(Base::Base64);
        let json = to_newlined_json(&bytes);
        println!("{:?}\n{:?}", format_vec(&json), &json);

        let (_, actual) = tokenizer::bytes(&json).unwrap();
        assert_eq!(Token::ByteStr(&byte_str), actual);
    }

    // Newline ends each vec
    fn to_newlined_json<T: Serialize>(t: T) -> Vec<u8> {
        let mut vec = to_vec(&t).unwrap();
        writeln!(&mut vec).unwrap();
        vec
    }

    fn format<T: std::fmt::Display>(t: T) -> String {
        format!("{}", t)
    }

    fn format_vec(v: &Vec<u8>) -> &str {
        std::str::from_utf8(v).unwrap()
    }
}
