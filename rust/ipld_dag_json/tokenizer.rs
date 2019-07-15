use ipld_dag::{
    base::{from_name, Decodable},
    Token,
};
use nom::{
    branch::alt,
    bytes::streaming::{escaped, tag, take_until, take_while},
    character::{is_digit, streaming::digit1},
    combinator::{flat_map, map, map_opt, map_parser, map_res, opt, peek, rest, value},
    error::{ErrorKind, ParseError},
    number::streaming::{double, recognize_float},
    sequence::tuple,
    IResult,
};
use std::str::from_utf8;

type TokenResult<'a> = IResult<&'a [u8], Token<'a>>;

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
 *  Integer
 */
named!(pub integer<&[u8], Token>, alt!(signed | unsigned));
named!(signed<&[u8], Token>, do_parse!(
    tag!(b"-") >>
    tap!(s: rest => println!("signed rest: {:?}", s)) >>
    token: alt!(
        map!(parse_to!(i64), |n| Token::Integer((-1 * n).into())) |
        map!(parse_to!(i128), |n| Token::Integer((-1 * n).into()))
    ) >>
    (token)
));
named!(unsigned<&[u8], Token>, do_parse!(
    opt!(tag!(b"+")) >>
    tap!(s: rest => println!("unsigned rest: {:?}", s)) >>
    token: alt!(
        map!(parse_to!(u64), |n| Token::Integer(n.into())) |
        map!(parse_to!(u128), |n| Token::Integer(n.into()))
    ) >>
    (token)
));

/*
 *  Float
 */
named!(pub float<&[u8], Token>, map!(double, |f| Token::Float(f.into())));

/*
 *  String
 */
named!(pub string<&[u8], Token>, map!(util::string, Token::Str));

/*
 *  Bytes
 */
use util::string as util_string;
named!(pub bytes<&[u8], Token>, do_parse!(
    tag!(b"{\"/\":{") >>

    tap!(s: util_string => println!("tapped base: {}", s)) >>
    util_string >>

    eat_separator!(b":") >>

    tap!(s: util_string => println!("tapped base-encoded str: {}", s)) >>
    s: util_string >>

    // tap!(s: rest => println!("rest: {:?}", s)) >>
    tag!(b"}}") >>
    (Token::ByteStr(&s))
));

mod util {
    use nom::{
        bytes::streaming::{tag, take_until},
        combinator::map,
        error::ParseError,
        sequence::tuple,
        IResult,
    };

    pub type StrResult<'a, E> = IResult<&'a [u8], &'a str, E>;

    // pub fn btos<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> StrResult<'a, E> {

    // }

    // named!(string<&[u8], &str>, do_parse!(
    //     tag!(b"\"") >>
    //     s: take_until!("\"") >>
    //     map_res!()
    // ));

    pub fn string<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> StrResult<'a, E> {
        map(tuple((tag(b"\""), take_until("\""))), |(_, bytes)| {
            (std::str::from_utf8(bytes).unwrap())
        })(i)
    }
}

/******************************************************************************
 *****************************************************************************/

// named!(whitespace<&[u8], Token>, ws!)
// named!(esc_quote<&[u8]>, escaped!(b"\\\"", '\\', |_| "\""));

#[cfg(test)]
mod tests {
    use crate::{encoder::to_vec, parser};
    use ipld_dag::Token;
    use nom::error::VerboseError;
    use serde::Serialize;

    // type E<'a> = VerboseError<&'a [u8]>;

    #[test]
    fn test_null() {
        let json = to_json(None as Option<()>);
        let (_, actual) = parser::null(&json).unwrap();
        assert_eq!(Token::Null, actual);
    }

    #[test]
    fn test_true() {
        let json = to_json(true);
        let (_, actual) = parser::boolean(&json).unwrap();
        assert_eq!(Token::Bool(true), actual);
    }

    #[test]
    fn test_false() {
        let json = to_json(false);
        let (_, actual) = parser::boolean(&json).unwrap();
        assert_eq!(Token::Bool(false), actual);
    }

    #[test]
    fn test_integer() {
        let num: u128 = std::u128::MAX;
        let json = to_json(&num);
        let res = parser::integer(&json);
        println!(
            "{:?} {}\nexp: {:?}\nact: {:?}",
            num,
            std::str::from_utf8(&json).unwrap(),
            &json,
            res,
        );

        assert_eq!(Token::Integer(num.into()), res.unwrap().1);
    }

    #[test]
    fn test_float() {
        let pi: f64 = 3.14159265358979323846264338327950288;
        let json = to_json(&pi);
        let (_, actual) = parser::float(&json).unwrap();
        assert_eq!(Token::Float(pi.into()), actual);
    }

    #[test]
    fn test_string() {
        let string = "hello world";
        let json = to_json(&string);
        let (_, actual) = parser::string(&json).unwrap();
        assert_eq!(Token::Str(&string), actual);

        let string = r#"hello "double-quoted world""#;
        let json = to_json(&string);
        let (_, actual) = parser::string(&json).unwrap();
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

        let (_, actual) = parser::bytes(&json).unwrap();

        assert_eq!(Token::ByteStr(&byte_str), actual);
    }

    fn to_json<T: Serialize>(t: T) -> Vec<u8> {
        let mut vec = to_vec(&t).unwrap();
        vec.push('\n' as u8);
        vec
    }
}
