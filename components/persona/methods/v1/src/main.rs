#![no_main]
#![no_std]

risc0_zkvm::guest::entry!(main);
pub fn main() {
    datalove_persona_core::exec().expect("failed to exec guest state machine");
}
