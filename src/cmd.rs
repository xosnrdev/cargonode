pub mod project;

use crate::{config::Config, error::CliError};
use log::{debug, error};
use std::process::Command;

pub fn do_call(config: &Config) -> Result<(), CliError> {
    let global_scope = config.get_global_scope();
    let executable = global_scope.get_executable().as_ref().unwrap();

    debug!(
        "Executing command: {} with args: {:?}",
        executable.display(),
        global_scope.get_args()
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
        debug!("Using environment variables: {:?}", env_vars.keys());
    }

    let mut child = cmd.spawn().map_err(|e| {
        error!("Failed to spawn command '{}': {}", executable.display(), e);
        CliError::from(e)
    })?;

    let status = child.wait().map_err(|e| {
        error!(
            "Failed to wait on command '{}': {}",
            executable.display(),
            e
        );
        CliError::from(e)
    })?;

    if !status.success() {
        error!(
            "Command '{}' exited with status: {}",
            executable.display(),
            status
        );
        return Err(CliError::from(status.code().unwrap_or(-1)));
    }

    debug!("Command '{}' executed successfully.", executable.display());
    Ok(())
}
