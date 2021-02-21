use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

lazy_static! {
    pub static ref DEFAULT_RESERVED_SIZE: String = String::from("64MiB");
    pub static ref DEFAULT_OPT_LEVEL: usize = 0;
}

///
#[structopt(
    name = "build",
    help = "\
        compiles a Rust binary into WASI, then uses lucetc to \
        produce a shared library.\\n
        It can be found in `target/wasm32-wasi/debug/<bin>.so`.
    "
)]
pub struct Args {
    #[structopt(
        short = "-n",
        long = "--name",
        help = "the name of the Rust crate you want to build with `cargo wasi`"
    )]
    name: String,

    #[structopt(short = "-m", long = "--memory")]
    reserved_size: Option<String>,

    #[structopt(long = "--opt-level")]
    opt_level: Option<usize>,
}

pub fn apply(args: Args) -> () {
    let wasi_status = Command::new("cargo")
        .arg("wasi")
        .arg("build")
        .arg("--bin")
        .arg(&args.name)
        .arg("--color")
        .arg("always")
        .status()
        .expect("failed to build wasm");

    assert!(wasi_status.success());
    println!(
        "Successfully built your Rust crate into WASM!\n\
         Now using `lucetc` to compile it into a native library..."
    );

    let input_wasm = format!("target/wasm32-wasi/debug/{}.wasi.wasm", args.name);
    let output_dir = output.join(Path::new(&args.name).with_extension(".so"));
    let reserved_size = reserved_size.unwrap_or(DEFAULT_RESERVED_SIZE);
    let opt_level: String = opt_level.unwrap_or(DEFAULT_OPT_LEVEL).into();
    let lucet_output = Command::new("lucetc")
        .arg(input_wasm)
        .arg("--output")
        .arg(&args.output_dir)
        .arg("--reserved-size")
        .arg(&args.reserved_size)
        .arg("--opt-level")
        .arg(&args.opt_level)
        .arg("--wasi_exe")
        .output()
        .expect("failed to build .so");

    io::stdout().write_all(&lucet_output.stdout).unwrap();
    io::stderr().write_all(&lucet_output.stderr).unwrap();
    assert!(lucet_output.status.success());
}
