use serde::{Serialize, Serializer};

/// Signed and unsigned integer wrapper
#[derive(Clone, Debug, From, Hash, PartialEq, Eq)]
pub enum Int {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}

impl Serialize for Int {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Int::U8(num) => serializer.serialize_u8(num),
            Int::U16(num) => serializer.serialize_u16(num),
            Int::U32(num) => serializer.serialize_u32(num),
            Int::U64(num) => serializer.serialize_u64(num),
            Int::U128(num) => serializer.serialize_u128(num),
            Int::I8(num) => serializer.serialize_i8(num),
            Int::I16(num) => serializer.serialize_i16(num),
            Int::I32(num) => serializer.serialize_i32(num),
            Int::I64(num) => serializer.serialize_i64(num),
            Int::I128(num) => serializer.serialize_i128(num),
        }
    }
}
