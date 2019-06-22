use crate::{cid::CID, dag::Dag};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Link
// pub enum Link<'de, T: Serialize + Deserialize<'de>> {
pub enum Link<T: Dag> {
    CID(CID),
    Dag(Box<T>),
}

impl<T: Dag> Serialize for Link<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Link::Dag(dag) => (*dag).serialize(serializer),
            Link::CID(cid) => serializer.serialize_newtype_struct("CID", &cid.to_vec()),
        }
    }
}

// TODO: how to convert from one generic to another
// impl<'a> From<Link<Dag<'a>>> for Link<JsonDag<'a>> {
//     fn from(link: Link<Dag<'a>>) -> Self {
//         match link {
//             Link::Dag(dag) => {
//                 let json_dag: JsonDag = (*dag).into();
//                 Link::Dag(Box::new(json_dag))
//             }
//             Link::CID(cid) => Link<JsonDag>::CID(cid),
//         }
//     }
// }

// impl<'a> Serialize for Link<'a> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match *self {
//             Link::Dag(dag) => dag.serialize(serializer),
//             Link::CID(cid) => {
//                 let bytes: &[u8] = cid.to_bytes();
//                 serializer.serialize_newtype_variant("Link", 0, "CID", bytes)
//             }
//         }
//     }
// }

// impl<'de, 'a: 'de> Deserialize<'de> for Link<'a> {
//     fn deserialize<D>(deserializer: D) -> Result<Link<'a>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let cid = CID::new();
//         Ok(Link::CID(cid))
//     }
// }

// impl<'de> Visitor<'de> for LinkVisitor {

// }
