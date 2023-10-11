use clap::{arg, command, Args, Parser, Subcommand};
use colored::Colorize;
pub(crate) use log::{error, info, warn};
pub(crate) use rand::{thread_rng, Rng};
pub(crate) use toml::{Table, Value};

///A NeoSPG write in rust
#[derive(Parser)]
#[command(name = "spg-rs")]
#[command(author = "UrsusFeline <ursusfeline07@gmail.com>")]
#[command(version = "0.1.0")]
#[command(
    about = "A NeoSPG write in rust",
    long_about = "A NeoSPG(C-NeoSPG was made by Lingrottin: https://github.com/lingrottin/neospg/) write in rust, made by UrsusFeline(https://github.com/LGY07/LearnRust/)"
)]
struct Cli {
    #[arg(
        short = 's',
        long = "simple",
        value_name = "SIMPLE_MODE",
        help = "Simple mode without password vault function."
    )]
    simple: bool,

    #[arg(
        short = 'c',
        long = "cspg",
        value_name = "C_MODE",
        help = "Traditional C-NeoSPG compatibility"
    )]
    c_lang: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    todo!()
}
