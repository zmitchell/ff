use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    #[command(about = "Build an artifact")]
    #[command(visible_alias = "b")]
    Build(BuildArgs),
    #[command(about = "Run (parts of) the test suite")]
    #[command(visible_alias = "t")]
    Test(TestArgs),
    #[command(about = "Remove all build artifacts")]
    #[command(visible_alias = "c")]
    Clean(CleanArgs),
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
    #[arg(short, long, help = "Run unit tests", value_name = "REGEX")]
    pub unit: Option<String>,
    #[arg(short, long, help = "Run integration tests", value_name = "BATS_ARGS")]
    pub int: Option<String>,
    #[arg(short, long, help = "Run integration tests against a Nix-built binary")]
    pub nix: bool,
    #[arg(
        short,
        long,
        help = "Build the specified artifact before running tests"
    )]
    pub build: Option<String>,
}

#[derive(Debug, Clone, clap::Args)]
pub struct CleanArgs {
    #[arg(help = "The artifact to clean (default: all)", default_value = "all")]
    pub artifact: String,
}
