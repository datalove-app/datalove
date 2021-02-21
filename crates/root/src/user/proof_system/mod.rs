mod circuit;
mod clock;
mod recursive;
#[macro_use]
mod biguint;
mod keyset;

mod dev {
    pub use crate::make_biguint;
    pub use algebra::prelude::*;
    pub use algebra::{FftParameters, PairingEngine as PE, ToBytes, ToConstraintField};
    pub use crypto_primitives::{NIZKVerifierGadget, NIZK};
    pub use r1cs_core::{ConstraintSynthesizer, ConstraintSystemRef, Namespace, SynthesisError};
    pub use r1cs_std::prelude::*;
    pub use r1cs_std::{fields::fp::FpVar, pairing::PairingVar as PVar, Assignment};
    pub use rand::Rng;

    make_biguint!(BigUint256, biguint256, 256, "256");
    pub use biguint256::*;
}
