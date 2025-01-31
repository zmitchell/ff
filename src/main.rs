use clap::Parser;
use cli::{handle_cmd, Cli};

mod cli;
mod components;
mod util;

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    handle_cmd(&args.cmd)
}
