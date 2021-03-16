use std::{fmt::Debug, ops::AddAssign};

///
pub(crate) trait Timestamp: Clone + Debug + Default + Eq + AddAssign {}

///
pub(crate) trait TimestampVar<T: Timestamp, F: Field>: AllocVar<T, F> {}
