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

pub fn try_from_bytes<'a>(env: Env<'a>, bytes: &[u8]) -> Result<Term<'a>, Error> {
    match Atom::try_from_bytes(env, bytes) {
        Ok(Some(term)) => Ok(term.encode(env)),
        _ => {
            let string = from_utf8(bytes).map_err(|_| Error::InvalidUTF8Bytes)?;
            Ok(string.encode(env))
        }
    }
}

pub fn try_from_str<'a>(env: Env<'a>, string: &str) -> Result<Term<'a>, Error> {
    match Atom::try_from_bytes(env, string.as_bytes()) {
        Ok(Some(term)) => Ok(term.encode(env)),
        _ => Ok(string.encode(env)),
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidAtom {
            description("Unable to create atom")
        }
        InvalidUTF8Bytes {
            description("Invalid UTF-8 string")
        }
    }
}
