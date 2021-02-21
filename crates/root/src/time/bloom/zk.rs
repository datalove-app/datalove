use super::dev::*;
use datalove_core::dev::*;
use std::{borrow::Borrow, fmt::Debug, marker::PhantomData};

#[derive(Clone, Debug)]
pub struct BloomClock<C, CV, E, P> {
    elements: [CV; <BloomClock<C> as Clock>::SIZE],
    // elements: Vec<CV>,
    value: Option<BloomClock<C>>,
    _engine: PhantomData<E>,
    _pairing_var: PhantomData<P>,
}

impl<C, CV, E, P> BloomClock<C, CV, E, P>
where
    E: PairingEngine,
{
    pub fn increment(&self) -> Result<Self, SynthesisError> {
        unimplemented!()
    }

    pub fn merge(&self, other: &Self) -> Result<Self, SynthesisError> {
        unimplemented!()
    }
}

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

impl<C, CV, E, P> AllocVar<BloomClock<C>, E::Fr> for BloomClock<C, CV, E, P>
where
    C: ClockElement,
    CV: AllocVar<C, E::Fr>,
    E: PairingEngine,
    P: PairingVar<E>,
{
    fn new_variable<T: Borrow<BloomClock<C>>>(
        cs: impl Into<Namespace<E::Fr>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let ns = cs.into();
        let cs = ns.cs();

        // f().and_then(|clock| unimplemented!())
        unimplemented!()
    }
}

impl<C, CV, E, P> EqGadget<E::Fr> for BloomClock<C, CV, E, P>
where
    C: ClockElement,
    CV: EqGadget<E::Fr>,
    [CV]: EqGadget<E::Fr>,
    E: PairingEngine,
    P: PairingVar<E>,
{
    fn is_eq(&self, other: &Self) -> Result<Boolean<E::Fr>, SynthesisError> {
        self.elements.as_slice().is_eq(&other.elements)
    }

    fn conditional_enforce_equal(
        &self,
        other: &Self,
        condition: &Boolean<E::Fr>,
    ) -> Result<(), SynthesisError> {
        self.elements
            .conditional_enforce_equal(&other.elements, condition)
    }

    fn conditional_enforce_not_equal(
        &self,
        other: &Self,
        condition: &Boolean<E::Fr>,
    ) -> Result<(), SynthesisError> {
        self.elements
            .conditional_enforce_not_equal(&other.elements, condition)
    }
}

impl<C, CV, E, P> ToBytesGadget<E::Fr> for BloomClock<C, CV, E, P>
where
    C: ClockElement,
    CV: ToBytesGadget<E::Fr>,
    // [CV]: ToBytesGadget<E::Fr>,
    E: PairingEngine,
    P: PairingVar<E>,
{
    fn to_bytes(&self) -> Result<Vec<UInt8<E::Fr>>, SynthesisError> {
        unimplemented!()
    }
}

// impl<F: Field> ConstraintSynthesizer<F> for BloomClock {
//     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
//         unimplemented!()
//     }
// }
