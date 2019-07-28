use crate::{base::Base, CID};
use serde::Serializer;

///
pub trait Encoder: Serializer {
    // /// Encodes an IPLD Node, returning it's encoded representation as `Vec<u8>` and the resulting `CID`.
    // fn encode<'a, N>(self, node: N) -> Result<(CID, Vec<u8>), Self::Error>
    // where
    //     N: Node<'a>;

    // ///
    // fn encode_into<'a, N, W>(self, node: N, writer: &mut W) -> Result<(), Self::Error>
    // where
    //     N: Node<'a>,
    //     W: std::io::Write;

    /// Encodes `&[u8]`.
    ///
    /// By default, serializes `&[u8]` as bytes, or as a `multibase`-encoded `str`.
    fn encode_bytes(self, bytes: &[u8], base: Option<Base>) -> Result<Self::Ok, Self::Error>;

    /// Encodes an IPLD Link `CID`.
    ///
    /// Encodes `CID` as bytes if `multibase::Base` is missing, otherwise as a string.
    fn encode_link(self, cid: &CID) -> Result<Self::Ok, Self::Error>;
}

/// Blanket impl of `Encoder` for all `Serializer`s that can be [`specialized`] by downstream impls.
impl<T> Encoder for T
where
    T: Serializer,
{
    /// By default, serializes `&[u8]` as bytes, or as a `multibase`-encoded `str`.
    default fn encode_bytes(
        self,
        bytes: &[u8],
        _base: Option<Base>,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(bytes)
    }

    /// Encodes a `CID` as bytes if `multibase::Base` is missing, otherwise as a string.
    default fn encode_link(self, cid: &CID) -> Result<Self::Ok, Self::Error> {
        match cid.base() {
            None => self.serialize_bytes(&cid.to_vec()),
            Some(_) => self.serialize_str(&cid.to_string(None)),
        }
    }
}
