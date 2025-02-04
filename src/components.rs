use anyhow::{Context, Result};
use duct::cmd;
use tracing::debug;

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
        debug!(%with_nix, "building flox");
        if with_nix {
            let cmd = cmd!("nix", "build", ".#flox-cli");
            debug!(?cmd, "build command");
            cmd.run()?;
        } else {
            let cmd = cmd!(
                "cargo",
                "build",
                "--manifest-path",
                cargo_workspace_manifest()?,
                "-p",
                "flox"
            );
            debug!(?cmd, "build command");
            cmd.run()?;
        }
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        debug!("cleaning flox");
        let cmd = cmd!(
            "cargo",
            "clean",
            "--manifest-path",
            cargo_workspace_manifest()?,
            "-p",
            "flox"
        );
        debug!(?cmd, "clean command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }
}

pub struct Watchdog;

impl Component for Watchdog {
    fn build(&self, with_nix: bool) -> Result<()> {
        debug!(%with_nix, "building watchdog");
        if with_nix {
            let cmd = cmd!("nix", "build", ".#flox-watchdog");
            debug!(?cmd, "build command");
            cmd.run()?;
        } else {
            let cmd = cmd!(
                "cargo",
                "build",
                "--manifest-path",
                cargo_workspace_manifest()?,
                "-p",
                "flox-watchdog"
            );
            debug!(?cmd, "build command");
            cmd.run()?;
        }
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        debug!("cleaning watchdog");
        let cmd = cmd!(
            "cargo",
            "clean",
            "--manifest-path",
            cargo_workspace_manifest()?,
            "-p",
            "flox-watchdog"
        );
        debug!(?cmd, "clean command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }
}

pub struct FloxActivations;

impl Component for FloxActivations {
    fn build(&self, with_nix: bool) -> Result<()> {
        debug!(%with_nix, "building flox-activations");
        if with_nix {
            let cmd = cmd!("nix", "build", ".#flox-activations");
            debug!(?cmd, "build command");
            cmd.run()?;
        } else {
            let cmd = cmd!(
                "cargo",
                "build",
                "--manifest-path",
                cargo_workspace_manifest()?,
                "-p",
                "flox-activations"
            );
            debug!(?cmd, "build command");
            cmd.run()?;
        }
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        let cmd = cmd!(
            "cargo",
            "clean",
            "--manifest-path",
            cargo_workspace_manifest()?,
            "-p",
            "flox-activations"
        );
        debug!(?cmd, "clean command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }
}

pub struct NixPlugin;

impl Component for NixPlugin {
    fn build(&self, with_nix: bool) -> Result<()> {
        debug!(%with_nix, "building nix plugins");
        if with_nix {
            let cmd = cmd!("nix", "build", ".#flox-nix-plugins");
            debug!(?cmd, "build command");
            cmd.run()?;
        } else {
            let cmd = cmd!("meson", "compile", "-C", meson_builddir()?);
            debug!(?cmd, "compile command");
            cmd.run()?;
            let cmd = cmd!("meson", "install", "-C", meson_builddir()?);
            debug!(?cmd, "install command");
            cmd.run()?;
        }
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        debug!("cleaning nix plugins");
        let cmd = cmd!("meson", "compile", "-C", meson_builddir()?, "--clean");
        debug!(?cmd, "clean command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }
}

pub struct ManPages;

impl Component for ManPages {
    fn build(&self, _with_nix: bool) -> Result<()> {
        debug!("building man pages");
        let cmd = cmd!(
            "nix",
            "build",
            ".#flox-manpages",
            "-o",
            repo_root()?.join("build/flox-manpages")
        );
        debug!(?cmd, "build command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        debug!("cleaning man pages (no-op)");
        no_op()
    }
}

pub struct ActivationScripts;

impl Component for ActivationScripts {
    fn build(&self, _with_nix: bool) -> Result<()> {
        debug!("building activation scripts");
        debug!("first building flox-activations");
        FloxActivations.build(false)?;
        let cmd = cmd!(
            "nix",
            "build",
            "--option",
            "pure-eval",
            "false",
            ".#floxDevelopmentPackages.flox-activation-scripts^*",
            "-o",
            std::env::var("FLOX_INTERPRETER").context("FLOX_INTERPRETER not set")?
        );
        debug!(?cmd, "build command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        debug!("cleaning activation scripts");
        let path = build_dir()?.join("flox-activation-scripts");
        debug!(
            path = path.to_string_lossy().to_string(),
            "removing symlink"
        );
        // Swallow this error if the artifact doesn't exist
        let _ = std::fs::remove_file(path);
        debug!("succeeded");
        Ok(())
    }
}

pub struct PackageBuilder;

impl Component for PackageBuilder {
    fn build(&self, _with_nix: bool) -> Result<()> {
        debug!("building package builder");
        let cmd = cmd!(
            "nix",
            "build",
            ".#floxDevelopmentPackages.flox-package-builder^*",
            "-o",
            std::env::var("FLOX_PACKAGE_BUILDER").context("FLOX_PACKAGE_BUILDER not set")?
        );
        debug!(?cmd, "build command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        debug!("cleaning package builder");
        let path = build_dir()?.join("flox-package-builder");
        debug!(
            path = path.to_string_lossy().to_string(),
            "removing symlink"
        );
        // Swallow this error if the artifact doesn't exist
        let _ = std::fs::remove_file(path);
        debug!("succeeded");
        Ok(())
    }
}

pub struct Buildenv;

impl Component for Buildenv {
    fn build(&self, _with_nix: bool) -> Result<()> {
        debug!("building buildenv");
        let cmd = cmd!(
            "nix",
            "build",
            "--option",
            "pure-eval",
            "false",
            ".#floxDevelopmentPackages.flox-buildenv^*",
            "-o",
            std::env::var("FLOX_BUILDENV").context("FLOX_BUILDENV not set")?
        );
        debug!(?cmd, "build command");
        cmd.run()?;
        debug!("succeeded");
        Ok(())
    }

    fn clean(&self) -> Result<()> {
        debug!("cleaning buildenv");
        let path = build_dir()?.join("flox-buildenv");
        debug!(
            path = path.to_string_lossy().to_string(),
            "removing symlink"
        );
        // Swallow this error if the artifact doesn't exist
        let _ = std::fs::remove_file(path);
        debug!("succeeded");
        Ok(())
    }
}
