use super::dev::*;
use algebra::{mnt4_298, mnt6_298, test_rng};
use crypto_primitives::nizk::groth16::{
    constraints::{Groth16VerifierGadget, ProofVar, VerifyingKeyVar},
    Groth16,
};
use groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
    Parameters, Proof, VerifyingKey,
};
use r1cs_std::{mnt4_298::PairingVar as MNT4_298PV, mnt6_298::PairingVar as MNT6_298PV};
use std::{marker::PhantomData, ops::MulAssign};

#[derive(Clone, Debug)]
pub struct MNT46;
impl CurvePair for MNT46 {
    type Tick = mnt4_298::MNT4_298;
    type Tock = mnt6_298::MNT6_298;
}

#[derive(Clone, Debug)]
pub struct MNT64;
impl CurvePair for MNT64 {
    type Tick = mnt6_298::MNT6_298;
    type Tock = mnt4_298::MNT4_298;
}

pub trait CurvePair: Clone
where
    <Self::Tick as PE>::G1Projective: MulAssign<<Self::Tock as PE>::Fq>,
    <Self::Tick as PE>::G2Projective: MulAssign<<Self::Tock as PE>::Fq>,
    <Self::Tick as PE>::G1Affine:
        ToConstraintField<<<Self::Tock as PE>::Fr as Field>::BasePrimeField>,
    <Self::Tick as PE>::G2Affine:
        ToConstraintField<<<Self::Tock as PE>::Fr as Field>::BasePrimeField>,
{
    type Tick: PE<
        Fr = <Self::Tock as PE>::Fq,
        Fq = <Self::Tock as PE>::Fr,
    >;
    type Tock: PE;

    // type TickVar: PVar<C::Tick>;
    // type TockVar: PVar<C::Tock>;

    // const TICK_CURVE: &'static str;
    // const TOCK_CURVE: &'static str;
}

trait GenericConstraintSynthesizer<F: Field>
where
    Self: Clone + ConstraintSynthesizer<F>,
{
    fn new(inputs: Vec<F>) -> Self;
}

fn prove(index: u128) -> () {
    if index % 2 == 0 {
        // RecursiveCircuit::<Ci, MNT46, ..., ...>::prove()
    } else {
        // RecursiveCircuit::<Ci, MNT64, ..., ...>::prove()
    }
}

// /****************************************
//  * Main Circuit
//
// we will have a “base” proof π0 which attests to the prover knowing some input (x0,w0) such that R(x0,w0)=1. The proof πn for any n>0 will then prove that the prover knows (xn,wn) such that R(xn,wn)=1 and that a proof πn−1 was produced attesting to the knowledge of (xn−1,wn−1)
//  ****************************************/























// #[derive(Clone, Debug)]
// pub struct RecursiveCircuit<Ci, C, Tick, Tock>
// where
//     Ci: RecursiveConstraintSynthesizer<C>,
//     C: CurvePair,
//     Tick: PVar<C::Tick> + Clone,
//     Tock: PVar<C::Tock> + Clone,
//     <C::Tick as PE>::G1Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G2Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G1Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
//     <C::Tick as PE>::G2Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// {
//     // ///
//     // pub inputs: Vec<Option<<C::Tick as PE>::Fr>>,
//     // ///
//     // witnesses: Vec<<C::Tick as PE>::Fr>,

//     // inner
//     inner_params: Parameters<C::Tick>,
//     inner_proof: Proof<C::Tick>,
//     // outer
//     outer_params: Parameters<C::Tock>,
//     outer_proof: Proof<C::Tock>,
//     // // main
//     // main_params: Option<Parameters<C::Tick>>,

//     _circuit: PhantomData<Ci>,
//     _curve_pair: PhantomData<C>,
//     _tick: PhantomData<Tick>,
//     _tock: PhantomData<Tock>,
// }

// impl<Ci, C, Tick, Tock> RecursiveCircuit<Ci, C, Tick, Tock>
// where
//     Ci: RecursiveConstraintSynthesizer<C>,
//     C: CurvePair,
//     Tick: PVar<C::Tick> + Clone,
//     Tock: PVar<C::Tock> + Clone,
//     <C::Tick as PE>::G1Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G2Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G1Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
//     <C::Tick as PE>::G2Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// {
//     pub fn new(
//         // inputs: Vec<Option<<C::Tick as PE>::Fr>>,
//         // witnesses: Vec<<C::Tick as PE>::Fr>,
//         inner_params: Parameters<C::Tick>,
//         inner_proof: Proof<C::Tick>,
//         outer_params: Parameters<C::Tock>,
//         outer_proof: Proof<C::Tock>,
//         main_params: Option<Parameters<C::Tick>>,
//     ) -> Self {
//         Self {
//             // inputs,
//             // witnesses,
//             inner_params,
//             inner_proof,
//             outer_params,
//             outer_proof,
//             main_params,
//             _circuit: PhantomData,
//             _curve_pair: PhantomData,
//             _tick: PhantomData,
//             _tock: PhantomData,
//         }
//     }

    //
    // if base:
    //  - in: root inputs (MNT4)
    //  - out: proof (MNT4)
    // if odd:
    //  - in: current inputs (MNT6), prev inputs + proof (MNT4)
    //  - circuit makes new proof (MNT6)
    //  - middle
    //  - outer verifies prev inputs + proof
    //  - out: returns new proof (Tock)
    // if even:
    //  - in: inputs (MNT4)
    //  - circuit makes inner proof (MNT4)
    //  - outer circuit makes outer proof (MNT6)
    //  - out: proof (MNT6)
//     fn prove<R: Rng>(
//         inputs: Vec<<C::Tick as PE>::Fr>,
//         // witnesses: Vec<<C::Tick as PE>::Fr>,
//         mut rng: R,
//     ) -> Result<Self, SynthesisError> {
//         // // create empty inputs
//         // let inner_inputs: Vec<Option<<C::Tick as PE>::Fr>> = vec![None; Ci::NUM_INPUTS];

//         // generate inner recursive circuit params
//         let inner_circuit = Ci::new(inputs.clone());
//         let inner_params = generate_random_parameters::<C::Tick, _, _>(inner_circuit.clone(), &mut rng)?;
//         let inner_proof = create_random_proof(inner_circuit, &inner_params, &mut rng)?;

//         // generate outer recursive circuit params
//         let outer_circuit = OuterCircuit::<Ci, C, Tick>::new(
//             &inputs,
//             inner_params.clone(),
//             inner_proof.clone(),
//         );
//         let outer_params = generate_random_parameters::<C::Tock, _, _>(outer_circuit, &mut rng)?;
//         let outer_proof = create_random_proof(outer_circuit, &outer_params, &mut rng)?;

//         // generate final main circuit params
//         let mut main_circuit = Self::new(
//             // inputs,
//             // witnesses,
//             inner_params,
//             inner_proof,
//             outer_params,
//             outer_proof,
//             None,
//         // );
//         // let main_params =
//         //     generate_random_parameters::<C::Tick, _, _>(main_circuit.clone(), &mut rng)?;
//         // main_circuit.main_params = Some(main_params);

//         Ok(main_circuit)
//     }


// }

// // impl<Ci, C, Tick, Tock> ConstraintSynthesizer<<C::Tick as PE>::Fr>
// //     for RecursiveCircuit<Ci, C, Tick, Tock>
// // where
// //     Ci: RecursiveConstraintSynthesizer<C>,
// //     C: CurvePair,
// //     Tick: PVar<C::Tick> + Clone,
// //     Tock: PVar<C::Tock> + Clone,
// //     <C::Tick as PE>::G1Projective: MulAssign<<C::Tock as PE>::Fq>,
// //     <C::Tick as PE>::G2Projective: MulAssign<<C::Tock as PE>::Fq>,
// //     <C::Tick as PE>::G1Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// //     <C::Tick as PE>::G2Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// // {
// //     // TODO: generate_constraints() for inner circuit, then allow and verify previous proof + pvk
// //     fn generate_constraints(
// //         self,
// //         cs: ConstraintSystemRef<<C::Tick as PE>::Fr>,
// //     ) -> Result<(), SynthesisError> {
// //         // let inputs = self.inputs;
// //         // let outer_params = self.outer_params;
// //         // let outer_proof = self.outer_proof;
// //         let mut input_gadgets = Vec::new();

// //         /*
// //         {
// //             let bigint_size = <<C::Tick as PE>::Fr as PrimeField>::BigInt::NUM_LIMBS * 64;
// //             let mut input_bits = Vec::new();
// //             let ns = r1cs_core::ns!(cs, "Allocate Input");
// //             let cs = ns.cs();

// //             for input in inputs.into_iter() {
// //                 let input_gadget = FpVar::new_input(r1cs_core::ns!(cs, "Input"), || Ok(input))?;
// //                 let mut fp_bits = input_gadget.to_bits_le()?;

// //                 // Use 320 bits per element.
// //                 for _ in fp_bits.len()..bigint_size {
// //                     fp_bits.push(Boolean::constant(false));
// //                 }
// //                 input_bits.extend_from_slice(&fp_bits);
// //             }

// //             // Pack input bits into field elements of the underlying circuit.
// //             let max_size = 8
// //                 * (<<<C::Tock as PE>::Fr as PrimeField>::Params as FpParameters>::CAPACITY / 8)
// //                     as usize;
// //             let bigint_size =
// //                 <<<C::Tock as PE>::Fr as PrimeField>::Params as FftParameters>::BigInt::NUM_LIMBS
// //                     * 64;
// //             for chunk in input_bits.chunks(max_size) {
// //                 let mut chunk = chunk.to_vec();
// //                 let len = chunk.len();
// //                 for _ in len..bigint_size {
// //                     chunk.push(Boolean::constant(false));
// //                 }
// //                 input_gadgets.push(chunk);
// //             }
// //         }
// //          */

// //         // println!("|---- Num inputs for sub-SNARK: {}", input_gadgets.len());
// //         // let num_constraints = cs.num_constraints();
// //         // println!(
// //         //     "|---- Num constraints to prepare inputs: {}",
// //         //     num_constraints
// //         // );

// //         let outer_vk_var = OuterVkVar::<C, Tock>::new_witness(r1cs_core::ns!(cs, "Vk"), || {
// //             Ok(self.outer_params.vk)
// //         })?;
// //         let outer_proof_var = OuterProofVar::<C, Tock>::new_witness(r1cs_core::ns!(cs, "Proof"), || {
// //             Ok(self.outer_proof.clone())
// //         })?;

// //         <OuterVerifierGadget<C, Tock> as NIZKVerifierGadget<
// //             OuterProofSystem<C, Ci, Tick>,
// //             <C::Tick as PE>::Fr,
// //         >>::verify(&outer_vk_var, &input_gadgets, &outer_proof_var)?
// //         .enforce_equal(&Boolean::TRUE)?;

// //         // println!(
// //         //     "|---- Num constraints for sub-SNARK verification: {}",
// //         //     cs.num_constraints() - num_constraints
// //         // );

// //         Ok(())
// //     }
// // }

// /****************************************/
// /* Outer Circuit */
// /****************************************/

// type OuterProofSystem<C, Ci, PV> = Groth16<
//     <C as CurvePair>::Tock,
//     // Ci,
//     OuterCircuit<Ci, C, PV>,
//     <<C as CurvePair>::Tock as PE>::Fr,
// >;
// type OuterVerifierGadget<C, PV> = Groth16VerifierGadget<<C as CurvePair>::Tock, PV>;

// type OuterProofVar<C, PV> = ProofVar<<C as CurvePair>::Tock, PV>;
// type OuterVkVar<C, PV> = VerifyingKeyVar<<C as CurvePair>::Tock, PV>;

// pub struct OuterCircuit<Ci, C, Tick>
// where
//     Ci: RecursiveConstraintSynthesizer<C>,
//     C: CurvePair,
//     Tick: PVar<C::Tick>,
//     <C::Tick as PE>::G1Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G2Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G1Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
//     <C::Tick as PE>::G2Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// {
//     inputs: Vec<<C::Tock as PE>::Fr>,
//     inner_params: Parameters<C::Tick>,
//     inner_proof: Proof<C::Tick>,
//     _circuit: PhantomData<Ci>,
//     _curve_pair: PhantomData<C>,
//     _tick: PhantomData<Tick>,
// }

// impl<Ci, C, Tick> OuterCircuit<Ci, C, Tick>
// where
//     Ci: RecursiveConstraintSynthesizer<C>,
//     C: CurvePair,
//     Tick: PVar<C::Tick>,
//     <C::Tick as PE>::G1Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G2Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G1Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
//     <C::Tick as PE>::G2Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// {
//     pub fn new(
//         inputs: &[<C::Tick as PE>::Fr],
//         inner_params: Parameters<C::Tick>,
//         inner_proof: Proof<C::Tick>,
//     ) -> Self {
//         Self {
//             inputs: Self::inputs(inputs),
//             inner_params,
//             inner_proof,
//             _circuit: PhantomData,
//             _curve_pair: PhantomData,
//             _tick: PhantomData,
//         }
//     }

//     pub fn inputs(inputs: &[<C::Tick as PE>::Fr]) -> Vec<<C::Tock as PE>::Fr> {
//         let input_bytes = inputs
//             .iter()
//             .flat_map(|input| {
//                 input
//                     .into_repr()
//                     .as_ref()
//                     .iter()
//                     .flat_map(|l| l.to_le_bytes().to_vec())
//                     .collect::<Vec<_>>()
//             })
//             .collect::<Vec<_>>();

//         input_bytes[..].to_field_elements().unwrap()
//     }
// }

// impl<Ci, C, Tick> ConstraintSynthesizer<<C::Tock as PE>::Fr> for OuterCircuit<Ci, C, Tick>
// where
//     Ci: RecursiveConstraintSynthesizer<C>,
//     C: CurvePair,
//     Tick: PVar<C::Tick>,
//     <C::Tick as PE>::G1Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G2Projective: MulAssign<<C::Tock as PE>::Fq>,
//     <C::Tick as PE>::G1Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
//     <C::Tick as PE>::G2Affine: ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// {
//     fn generate_constraints(
//         self,
//         cs: ConstraintSystemRef<<C::Tock as PE>::Fr>,
//     ) -> Result<(), SynthesisError> {
//         let inputs = self.inputs;
//         let inner_params = self.inner_params;
//         let inner_proof = self.inner_proof;
//         let mut input_gadgets = Vec::new();

//         {
//             let ns = r1cs_core::ns!(cs, "Allocate Input");
//             let cs = ns.cs();

//             /*
//             // Chain all input values in one large byte array.
//             let input_bytes = inputs
//                 .into_iter()
//                 .flat_map(|input| {
//                     input
//                         .into_repr()
//                         .as_ref()
//                         .iter()
//                         .flat_map(|l| l.to_le_bytes().to_vec())
//                         .collect::<Vec<_>>()
//                 })
//                 .collect::<Vec<_>>();

//             // Allocate this byte array as input packed into field elements.
//             let input_bytes = UInt8::new_input_vec(r1cs_core::ns!(cs, "Input"), &input_bytes[..])?;
//             // 40 byte
//             let element_size =
//                 <<<C::Tick as PairingEngine>::Fr as PrimeField>::Params as FftParameters>::BigInt::NUM_LIMBS * 8;
//             input_gadgets = input_bytes
//                 .chunks(element_size)
//                 .map(|chunk| {
//                     chunk
//                         .iter()
//                         .flat_map(|byte| byte.to_bits_le().unwrap())
//                         .collect::<Vec<_>>()
//                 })
//                 .collect::<Vec<_>>();
//              */
//         }

//         // println!("|---- Num inputs for sub-SNARK: {}", input_gadgets.len());
//         // let num_constraints = cs.num_constraints();
//         // println!(
//         //     "|---- Num constraints to prepare inputs: {}",
//         //     num_constraints
//         // );

//         let inner_vk_var =
//             InnerVkVar::<C, Tick>::new_witness(r1cs_core::ns!(cs, "Vk"), || Ok(inner_params.vk))?;
//         let inner_proof_var =
//             InnerProofVar::<C, Tick>::new_witness(r1cs_core::ns!(cs, "Proof"), || Ok(inner_proof))?;

//         <InnerVerifierGadget<C, Tick> as NIZKVerifierGadget<
//             InnerProofSystem<Ci, C>,
//             <C::Tock as PE>::Fr,
//         >>::verify(&inner_vk_var, input_gadgets.iter(), &inner_proof_var)?
//         .enforce_equal(&Boolean::TRUE)?;

//         // println!(
//         //     "|---- Num constraints for sub-SNARK verification: {}",
//         //     cs.num_constraints() - num_constraints
//         // );

//         Ok(())
//     }
// }

// /****************************************/
// /* Inner Circuit */
// /****************************************/

// type InnerProofSystem<Ci, C> = Groth16<
//     <C as CurvePair>::Tick,
//     Ci,
//     // InnerCircuit<<<C as CurvePair>::Tick as PE>::Fr, Ci>,
//     <<C as CurvePair>::Tick as PE>::Fr,
// >;
// type InnerVerifierGadget<C, PV> = Groth16VerifierGadget<<C as CurvePair>::Tick, PV>;

// type InnerProofVar<C, PV> = ProofVar<<C as CurvePair>::Tick, PV>;
// type InnerVkVar<C, PV> = VerifyingKeyVar<<C as CurvePair>::Tick, PV>;

// // struct InnerCircuit<F: Field, Ci: ConstraintSynthesizer<F>> {
// //     circuit: Ci,
// //     inputs: Vec<F>,
// // }

// // impl<F: Field, Ci: ConstraintSynthesizer<F>> InnerCircuit<F, Ci> {
// //     pub fn new(circuit: Ci, inputs: Vec<F>) -> Self {
// //         Self { circuit, inputs }
// //     }
// // }

// // impl<F: Field, Ci: ConstraintSynthesizer<F>> ConstraintSynthesizer<F> for InnerCircuit<F, Ci> {
// //     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
// //         self.circuit.generate_constraints(cs)
// //     }
// // }

// // fn run<C: CurvePair, TickPairing: PVar<C::Tick>, TockPairing: PVar<C::Tock>>(
// //     num_constraints: usize,
// //     output_file_path: PathBuf,
// // ) -> Result<(), Box<dyn Error>>
// // where
// //     <C::Tick as PE>::G1Projective: MulAssign<<C::Tock as PE>::Fq>,
// //     <C::Tick as PE>::G2Projective: MulAssign<<C::Tock as PE>::Fq>,
// //     <C::Tick as PE>::G1Affine:
// //         ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// //     <C::Tick as PE>::G2Affine:
// //         ToConstraintField<<<C::Tock as PE>::Fr as Field>::BasePrimeField>,
// // {
// //     let mut wtr = if !output_file_path.exists() {
// //         println!("Creating output file");
// //         let f = OpenOptions::new()
// //             .create(true)
// //             .append(true)
// //             .open(output_file_path)?;
// //         let mut wtr = csv::Writer::from_writer(f);
// //         wtr.write_record(&[
// //             "num_constraints",
// //             "setup_inner",
// //             "prover_inner",
// //             "setup_Outer",
// //             "prover_Outer",
// //             "setup_outer",
// //             "prover_outer",
// //             "verifier_outer",
// //         ])?;
// //         wtr
// //     } else if output_file_path.is_file() {
// //         let f = OpenOptions::new().append(true).open(output_file_path)?;
// //         csv::Writer::from_writer(f)
// //     } else {
// //         println!("Path to output file does not point to a file.");
// //         process::exit(1);
// //     };
// //     // This may not be cryptographically safe, use
// //     // `OsRng` (for example) in production software.
// //     let rng = &mut test_rng();

// //     // Let's benchmark stuff!
// //     let samples = 1;
// //     let mut total_setup_inner = Duration::new(0, 0);
// //     let mut total_proving_inner = Duration::new(0, 0);
// //     let mut total_setup_Outer = Duration::new(0, 0);
// //     let mut total_proving_Outer = Duration::new(0, 0);
// //     let mut total_setup_outer = Duration::new(0, 0);
// //     let mut total_proving_outer = Duration::new(0, 0);
// //     let mut total_verifying_outer = Duration::new(0, 0);

// //     // Just a place to put the proof data, so we can
// //     // benchmark deserialization.
// //     // let mut proof_vec = vec![];

// //     for sample in 0..samples {
// //         println!("Running sample {}/{}", sample + 1, samples);
// //         let mut inputs: Vec<<C::Tick as PE>::Fr> =
// //             Vec::with_capacity(num_constraints);
// //         for _ in 0..num_constraints {
// //             inputs.push(<<C::Tick as PE>::Fr as UniformRand>::rand(
// //                 rng,
// //             ));
// //         }

// //         // Create parameters for our inner circuit
// //         println!("|-- Generating inner parameters ({})", C::TICK_CURVE);
// //         let start = Instant::now();
// //         let params_inner = {
// //             let c = InnerCircuit::<<C::Tick as PE>::Fr>::new(
// //                 num_constraints,
// //                 inputs.clone(),
// //             );
// //             generate_random_parameters(c, rng)?
// //         };
// //         total_setup_inner += start.elapsed();

// //         // proof_vec.truncate(0);
// //         println!("|-- Generating inner proof ({})", C::TICK_CURVE);
// //         let start = Instant::now();
// //         let proof_inner = {
// //             // Create an instance of our inner circuit (with the witness)
// //             let c = InnerCircuit::new(num_constraints, inputs.clone());
// //             // Create a proof with our parameters.
// //             create_random_proof(c, &params_inner, rng)?
// //         };
// //         total_proving_inner += start.elapsed();

// //         // Verify inner proof.
// //         let pvk = prepare_verifying_key(&params_inner.vk);
// //         assert!(verify_proof(&pvk, &proof_inner, &inputs).unwrap());

// //         // Create parameters for our Outer circuit
// //         println!("|-- Generating Outer parameters ({})", C::TOCK_CURVE);
// //         let start = Instant::now();
// //         let params_Outer = {
// //             let c = OuterCircuit::<C, TickPairing>::new(
// //                 inputs.clone(),
// //                 params_inner.clone(),
// //                 proof_inner.clone(),
// //             );
// //             generate_random_parameters(c, rng)?
// //         };
// //         total_setup_Outer += start.elapsed();

// //         // proof_vec.truncate(0);
// //         println!("|-- Generating Outer proof ({})", C::TOCK_CURVE);
// //         let start = Instant::now();
// //         let proof_Outer = {
// //             // Create an instance of our Outer circuit (with the witness)
// //             let c = OuterCircuit::<C, TickPairing>::new(
// //                 inputs.clone(),
// //                 params_inner.clone(),
// //                 proof_inner.clone(),
// //             );
// //             // Create a proof with our parameters.
// //             create_random_proof(c, &params_Outer, rng)?
// //         };
// //         total_proving_Outer += start.elapsed();

// //         {
// //             let pvk = prepare_verifying_key(&params_Outer.vk);
// //             assert!(verify_proof(
// //                 &pvk,
// //                 &proof_Outer,
// //                 &OuterCircuit::<C, TickPairing>::inputs(&inputs)
// //             )
// //             .unwrap());
// //         }

// //         // Create parameters for our outer circuit
// //         println!("|-- Generating outer parameters ({})", C::TICK_CURVE);
// //         let start = Instant::now();
// //         let params_outer = {
// //             let c = RecursiveCircuit::<C, TockPairing, TickPairing>::new(
// //                 inputs.clone(),
// //                 params_Outer.clone(),
// //                 proof_Outer.clone(),
// //             );
// //             generate_random_parameters::<C::Tick, _, _>(c, rng)?
// //         };

// //         // Prepare the verification key (for proof verification)
// //         let pvk = prepare_verifying_key(&params_outer.vk);
// //         total_setup_outer += start.elapsed();

// //         // proof_vec.truncate(0);
// //         println!("|-- Generating outer proof ({})", C::TICK_CURVE);
// //         let start = Instant::now();
// //         let proof_outer = {
// //             // Create an instance of our outer circuit (with the witness)
// //             let c = RecursiveCircuit::<C, TockPairing, TickPairing>::new(
// //                 inputs.clone(),
// //                 params_Outer.clone(),
// //                 proof_Outer.clone(),
// //             );
// //             // Create a proof with our parameters.
// //             create_random_proof(c, &params_outer, rng)?
// //         };
// //         total_proving_outer += start.elapsed();

// //         println!("|-- Verify outer proof ({})", C::TICK_CURVE);
// //         let start = Instant::now();
// //         // let proof = Proof::read(&proof_vec[..]).unwrap();
// //         // Check the proof
// //         let r = verify_proof(&pvk, &proof_outer, &inputs).unwrap();
// //         assert!(r);
// //         total_verifying_outer += start.elapsed();
// //     }

// //     let setup_inner_avg = total_setup_inner / samples;
// //     let setup_inner_avg = setup_inner_avg.subsec_nanos() as f64 / 1_000_000_000f64
// //         + (setup_inner_avg.as_secs() as f64);

// //     let proving_inner_avg = total_proving_inner / samples;
// //     let proving_inner_avg = proving_inner_avg.subsec_nanos() as f64 / 1_000_000_000f64
// //         + (proving_inner_avg.as_secs() as f64);

// //     let setup_Outer_avg = total_setup_Outer / samples;
// //     let setup_Outer_avg = setup_Outer_avg.subsec_nanos() as f64 / 1_000_000_000f64
// //         + (setup_Outer_avg.as_secs() as f64);

// //     let proving_Outer_avg = total_proving_Outer / samples;
// //     let proving_Outer_avg = proving_Outer_avg.subsec_nanos() as f64 / 1_000_000_000f64
// //         + (proving_Outer_avg.as_secs() as f64);

// //     let setup_outer_avg = total_setup_outer / samples;
// //     let setup_outer_avg = setup_outer_avg.subsec_nanos() as f64 / 1_000_000_000f64
// //         + (setup_outer_avg.as_secs() as f64);

// //     let proving_outer_avg = total_proving_outer / samples;
// //     let proving_outer_avg = proving_outer_avg.subsec_nanos() as f64 / 1_000_000_000f64
// //         + (proving_outer_avg.as_secs() as f64);

// //     let verifying_outer_avg = total_verifying_outer / samples;
// //     let verifying_outer_avg = verifying_outer_avg.subsec_nanos() as f64 / 1_000_000_000f64
// //         + (verifying_outer_avg.as_secs() as f64);

// //     println!(
// //         "=== Benchmarking recursive Groth16 with {} constraints on inner circuit: ====",
// //         num_constraints
// //     );
// //     println!(
// //         "Average setup time (inner circuit): {:?} seconds",
// //         setup_inner_avg
// //     );
// //     println!(
// //         "Average proving time (inner circuit): {:?} seconds",
// //         proving_inner_avg
// //     );
// //     println!(
// //         "Average setup time (Outer circuit): {:?} seconds",
// //         setup_Outer_avg
// //     );
// //     println!(
// //         "Average proving time (Outer circuit): {:?} seconds",
// //         proving_Outer_avg
// //     );
// //     println!(
// //         "Average setup time (outer circuit): {:?} seconds",
// //         setup_outer_avg
// //     );
// //     println!(
// //         "Average proving time (outer circuit): {:?} seconds",
// //         proving_outer_avg
// //     );
// //     println!(
// //         "Average verifying time (outer circuit): {:?} seconds",
// //         verifying_outer_avg
// //     );

// //     wtr.write_record(&[
// //         format!("{}", num_constraints),
// //         format!("{}", setup_inner_avg),
// //         format!("{}", proving_inner_avg),
// //         format!("{}", setup_Outer_avg),
// //         format!("{}", proving_Outer_avg),
// //         format!("{}", setup_outer_avg),
// //         format!("{}", proving_outer_avg),
// //         format!("{}", verifying_outer_avg),
// //     ])?;
// //     wtr.flush()?;
// //     Ok(())
// // }
