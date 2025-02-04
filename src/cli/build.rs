use crate::components::{
    ActivationScripts, Buildenv, Component, Flox, FloxActivations, NixPlugin, PackageBuilder,
    Watchdog,
};
use anyhow::Result;

use super::BuildArgs;

pub fn build(args: &BuildArgs) -> Result<()> {
    match args.artifact.as_str() {
        "all" => build_all(args.nix),
        "scripts" => ActivationScripts.build(args.nix),
        "activations" | "act" => FloxActivations.build(args.nix),
        "flox" | "cli" => Flox.build(args.nix),
        "plugins" | "nix-plugins" => NixPlugin.build(args.nix),
        "buildenv" => Buildenv.build(args.nix),
        "package-builder" => PackageBuilder.build(args.nix),
        "watchdog" => Watchdog.build(args.nix),
        _ => anyhow::bail!("unknown artifact: {}", args.artifact),
    }
}

pub fn build_all(with_nix: bool) -> Result<()> {
    ActivationScripts.build(with_nix)?;
    Watchdog.build(with_nix)?;
    NixPlugin.build(with_nix)?;
    PackageBuilder.build(with_nix)?;
    Buildenv.build(with_nix)?;
    Flox.build(with_nix)?;
    Ok(())
}
