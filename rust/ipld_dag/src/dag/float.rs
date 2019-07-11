use serde::{Serialize, Serializer};

/// Float wrapper
#[derive(Clone, Debug, From)]
pub enum Float {
    /// `f32`
    F32(f32),

    /// `f64`
    F64(f64),
}

impl Serialize for Float {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Float::F32(num) => serializer.serialize_f32(num),
            Float::F64(num) => serializer.serialize_f64(num),
        }
    }
}
