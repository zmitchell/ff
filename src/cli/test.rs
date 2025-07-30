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
    if args.test_type.unit.is_none() && args.test_type.int.is_none() && args.nix {
        run_unit_tests(args.test_type.unit.as_ref(), None)?;
        run_integration_tests_with_nix(args.test_type.int.as_ref(), None)?;
        return Ok(());
    } else if args.test_type.unit.is_none() && args.test_type.int.is_none() {
        run_unit_tests(args.test_type.unit.as_ref(), None)?;
        run_integration_tests(args.test_type.int.as_ref(), None)?;
        return Ok(());
    }
    if args.test_type.unit.is_some() {
        run_unit_tests(args.test_type.unit.as_ref(), args.additional_args.as_ref())?;
        return Ok(());
    }
    if args.test_type.int.is_some() {
        if args.nix {
            run_integration_tests_with_nix(
                args.test_type.int.as_ref(),
                args.additional_args.as_ref(),
            )?;
        } else {
            run_integration_tests(args.test_type.int.as_ref(), args.additional_args.as_ref())?;
        }
        return Ok(());
    }
    Ok(())
}

fn run_unit_tests(filter: Option<&String>, additional_args: Option<&Vec<String>>) -> Result<()> {
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
    if let Some(additional_args) = additional_args {
        args.extend_from_slice(additional_args);
    }
    let cmd = duct::cmd("cargo", args).env("FLOX_VERSION", flox_version()?);
    debug!(?cmd, "test command");
    cmd.run()?;
    debug!("succeeded");
    Ok(())
}

fn run_integration_tests(
    bats_args: Option<&Vec<String>>,
    additional_args: Option<&Vec<String>>,
) -> Result<()> {
    debug!(with_nix = false, "running integration tests");
    let mut test_args = vec![];
    if let Some(bats_args) = bats_args {
        if bats_args[0] != "all" {
            test_args.extend(bats_args.clone());
        }
    }
    if let Some(additional_args) = additional_args {
        test_args.extend_from_slice(additional_args);
    }
    let cmd = duct::cmd("flox-cli-tests", test_args).env("FLOX_VERSION", flox_version()?);
    debug!(?cmd, "test command");
    cmd.run()?;
    debug!("succeeded");
    Ok(())
}

fn run_integration_tests_with_nix(
    bats_args: Option<&Vec<String>>,
    additional_args: Option<&Vec<String>>,
) -> Result<()> {
    debug!(with_nix = true, "running integration tests");
    let mut args = vec!["run".to_string(), ".#flox-cli-tests".to_string()];
    if let Some(bats_args) = bats_args {
        if bats_args[0] != "all" {
            args.extend(bats_args.clone());
        }
    }
    if let Some(additional_args) = additional_args {
        args.extend_from_slice(additional_args);
    }
    let cmd = duct::cmd("nix", args).env("FLOX_VERSION", flox_version()?);
    debug!(?cmd, "test command");
    cmd.run()?;
    debug!("succeeded");
    Ok(())
}
