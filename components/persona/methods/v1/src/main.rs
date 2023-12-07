#![no_main]
#![no_std]

risc0_zkvm::guest::entry!(main);

pub fn main() {
    use risc0_zkvm::guest::env;

    datalove_persona_core::exec(env::stdin(), env::stdout(), env::journal()).unwrap();
}
