use crate::{
    cid::CID,
    dag::Dag,
    error::Error,
    format::{Encode, Encoder},
};
use multibase::Base;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// pub enum Link<'de, T: Serialize + Deserialize<'de>> {
/// An IPLD Dag [Link], wrapping either a [`CID`] or a `Box<Dag>`.
///
/// [link]: https://github.com/ipld/specs/blob/master/data-model-layer/data-model.md#link-kind
/// [`CID`]: https://github.com/ipld/specs/blob/master/block-layer/CID.md
#[derive(From)]
// #[serde(untagged)]
pub enum Link<T: Dag> {
    CID(CID, Option<Base>),
    Dag(Box<T>),
}

impl<T: Dag> Encode for Link<T> {
    #[inline]
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
        <E as serde::Serializer>::Error: Into<Error>,
    {
        match self {
            Link::CID(cid, base) => encoder.encode_link(cid, *base),
            Link::Dag(dag) => dag.encode(encoder),
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

impl<T: Dag> From<CID> for Link<T> {
    fn from(cid: CID) -> Link<T> {
        Link::CID(cid, None)
    }
}

impl<T: Dag> From<T> for Link<T> {
    fn from(dag: T) -> Link<T> {
        Link::Dag(Box::new(dag))
    }
}
