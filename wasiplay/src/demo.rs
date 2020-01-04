#[macro_use]
extern crate lazy_static;

mod runtime;
mod wasi;

use runtime::Host;
use std::env;

/** TODO:
 *  - get this to run (capabilities? arguments?)
 *  - start setting up a wasm context that:
 *      - uses wasi fns with a custom, backwards-compatible protocol
 *      - ? has state machine for recursing and getting blocks?
 */

lazy_static! {
    static ref WASM_LOCATION: String = {
        let path = env::var("WASM_FILE").expect("Failed to load wasm file env var WASM_FILE");

        String::from(path)
    };
}

fn main() {
    let host = Host::new(WASM_LOCATION.as_str());

    println!("loaded wasm");

    // host.call("answer")

    // let answer = instance
    //     .find_export_by_name("answer")
    //     .expect("answer")
    //     .func()
    //     .expect("function");
    // let result = answer.borrow().call(&[]).expect("success");
    // println!("Answer: {}", result[0].i32());
}
