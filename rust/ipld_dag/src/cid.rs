//!

// TODO: fix Error
// TODO: add decoder functions for the various rust types (that can parse multibases from strings)

use crate::{
    base::{Base, Decodable, Encodable},
    error::Error,
    format::Encoder,
    Prefix, Version,
};
use ::cid::{Cid, Codec, ToCid};
use multihash::Hash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An IPLD [`CID`](https://github.com/ipld/specs/blob/master/block-layer/CID.md).
#[derive(Clone, Debug)]
pub struct CID {
    base: Option<Base>,
    cid: Cid,
    prefix: Prefix,
}

impl CID {
    /// Creates a new `CID`.
    pub fn from(mh: &[u8]) -> Result<CID, Error> {
        let prefix = Prefix::new_from_bytes(mh)?;
        Ok(CID::from_prefix(prefix, mh))
    }

    /// Creates a new CID from a known `Prefix` and bytes.
    pub fn from_prefix(prefix: Prefix, mh: &[u8]) -> CID {
        let cid = Cid::new_from_prefix(&prefix, mh);
        CID {
            base: None,
            cid,
            prefix,
        }
    }

    /// Retrieves the underlying `multicodec::Codec`.
    #[inline]
    pub fn base(&self) -> &Option<Base> {
        &self.base
    }

    /// Retrieves the underlying `multicodec::Codec`.
    #[inline]
    pub fn codec(&self) -> &Codec {
        &self.prefix.codec
    }

    /// Retrieves the underlying `multihash` bytes.
    #[inline]
    pub fn hash(&self) -> &[u8] {
        &self.cid.hash
    }

    /// Retrieves the underlying `multihash::Hash`.
    #[inline]
    pub fn mh_type(&self) -> &Hash {
        &self.prefix.mh_type
    }

    /// Retrieves the underlying `CID` `Version`.
    #[inline]
    pub fn version(&self) -> &Version {
        &self.prefix.version
    }

    /// Encodes the `CID` to a string.
    /// If v0, just returns the underlying `multihash`, `base58btc` encoded.
    /// If v1, prefixes the string with the `CID` version and `multicodec::Codec`, then encodes the bytes with the specified `multibase::Base`, defaulting to `base58btc`.
    pub fn to_string(&self, mb: Option<Base>) -> String {
        match self.version() {
            Version::V0 => self.to_string_v0(),
            Version::V1 => self.to_string_v1(mb),
        }
    }

    /// Encodes the `CID` to a `Vec<u8>`.
    /// If v0, returns the underlying `multihash`.
    /// If v1, prefixes the bytes with the `CID` version and `multicodec::Codec`.
    pub fn to_vec(&self) -> Vec<u8> {
        match self.version() {
            Version::V0 => self.to_vec_v0(),
            Version::V1 => self.to_vec_v1(),
        }
    }

    #[inline]
    /// Defaults to `Base58Btc`.
    fn to_string_v0(&self) -> String {
        self.cid.to_string()
    }

    #[inline]
    fn to_string_v1(&self, base: Option<Base>) -> String {
        match base.or(*self.base()) {
            None => self.cid.to_string(),
            Some(base) => Encodable::encode(&self.to_vec_v1(), base),
        }
    }

    #[inline]
    fn to_vec_v0(&self) -> Vec<u8> {
        self.cid.hash.clone()
    }

    #[inline]
    fn to_vec_v1(&self) -> Vec<u8> {
        self.cid.to_bytes()
    }
}

impl Serialize for CID {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.encode_link(self)
    }
}

// impl<'de> Deserialize<'de> for CID {
//     fn deserialize<D>(deserializer: D) -> Result<CID, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         Ok(CID::new())
//     }
// }

impl Encodable for CID {
    #[inline]
    fn encode(&self, base: Base) -> String {
        self.to_string(Some(base))
    }
}

impl std::str::FromStr for CID {
    type Err = Error;

    // TODO
    /// Creates a new `CID` from an `str`, decoding its `multibase::Base`.
    fn from_str(s: &str) -> Result<Self, Error> {
        let (base, decoded) = if Version::is_v0_str(s) {
            let s = Base::Base58btc.code().to_string() + &s;
            Decodable::decode(&s)?
        } else {
            Decodable::decode(&s)?
        };

        println!(
            "base: {:?}, decoded: {:?}, {}",
            base,
            decoded,
            decoded.len()
        );

        let cid = decoded.to_cid()?;
        println!("decoded cid: {:?}", cid);
        let prefix = Prefix::new_from_bytes(&cid.hash)?;
        println!("decoded prefix: {:?}", prefix);
        Ok(CID {
            base: Some(base),
            cid,
            prefix,
        })
    }
}
