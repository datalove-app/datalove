use serde::ser::{Serialize, Serializer};

/// DagInt
pub enum DagInt {
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

impl Serialize for DagInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DagInt::U8(num) => serializer.serialize_u8(*num),
            DagInt::U16(num) => serializer.serialize_u16(*num),
            DagInt::U32(num) => serializer.serialize_u32(*num),
            DagInt::U64(num) => serializer.serialize_u64(*num),
            DagInt::U128(num) => serializer.serialize_u128(*num),
            DagInt::I8(num) => serializer.serialize_i8(*num),
            DagInt::I16(num) => serializer.serialize_i16(*num),
            DagInt::I32(num) => serializer.serialize_i32(*num),
            DagInt::I64(num) => serializer.serialize_i64(*num),
            DagInt::I128(num) => serializer.serialize_i128(*num),
        }
    }
}
