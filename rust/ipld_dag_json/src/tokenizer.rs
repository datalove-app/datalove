use ipld_dag::{Token, CID};
use nom::{number::streaming::double, IResult};
use util::parse_string;

/******************************************************************************
 * Main single token parser
 *****************************************************************************/

// TODO: wrap in ws!?
named!(lex<&[u8], Token>, alt!(
    null |
    boolean |
    number |
    string |
    bytes |
    link |
    map_start |
    map_end |
    list_start |
    list_end
));

named!(map_key<&[u8], Token>, alt!(integer | string));

/******************************************************************************
 * Raw token parsers
 *****************************************************************************/

/*
 *  Null
 */
named!(null<&[u8], Token>, value!(Token::Null, tag!(b"null")));

/*
 *  Boolean
 */
named!(boolean<&[u8], Token>, alt!(
    value!(Token::Bool(true), tag!(b"true")) |
    value!(Token::Bool(false), tag!(b"false"))
));

/*
 *  Number
 */
named!(number<&[u8], Token>, alt!(float | integer));
named!(integer<&[u8], Token>, alt!(util::signed | util::unsigned));
named!(float<&[u8], Token>, map!(double, |f| Token::Float(f.into())));

/*
 *  String
 */
named!(string<&[u8], Token>, map!(util::parse_string, Token::Str));

/*
 *  Bytes
 */
named!(bytes<&[u8], Token>, do_parse!(
        tag!(b"{\"/\":{")       >>
        parse_string            >>
        eat_separator!(b":")    >>
    s:  parse_string            >>
        tag!(b"}}")             >>
        (Token::ByteStr(&s))
));

/*
 * List
 */
named!(list_start<&[u8], Token>, value!(Token::List(None), tag!(b"[")));
named!(list_end<&[u8], Token>, value!(Token::ListEnd, tag!(b"]")));

/*
 * Map
 */
named!(map_start<&[u8], Token>, value!(Token::Map(None), tag!(b"{")));
named!(map_end<&[u8], Token>, value!(Token::MapEnd, tag!(b"}")));

/*
 * Link
 */
// TODO: possibly move parse until after the matches
// TODO - not doing so might not consume the end tag
named!(link<&[u8], Token>, do_parse!(
        tag!(b"{\"/\":")                                    >>
    c:  map_res!(parse_string, |s: &str| s.parse::<CID>())  >>
        tag!(b"}")                                          >>
        (Token::Link(c))
));

#[allow(dead_code)]
mod util {
    use ipld_dag::{Int, Token};
    use nom::{character::streaming::digit1, IResult};
    use std::str::{from_utf8, FromStr};

    type StrResult<'a, E> = IResult<&'a [u8], &'a str, E>;

    // TODO: possibly move parse until after the matches
    named!(pub(crate) parse_string<&[u8], &str>, do_parse!(
        tag!(b"\"") >>
        s: map_res!(take_until!("\""), from_utf8) >>
        tag!(b"\"") >>
        (s)
    ));

    /****************************************/

    named!(pub(crate) signed<&[u8], Token>, do_parse!(
        tag!(b"-") >>
        token: alt!(
            map_res!(parse_int_str, to_int_token::<i64>) |
            map_res!(parse_int_str, to_int_token::<i128>)
        ) >>
        (token)
    ));
    named!(pub(crate) unsigned<&[u8], Token>, do_parse!(
        opt!(tag!(b"+")) >>
        token: alt!(
            map_res!(parse_int_str, to_uint_token::<u64>) |
            map_res!(parse_int_str, to_uint_token::<u128>)
        ) >>
        (token)
    ));

    #[inline]
    fn to_int_token<N>(s: &str) -> Result<Token, N::Err>
    where
        N: FromStr + From<i8> + std::ops::Mul<Output = N> + Into<Int>,
    {
        // negated i64/i128 is always in-bounds, so this should always be safe
        s.parse::<N>()
            .map(|n| Token::Integer(n.mul(N::from(-1)).into()))
    }

    #[inline]
    fn to_uint_token<N>(s: &str) -> Result<Token, N::Err>
    where
        N: FromStr + Into<Int>,
    {
        s.parse::<N>().map(|n| Token::Integer(n.into()))
    }

    named!(parse_int_str<&[u8], &str>, map_res!(digit1, from_utf8));

    /****************************************/

    // named!(whitespace<&[u8], Token>, ws!)
    // named!(esc_quote<&[u8]>, escaped!(b"\\\"", '\\', |_| "\""));
}

#[cfg(test)]
mod tests {
    use crate::{encoder::to_vec, tokenizer};
    use ipld_dag::Token;
    use serde::Serialize;

    #[test]
    fn test_null() {
        let json = to_json(None as Option<()>);
        let (_, actual) = tokenizer::null(&json).unwrap();
        assert_eq!(Token::Null, actual);
    }

    #[test]
    fn test_true() {
        let json = to_json(true);
        let (_, actual) = tokenizer::boolean(&json).unwrap();
        assert_eq!(Token::Bool(true), actual);
    }

    #[test]
    fn test_false() {
        let json = to_json(false);
        let (_, actual) = tokenizer::boolean(&json).unwrap();
        assert_eq!(Token::Bool(false), actual);
    }

    #[test]
    fn test_integer() {
        let num: u128 = std::u128::MAX.into();
        let json = to_json(&num);
        let (_, actual) = tokenizer::integer(&json).unwrap();
        assert_eq!(Token::Integer(num.into()), actual);
    }

    #[test]
    fn test_float() {
        let pi: f64 = 3.14159265358979323846264338327950288;
        let json = to_json(&pi);
        let (_, actual) = tokenizer::float(&json).unwrap();
        assert_eq!(Token::Float(pi.into()), actual);
    }

    #[test]
    fn test_string() {
        let string = "hello world";
        let json = to_json(&string);
        let (_, actual) = tokenizer::string(&json).unwrap();
        assert_eq!(Token::Str(&string), actual);

        let string = r#"hello "double-quoted world""#;
        let json = to_json(&string);
        let (_, actual) = tokenizer::string(&json).unwrap();
        assert_eq!(Token::Str(&string), actual);
    }

    #[test]
    fn test_bytes() {
        use ipld_dag::base::{Base, Encodable};
        use serde_bytes::ByteBuf;

        let bytes = ByteBuf::from(vec![0, 1, 2, 3]);
        let byte_str = bytes.encode(Base::Base64);
        let json = to_json(&bytes);
        println!("{:?}\n{:?}", std::str::from_utf8(&json).unwrap(), &json);

        let (_, actual) = tokenizer::bytes(&json).unwrap();

        assert_eq!(Token::ByteStr(&byte_str), actual);
    }

    // Newline ends each vec
    fn to_json<T: Serialize>(t: T) -> Vec<u8> {
        use std::io::Write;
        let mut vec = to_vec(&t).unwrap();
        writeln!(&mut vec).unwrap();
        vec
    }
}
