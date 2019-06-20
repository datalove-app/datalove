use serde::ser::{Serialize, Serializer};

/// DagFloat
pub enum DagFloat {
    F32(f32),
    F64(f64),
}

impl Serialize for DagFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DagFloat::F32(num) => serializer.serialize_f32(*num),
            DagFloat::F64(num) => serializer.serialize_f64(*num),
        }
    }
}
