use super::wasi as datalovewasi;
// use anyhow::{bail, Result};
use datalovewasi::PreopenedDirs;
use std::{collections::HashMap, fs::read};
// use wasm_webidl_bindings::ast;
use wasmtime::{Engine, HostRef, Instance, Store, Val};
// use wasmtime_interface_types::{ModuleData, Value};

pub struct Host {
    // engine: HostRef<Engine>,
    // store: HostRef<Store>,
    // registry: HashMap<String, HostRef<Instance>>,
    app: HostRef<Instance>,
    // app_data: ModuleData,
}

impl Host {
    pub fn new(
        app_name: &str,
        app_location: &str,
        preopened_dirs: &PreopenedDirs,
        argv: &[String],
    ) -> Self {
        let engine = HostRef::new(Engine::default());
        let store = HostRef::new(Store::new(&engine));
        let mut registry = HashMap::new();

        // the wasi instance
        let wasi = datalovewasi::create_wasi_instance(&store, preopened_dirs, argv, &[])
            .expect("Failed to initialize wasi instance");
        registry.insert("wasi_snapshot_preview1".to_owned(), wasi);

        // the main wasm instance
        let app_bin = read(app_location)
            .unwrap_or_else(|_| panic!("Failed to load wasm file: {}", app_location));
        let (app, _app_module) =
            datalovewasi::create_app_instance(&store, &registry, app_name, &app_bin);
        // let app_data = ModuleData::new(&app_bin).unwrap_or_else(|_| panic!(
        //     "Failed to load wasm module data for module: {}",
        //     app_name
        // ));

        Host {
            // engine,
            // store,
            // registry,
            app,
            // app_data,
        }
    }

    // ///
    // /// TODO: be generic over args that can be ref'ed into instance memory
    // ///     then get the i32 addresses
    // pub fn call(&self, fn_name: &str, args: &[Val]) -> Result<(), String> {
    //     let instance = self.app.borrow();
    //     let func = instance
    //         .find_export_by_name(fn_name)
    //         .ok_or(format!("Failed to find func: {}", fn_name))?
    //         .func()
    //         .ok_or(format!("Failed to load func: {}", fn_name))?;
    //     let result = func.borrow().call(args).expect("success");
    //     println!("Answer: {}", result[0].i32());
    //     Ok(())
    // }
}

// fn invoke_export(
//     instance: HostRef<Instance>,
//     data: &ModuleData,
//     fn_name: &str,
//     // args: &Args,
// ) -> Result<(), ()> {
//     let mut handle = instance.borrow().handle().clone();

//     // Use the binding information in `ModuleData` to figure out what arguments
//     // need to be passed to the function that we're invoking. Currently we take
//     // the CLI parameters and attempt to parse them into function arguments for
//     // the function we'll invoke.
//     let binding = data.binding_for_export(&mut handle, fn_name).or(Err(()))?;
//     if binding.param_types().or(Err(()))?.len() > 0 {
//         eprintln!(
//             "warning: using `--invoke` with a function that takes arguments \
//              is experimental and may break in the future"
//         );
//     }
//     let mut values = Vec::new();
//     let args: Vec<String> = Vec::new();
//     let mut args_iter = args.iter();
//     for ty in binding.param_types().or(Err(()))? {
//         let val = match args_iter.next() {
//             Some(s) => s,
//             None => return Err(()),
//         };
//         values.push(match ty {
//             // TODO: integer parsing here should handle hexadecimal notation
//             // like `0x0...`, but the Rust standard library currently only
//             // parses base-10 representations.
//             ast::WebidlScalarType::Long => Value::I32(val.parse().or(Err(()))?),
//             ast::WebidlScalarType::LongLong => Value::I64(val.parse().or(Err(()))?),
//             ast::WebidlScalarType::UnsignedLong => Value::U32(val.parse().or(Err(()))?),
//             ast::WebidlScalarType::UnsignedLongLong => Value::U64(val.parse().or(Err(()))?),

//             ast::WebidlScalarType::Float | ast::WebidlScalarType::UnrestrictedFloat => {
//                 Value::F32(val.parse().or(Err(()))?)
//             }
//             ast::WebidlScalarType::Double | ast::WebidlScalarType::UnrestrictedDouble => {
//                 Value::F64(val.parse().or(Err(()))?)
//             }
//             ast::WebidlScalarType::DomString => Value::String(val.to_string()),
//             t => return Err(()),
//         });
//     }

//     // Invoke the function and then afterwards print all the results that came
//     // out, if there are any.
//     let results = data
//         .invoke_export(&instance, fn_name, &values)
//         .unwrap_or_else(|_| panic!("Failed to invoke fn {} on module", fn_name));
//     if results.len() > 0 {
//         eprintln!(
//             "warning: using `--invoke` with a function that returns values \
//              is experimental and may break in the future"
//         );
//     }
//     for result in results {
//         println!("{}", result);
//     }

//     Ok(())
// }

