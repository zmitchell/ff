use crate::util::{cargo_workspace_manifest, repo_root};

use super::{build::build_all, TestArgs};
use anyhow::{Context, Result};
use duct::cmd;

pub fn test(args: &TestArgs) -> Result<()> {
    if args.build {
        build_all(args.nix)?;
    }
    if let Some(ref filter) = args.unit {
        if filter == "all" {
            cmd!(
                "cargo",
                "nextest",
                "run",
                "--workspace",
                "--manifest-path",
                cargo_workspace_manifest()?
            )
            .run()?;
        } else {
            cmd!(
                "cargo",
                "nextest",
                "run",
                "--workspace",
                "--manifest-path",
                cargo_workspace_manifest()?,
                filter
            )
            .run()?;
        }
    }
    if args.nix {
        if let Some(ref bats_args) = args.int {
            let mut args = vec!["run".to_string(), ".#flox-cli-tests".to_string()];
            if bats_args[0] != "all" {
                args.extend(bats_args.clone());
            }
            duct::cmd("nix", args).run()?;
        }
    } else {
        let nix_plugins = std::env::var("NIX_PLUGINS").context("NIX_PLUGINS was unset")?;
        let flox_bin = std::env::var("FLOX_BIN").context("FLOX_BIN was unset")?;
        let watchdog_bin = std::env::var("WATCHDOG_BIN").context("WATCHDOG_BIN was unset")?;
        let input_data = repo_root()?.join("test_data/input_data");
        let generated_data = std::env::var("GENERATED_DATA").context("GENERATED_DATA was unset")?;
        let mut test_args = vec![
            "--nix-plugins".to_string(),
            nix_plugins,
            "--flox".to_string(),
            flox_bin,
            "--watchdog".to_string(),
            watchdog_bin,
            "--input-data".to_string(),
            input_data.to_string_lossy().into_owned(),
            "--generated-data".to_string(),
            generated_data,
        ];
        if let Some(ref bats_args) = args.int {
            if bats_args[0] != "all" {
                test_args.push("--".to_string());
                test_args.extend(bats_args.clone());
            }
            duct::cmd("flox-cli-tests", test_args).run()?;
        }
    }
    Ok(())
}
