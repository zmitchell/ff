use crate::{
    components::{
        ActivationScripts, Buildenv, Component, Flox, FloxActivations, NixPlugin, PackageBuilder,
        Watchdog,
    },
    util::{build_dir, cargo_workspace_manifest},
};
use anyhow::{Context, Result};
use duct::cmd;

pub fn clean(artifact: &str) -> Result<()> {
    match artifact {
        "all" => clean_all(),
        "scripts" => ActivationScripts.clean(),
        "activations" | "act" => FloxActivations.clean(),
        "flox" | "cli" => Flox.clean(),
        "plugins" | "nix-plugins" => NixPlugin.clean(),
        "buildenv" => Buildenv.clean(),
        "package-builder" => PackageBuilder.clean(),
        "watchdog" => Watchdog.clean(),
        "nix" => clean_nix_artifacts(),
        _ => anyhow::bail!("unknown artifact: {artifact}"),
    }
}

fn clean_all() -> Result<()> {
    std::fs::remove_dir_all(build_dir()?).context("failed to remove build dir")?;
    cmd!(
        "cargo",
        "clean",
        "--manifest-path",
        cargo_workspace_manifest()?
    )
    .run()?;
    Ok(())
}

fn clean_nix_artifacts() -> Result<()> {
    std::fs::remove_dir_all(build_dir()?).context("failed to remove build dir")
}
