use crate::dev::*;
use std::{cmp, fmt::Debug, hash::Hash, ops::AddAssign};

///
#[derive(Debug)]
pub enum Ordering {
    Partial(cmp::Ordering),
    Total(cmp::Ordering),
}

/// A type representing a peer-to-peerd distributed clock.
pub trait Clock: Sized
where
    Self: Debug + Default,
    Self: Serialize + for<'de> Deserialize<'de>,
{
    const K: usize;
    const SIZE: usize;

    /// Determines if the clock has witnessed the given event, and the confidence
    /// in that determination.
    fn contains<E: Hash>(&self, event: &E, since: Option<&Self>) -> (bool, f64);

    /// Returns an `Ordering` between two clocks.
    fn compare(&self, other: &Self) -> Ordering;

    /// Increments the clock with a new event.
    fn increment<E: Hash>(&mut self, event: &E);

    /// Merges another clock into this one.
    fn merge(&mut self, other: &Self);
}
