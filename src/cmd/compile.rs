// TODO: include build::Args
///
pub struct Args {
    ///
    #[structopt(long = "--release")]
    release: bool,
}

pub fn apply(args: Args) -> () {}
