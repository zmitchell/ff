use clap::Parser;
use cli::{handle_cmd, Cli};
use tracing::debug;

mod cli;
mod components;
mod util;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    debug!(?args, "called with arguments");
    handle_cmd(&args.cmd)
}
