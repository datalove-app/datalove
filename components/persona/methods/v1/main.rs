#![no_main]
#![no_std]

risc0_zkvm::guest::entry!(main);

pub fn main() {
    use risc0_zkvm::guest::env;

    datalove_persona_core::guest::exec(env::stdin(), env::stdout(), env::journal())
        .expect("failed to exec persona guest state machine");
}
