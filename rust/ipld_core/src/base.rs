//! Re-exports of some [`multibase`] types and traits.

use crate::error::Error;
pub use multibase::{decode, encode, Base, Decodable, Encodable};

///
#[inline]
pub fn to_name(base: Base) -> &'static str {
    match base {
        Base::Base2 => "base2",
        Base::Base8 => "base8",
        Base::Base10 => "base10",
        Base::Base16 => "base16",
        Base::Base16Upper => "base16upper",
        Base::Base32hex => "base32hex",
        Base::Base32hexUpper => "base32hexupper",
        Base::Base32 => "base32",
        Base::Base32Upper => "base32upper",
        Base::Base32z => "base32z",
        Base::Base58flickr => "base58flickr",
        Base::Base58btc => "base58btc",
        Base::Base64 => "base64",
        Base::Base64url => "base64url",
    }
}

///
#[inline]
pub fn from_name(name: &str) -> Result<Base, Error> {
    match name {
        "base2" => Ok(Base::Base2),
        "base8" => Ok(Base::Base8),
        "base10" => Ok(Base::Base10),
        "base16" => Ok(Base::Base16),
        "base16upper" => Ok(Base::Base16Upper),
        "base32hex" => Ok(Base::Base32hex),
        "base32hexupper" => Ok(Base::Base32hexUpper),
        "base32" => Ok(Base::Base32),
        "base32upper" => Ok(Base::Base32Upper),
        "base32z" => Ok(Base::Base32z),
        "base58flickr" => Ok(Base::Base58flickr),
        "base58btc" => Ok(Base::Base58btc),
        "base64" => Ok(Base::Base64),
        "base64url" => Ok(Base::Base64url),
        _ => Err(Error::Custom("unsupported multibase".into())),
    }
}
