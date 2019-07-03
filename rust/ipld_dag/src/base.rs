pub use multibase::{Base, Encodable};

///
pub fn name(base: Base) -> &'static str {
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
