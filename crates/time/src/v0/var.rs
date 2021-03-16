use super::value::{BloomClock, Counter};
use crate::interfaces::ClockVar;
use ark_ff::Field;
use ark_r1cs_std::alloc::{AllocVar, AllocationMode};
use ark_relations::r1cs::{ConstraintSystemRef, Namespace, SynthesisError};
use digest::Digest;
use std::{borrow::Borrow, fmt::Debug, marker::PhantomData};

///
#[derive(Clone, Debug)]
pub struct BloomClockVar<CV, C: Counter, D: Digest, const K: usize, const M: usize> {
    elements: [CV; M],
    value: Option<BloomClock<C, D, K, M>>,
    // _engine: PhantomData<E>,
    // _pairing_var: PhantomData<P>,
}

// impl<C, CV, E, P> BloomClock<C, CV, E, P>
// where
//     E: PairingEngine,
// {
//     pub fn increment(&self) -> Result<Self, SynthesisError> {
//         unimplemented!()
//     }

//     pub fn merge(&self, other: &Self) -> Result<Self, SynthesisError> {
//         unimplemented!()
//     }
// }

// impl<F: Field> ConstraintSynthesizer<F> for BloomClock {
//     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
//         unimplemented!()
//     }
// }

// impl<C, CV, E, P> R1CSVar<E::Fr> for BloomClock<C, CV, E, P>
// where
//     C: ClockElement,
//     CV: R1CSVar<E::Fr, Value = C>,
//     E: PairingEngine,
//     P: PairingVar<E>,
// {
//     type Value = BloomClock<C>;

//     fn cs(&self) -> ConstraintSystemRef<E::Fr> {
//         self.elements.as_slice().cs()
//     }

//     fn value(&self) -> Result<Self::Value, SynthesisError> {
//         unimplemented!()
//     }
// }

impl<F, CV, C, D, const K: usize, const M: usize> AllocVar<BloomClock<C, D, K, M>, F>
    for BloomClockVar<CV, C, D, K, M>
where
    F: Field,
    CV: AllocVar<C, F>,
    // E: PairingEngine,
    C: Counter,
    D: Digest,
{
    fn new_variable<T: Borrow<BloomClock<C, D, K, M>>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let ns = cs.into();
        let cs = ns.cs();

        // f().and_then(|clock| unimplemented!())
        unimplemented!()
    }
}

// impl<C, CV, E, P> EqGadget<E::Fr> for BloomClock<C, CV, E, P>
// where
//     C: ClockElement,
//     CV: EqGadget<E::Fr>,
//     [CV]: EqGadget<E::Fr>,
//     E: PairingEngine,
//     P: PairingVar<E>,
// {
//     fn is_eq(&self, other: &Self) -> Result<Boolean<E::Fr>, SynthesisError> {
//         self.elements.as_slice().is_eq(&other.elements)
//     }

//     fn conditional_enforce_equal(
//         &self,
//         other: &Self,
//         condition: &Boolean<E::Fr>,
//     ) -> Result<(), SynthesisError> {
//         self.elements
//             .conditional_enforce_equal(&other.elements, condition)
//     }

//     fn conditional_enforce_not_equal(
//         &self,
//         other: &Self,
//         condition: &Boolean<E::Fr>,
//     ) -> Result<(), SynthesisError> {
//         self.elements
//             .conditional_enforce_not_equal(&other.elements, condition)
//     }
// }

// impl<C, CV, E, P> ToBytesGadget<E::Fr> for BloomClock<C, CV, E, P>
// where
//     C: ClockElement,
//     CV: ToBytesGadget<E::Fr>,
//     // [CV]: ToBytesGadget<E::Fr>,
//     E: PairingEngine,
//     P: PairingVar<E>,
// {
//     fn to_bytes(&self) -> Result<Vec<UInt8<E::Fr>>, SynthesisError> {
//         unimplemented!()
//     }
// }
