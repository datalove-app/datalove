//! datalove-persona-risc0-guest
//! ============================
//!
//! self-soveriegn identity system

#![no_main]
// #![no_std]

use datalove_persona_core::State;
use risc0_zkvm::guest::env;

// input: state,

risc0_zkvm::guest::entry!(main);
pub fn main() {
    let mut reader = env::stdin();

    // let state = State::deserialize_reader(&mut reader).unwrap();
    // let op =
    // op.apply(&mut state).unwrap();

    env::commit(&());
}
