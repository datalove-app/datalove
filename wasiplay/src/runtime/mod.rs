use super::wasi as datalovewasi;
use std::fs::read;
use wasmtime::{Engine, HostRef, Instance, Module, Store};

pub struct Host {
    engine: HostRef<Engine>,
    store: HostRef<Store>,
    module: HostRef<Module>,
    instance: Instance,
}

impl Host {
    pub fn new(wasm_location: &str) -> Self {
        let engine = HostRef::new(Engine::default());
        let store = HostRef::new(Store::new(&engine));

        let wasm =
            read(wasm_location).expect(&format!("Failed to load wasm file: {}", wasm_location));

        let module =
            HostRef::new(Module::new(&store, &wasm).expect("Failed to initialize wasm module"));
        let argv = [
            String::from("apps/samplewasm/test.txt"),
            String::from("apps/samplewasm/test_output.txt"),
        ];
        let instance = datalovewasi::create_wasi_instance(&store, &[], &argv, &[])
            .expect("Failed to initialize wasi instance");

        Host {
            engine,
            store,
            module,
            instance,
        }
    }

    fn call(&self, fn_name: &str) -> () {
        self.instance
            .find_export_by_name(fn_name)
            .expect(&format!("failed to load function: {}", fn_name));
    }
}
