use crate::config::Config;
use crate::error::AppResult;
use log::{debug, error};
use std::process::Command;

//----------------------------------------------------------------------
// Exports
//----------------------------------------------------------------------

pub mod build;
pub mod check;
pub mod fmt;
pub mod init;
pub mod new;
pub mod release;
pub mod test;

//----------------------------------------------------------------------
// Functions
//----------------------------------------------------------------------

/// Calls any executables, `Config` provides.
pub fn do_call(config: &Config) -> AppResult<()> {
    debug!(
        "Preparing to execute command: {}",
        config.command.executable
    );

    let mut binding = Command::new(&config.command.executable);
    let command = binding
        .args(&config.command.args)
        .envs(&config.command.env_vars);

    if let Some(dir) = &config.command.working_dir {
        command.current_dir(dir);
    }

    // Log environment variable for debugging purposes.
    // HACK: While `env_vars` can contain sensitive information, we can log the keys only.
    if !config.command.env_vars.is_empty() {
        let env_keys: Vec<&String> = config.command.env_vars.keys().collect();
        debug!("Setting environment variables: {:?}", env_keys);
    }

    let mut child = command.spawn().map_err(|e| {
        error!(
            "Failed to spawn command '{}': {}",
            config.command.executable, e
        );
        e
    })?;

    let status = child.wait().map_err(|e| {
        error!(
            "Failed to wait on child process '{}': {}",
            config.command.executable, e
        );
        e
    })?;

    if !status.success() {
        error!(
            "Command '{}' exited with status: {}",
            config.command.executable, status
        );
        anyhow::bail!(
            "Command '{}' exited with status: {}",
            config.command.executable,
            status
        );
    }

    debug!(
        "Command '{}' executed successfully.",
        config.command.executable
    );
    Ok(())
}
