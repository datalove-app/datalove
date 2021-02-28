use ark_bn254::{Bn254, Parameters};
use ark_ec::bn::{G1Affine, G2Affine};
use ark_ff::{BigInteger, BigInteger256};
use ark_groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key};
use ark_std::{rand::Rng, test_rng};
use datalove_circuits::mimc::{MiMCDemo, MIMC_ROUNDS};
use std::fmt::Write;

fn main() {
    let rng = &mut test_rng();
    let constants = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect::<Vec<_>>();

    // Create parameters for our circuit
    let c = MiMCDemo::from(constants.as_slice());
    let params = generate_random_parameters::<Bn254, _, _>(c, rng).unwrap();

    // Prepare the verification key (for proof verification)
    println!("generating verification key");
    let pvk = prepare_verifying_key(&params.vk);
    for b in &g1_be_bytes(&params.vk.alpha_g1) {
        println!("alpha {}", hex(&b));
    }
    for b in &g2_be_bytes(&params.vk.beta_g2) {
        println!("beta {}", hex(&b));
    }
    for b in &g2_be_bytes(&params.vk.gamma_g2) {
        println!("gamma {}", hex(&b));
    }
    for b in &g2_be_bytes(&params.vk.delta_g2) {
        println!("delta {}", hex(&b));
    }
    for b in params
        .vk
        .gamma_abc_g1
        .iter()
        .flat_map(|g1| g1_be_bytes(g1).to_vec().into_iter())
    {
        println!("gammaABC {}", hex(&b));
    }

    // Run once with some witnesses
    let c = MiMCDemo::new(42u128, 42u128, constants.as_slice());

    // Prepare proof for submission to contract
    println!("generating proof");
    let proof = create_random_proof(c, &params, rng).unwrap();
    for b in &g1_be_bytes(&proof.a) {
        println!("proof.A {}", hex(&b));
    }
    for b in &g2_be_bytes(&proof.b) {
        println!("proof.B {}", hex(&b));
    }
    for b in &g1_be_bytes(&proof.c) {
        println!("proof.C {}", hex(&b));
    }
}

fn hex(byte_vec: &[u8]) -> String {
    // let mut chunks = byte_vec.as_slice().chunks(64);

    let mut s = String::with_capacity(2 * byte_vec.len());
    for b in byte_vec.iter() {
        write!(s, "{:02x}", b);
    }
    s
}

fn g1_be_bytes(g1: &G1Affine<Parameters>) -> [Vec<u8>; 2] {
    let x: BigInteger256 = g1.x.into();
    let y: BigInteger256 = g1.y.into();
    [x.to_bytes_be(), y.to_bytes_be()]
}

fn g2_be_bytes(g2: &G2Affine<Parameters>) -> [Vec<u8>; 4] {
    let x1: BigInteger256 = g2.x.c1.into();
    let x0: BigInteger256 = g2.x.c0.into();
    let y1: BigInteger256 = g2.y.c1.into();
    let y0: BigInteger256 = g2.y.c0.into();
    [
        x1.to_bytes_be(),
        x0.to_bytes_be(),
        y1.to_bytes_be(),
        y0.to_bytes_be(),
    ]
}
