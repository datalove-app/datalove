use crate::{format::Encoder, FreeNode};
use serde::{Serialize, Serializer};

impl Serialize for FreeNode {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            // encoder-specific
            FreeNode::ByteBuf(buf, base) => serializer.encode_bytes(buf, *base),
            FreeNode::Link(cid, _) => serializer.encode_link(cid),

            // standard
            FreeNode::Null => serializer.serialize_none(),
            FreeNode::Bool(b) => serializer.serialize_bool(*b),
            FreeNode::Integer(int) => int.serialize(serializer),
            FreeNode::Float(float) => float.serialize(serializer),
            FreeNode::String(s) => serializer.serialize_str(s),
            FreeNode::List(seq) => serializer.collect_seq(seq),
            FreeNode::Map(map) => serializer.collect_map(map),
        }
    }
}
