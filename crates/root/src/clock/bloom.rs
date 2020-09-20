//!

use datalove_core::dev::*;
use std::{cmp, hash::Hash, marker::PhantomData, ops::Add};

/// A simple bloom clock, based on a counting bloom filter, as defined by [arXiv:1905.13064].
///
/// [arXiv:1905.13064]: https://arxiv.org/pdf/1905.13064.pdf
pub struct BloomClock<C, E, const K: u16, const M: usize>
where
    C: Copy + Ord,
{
    inner: [C; M],
    event: PhantomData<E>,
}

// impl<C: Copy + Ord, E> BloomClock<C, E, K, M> {
//     // pub fn size(&self) -> C {
//     //     self.inner.iter().max()
//     // }
// }

impl<C, E, const K: u16, const M: usize> PeerClock for BloomClock<C, E, K, M>
where
    C: Add + Copy + Ord,
    E: Hash,
{
    type Event = E;

    fn increment(&mut self, event: &Self::Event) {
        // perform k hashes on event, producing a value in 0..M
        // increment each corresponding index
        unimplemented!()
    }

    fn contains(&self, event: &Self::Event) -> (bool, f64) {
        // self's n = k * sum
        //
        unimplemented!()
    }

    fn compare(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

impl<C, E, const K: u16, const M: usize> CvRDT for BloomClock<C, E, K, M>
where
    C: Copy + Ord,
{
    fn merge(&mut self, other: Self) {
        for (idx, theirs) in other.inner.iter().enumerate() {
            let ours = &mut self.inner[idx];
            if theirs > ours {
                *ours = *theirs;
            }
        }
    }
}

impl<C, E, const K: u16, const M: usize> Eq for BloomClock<C, E, K, M> where C: Copy + Ord {}
impl<C, E, const K: u16, const M: usize> PartialEq for BloomClock<C, E, K, M>
where
    C: Copy + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(cmp::Ordering::Equal)
    }
}

impl<C, E, const K: u16, const M: usize> PartialOrd for BloomClock<C, E, K, M>
where
    C: Copy + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use cmp::Ordering::*;

        let mut cmp = None;
        for (l, r) in self.inner.iter().zip(other.inner.iter()) {
            cmp = match (cmp, l, r) {
                // if first or equal, set comparison
                (None, l, r) => l.partial_cmp(r),
                (Some(Equal), l, r) => l.partial_cmp(r),
                // else if already non-equal, re-assert existing comparison
                (Some(Greater), l, r) if l >= r => cmp,
                (Some(Less), l, r) if l <= r => cmp,
                // else, the clocks are concurrent
                (_, _, _) => return None,
            };
        }
        cmp
    }
}

fn filter_false_pos_rate(k: u16, m: u16, n: usize) -> f64 {
    let bit_not_one = 1f64 - (m as f64).recip();
    let k_bits_not_one = bit_not_one.powi(k as i32);
    let k_bits_not_one_after_n = k_bits_not_one.powf(n as f64);
    (1f64 - k_bits_not_one_after_n).powi(k as i32)
}

fn clock_false_pos_rate(k: u16, m: u16) -> f64 {
    unimplemented!()
}
