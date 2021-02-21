use crate::dev::*;
use std::{cmp, fmt::Debug, hash::Hash, ops::AddAssign};

///
#[derive(Debug)]
pub enum Ordering {
    Concurrent,
    Total(cmp::Ordering),
}

/// A type representing a peer-to-peer vector clock.
pub trait Clock: Sized
where
    Self: Debug + Default,
    Self: Serialize + for<'de> Deserialize<'de>,
{
    // const K: usize;
    // const SIZE: usize;

    // type IncrementOutput;
    // type MergeOutput;

    /// Increments the clock with a new event.
    fn increment<E: AsRef<[u8]>>(&mut self, event: &E);

    /// Merges another clock into this one.
    fn merge(&mut self, other: &Self);
}

/// Extension trait for peer-to-peer vector clocks.
pub trait ClockExt: Clock {
    /// Returns an `Ordering` between two clocks.
    fn compare(&self, other: &Self) -> Ordering;

    /// Determines if the clock has witnessed the given event, and the confidence
    /// in that determination.
    fn contains<E: AsRef<[u8]>>(&self, event: &E, since: Option<&Self>) -> (bool, f64);
}
