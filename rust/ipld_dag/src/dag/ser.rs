use crate::{format::Encoder, Dag};
use serde::{Serialize, Serializer};

impl Serialize for Dag {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            // encoder-specific
            Dag::ByteBuf(buf, base) => serializer.encode_bytes(buf, *base),
            Dag::Link(cid, _) => serializer.encode_link(cid),

            // standard
            Dag::Null => serializer.serialize_none(),
            Dag::Bool(b) => serializer.serialize_bool(*b),
            Dag::Integer(int) => int.serialize(serializer),
            Dag::Float(float) => float.serialize(serializer),
            Dag::String(s) => serializer.serialize_str(s),
            Dag::List(seq) => serializer.collect_seq(seq),
            Dag::Map(map) => serializer.collect_map(map),
        }
    }
}
