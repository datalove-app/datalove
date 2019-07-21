//!

// TODO: add decoder functions for the various rust types (that can parse multibases from strings)

use crate::{
    base::{Base, Decodable, Encodable},
    error::Error,
    format::Encoder,
    Prefix, Version,
};
use ::cid::{Cid, Codec};
use integer_encoding::VarIntWriter;
use multihash::Hash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, hash, str};

const V0_PREFIX: Prefix = Prefix {
    version: Version::V0,
    codec: Codec::DagProtobuf,
    mh_type: Hash::SHA2256,
    mh_len: 34,
};

/// An IPLD [`CID`](https://github.com/ipld/specs/blob/master/block-layer/CID.md).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CID {
    base: Option<Base>,
    prefix: Prefix,
    hash: Vec<u8>,
}

impl CID {
    /// Creates a new `CID` from `multihash` bytes.
    #[inline]
    pub fn from(mh: &[u8]) -> Result<CID, Error> {
        let prefix = Prefix::new_from_bytes(mh)?;
        Ok(CID::from_prefix(prefix, mh))
    }

    /// Creates a new CID from a known `Prefix` and raw hash bytes.
    #[inline]
    pub fn from_prefix(prefix: Prefix, mh: &[u8]) -> CID {
        let cid = Cid::new_from_prefix(&prefix, mh);
        CID {
            base: None,
            prefix,
            hash: cid.hash,
        }
    }

    /// Retrieves the underlying `multicodec::Codec`.
    #[inline]
    pub fn base(&self) -> &Option<Base> {
        &self.base
    }

    /// Retrieves the underlying `multihash` bytes.
    #[inline]
    pub fn mh(&self) -> &[u8] {
        &self.hash
    }

    /// Retrieves the underlying `multicodec::Codec`.
    #[inline]
    pub fn codec(&self) -> &Codec {
        &self.prefix.codec
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
    #[inline]
    pub fn to_string(&self, mb: Option<Base>) -> String {
        match self.version() {
            Version::V0 => self.to_string_v0(),
            Version::V1 => self.to_string_v1(mb),
        }
    }

    /// Encodes the `CID` to a `Vec<u8>`.
    /// If v0, returns the underlying `multihash`.
    /// If v1, prefixes the bytes with the `CID` version and `multicodec::Codec`.
    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        match self.version() {
            Version::V0 => self.to_vec_v0(),
            Version::V1 => self.to_vec_v1(),
        }
    }

    /// Defaults to `Base58Btc`.
    #[inline]
    fn to_string_v0(&self) -> String {
        let mut string = self.hash.as_slice().encode(Base::Base58btc);
        // remove leading char added by `multibase`
        string.remove(0);
        string
    }

    #[inline]
    fn to_string_v1(&self, base: Option<Base>) -> String {
        let base = base.or(*self.base()).or(Some(Base::Base58btc)).unwrap();
        self.to_vec_v1().encode(base)
    }

    #[inline]
    fn to_vec_v0(&self) -> Vec<u8> {
        self.hash.clone()
    }

    #[inline]
    fn to_vec_v1(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(16);
        res.write_varint(u64::from(self.prefix.version)).unwrap();
        res.write_varint(u64::from(self.prefix.codec)).unwrap();
        res.extend_from_slice(&self.hash);
        res
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

impl hash::Hash for CID {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl str::FromStr for CID {
    type Err = Error;

    // TODO: CIDs can be context dependent, so v1 is some cases impliable...

    /// Creates a new `CID` from an `str`, decoding its `multibase::Base`.
    fn from_str(s: &str) -> Result<Self, Error> {
        if Version::is_v0_str(s) {
            let s = Base::Base58btc.code().to_string() + &s;
            let (_, decoded) =
                Decodable::decode(&s)
                    .map_err(|err| err.into())
                    .and_then(|(base, decoded)| match base {
                        Base::Base58btc => Ok((base, decoded)),
                        _ => Err(Error::InvalidCIDStr),
                    })?;

            Ok(CID {
                base: Some(Base::Base58btc),
                prefix: V0_PREFIX,
                hash: decoded,
            })
        } else {
            let (base, decoded) = Decodable::decode(&s)?;
            Ok(CID {
                base: Some(base),
                prefix: Prefix::new_from_bytes(&decoded)?,
                hash: decoded,
            })
        }
    }
}
