use anyhow::{Context, Result};
use duct::cmd;

use crate::util::{build_dir, cargo_workspace_manifest, meson_builddir, repo_root};

pub trait Component {
    fn build(&self, with_nix: bool) -> Result<()>;
    fn clean(&self) -> Result<()>;
}

fn no_op() -> Result<()> {
    eprintln!("Nothing to do");
    Ok(())
}

pub struct Flox;

impl Component for Flox {
    fn build(&self, with_nix: bool) -> Result<()> {
        if with_nix {
            cmd!("nix", "build", ".#flox-cli").run()?;
        } else {
            cmd!(
                "cargo",
                "build",
                "--manifest-path",
                cargo_workspace_manifest()?,
                "-p",
                "flox"
            )
            .run()?;
        }
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        cmd!(
            "cargo",
            "clean",
            "--manifest-path",
            cargo_workspace_manifest()?,
            "-p",
            "flox"
        )
        .run()?;
        Ok(())
    }
}

pub struct Watchdog;

impl Component for Watchdog {
    fn build(&self, with_nix: bool) -> Result<()> {
        if with_nix {
            cmd!("nix", "build", ".#flox-watchdog").run()?;
        } else {
            cmd!(
                "cargo",
                "build",
                "--manifest-path",
                cargo_workspace_manifest()?,
                "-p",
                "flox-watchdog"
            )
            .run()?;
        }
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        cmd!(
            "cargo",
            "clean",
            "--manifest-path",
            cargo_workspace_manifest()?,
            "-p",
            "flox-watchdog"
        )
        .run()?;
        Ok(())
    }
}

pub struct FloxActivations;

impl Component for FloxActivations {
    fn build(&self, with_nix: bool) -> Result<()> {
        if with_nix {
            cmd!("nix", "build", ".#flox-activations").run()?;
        } else {
            cmd!(
                "cargo",
                "build",
                "--manifest-path",
                cargo_workspace_manifest()?,
                "-p",
                "flox-activations"
            )
            .run()?;
        }
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        cmd!(
            "cargo",
            "clean",
            "--manifest-path",
            cargo_workspace_manifest()?,
            "-p",
            "flox-activations"
        )
        .run()?;
        Ok(())
    }
}

pub struct NixPlugin;

impl Component for NixPlugin {
    fn build(&self, with_nix: bool) -> Result<()> {
        if with_nix {
            cmd!("nix", "build", ".#flox-nix-plugins").run()?;
        } else {
            cmd!("meson", "compile", "-C", meson_builddir()?).run()?;
            cmd!("meson", "install", "-C", meson_builddir()?).run()?;
        }
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        cmd!("meson", "compile", "-C", meson_builddir()?, "--clean").run()?;
        Ok(())
    }
}

pub struct ManPages;

impl Component for ManPages {
    fn build(&self, _with_nix: bool) -> Result<()> {
        cmd!(
            "nix",
            "build",
            ".#flox-manpages",
            "-o",
            repo_root()?.join("build/flox-manpages")
        )
        .run()?;
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        no_op()
    }
}

pub struct ActivationScripts;

impl Component for ActivationScripts {
    fn build(&self, _with_nix: bool) -> Result<()> {
        FloxActivations.build(false)?;
        cmd!(
            "nix",
            "build",
            "--option",
            "pure-eval",
            "false",
            ".#floxDevelopmentPackages.flox-activation-scripts^*",
            "-o",
            std::env::var("FLOX_INTERPRETER").context("FLOX_INTERPRETER not set")?
        )
        .run()?;
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        // Swallow this error if the artifact doesn't exist
        let _ = std::fs::remove_file(build_dir()?.join("flox-activation-scripts"));
        Ok(())
    }
}

pub struct PackageBuilder;

impl Component for PackageBuilder {
    fn build(&self, _with_nix: bool) -> Result<()> {
        cmd!(
            "nix",
            "build",
            ".#floxDevelopmentPackages.flox-package-builder^*",
            "-o",
            std::env::var("FLOX_PACKAGE_BUILDER").context("FLOX_PACKAGE_BUILDER not set")?
        )
        .run()?;
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        // Swallow this error if the artifact doesn't exist
        let _ = std::fs::remove_file(build_dir()?.join("flox-activation-scripts"));
        Ok(())
    }
}

pub struct Buildenv;

impl Component for Buildenv {
    fn build(&self, _with_nix: bool) -> Result<()> {
        cmd!(
            "nix",
            "build",
            "--option",
            "pure-eval",
            "false",
            ".#floxDevelopmentPackages.flox-buildenv^*",
            "-o",
            std::env::var("FLOX_BUILDENV").context("FLOX_BUILDENV not set")?
        )
        .run()?;
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        // Swallow this error if the artifact doesn't exist
        let _ = std::fs::remove_file(build_dir()?.join("flox-activation-scripts"));
        Ok(())
    }
}
