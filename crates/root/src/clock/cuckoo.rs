use datalove_core::dev::*;
use std::marker::PhantomData;

pub struct CuckooFilter;

/// A bloom clock, based on a cuckoo filter.
pub struct CuckooClock<C, E> {
    counter: PhantomData<C>,
    event: PhantomData<E>,
}
