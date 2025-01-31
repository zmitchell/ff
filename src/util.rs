use std::path::PathBuf;

use anyhow::{Context, Result};
use duct::cmd;

pub fn repo_root() -> Result<PathBuf> {
    cmd!("git", "rev-parse", "--show-toplevel")
        .read()
        .map(PathBuf::from)
        .context("couldn't find repo root")
}

pub fn cli_directory() -> Result<PathBuf> {
    repo_root().map(|p| p.join("cli"))
}

pub fn build_dir() -> Result<PathBuf> {
    repo_root().map(|p| p.join("build"))
}

pub fn cargo_workspace_manifest() -> Result<PathBuf> {
    cli_directory().map(|p| p.join("Cargo.toml"))
}

pub fn meson_builddir() -> Result<PathBuf> {
    repo_root().map(|p| p.join("nix-plugins/builddir"))
}
