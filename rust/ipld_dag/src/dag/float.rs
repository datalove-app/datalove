use serde::ser::{Serialize, Serializer};

/// Float
pub enum Float {
    F32(f32),
    F64(f64),
}

impl Serialize for Float {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Float::F32(num) => serializer.serialize_f32(*num),
            Float::F64(num) => serializer.serialize_f64(*num),
        }
    }
}
