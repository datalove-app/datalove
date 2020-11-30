use crate::dev::*;
use serde::{Deserializer, Serializer};

///
#[derive(Debug)]
pub struct Did(PeerId);

impl Serialize for Did {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for Did {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        unimplemented!()
    }
}

lazy_static! {
    pub static ref GENESIS: UserRecord = { unimplemented!() };
}

