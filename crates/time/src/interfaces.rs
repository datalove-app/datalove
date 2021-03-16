use ark_ff::Field;
use ark_r1cs_std::alloc::AllocVar;
use std::{fmt::Debug, ops::{AddAssign, Sub}};

/// A type representing a peer-to-peer vector clock.
pub(crate) trait Clock:
    Clone
    + Default
    + for<'a> AddAssign<&'a [u8]>
    + for<'a> AddAssign<&'a Self>
{
    /// Increments the clock with a new event.
    fn increment(&mut self, event: &impl AsRef<[u8]>) {
        *self += event.as_ref()
    }

    /// Merges another clock into this one.
    fn merge(&mut self, other: &Self) {
        *self += other
    }
}

/// Extension trait for peer-to-peer vector clocks.
pub(crate) trait ClockExt: Clock + PartialOrd + PartialEq {
    /// Determines if the clock has witnessed the given event, and the confidence
    /// in that determination.
    fn contains(&self, event: &impl AsRef<[u8]>, since: Option<&Self>) -> Option<(bool, f64)>;
}

pub(crate) trait ClockVar<C: Clock, F: Field>: AllocVar<C, F> {}
