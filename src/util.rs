use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use duct::cmd;
use tracing::debug;

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

pub fn path_to_string(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

pub fn flox_version() -> Result<String> {
    debug!("getting flox version");
    let version_file_path = repo_root()?.join("VERSION");
    debug!(
        path = path_to_string(&version_file_path),
        "reading version file"
    );
    let version =
        std::fs::read_to_string(version_file_path).context("failed to read version file")?;
    let version = version.trim();
    debug!(%version, "found version in file");
    let git_rev = cmd!("git", "rev-parse", "--short", "HEAD").read()?;
    debug!(rev = git_rev, "got git revision");
    let version = format!("{version}-g{git_rev}");
    debug!(%version, "assembled flox version");
    Ok(version)
}
