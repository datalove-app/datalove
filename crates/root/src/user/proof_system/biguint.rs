#[doc(hidden)]
#[macro_export]
macro_rules! make_biguint {
    ($name:ident, $mod_name:ident, $size:expr, $size_str:expr) => {
        mod $mod_name {
            use crate::proof_system::dev::*;
            use datalove_core::dev::*;
            use num_bigint::BigUint;
            use std::borrow::Borrow;

            pub(crate) static ZERO: BigUint = BigUint::new(vec![0u32; $size / 32]);
            pub(crate) static ONE: BigUint = ZERO + 1u32;

            #[derive(Clone, Debug)]
            pub struct $name<F: Field> {
                // Least significant bit first
                bits: Vec<Boolean<F>>,
                value: Option<BigUint>,
            }

            impl<F: Field> $name<F> {
                #[doc = "Construct a constant `BigUint"]
                #[doc = $size_str]
                #[doc = "` from the native `BigUint` type."]
                pub fn constant(value: BigUint) -> Self {
                    let mut bits = Vec::with_capacity($size);

                    let mut value = value;
                    for _ in 0..$size {
                        if value & ONE == ONE {
                            bits.push(Boolean::constant(true))
                        } else {
                            bits.push(Boolean::constant(false))
                        }
                        value >>= 1;
                    }

                    $name { bits, value: Some(value) }
                }

                /// Turns `self` into the underlying little-endian bits.
                pub fn to_bits_le(&self) -> Vec<Boolean<F>> {
                    self.bits.clone()
                }

                /// Construct `Self` from a slice of `Boolean`s.
                ///
                /// # Panics
                ///
                /// This method panics if `bits.len() != u
                #[doc($size_str)]
                #[doc("`.")]
                pub fn from_bits_le(bits: &[Boolean<F>]) -> Self {
                    assert_eq!(bits.len(), $size);

                    let bits = bits.to_vec();
                    let mut value = Some(ZERO);
                    for b in bits.iter().rev() {
                        value.as_mut().map(|v| *v <<= 1);

                        match (b, b.value()) {
                            (&Boolean::Constant(b), _)
                            | (&Boolean::Is(_), Ok(b)) if b => {
                                value.as_mut().map(|v| *v |= ONE);
                            },
                            (&Boolean::Constant(b), _)
                            | (&Boolean::Is(_), Ok(b)) if !b => {
                                value.as_mut().map(|v| *v |= ZERO);
                            },
                            (&Boolean::Not(_), Ok(b)) if !b => {
                                value.as_mut().map(|v| *v |= ONE);
                            }
                            (&Boolean::Not(_), Ok(b)) if b => {
                                value.as_mut().map(|v| *v |= ZERO);
                            }
                            _ => value = None,
                        }
                    }

                    Self { value, bits }
                }

                /*
                /// Rotates `self` to the right by `by` steps, wrapping around.
                #[tracing::instrument(target = "r1cs", skip(self))]
                pub fn rotr(&self, by: usize) -> Self {
                    let by = by % $size;

                    let new_bits = self
                        .bits
                        .iter()
                        .skip(by)
                        .chain(self.bits.iter())
                        .take($size)
                        .cloned()
                        .collect();

                    $name {
                        bits: new_bits,
                        value: self
                            .value
                            .map(|v| v.rotate_right(u32::try_from(by).unwrap())),
                    }
                }

                /// Outputs `self ^ other`.
                ///
                /// If at least one of `self` and `other` are constants, then this method
                /// *does not* create any constraints or variables.
                #[tracing::instrument(target = "r1cs", skip(self, other))]
                pub fn xor(&self, other: &Self) -> Result<Self, SynthesisError> {
                    let new_value = match (self.value, other.value) {
                        (Some(a), Some(b)) => Some(a ^ b),
                        _ => None,
                    };

                    let bits = self
                        .bits
                        .iter()
                        .zip(other.bits.iter())
                        .map(|(a, b)| a.xor(b))
                        .collect::<Result<_, _>>()?;

                    Ok($name {
                        bits,
                        value: new_value,
                    })
                }

                /// Perform modular addition of `operands`.
                ///
                /// The user must ensure that overflow does not occur.
                #[tracing::instrument(target = "r1cs", skip(operands))]
                pub fn addmany(operands: &[Self]) -> Result<Self, SynthesisError>
                where
                    F: PrimeField,
                {
                    // Make some arbitrary bounds for ourselves to avoid overflows
                    // in the scalar field
                    assert!(F::Params::MODULUS_BITS >= 2 * $size);

                    assert!(operands.len() >= 1);
                    assert!($size * operands.len() <= F::Params::MODULUS_BITS as usize);

                    if operands.len() == 1 {
                        return Ok(operands[0].clone());
                    }

                    // Compute the maximum value of the sum so we allocate enough bits for
                    // the result
                    let mut max_value = (operands.len() as u128) * u128::from($native::max_value());

                    // Keep track of the resulting value
                    let mut result_value = Some(0u128);

                    // This is a linear combination that we will enforce to be "zero"
                    let mut lc = LinearCombination::zero();

                    let mut all_constants = true;

                    // Iterate over the operands
                    for op in operands {
                        // Accumulate the value
                        match op.value {
                            Some(val) => {
                                result_value.as_mut().map(|v| *v += u128::from(val));
                            }

                            None => {
                                // If any of our operands have unknown value, we won't
                                // know the value of the result
                                result_value = None;
                            }
                        }

                        // Iterate over each bit_gadget of the operand and add the operand to
                        // the linear combination
                        let mut coeff = F::one();
                        for bit in &op.bits {
                            match *bit {
                                Boolean::Is(ref bit) => {
                                    all_constants = false;

                                    // Add coeff * bit_gadget
                                    lc += (coeff, bit.variable());
                                }
                                Boolean::Not(ref bit) => {
                                    all_constants = false;

                                    // Add coeff * (1 - bit_gadget) = coeff * ONE - coeff * bit_gadget
                                    lc = lc + (coeff, Variable::One) - (coeff, bit.variable());
                                }
                                Boolean::Constant(bit) => {
                                    if bit {
                                        lc += (coeff, Variable::One);
                                    }
                                }
                            }

                            coeff.double_in_place();
                        }
                    }

                    // The value of the actual result is modulo 2^$size
                    let modular_value = result_value.map(|v| v as $native);

                    if all_constants && modular_value.is_some() {
                        // We can just return a constant, rather than
                        // unpacking the result into allocated bits.

                        return Ok($name::constant(modular_value.unwrap()));
                    }
                    let cs = operands.cs();

                    // Storage area for the resulting bits
                    let mut result_bits = vec![];

                    // Allocate each bit_gadget of the result
                    let mut coeff = F::one();
                    let mut i = 0;
                    while max_value != 0 {
                        // Allocate the bit_gadget
                        let b = AllocatedBit::new_witness(cs.clone(), || {
                            result_value.map(|v| (v >> i) & 1 == 1).get()
                        })?;

                        // Subtract this bit_gadget from the linear combination to ensure the sums
                        // balance out
                        lc = lc - (coeff, b.variable());

                        result_bits.push(b.into());

                        max_value >>= 1;
                        i += 1;
                        coeff.double_in_place();
                    }

                    // Enforce that the linear combination equals zero
                    cs.enforce_constraint(lc!(), lc!(), lc)?;

                    // Discard carry bits that we don't care about
                    result_bits.truncate($size);

                    Ok($name {
                        bits: result_bits,
                        value: modular_value,
                    })
                }
                 */
            }

            impl<F: Field> R1CSVar<F> for $name<F> {
                type Value = BigUint;

                fn cs(&self) -> ConstraintSystemRef<F> {
                    self.bits.as_slice().cs()
                }

                fn value(&self) -> Result<Self::Value, SynthesisError> {
                    let mut value = None;
                    for (i, bit) in self.bits.iter().enumerate() {
                        value = match (value, bit.value()?) {
                            (None, b) if b => Some(ONE << i),
                            (None, b) if !b => Some(ZERO << i),
                            (Some(value), b) if b => Some(value + (ONE << i)),
                            (Some(value), b) if !b => Some(value + (ZERO << i)),
                        };
                    }
                    debug_assert_eq!(self.value, value);
                    value.get()
                }
            }

            impl<F: Field> AllocVar<BigUint, F> for $name<F> {
                fn new_variable<T: Borrow<BigUint>>(
                    cs: impl Into<Namespace<F>>,
                    f: impl FnOnce() -> Result<T, SynthesisError>,
                    mode: AllocationMode,
                ) -> Result<Self, SynthesisError> {
                    let ns = cs.into();
                    let cs = ns.cs();

                    let value = *f()?.borrow();
                    let bits = (0..$size)
                        .map(|i| Some((value >> i) & ONE == ONE))
                        .map(|v| Boolean::new_variable(cs.clone(), || v.get(), mode))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(Self { bits, value: Some(value) })
                }
            }

            impl<F: Field> EqGadget<F> for $name<F> {
                fn is_eq(&self, other: &Self) -> Result<Boolean<F>, SynthesisError> {
                    self.bits.as_slice().is_eq(&other.bits)
                }

                fn conditional_enforce_equal(
                    &self,
                    other: &Self,
                    condition: &Boolean<F>,
                ) -> Result<(), SynthesisError> {
                    self.bits.conditional_enforce_equal(&other.bits, condition)
                }

                fn conditional_enforce_not_equal(
                    &self,
                    other: &Self,
                    condition: &Boolean<F>,
                ) -> Result<(), SynthesisError> {
                    self.bits
                        .conditional_enforce_not_equal(&other.bits, condition)
                }
            }

            impl<F: Field> ToBytesGadget<F> for $name<F> {
                fn to_bytes(&self) -> Result<Vec<UInt8<F>>, SynthesisError> {
                    Ok(self.bits.chunks(8).map(UInt8::from_bits_le).collect())
                }
            }

            #[cfg(test)]
            mod test {
                use super::$name;
                use crate::proof_system::dev::*;
                use crate::{bits::boolean::Boolean, prelude::*, Vec};
                use algebra::bls12_381::Fr;
                use rand::{Rng, SeedableRng};
                use rand_xorshift::XorShiftRng;

                #[test]
                fn test_from_bits() -> Result<(), SynthesisError> {
                    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

                    for _ in 0..1000 {
                        let v = (0..$size)
                            .map(|_| Boolean::constant(rng.gen()))
                            .collect::<Vec<Boolean<Fr>>>();

                        let b = $name::from_bits_le(&v);

                        for (i, bit) in b.bits.iter().enumerate() {
                            match bit {
                                &Boolean::Constant(bit) => {
                                    assert_eq!(bit, ((b.value()? >> i) & 1 == 1));
                                }
                                _ => unreachable!(),
                            }
                        }

                        let expected_to_be_same = b.to_bits_le();

                        for x in v.iter().zip(expected_to_be_same.iter()) {
                            match x {
                                (&Boolean::Constant(true), &Boolean::Constant(true)) => {}
                                (&Boolean::Constant(false), &Boolean::Constant(false)) => {}
                                _ => unreachable!(),
                            }
                        }
                    }
                    Ok(())
                }
            }
        }
    };
}

// impl<F: Field> AllocVar<BigUint, F> for BigUintVar<F> {
//     fn new_variable<T: Borrow<BigUint>>(
//         cs: impl Into<Namespace<F>>,
//         f: impl FnOnce() -> Result<T, SynthesisError>,
//         mode: AllocationMode,
//     ) -> Result<Self, SynthesisError> {
//         let ns = cs.into();
//         let cs = ns.cs();
//         let value = f().map(|f| *f.borrow());
//         let values = match value {
//             Ok(val) => (0..$size).map(|i| Some((val >> i) & 1 == 1)).collect(),
//             _ => vec![None; $size],
//         };
//         let bits = values
//             .into_iter()
//             .map(|v| Boolean::new_variable(cs.clone(), || v.get(), mode))
//             .collect::<Result<Vec<_>, _>>()?;
//         Ok(Self {
//             bits,
//             value: value.ok(),
//         })
//     }
// }

// impl<F: Field> EqGadget<F> for BigUintVar<F> {
//     fn is_eq(&self, other: &Self) -> Result<Boolean<F>, SynthesisError> {
//         // self.bits.as_slice().is_eq(&other.bits)
//         unimplemented!()
//     }

//     fn conditional_enforce_equal(
//         &self,
//         other: &Self,
//         condition: &Boolean<F>,
//     ) -> Result<(), SynthesisError> {
//         // self.bits.conditional_enforce_equal(&other.bits, condition)
//         unimplemented!()
//     }

//     fn conditional_enforce_not_equal(
//         &self,
//         other: &Self,
//         condition: &Boolean<F>,
//     ) -> Result<(), SynthesisError> {
//         // self.bits
//         //     .conditional_enforce_not_equal(&other.bits, condition)
//         unimplemented!()
//     }
// }

// impl<F: Field> ToBytesGadget<F> for BigUintVar<F> {
//     fn to_bytes(&self) -> Result<Vec<UInt8<F>>, SynthesisError> {
//         unimplemented!()
//     }
// }
