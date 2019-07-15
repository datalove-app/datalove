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

// Null
named!(null<&[u8], Token>, value!(Token::Null, tag!(b"null")));
// fn null<'a, E>(i: &'a [u8]) -> IResult<&'a [u8], Token<'a>, E>
// where
//     E: ParseError<&'a [u8]>,
// {
//     value(Token::Null, tag(b"null"))(i)
// }

// Boolean
named!(boolean<&[u8], Token>, alt!(
    value!(Token::Bool(true), tag!(b"true")) |
    value!(Token::Bool(false), tag!(b"false"))
));
// fn boolean<'a, E>(i: &'a [u8]) -> TokenResult<'a, E>
// where
//     E: ParseError<&'a [u8]>,
// {
//     alt((
//         value(Token::Bool(true), tag(b"true")),
//         value(Token::Bool(false), tag(b"false")),
//     ))(i)
// }

// Integer
named!(integer<&[u8], Token>, alt!(signed | unsigned));
// named!(integer<&[u8], Token>, do_parse!(
//     map_res!(do_parse!(opt!(tag!("-")) >> bytes: digit1 >> (bytes)), from_utf8) >>
// ));
// fn integer<'a>(i: &'a [u8]) -> TokenResult<'a> {
//     flat_map(int_str, alt((signed, unsigned)))(i)
// }
named!(signed<&[u8], Token>, do_parse!(
    tag!(b"-") >>
    token: alt!(
        map!(parse_to!(i64), |n| Token::Integer((-1 * n).into())) |
        map!(parse_to!(i128), |n| Token::Integer((-1 * n).into()))
    ) >>
    (token)
));
named!(unsigned<&[u8], Token>, alt!(
    map!(parse_to!(u64), |n| Token::Integer(n.into())) |
    map!(parse_to!(u128), |n| Token::Integer(n.into()))
));

// named!(int_str<&[u8], &str>, map_res!(
//     do_parse!(opt!(tag!("-")) >> bytes: digit1 >> (bytes)),
//     from_utf8
// ));
// named!(_u64<&str, u64>, parse_to!(u64));
// named!(_u128<&str, u128>, parse_to!(u128));
// named!(_i64<&str, i64>, parse_to!(i64));
// named!(_i128<&str, i128>, parse_to!(i128));

// Float
named!(float<&[u8], Token>, map!(double, |f| Token::Float(f.into())));
// fn float<'a, E>(i: &'a [u8]) -> TokenResult<'a, E>
// where
//     E: ParseError<&'a [u8]>,
// {
//     map(double, |f| Token::Float(f.into()))(i)
// }

// String
named!(string<&[u8], Token>, map!(util::string, Token::Str));
// fn string<'a, E>(i: &'a [u8]) -> TokenResult<'a, E>
// where
//     E: ParseError<&'a [u8]>,
// {
//     map(util::string, Token::Str)(i)
// }

// Bytes
use util::string as util_string;
named!(bytes<&[u8], Token>,
    do_parse!(
        tag!(b"{\"/\":{") >>

        tap!(s: util_string => println!("tapped base: {}", s)) >>
        util_string >>

        eat_separator!(b":") >>

        tap!(s: util_string => println!("tapped base-encoded str: {}", s)) >>
        s: util_string >>

        tap!(s: tag(b"}}") => println!("rest: {:?}", s)) >>
        tag!(b"}}") >>
        (Token::ByteStr(&s))
    )
);

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

    type E<'a> = VerboseError<&'a [u8]>;

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
        let res = parser::float(&json);
        println!(
            "{:?} {}\n{:?} {:?}",
            num,
            std::str::from_utf8(&json).unwrap(),
            res,
            &json
        );

        assert_eq!(Token::Integer(num.into()), res.unwrap().1);
    }

    #[test]
    fn test_float() {
        let pi: f64 = 3.141592653589793; // 3.14159265358979323846264338327950288
        let json = to_json(&pi);
        let res = parser::float(&json);
        println!(
            "{:?} {}\n{:?} {:?}",
            pi,
            std::str::from_utf8(&json).unwrap(),
            res,
            &json
        );

        assert_eq!(Token::Float(pi.into()), res.unwrap().1);
    }

    #[test]
    fn test_string() {
        let string = "hello world";
        let json = to_json(&string);
        let (_, actual) = parser::string(&json).unwrap();
        assert_eq!(Token::Str(&string), actual);

        let string = "hello \"double-quoted world\"";
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
        to_vec(&t).unwrap()
    }
}
