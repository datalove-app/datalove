use cid::{Cid, Codec, Version};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct CID(Cid);

impl CID {
    pub fn new() -> CID {
        CID(Cid::new(Codec::Raw, Version::V1, &[]))
    }

    // pub fn to_bytes(&self) -> &[u8] {
    //     &self.0.to_bytes()
    // }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl Serialize for CID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0.to_bytes())
    }
}

impl<'de> Deserialize<'de> for CID {
    fn deserialize<D>(deserializer: D) -> Result<CID, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CID::new())
    }
}
