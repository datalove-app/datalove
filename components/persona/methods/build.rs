use std::env;

fn main() {
    // tracing_subscriber::fmt()
    //     .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
    //     .init();
    // ();

    if env::var("CARGO_CFG_TARGET_OS").unwrap().contains("zkvm") {
        // Guest shouldn't recursively depend on itself.
        return;
    }

    risc0_build::embed_methods();
}
