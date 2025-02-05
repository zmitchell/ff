use crate::{
    components::{
        ActivationScripts, Buildenv, Component, Flox, FloxActivations, NixPlugin, PackageBuilder,
        Watchdog,
    },
    util::{build_dir, cargo_workspace_manifest, path_to_string},
};
use anyhow::{Context, Result};
use duct::cmd;
use tracing::debug;

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
    debug!("cleaning all artifacts");
    let build_dir_path = build_dir()?;
    debug!(path = path_to_string(&build_dir_path), "removing build dir");
    std::fs::remove_dir_all(build_dir_path).context("failed to remove build dir")?;
    debug!("succeeded");
    debug!("cleaning all cargo artifacts");
    let cmd = cmd!(
        "cargo",
        "clean",
        "--manifest-path",
        cargo_workspace_manifest()?
    );
    debug!(?cmd, "clean command");
    cmd.run()?;
    debug!("succeeded");
    Ok(())
}

fn clean_nix_artifacts() -> Result<()> {
    debug!("cleaning nix artifacts");
    let build_dir_path = build_dir()?;
    debug!(path = path_to_string(&build_dir_path), "removing build dir");
    std::fs::remove_dir_all(build_dir_path).context("failed to remove build dir")?;
    debug!("succeeded");
    Ok(())
}
