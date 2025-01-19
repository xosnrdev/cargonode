pub mod project;

use crate::{config::Config, error::CliError, StepCommand};
use std::{path::PathBuf, process::Command};

pub fn do_call(config: &Config) -> Result<(), CliError> {
    let global_scope = config.get_global_scope();
    let executable = global_scope.get_executable().as_ref().unwrap();

    println!(
        "Running `{} {}`",
        executable.display(),
        global_scope.get_args().join(" ")
    );

    log::debug!(
        "Executing command `{}` with arguments `{}`",
        executable.display(),
        global_scope.get_args().join(" ")
    );

    let mut cmd = Command::new(executable);
    cmd.args(global_scope.get_args())
        .envs(global_scope.get_env_vars());

    let working_dir = global_scope.get_working_dir();
    if !working_dir.as_os_str().is_empty() {
        cmd.current_dir(working_dir);
    }

    let env_vars = global_scope.get_env_vars();
    if !env_vars.is_empty() {
        log::debug!("Using environment variables: {:?}", env_vars.keys());
    }

    let mut child = cmd.spawn().map_err(|e| {
        log::error!("Failed to spawn command '{}': {}", executable.display(), e);
        CliError::from(e)
    })?;

    let status = child.wait().map_err(|e| {
        log::error!(
            "Failed to wait on command '{}': {}",
            executable.display(),
            e
        );
        CliError::from(e)
    })?;

    if !status.success() {
        log::error!(
            "Command '{}' exited with status: {}",
            executable.display(),
            status
        );
        return Err(CliError::from(status.code().unwrap_or(-1)));
    }

    Ok(())
}

pub fn do_call_with_package_manager(
    package_manager: PathBuf,
    working_dir: PathBuf,
) -> Result<(), CliError> {
    let mut step_command = StepCommand::default();
    *step_command.get_executable_mut() = Some(package_manager);
    *step_command.get_args_mut() = vec!["install".to_string()];
    *step_command.get_working_dir_mut() = working_dir;

    let mut config = Config::default();
    *config.get_global_scope_mut() = step_command;
    config.validate()?;

    do_call(&config)
}
