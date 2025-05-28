use crate::components::{
    ActivationScripts, Buildenv, Component, Flox, FloxActivations, NixPlugin, PackageBuilder,
    Watchdog,
};
use anyhow::Result;
use tracing::debug;

use super::BuildArgs;

pub fn build(args: &BuildArgs) -> Result<()> {
    build_component(&args.artifact, args.nix)
}

pub fn build_component(component: &str, with_nix: bool) -> Result<()> {
    match component {
        "all" => build_all(with_nix),
        "scripts" => ActivationScripts.build(with_nix),
        "activations" | "act" => FloxActivations.build(with_nix),
        "flox" | "cli" => Flox.build(with_nix),
        "plugins" | "nix-plugins" => NixPlugin.build(with_nix),
        "buildenv" => Buildenv.build(with_nix),
        "package-builder" => PackageBuilder.build(with_nix),
        "watchdog" => Watchdog.build(with_nix),
        "nix" => build_nix_components(),
        _ => anyhow::bail!("unknown artifact: {}", component),
    }
}

pub fn build_all(with_nix: bool) -> Result<()> {
    debug!("building all components");
    NixPlugin.build(with_nix)?;
    PackageBuilder.build(with_nix)?;
    ActivationScripts.build(with_nix)?;
    Watchdog.build(with_nix)?;
    Buildenv.build(with_nix)?;
    Flox.build(with_nix)?;
    Ok(())
}

fn build_nix_components() -> Result<()> {
    debug!("building all components");
    NixPlugin.build(true)?;
    PackageBuilder.build(true)?;
    ActivationScripts.build(true)?;
    Buildenv.build(true)?;
    Ok(())
}
