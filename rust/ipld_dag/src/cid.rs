// TODO: fix Error
// TODO: add decoder functions for the various rust types (that can parse multibases from strings)

use cid::{Cid, Error, ToCid};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use ::cid::{Codec, Prefix, Version};
pub use multibase::Base;

#[derive(Clone, Debug)]
pub struct CID {
    mb: Option<Base>,
    cid: Cid,
}

impl CID {
    pub fn from<T: ToCid>(data: T, mb: Option<Base>) -> Result<CID, Error> {
        Ok(CID {
            mb: mb,
            cid: Cid::from(data)?,
        })
    }

    ///
    pub fn codec(&self) -> &Codec {
        &self.cid.codec
    }

    ///
    pub fn prefix(&self) -> Prefix {
        self.cid.prefix()
    }

    ///
    pub fn version(&self) -> &Version {
        &self.cid.version
    }

    ///
    pub fn to_string(&self) -> String {
        match self.version() {
            Version::V0 => self.to_string_v0(),
            Version::V1 => self.to_string_v1(),
        }
    }

    ///
    pub fn to_vec(&self) -> Vec<u8> {
        match self.version() {
            Version::V0 => self.to_vec_v0(),
            Version::V1 => self.to_vec_v1(),
        }
    }

    #[inline]
    fn to_string_v0(&self) -> String {
        self.cid.to_string()
    }

    ///
    #[inline]
    fn to_string_v1(&self) -> String {
        use multibase::encode;
        match &self.mb {
            // defaults to Base58Btc
            None => self.cid.to_string(),
            Some(mb) => encode(*mb, self.to_vec_v1().as_slice()),
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

impl std::str::FromStr for CID {
    type Err = Error;
    fn from_str(src: &str) -> Result<Self, Error> {
        Ok(CID {
            mb: None,
            cid: src.to_cid()?,
        })
    }
}

impl Serialize for CID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.to_vec())
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
