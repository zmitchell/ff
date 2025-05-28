use crate::{
    cli::build::build_component,
    util::{cargo_workspace_manifest, flox_version},
};

use super::TestArgs;
use anyhow::Result;
use tracing::debug;

pub fn test(args: &TestArgs) -> Result<()> {
    if let Some(ref component) = args.build {
        debug!("doing pre-test build");
        build_component(component, false)?;
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
    debug!("running unit tests");
    let mut args = vec![
        "nextest".to_string(),
        "run".to_string(),
        "--workspace".to_string(),
        "--manifest-path".to_string(),
        cargo_workspace_manifest()?.to_string_lossy().into_owned(),
    ];
    if let Some(filter) = filter {
        if filter != "all" {
            args.push(filter.clone());
        }
    }
    let cmd = duct::cmd("cargo", args).env("FLOX_VERSION", flox_version()?);
    debug!(?cmd, "test command");
    cmd.run()?;
    debug!("succeeded");
    Ok(())
}

fn run_integration_tests(bats_args: Option<&Vec<String>>) -> Result<()> {
    debug!(with_nix = false, "running integration tests");
    let mut test_args = vec![];
    if let Some(bats_args) = bats_args {
        if bats_args[0] != "all" {
            test_args.extend(bats_args.clone());
        }
    }
    let cmd = duct::cmd("flox-cli-tests", test_args).env("FLOX_VERSION", flox_version()?);
    debug!(?cmd, "test command");
    cmd.run()?;
    debug!("succeeded");
    Ok(())
}

fn run_integration_tests_with_nix(bats_args: Option<&Vec<String>>) -> Result<()> {
    debug!(with_nix = true, "running integration tests");
    let mut args = vec!["run".to_string(), ".#flox-cli-tests".to_string()];
    if let Some(bats_args) = bats_args {
        if bats_args[0] != "all" {
            args.extend(bats_args.clone());
        }
    }
    let cmd = duct::cmd("nix", args).env("FLOX_VERSION", flox_version()?);
    debug!(?cmd, "test command");
    cmd.run()?;
    debug!("succeeded");
    Ok(())
}
