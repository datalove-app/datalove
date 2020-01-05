use super::syscalls;
use cranelift_codegen::{
    ir::{self, types},
    isa,
};
use cranelift_entity::PrimaryMap;
use cranelift_wasm::DefinedFuncIndex;
use std::{cell::RefCell, collections::HashMap, fs::File, rc::Rc};
use target_lexicon::HOST;
use wasi_common::{WasiCtx, WasiCtxBuilder};
use wasmtime::{HostRef, Instance, Module as WasmModule, Store};
use wasmtime_environ::{translate_signature, Export, Module};
use wasmtime_runtime::{Imports, InstanceHandle, InstantiationError, LinkError, VMFunctionBody};

pub type Argv = [String];
pub type EnvVars = [(String, String)];
pub type VMFuncMap = PrimaryMap<DefinedFuncIndex, *const VMFunctionBody>;
pub type PreopenedDir = (String, File);
pub type PreopenedDirs = [PreopenedDir];

/// Creates `wasmtime::Instance` object implementing the "wasi" interface.
pub fn create_wasi_instance(
    store: &HostRef<Store>,
    preopened_dirs: &PreopenedDirs,
    argv: &Argv,
    environ: &EnvVars,
) -> Result<HostRef<Instance>, InstantiationError> {
    let global_exports = store.borrow().global_exports().clone();

    let wasi_ctx = create_wasi_context(preopened_dirs, argv, environ)?;
    let wasi = instantiate_wasi_with_context(global_exports, wasi_ctx)?;
    let instance = HostRef::new(Instance::from_handle(&store, wasi));
    Ok(instance)
}

/// Creates a `wasmtime::Instance` object from a WASM module.
pub fn create_app_instance(
    store: &HostRef<Store>,
    registry: &HashMap<String, HostRef<Instance>>,
    module_name: &str,
    module_bin: &[u8],
) -> (HostRef<Instance>, HostRef<WasmModule>) {
    let module = HostRef::new(
        WasmModule::new(store, module_bin)
            .unwrap_or_else(|_| panic!("Failed to load wasm module: {}", module_name)),
    );

    // Resolve import using registry.
    let imports = module
        .borrow()
        .imports()
        .iter()
        .map(|i| {
            let module_name = i.module().as_str();
            if let Some(instance) = registry.get(module_name) {
                let field_name = i.name().as_str();
                if let Some(export) = instance.borrow().find_export_by_name(field_name) {
                    Ok(export.clone())
                } else {
                    Err(InstantiationError::Link(LinkError(format!(
                        "Import {} was not found in module {}",
                        field_name, module_name
                    ))))
                }
            } else {
                Err(InstantiationError::Link(LinkError(format!(
                    "Import module {} was not found",
                    module_name
                ))))
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("Failed to resolve imports for module: {}", module_name));

    let instance = HostRef::new(
        Instance::new(store, &module, &imports)
            .unwrap_or_else(|_| panic!("Failed to initialize module instance: {}", module_name)),
    );

    (instance, module)
}

/// Creates `WasiCtx`.
fn create_wasi_context(
    preopened_dirs: &PreopenedDirs,
    argv: &Argv,
    environ: &EnvVars,
) -> Result<WasiCtx, InstantiationError> {
    let mut wasi_ctx_builder = WasiCtxBuilder::new()
        .inherit_stdio()
        .args(argv)
        .envs(environ);

    for (dir, f) in preopened_dirs {
        wasi_ctx_builder = wasi_ctx_builder.preopened_dir(
            f.try_clone().map_err(|err| {
                InstantiationError::Resource(format!(
                    "couldn't clone an instance handle to pre-opened dir: {}",
                    err
                ))
            })?,
            dir,
        );
    }

    wasi_ctx_builder.build().map_err(|err| {
        InstantiationError::Resource(format!("couldn't assemble WASI context object: {}", err))
    })
}

/// Return an instance implementing the "wasi" interface.
///
/// The wasi context is configured by
fn instantiate_wasi_with_context(
    global_exports: Rc<RefCell<HashMap<String, Option<wasmtime_runtime::Export>>>>,
    wasi_ctx: WasiCtx,
) -> Result<InstanceHandle, InstantiationError> {
    let pointer_type = types::Type::triple_pointer_type(&HOST);
    let mut module = Module::new();
    let mut finished_functions: VMFuncMap = PrimaryMap::new();
    let call_conv = isa::CallConv::triple_default(&HOST);

    macro_rules! signature {
        ($name:ident) => {{
            let sig = module.signatures.push(translate_signature(
                ir::Signature {
                    params: syscalls::$name::params()
                        .into_iter()
                        .map(ir::AbiParam::new)
                        .collect(),
                    returns: syscalls::$name::results()
                        .into_iter()
                        .map(ir::AbiParam::new)
                        .collect(),
                    call_conv,
                },
                pointer_type,
            ));
            let func = module.functions.push(sig);
            module
                .exports
                .insert(stringify!($name).to_owned(), Export::Function(func));
            finished_functions.push(syscalls::$name::SHIM as *const VMFunctionBody);
        }};
    }

    signature!(args_get);
    signature!(args_sizes_get);
    signature!(clock_res_get);
    signature!(clock_time_get);
    signature!(environ_get);
    signature!(environ_sizes_get);
    signature!(fd_prestat_get);
    signature!(fd_prestat_dir_name);
    signature!(fd_close);
    signature!(fd_datasync);
    signature!(fd_pread);
    signature!(fd_pwrite);
    signature!(fd_read);
    signature!(fd_renumber);
    signature!(fd_seek);
    signature!(fd_tell);
    signature!(fd_fdstat_get);
    signature!(fd_fdstat_set_flags);
    signature!(fd_fdstat_set_rights);
    signature!(fd_sync);
    signature!(fd_write);
    signature!(fd_advise);
    signature!(fd_allocate);
    signature!(path_create_directory);
    signature!(path_link);
    signature!(path_open);
    signature!(fd_readdir);
    signature!(path_readlink);
    signature!(path_rename);
    signature!(fd_filestat_get);
    signature!(fd_filestat_set_times);
    signature!(fd_filestat_set_size);
    signature!(path_filestat_get);
    signature!(path_filestat_set_times);
    signature!(path_symlink);
    signature!(path_unlink_file);
    signature!(path_remove_directory);
    signature!(poll_oneoff);
    signature!(proc_exit);
    signature!(proc_raise);
    signature!(random_get);
    signature!(sched_yield);
    signature!(sock_recv);
    signature!(sock_send);
    signature!(sock_shutdown);

    let imports = Imports::none();
    let data_initializers = Vec::new();
    let signatures = PrimaryMap::new();

    InstanceHandle::new(
        Rc::new(module),
        global_exports,
        finished_functions.into_boxed_slice(),
        // these are empty defaults
        imports,
        &data_initializers,
        signatures.into_boxed_slice(),
        None,
        Box::new(wasi_ctx),
    )
}
