//use libipld_schema::{Context, Error, Representation};
use std::{
    env,
    fs::File,
    io::{Read, Write},
};

//schema!(type Bool bool);

fn log0(op: &str) {
    eprintln!("<||.wasm||> rustcall \n    {}\n\n", op);
}

fn log1(op: &str, arg1: &str) {
    eprintln!(
        "<||.wasm||> rustcall \n    {}    with args {}\n\n",
        op, arg1
    );
}

fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    log1("File::open", input_fname);
    let mut input_file =
        File::open(input_fname).map_err(|err| format!("error opening input: {}", err))?;
    let mut contents = Vec::new();
    log1("File::read_to_end", input_fname);
    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("read error: {}", err))?;

    log1("File::create", output_fname);
    let mut output_file = File::create(output_fname)
        .map_err(|err| format!("error opening output '{}': {}", output_fname, err))?;
    log1("File::write_all", output_fname);
    output_file
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))
}

fn main() {
    log0("env::args::collect");
    let args: Vec<String> = env::args().collect();
    log1("&args[0]", &args[0]);
    log1("&args[1]", &args[1]);
    log1("&args[2]", &args[2]);

    log0("args[0]::clone");
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("{} <input_file> <output_file>", program);
        return;
    }

    let res = process(&args[1], &args[2]);
    if let Err(err) = res {
        eprintln!("{}", err)
    }
}
