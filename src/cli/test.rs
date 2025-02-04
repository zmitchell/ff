use crate::util::{cargo_workspace_manifest, repo_root};

use super::{build::build_all, TestArgs};
use anyhow::{Context, Result};

pub fn test(args: &TestArgs) -> Result<()> {
    if args.build {
        build_all(args.nix)?;
    }
    if args.unit.is_none() && args.int.is_none() && args.nix {
        run_unit_tests(args.unit.as_ref())?;
        run_integration_tests_with_nix(args.int.as_ref())?;
        return Ok(());
    } else if args.unit.is_none() && args.int.is_none() {
        run_unit_tests(args.unit.as_ref())?;
        run_integration_tests(args.int.as_ref())?;
        return Ok(());
    }
    if args.unit.is_some() {
        run_unit_tests(args.unit.as_ref())?;
    }
    if args.int.is_some() {
        if args.nix {
            run_integration_tests_with_nix(args.int.as_ref())?;
        } else {
            run_integration_tests(args.int.as_ref())?;
        }
    }
    Ok(())
}

fn run_unit_tests(filter: Option<&String>) -> Result<()> {
    let mut args = vec![
        "nextest".to_string(),
        "run".to_string(),
        "--workspace".to_string(),
        "--manifest-path".to_string(),
        cargo_workspace_manifest()?.to_string_lossy().into_owned(),
    ];
    if let Some(filter) = filter {
        args.push(filter.clone());
    }
    duct::cmd("cargo", args).run()?;
    Ok(())
}

fn run_integration_tests(bats_args: Option<&Vec<String>>) -> Result<()> {
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
    if let Some(bats_args) = bats_args {
        if bats_args[0] != "all" {
            test_args.push("--".to_string());
            test_args.extend(bats_args.clone());
        }
        duct::cmd("flox-cli-tests", test_args).run()?;
    }
    Ok(())
}

fn run_integration_tests_with_nix(bats_args: Option<&Vec<String>>) -> Result<()> {
    let mut args = vec!["run".to_string(), ".#flox-cli-tests".to_string()];
    if let Some(bats_args) = bats_args {
        if bats_args[0] != "all" {
            args.extend(bats_args.clone());
        }
    }
    duct::cmd("nix", args).run()?;
    Ok(())
}
