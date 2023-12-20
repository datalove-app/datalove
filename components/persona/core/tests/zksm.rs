use anyhow::Result;
use borsh::{from_slice, to_vec};
use datalove_persona_core::{
    util::{ImageId, Sha256Digest},
    zksm::tests::{Mod7SM, Op, PState, VState},
    Error,
};
use datalove_persona_risc0::{ZKSM_ELF, ZKSM_ID};
use risc0_zkvm::{
    default_prover, sha::Digestible, ExecutorEnv, ProverOpts, Receipt, ReceiptClaim,
    VerifierContext,
};
use std::{io, path, time::Instant};

// fn random_member() -> Member {
//     todo!()
// }

fn prove_transition(op: Op, prev: Option<(Mod7SM, Receipt)>) -> Result<(Mod7SM, Receipt)> {
    println!("\n\nprove_transition: {:?}", &op);

    // init proving env
    let mut stdout = Vec::new();
    let mut env_builder = ExecutorEnv::builder();

    // serialize transition and prev receipt as an assumption, and sm if provided
    let (transition, sm) = match prev {
        None => {
            let sm = Mod7SM::new(ZKSM_ID.into());
            let transition = sm.new_transition(op);
            (transition, None)
        }
        Some((sm, receipt)) => {
            let transition = sm.new_transition(op);
            env_builder.add_assumption(receipt.into());
            (transition, Some(sm))
        }
    };

    // prove
    let receipt = {
        let env = env_builder
            .stdout(&mut stdout)
            .enable_profiler(path::Path::new("tests/profile.zksm.txt"))
            .env_var(
                "SELF_IMAGE_ID",
                &hex::encode(bytemuck::cast_slice(&ZKSM_ID)),
            )
            .write_slice(&to_vec(&transition)?)
            .write_slice(&to_vec(&sm)?)
            .build()?;

        let now = Instant::now();
        let prover = default_prover();
        let receipt = prover.prove(env, ZKSM_ELF)?;
        println!("exec prove: {:?}", now.elapsed());

        drop(env_builder);
        receipt
    };

    // verify
    {
        // receipt.verify(VerifierContext::default())?;
    }

    let sm = Mod7SM::load(io::Read::chain(
        stdout.as_slice(),
        receipt.journal.bytes.as_slice(),
    ))?;

    Ok((sm, receipt))
}

fn test_prove_transition(
    op: Op,
    prev: Option<(Mod7SM, Receipt)>,
    pstate: u32,
    vstate: u32,
) -> Result<(Mod7SM, Receipt)> {
    let (sm, receipt) = prove_transition(op, prev)?;
    assert_eq!(sm.prover_state_ref().0, pstate);
    assert_eq!(sm.verifier_state_ref().0, vstate);
    assert_eq!(sm.verifier_commitment(), &sm.prover_digest());

    let now = Instant::now();
    let claim = receipt.get_claim()?;
    println!(
        "claim ({:?})\n\tpre {:?}\n\tinput {:?}\n\tassumptions {:#?}",
        now.elapsed(),
        &claim.pre.digest(),
        &claim.input,
        &claim.output
    );

    Ok((sm, receipt))
}

#[test]
fn can_prove_sm_chain() -> Result<()> {
    let op = Op::Init(9);
    let prev = test_prove_transition(op, None, 9, 2)?;

    let op = Op::Inc(6);
    let prev = test_prove_transition(op, Some(prev), 15, 1)?;

    let op = Op::Inc(6);
    let prev = test_prove_transition(op, Some(prev), 21, 0)?;

    let op = Op::Inc(9);
    let prev = test_prove_transition(op, Some(prev), 30, 2)?;

    Ok(())
}
