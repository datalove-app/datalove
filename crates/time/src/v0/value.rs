use crate::interfaces::{Clock, ClockExt};
use digest::Digest;
use num_traits::{One, Unsigned, Zero};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    ops::{AddAssign, Sub},
};

/// Marker trait for types that can be used as a clock element.
pub trait Counter: Copy + Debug + Ord + Unsigned + One + Zero + AddAssign {}
impl<C: Copy + Debug + Ord + Unsigned + One + Zero + AddAssign> Counter for C {}

/// A simple bloom clock, based on a counting bloom filter, as defined by
/// [arXiv:1905.13064].
///
/// [arXiv:1905.13064]: https://arxiv.org/pdf/1905.13064.pdf
#[derive(Copy, Clone, Debug)]
pub struct BloomClock<C: Counter, D: Digest, const K: usize, const M: usize> {
    inner: [C; M],
    _digest: PhantomData<D>,
}

impl<C: Counter, D: Digest, const K: usize, const M: usize> Default for BloomClock<C, D, K, M> {
    fn default() -> Self {
        assert!(M % K == 0);

        Self {
            inner: [C::zero(); M],
            _digest: PhantomData,
        }
    }
}

impl<C: Counter, D: Digest, const K: usize, const M: usize> BloomClock<C, D, K, M> {
    /// Provides access to the inner slice.
    pub fn inner(&self) -> &[C; M] {
        &self.inner
    }

    ///
    pub fn max(&self) -> C {
        *self
            .inner
            .iter()
            .max()
            .expect("should have elements already")
    }

    /// Subtracts another clock from this one. In the event they are
    /// incomparable, this returns `None`.
    pub fn sub(&self, other: &Self) -> Option<Self> {
        let mut diff_clock = Self::default();
        for (i, (l, r)) in self.inner.iter().zip(other.inner.iter()).enumerate() {
            if r > l {
                return None;
            } else {
                diff_clock.inner[i] = *l - *r;
            }
        }
        Some(diff_clock)
    }

    /// Returns the indices in the clock to be incremented for a given event's
    /// bytes.
    pub fn indices_for_event(event: &[u8]) -> [usize; K] {
        Self(D::new().chain(event))
    }

    ///
    pub fn indices_for_prehashed(prehashed: D) -> [usize; K] {
        let mut indices = [0usize; K];
        let mut digest = prehashed.finalize();
        indices[0] = Self::index_for_digest(digest.as_slice());

        for round in 1..K {
            digest = D::digest(digest.as_slice());
            indices[round] = Self::index_for_digest(digest.as_slice());
        }

        indices
    }

    /// This currently takes the count of the digest's 1 bits mod M. An
    /// alternative could just count 1 bits in the digest's first M bits.
    /// TODO: is this random enough?
    fn index_for_digest(digest: &[u8]) -> usize {
        digest.iter().map(|b| b.count_ones()).sum::<u32>() as usize
    }

    fn contains(&self, indices: [usize; K]) -> (bool, f64) {
        let mut counts = HashMap::<usize, C>::new();
        for idx in indices.iter() {
            if let Some(count) = counts.get_mut(idx) {
                *count += C::one();
            } else {
                counts.insert(*idx, C::one());
            }
        }

        // if the clock doesn't have the index or the count is to low, it
        // definitely doesn't contain the event
        if !counts.iter().all(|(idx, count)| &self.inner[*idx] >= count) {
            return (false, 1.0);
        }

        unimplemented!()
    }

    fn contains_since(&self, indices: [usize; K], other: &Self) -> Option<(bool, f64)> {
        // produce a clock from self - other; if incomparable, return None
        let diffed = match self.sub(other) {
            None => return None,
            Some(diffed) => diffed,
        };

        unimplemented!()
    }
}

impl<C: Counter, D: Digest + Clone, const K: usize, const M: usize> Clock
    for BloomClock<C, D, K, M>
{
}

impl<C: Counter, D: Digest + Clone, const K: usize, const M: usize> ClockExt
    for BloomClock<C, D, K, M>
{
    fn contains(&self, event: &impl AsRef<[u8]>, since: Option<&Self>) -> Option<(bool, f64)> {
        // self's n = k * sum
        //
        let indices = Self::indices_for_event(event.as_ref());
        match since {
            None => Some(self.contains(indices)),
            Some(other) => self.contains_since(indices, other),
        }
    }
}

impl<'a, C: Counter, D: Digest, const K: usize, const M: usize> AddAssign<&'a [u8]>
    for BloomClock<C, D, K, M>
{
    fn add_assign(&mut self, rhs: &[u8]) {
        for idx in Self::indices_for_event(rhs).iter() {
            self.inner[*idx] += C::one();
        }
    }
}

impl<'a, C: Counter, D: Digest, const K: usize, const M: usize> AddAssign<&'a Self>
    for BloomClock<C, D, K, M>
{
    fn add_assign(&mut self, rhs: &Self) {
        for (idx, theirs) in rhs.inner.iter().enumerate() {
            if &self.inner[idx] <= theirs {
                self.inner[idx] = *theirs;
            }
        }
    }
}

impl<C: Counter, D: Digest, const K: usize, const M: usize> PartialOrd for BloomClock<C, D, K, M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut cmp = None;

        for (i, (l, r)) in self.inner.iter().zip(other.inner.iter()).enumerate() {
            cmp = match (i, cmp, l, r) {
                // if first or equal, set comparison
                (0, None, l, r) => l.partial_cmp(r),
                (_, Some(Ordering::Equal), l, r) => l.partial_cmp(r),
                // else if already non-equal, re-assert existing comparison
                (_, Some(Ordering::Greater), l, r) if l >= r => cmp,
                (_, Some(Ordering::Less), l, r) if l <= r => cmp,
                // else, the clocks are concurrent
                (_, _, _, _) => return None,
            };
        }

        cmp
    }
}

// impl<C: Counter, D: Digest, const K: usize, const M: usize> Eq for
// BloomClock<C> {}

impl<C: Counter, D: Digest, const K: usize, const M: usize> PartialEq for BloomClock<C, D, K, M> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
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
