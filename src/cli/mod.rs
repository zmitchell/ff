use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Build(BuildArgs),
    Test(TestArgs),
    Clean(CleanArgs),
}

#[derive(Debug, Clone, clap::Args)]
pub struct BuildArgs {}

#[derive(Debug, Clone, clap::Args)]
pub struct TestArgs {}

#[derive(Debug, Clone, clap::Args)]
pub struct CleanArgs {}
