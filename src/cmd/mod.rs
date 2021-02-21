mod build;
mod compile;
mod init;
mod reset;
mod start;
mod stop;

///
#[derive(Debug, StructOpt)]
#[structopt(
    name = "datalove",
    version = "0.0.1",
    about = "A minimal tool for quickly building and running WASI apps"
)]
pub enum Command {
    // Basic commands
    ///
    Init,

    ///
    Reset,
    //    ///
    //    Start(start::Args),
    //
    //    ///
    //    Stop(stop::Args),
    //
    //    ///
    //    Build(build::Args),
    //
    //    //
    //    ///
    //    Login,
    //
    //    ///
    //    Logout,
    //
    //    // Docker-ish commands
    //    ///
    //    Push,
    //
    //    ///
    //    Pull,
    //
    //    ///
    //    Stats,
    //
    //    // K8s-sh commands
    //    ///
    //    Apply,
    //    #[structopt(
    //        name = "run",
    //        help = "\
    //            executes the `_start` function in a lucetc-compiled WASI library \
    //            or binary\
    //        "
    //    )]
    //    Run {
    //        #[structopt(short = "-b", long = "--cmd", parse(from_os_str))]
    //        bin_path: PathBuf,
    //
    //        #[structopt(short = "-m", long = "--memory")]
    //        memory: Option<usize>,
    //
    //        #[structopt(short = "-a", long = "--args")]
    //        args: Vec<String>,
    //
    //        #[structopt(short = "-e", long = "--env")]
    //        env: Vec<String>,
    //
    //        #[structopt(long = "--mount")]
    //        mounts: Vec<Mount>,
    //
    //        #[structopt(short = "-d", long = "--detach")]
    //        detach: bool,
    //    },
    //
    //    #[structopt(
    //        name = "exec",
    //        help = "\
    //            executes a named function in a lucetc-compiled WASI library or \
    //            binary\
    //        "
    //    )]
    //    Exec {
    //        #[structopt(short = "-b", long = "--cmd", parse(from_os_str))]
    //        bin_path: PathBuf,
    //
    //        #[structopt(short = "-m", long = "--memory")]
    //        memory: Option<usize>,
    //
    //        #[structopt(short = "-a", long = "--args")]
    //        args: Vec<String>,
    //
    //        #[structopt(short = "-e", long = "--env")]
    //        env: Vec<String>,
    //
    //        #[structopt(long = "--mount")]
    //        mounts: Vec<Mount>,
    //
    //        #[structopt(short = "-d", long = "--detach")]
    //        detach: bool,
    //    }
}

impl Command {
    fn apply(self) -> () {
        match self {
            Command::Init => init::apply(),
            Command::Reset => reset::apply(),
            //            Command::Start(args) => start::apply(args),
            //            Command::Stop(args) => stop::apply(args),
            //            Command::Build(args) => build::apply(args),
            //            Command::Login => (),
            //            Command::Logout => (),
            //            Command::Push => (),
            //            Command::Pull => (),
            //            Command::Stats => (),
            //            Command::Apply => (),
            //            Command::Run {..} => (),
            //            Command::Exec {..} => (),
        }
    }
}
