use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::debug;

use crate::util::cli_directory;

mod build;
mod clean;
mod test;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    #[command(about = "Build artifacts")]
    #[command(visible_alias = "b")]
    Build(BuildArgs),
    #[command(about = "Run (parts of) the test suite")]
    #[command(visible_alias = "t")]
    Test(TestArgs),
    #[command(about = "Remove build artifacts")]
    #[command(visible_alias = "c")]
    Clean(CleanArgs),
    #[command(about = "Prints the path to the flox binary")]
    Bin(BinArgs),
}

#[derive(Debug, Clone, clap::Args)]
pub struct BuildArgs {
    #[arg(short, long, help = "Build the artifact with Nix")]
    pub nix: bool,
    #[arg(help = "The artifact to build", default_value = "all")]
    pub artifact: String,
}

#[derive(Debug, Clone, clap::Args)]
pub struct TestArgs {
    #[arg(short, long, help = "Run integration tests against a Nix-built binary")]
    pub nix: bool,
    #[arg(
        short,
        long,
        help = "Build the specified artifact before running tests"
    )]
    pub build: Option<String>,
    #[arg(
        short,
        long,
        help = "Run unit tests",
        value_name = "REGEX",
        num_args = 0..=1,
        default_missing_value = "all"
    )]
    pub unit: Option<String>,
    #[arg(
        short,
        long,
        help = "Run integration tests",
        value_name = "BATS_ARGS",
        num_args = 0..,
        default_missing_value = "all"
    )]
    pub int: Option<Vec<String>>,
}

#[derive(Debug, Clone, clap::Args)]
pub struct CleanArgs {
    #[arg(help = "The artifact to clean (default: all)", default_value = "all")]
    pub artifact: String,
}

#[derive(Debug, Clone, clap::Args)]
pub struct BinArgs {
    #[arg(
        help = "Which build profile to use (default: debug)",
        default_value = "debug"
    )]
    pub profile: String,
}

pub fn handle_cmd(cmd: &Command) -> Result<()> {
    match cmd {
        Command::Build(build_args) => {
            debug!("running build command");
            build::build(build_args)
        }
        Command::Test(test_args) => {
            debug!("running test command");
            test::test(test_args)
        }
        Command::Clean(clean_args) => {
            debug!("running clean command");
            clean::clean(&clean_args.artifact)
        }
        Command::Bin(bin_args) => {
            debug!("running bin command");
            let fragment = format!("target/{}/flox", bin_args.profile);
            let dir = cli_directory()?.join(fragment);
            println!("{}", dir.display());
            Ok(())
        }
    }
}
