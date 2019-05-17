use quick_error::quick_error;
use rustler::{types::atom::Atom, Encoder, Env, Term};
use std::str::from_utf8;

rustler_atoms! {
    atom nil;
    atom ok;
    atom error;
    atom true_ = "true";
    atom false_ = "false";
    atom __struct__;
}

pub fn term_from_bytes<'a>(env: Env<'a>, bytes: &[u8]) -> Result<Term<'a>, Error> {
    match Atom::try_from_bytes(env, bytes) {
        Ok(Some(term)) => Ok(term.encode(env)),
        Ok(None) => {
            let string = from_utf8(bytes).map_err(|_| Error::InvalidUTF8Bytes)?;
            Ok(string.encode(env))
        }
        _ => Err(Error::InvalidAtomBytes),
    }
}

pub fn term_from_str<'a>(env: Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    match Atom::try_from_bytes(env, string.as_bytes()) {
        Ok(Some(term)) => Ok(term.encode(env)),
        Ok(None) => Ok(string.encode(env)),
        _ => Err(Error::InvalidUTF8String),
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidAtomBytes {
            description("Invalid atom bytes")
        }
        InvalidUTF8Bytes {
            description("Invalid UTF-8 bytes")
        }
        InvalidUTF8String {
            description("Invalid UTF-8 string")
        }
    }
}
