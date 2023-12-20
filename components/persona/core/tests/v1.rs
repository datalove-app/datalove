use borsh::{BorshDeserialize, BorshSerialize};
use datalove_persona_core::{
    util::Sha256Digest,
    zksm::{Operation, ProverState, StateMachine, VerifierState},
    Error, Group, Member, MemberSignature, SignedOperation,
};
use datalove_persona_risc0::DATALOVE_PERSONA_RISC0_GUEST_V1_ELF as V1_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use std::io;

fn random_member() -> Member {
    todo!()
}

#[test]
fn can_prove() -> io::Result<()> {
    // let op = Operation::Init()

    // let env = ExecutorEnv::builder()
    //     .write_slice(&[20])
    //     .build()
    //     .expect("cannot build env");
    // let prover = default_prover();
    // let receipt = prover.prove(env, V1_ELF).expect("cannot prove");

    Ok(())
}
