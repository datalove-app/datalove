use crate::{cid::CID, dag::Dag};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// pub enum Link<'de, T: Serialize + Deserialize<'de>> {
/// An IPLD Dag [Link], wrapping either a [`CID`] or a `Box<Dag>`.
///
/// [link]: https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind
/// [`CID`]: https://github.com/ipld/specs/blob/master/block-layer/CID.md
#[derive(From)]
pub enum Link<T: Dag> {
    CID(CID),
    Dag(Box<T>),
}

impl<T: Dag> Serialize for Link<T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Link::Dag(dag) => (*dag).serialize(serializer),
            Link::CID(cid) => serializer.serialize_newtype_struct("CID", &cid),
        }
    }
}

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
