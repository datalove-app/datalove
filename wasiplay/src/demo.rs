#[macro_use]
extern crate lazy_static;

mod host;
mod wasi;

use host::Host;
use std::{env, fs::File};

/** TODO:
 *  - get this to run (capabilities? arguments?)
 *  - start setting up a wasm context that:
 *      - uses wasi fns with a custom, backwards-compatible protocol
 *      - ? has state machine for recursing and getting blocks?
 */

lazy_static! {
    static ref APP_NAME: String =
        env::var("APP_NAME").expect("Failed to load wasm file env var APP_NAME");
    static ref APP_LOCATION: String =
        env::var("APP_LOCATION").expect("Failed to load wasm file env var APP_LOCATION");
}

fn main() {
    let main_dir_path = String::from(".");
    let main_dir = File::open(&main_dir_path)
        .unwrap_or_else(|_| panic!("Failed to preopen dir: {}", &main_dir_path));
    let preopened_dirs = [(main_dir_path, main_dir)];
    let argv = [
        APP_NAME.clone(),
        String::from("wasiplay/test.txt"),
        String::from("wasiplay/test_output.txt"),
    ];

    let _host = Host::new(
        APP_NAME.as_str(),
        APP_LOCATION.as_str(),
        &preopened_dirs,
        &argv,
    );

    println!("loaded wasm");

    // let start_fn = "_start";
    // host.call(start_fn, &[])
    //     .expect(&format!("Function `{}` failed to execute", start_fn));

    // let answer = instance
    //     .find_export_by_name("answer")
    //     .expect("answer")
    //     .func()
    //     .expect("function");
    // let result = answer.borrow().call(&[]).expect("success");
    // println!("Answer: {}", result[0].i32());
}
