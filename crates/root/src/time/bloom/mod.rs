//!

use datalove_core::dev::*;
use serde::{Deserializer, Serializer};
use std::{
    cmp,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
};

/// Marker trait for types that can be used as a clock element.
pub trait ClockElement: Unsigned + Ord + Clone + Debug + Default {}
impl<C: Unsigned + Ord + Clone + Debug + Default> ClockElement for C {}

const K: u16 = 2;
const M: usize = 8;

/// A simple bloom clock, based on a counting bloom filter, as defined by [arXiv:1905.13064].
///
/// [arXiv:1905.13064]: https://arxiv.org/pdf/1905.13064.pdf
#[derive(Clone, Debug, Default)]
pub struct BloomClock<C>([C; M]);

impl<C: ClockElement> BloomClock<C> {
    ///
    pub fn max(&self) -> C {
        self.0
            .iter()
            .max()
            .expect("should have elements already")
            .clone()
    }
}

impl<C: ClockElement> Clock for BloomClock<C> {
    // const K: usize = K as usize;
    // const SIZE: usize = M;

    fn increment<E: AsRef<[u8]>>(&mut self, event: &E) {
        // perform k hashes on event, producing a value in 0..M
        // increment each corresponding index
        unimplemented!()
    }

    fn merge(&mut self, other: &Self) {
        for (idx, theirs) in other.0.iter().enumerate() {
            let ours = &mut self.0[idx];
            if theirs > ours {
                *ours = theirs.clone();
            }
        }
    }
}

impl<C: ClockElement> ClockExt for BloomClock<C> {
    fn compare(&self, other: &Self) -> Ordering {
        // use cmp::Ordering::*;

        // let mut cmp = None;
        // for (l, r) in self.0.iter().zip(other.0.iter()) {
        //     cmp = match (cmp, l, r) {
        //         // if first or equal, set comparison
        //         (None, l, r) => l.partial_cmp(r),
        //         (Some(Equal), l, r) => l.partial_cmp(r),
        //         // else if already non-equal, re-assert existing comparison
        //         (Some(Greater), l, r) if l >= r => cmp,
        //         (Some(Less), l, r) if l <= r => cmp,
        //         // else, the clocks are concurrent
        //         (_, _, _) => return None,
        //     };
        // }
        // cmp

        unimplemented!()
    }

    fn contains<E: AsRef<[u8]>>(&self, event: &E, since: Option<&Self>) -> (bool, f64) {
        // self's n = k * sum
        //
        unimplemented!()
    }
}

impl<C: ClockElement> Eq for BloomClock<C> {}

impl<C: ClockElement> PartialEq for BloomClock<C> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(cmp::Ordering::Equal)
    }
}

impl<C: ClockElement> PartialOrd for BloomClock<C> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        unimplemented!()
    }
}

impl<C: ClockElement> Serialize for BloomClock<C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unimplemented!()
    }
}

impl<'de, C: ClockElement> Deserialize<'de> for BloomClock<C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        unimplemented!()
    }
}

///
fn filter_false_pos_rate(k: u16, m: usize, n: usize) -> f64 {
    let bit_not_one = 1f64 - (m as f64).recip();
    let k_bits_not_one = bit_not_one.powi(k as i32);
    let k_bits_not_one_after_n = k_bits_not_one.powf(n as f64);
    (1f64 - k_bits_not_one_after_n).powi(k as i32)
}

///
fn clock_false_pos_rate(k: u16, m: usize) -> f64 {
    unimplemented!()
}
