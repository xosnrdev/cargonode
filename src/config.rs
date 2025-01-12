use std::{
    collections::HashMap,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};

use crate::{error::AppResult, ops::parser::ConfigArgs};

//----------------------------------------------------------------------
// Exports
//----------------------------------------------------------------------

pub mod source;

//----------------------------------------------------------------------
// Constants
//----------------------------------------------------------------------

/// Default configuration file name.
pub const DEFAULT_CONFIG_FILE: &str = "package.json";

/// Default configuration key, typically the package name.
pub const DEFAULT_CONFIG_KEY: &str = env!("CARGO_PKG_NAME");

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents the executable command and its parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct Command {
    /// Path to the executable.
    pub executable: String,
    /// Arguments to pass to the executable.
    pub args: Vec<String>,
    /// Environment variables to set for the executable.
    pub env_vars: HashMap<String, String>,
    /// Working directory for the executable.
    pub working_dir: Option<PathBuf>,
}

/// Represents the overall configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Command configuration.
    #[serde(flatten)]
    pub command: Command,

    /// Pre-execution checks or commands.
    pub pre_checks: Option<Vec<String>>,

    /// Timeout in seconds for the command execution.
    pub timeout: Option<u64>,

    /// Verbosity level for logging.
    pub verbose: u8,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Config {
    /// Merges another config into self, giving precedence to the other Config's fields if they are set.
    pub fn merge(&mut self, other: Config) {
        trace!("Merging {:?} into {:?}", other, self);

        if !other.command.executable.is_empty() {
            trace!(
                "Overriding executable: '{}' -> '{}'",
                self.command.executable,
                other.command.executable
            );
            self.command.executable = other.command.executable;
        }

        if !other.command.args.is_empty() {
            trace!(
                "Overriding args: {:?} -> {:?}",
                self.command.args,
                other.command.args
            );
            self.command.args = other.command.args;
        }

        if !other.command.env_vars.is_empty() {
            trace!(
                "Overriding env_vars: {:?} -> {:?}",
                self.command.env_vars,
                other.command.env_vars
            );
            self.command.env_vars = other.command.env_vars;
        }

        if let Some(dir) = other.command.working_dir {
            trace!(
                "Overriding working_dir: {:?} -> {:?}",
                self.command.working_dir,
                dir
            );
            self.command.working_dir = Some(dir);
        }

        if let Some(pre_checks) = other.pre_checks {
            trace!(
                "Overriding pre_checks: {:?} -> {:?}",
                self.pre_checks,
                pre_checks
            );
            self.pre_checks = Some(pre_checks);
        }

        if let Some(timeout) = other.timeout {
            trace!("Overriding timeout: {:?} -> {:?}", self.timeout, timeout);
            self.timeout = Some(timeout);
        }

        if other.verbose > 0 {
            trace!(
                "Overriding verbosity: {} -> {}",
                self.verbose,
                other.verbose
            );
            self.verbose = other.verbose;
        }
    }

    /// Validates the configuration fields.
    pub fn validate(&self) -> AppResult<()> {
        // Validate timeout
        if let Some(timeout) = self.timeout {
            if timeout == 0 {
                anyhow::bail!("Timeout must be greater than zero.");
            }
            if timeout > 3600 {
                warn!("Timeout value {} exceeds recommended limits.", timeout);
            }
        }

        if self.command.executable.trim().is_empty() {
            anyhow::bail!("Executable path cannot be empty.");
        }

        // Check if executable exists in PATH
        if which::which(&self.command.executable).is_err() {
            anyhow::bail!(
                "Executable '{}' not found in PATH.",
                self.command.executable
            );
        }

        if let Some(ref dir) = self.command.working_dir {
            if !dir.exists() || !dir.is_dir() {
                anyhow::bail!(
                    "Working directory '{:?}' does not exist or is not a directory.",
                    dir
                );
            }
        }

        Ok(())
    }

    /// Converts config from command-line args to what our `Config` can work with.
    pub fn from_args(&self, args: &ConfigArgs) -> AppResult<Config> {
        trace!("Converting {:?} to {:?}.", args, self.command);

        // Help convert env_vars from Vec<String> to HashMap
        fn parse_env_vars(env_vars: &[String]) -> AppResult<HashMap<String, String>> {
            let mut map = HashMap::new();
            for (index, var) in env_vars.iter().enumerate() {
                let parts: Vec<&str> = var.splitn(2, '=').collect();
                if parts.len() != 2 {
                    anyhow::bail!(
                        "Invalid env_var format at position {}: '{}'. Expected KEY=VALUE.",
                        index,
                        var
                    );
                }
                map.insert(parts[0].to_string(), parts[1].to_string());
            }
            Ok(map)
        }

        let mut command = Command::default();

        if let Some(ref executable) = args.executable {
            trace!("Overriding executable with: {}", executable);
            command.executable = executable.to_owned();
        }

        if let Some(ref args_vec) = args.args {
            trace!("Overriding arguments with: {:?}", args);
            command.args = args_vec.to_owned();
        }

        if let Some(ref env_vars) = args.env_vars {
            let parsed_env_vars = parse_env_vars(env_vars)?;
            trace!(
                "Overriding environment variables with keys: {:?}",
                parsed_env_vars.keys()
            );
            command.env_vars = parsed_env_vars;
        }

        if let Some(ref working_dir) = args.working_dir {
            trace!("Overriding working directory with: {:?}", working_dir);
            command.working_dir = Some(working_dir.to_path_buf());
        }

        let pre_checks = &args.pre_checks;
        if let Some(ref checks) = pre_checks {
            trace!("Overriding pre-checks with: {:?}", checks);
        }

        let timeout = args.timeout;
        if let Some(t) = timeout {
            trace!("Overriding timeout with: {:?}", t);
        }

        let verbose = args.verbose;
        if verbose > 0 {
            trace!("Overriding verbosity with: {:?}", verbose);
        }

        Ok(Config {
            command,
            pre_checks: pre_checks.clone(),
            timeout,
            verbose,
        })
    }

    /// Reads and parses the configuration from a `json` file.
    pub fn from_config_file(path: &Path, config_key: &str) -> AppResult<Config> {
        trace!("Attempting to open configuration file: {:?}", path);
        let file = std::fs::File::open(path)
            .with_context(|| format!("Failed to open configuration file: {:?}", path))?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader
            .read_to_string(&mut contents)
            .with_context(|| format!("Failed to read configuration file: {:?}", path))?;
        debug!("Successfully read configuration file.");

        debug!("Parsing JSON content.");
        let json: serde_json::Value = serde_json::from_str(&contents)
            .with_context(|| "Failed to parse JSON from configuration file.")?;

        trace!("Extracting configuration from key '{}'.", config_key);
        let config_value = json
            .get(config_key)
            .ok_or_else(|| anyhow::anyhow!("Configuration key '{}' not found.", config_key))?;

        trace!("Configuration key value: {:?}", config_value);

        debug!("Deserializing configuration into Config.");
        let config: Config =
            serde_json::from_value(config_value.to_owned()).with_context(|| {
                format!(
                    "Failed to deserialize configuration for key '{}'.",
                    config_key
                )
            })?;
        debug!("Successfully deserialized configuration.");

        Ok(config)
    }
}
