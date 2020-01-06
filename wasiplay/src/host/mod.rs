use super::wasi as datalove_wasi;
// use anyhow::{bail, Result};
use datalove_wasi::{WasiCtxBuilder, __wasi_exitcode_t};
use lucet_runtime::{
    DlModule, Error, InstanceHandle, Limits, MmapRegion, Region, RunResult, TerminationDetails, Val,
};
use std::{
    collections::HashMap,
    fs::{read, File},
    path::Path,
    sync::Arc,
};

pub type InitArgv = [String];
pub type CallArgv = [Val];
pub type EnvVars = [(String, String)];
pub type PreopenedDir = (String, File);
pub type PreopenedDirs = [PreopenedDir];

pub struct Host {
    region: Arc<MmapRegion>,
    app_instance: InstanceHandle,
}

impl Host {
    pub fn new(
        app_name: &str,
        app_location: &str,
        preopened_dirs: &PreopenedDirs,
        argv: &InitArgv,
        environ: &EnvVars,
    ) -> Self {
        // lucet_runtime::lucet_internal_ensure_linked();
        datalove_wasi::export_wasi_funcs();

        let mut ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .args(argv)
            .envs(environ);

        for (guest_path, ref dir) in preopened_dirs {
            ctx = ctx.preopened_dir(
                dir.try_clone()
                    .unwrap_or_else(|_| panic!("Failed to preopen required dir: {}", guest_path)),
                guest_path,
            );
        }

        let module = DlModule::load(app_location).unwrap_or_else(|err| {
            panic!(
                "Failed to load WASM program .so `{}` from {}: {:?}",
                app_name, app_location, err
            )
        });
        let region =
            MmapRegion::create(1, &Limits::default()).expect("Failed to create memory region");
        let app_instance = region
            .new_instance_builder(module)
            // TODO: here is where we put additional contexts into the instance (one of any given type)
            // TODO: then use `get
            .with_embed_ctx(ctx.build().expect("WASI ctx could not be created"))
            .build()
            .expect("Instance could not be created");

        Host {
            region,
            app_instance,
        }
    }

    ///
    /// TODO: be generic over args that can be ref'ed into instance memory
    ///     then get the i32 addresses
    pub fn init(&mut self) -> u32 {
        match self.app_instance.run("_start", &[]) {
            // normal termination implies 0 exit code
            Ok(RunResult::Returned(_)) => 0,
            // none of the WASI hostcalls use yield yet, so this shouldn't happen
            Ok(RunResult::Yielded(_)) => panic!("lucet-wasi unexpectedly yielded"),
            Err(Error::RuntimeTerminated(TerminationDetails::Provided(any))) => *any
                .downcast_ref::<__wasi_exitcode_t>()
                .expect("termination yields an exitcode"),
            Err(Error::RuntimeTerminated(TerminationDetails::Remote)) => {
                println!("Terminated via remote kill switch (likely a timeout)");
                std::u32::MAX
            }
            Err(e) => panic!("lucet-wasi runtime error: {}", e),
        }
    }
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
