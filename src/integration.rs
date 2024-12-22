use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs, path::Path, result, sync::LazyLock};

use crate::exec;

/// Represents errors encountered during command execution or configuration parsing.
#[derive(Debug)]
pub enum Error {
    /// Represents an error during command execution.
    Execution {
        /// Specifies the command that failed.
        command: String,
        /// Provides the source error.
        source: exec::Error,
    },
    /// Represents an error while parsing the TOML configuration file.
    Toml(toml::de::Error),
}

impl fmt::Display for Error {
    /// Formats the error for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Execution { command, source } => {
                write!(f, "Failed to execute command '{}': {}", command, source)
            }
            Error::Toml(err) => write!(f, "Failed to parse TOML configuration: {}", err),
        }
    }
}

/// Represents predefined commands for the CargoNode tool.
#[derive(Debug)]
pub enum Command {
    /// Formats the project code.
    Format,
    /// Runs linting and checks.
    Check,
    /// Builds the project.
    Build,
    /// Runs the test suite.
    Test,
    /// Releases the project.
    Release,
}

impl fmt::Display for Command {
    /// Converts the command to its string representation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Format => "format",
                Self::Check => "check",
                Self::Build => "build",
                Self::Test => "test",
                Self::Release => "release",
            }
        )
    }
}

/// Represents the configuration for a specific command.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CommandConfig {
    /// Specifies the command to be executed.
    pub command: String,
    /// Specifies the arguments to pass to the command.
    pub args: Vec<String>,
    /// Specifies a list of pre-check commands to execute before the main command.
    #[serde(default)]
    pub pre_checks: Vec<String>,
    /// Specifies environment variables to use during execution.
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
}

/// Generates a default configuration for a command.
macro_rules! command_config {
    ($cmd:expr, $args:expr, $pre_checks:expr) => {
        CommandConfig {
            command: $cmd.to_string(),
            args: $args.iter().map(ToString::to_string).collect(),
            pre_checks: $pre_checks.iter().map(ToString::to_string).collect(),
            ..Default::default()
        }
    };
}

impl Command {
    /// Maps a string to a corresponding `Command` enum variant.
    pub fn map_from_str(cmd: &str) -> Option<Self> {
        match cmd {
            "format" => Some(Self::Format),
            "check" => Some(Self::Check),
            "build" => Some(Self::Build),
            "test" => Some(Self::Test),
            "release" => Some(Self::Release),
            _ => None,
        }
    }

    /// Provides the default configuration for the command.
    pub fn default_config(&self) -> CommandConfig {
        match self {
            Self::Format => command_config!("biome", ["format"], [""]),
            Self::Check => command_config!("biome", ["check"], [""]),
            Self::Build => command_config!("tsup", [""], ["check"]),
            Self::Test => command_config!("vitest", [""], ["check"]),
            Self::Release => command_config!("release-it", [""], ["build"]),
        }
    }
}

/// Represents the configuration for the CargoNode tool.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Stores configurations for commands by their names.
    #[serde(default)]
    pub commands: HashMap<String, CommandConfig>,
}

impl Config {
    /// Loads the configuration from the `cargonode.toml` file or provides a default configuration.
    fn load() -> Self {
        match fs::read_to_string("cargonode.toml") {
            Ok(content) => toml::from_str(&content)
                .map_err(Error::Toml)
                .unwrap_or_default(),
            Err(err) => {
                eprintln!("Failed to read configuration file: {}", err);
                Self::default()
            }
        }
    }

    /// Merges the default configuration with any overrides from the loaded configuration file.
    pub fn merge(&self, default: &CommandConfig, command_name: &str) -> CommandConfig {
        if let Some(file_config) = self.commands.get(command_name) {
            CommandConfig {
                command: if !file_config.command.trim().is_empty() {
                    file_config.command.clone()
                } else {
                    default.command.clone()
                },
                args: if file_config.args.iter().any(|arg| !arg.trim().is_empty()) {
                    file_config
                        .args
                        .iter()
                        .filter(|arg| !arg.trim().is_empty())
                        .cloned()
                        .collect()
                } else {
                    default.args.clone()
                },
                pre_checks: if file_config
                    .pre_checks
                    .iter()
                    .any(|check| !check.trim().is_empty())
                {
                    file_config
                        .pre_checks
                        .iter()
                        .filter(|check| !check.trim().is_empty())
                        .cloned()
                        .collect()
                } else {
                    default.pre_checks.clone()
                },
                env_vars: {
                    let mut merged_env = default.env_vars.clone();
                    merged_env.extend(file_config.env_vars.clone());
                    merged_env
                },
            }
        } else {
            default.clone()
        }
    }
}

/// Represents the result type for this module.
type Result<T> = result::Result<T, Error>;

/// Stores the global configuration loaded from the configuration file.
static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);

/// Executes the specified command in the given working directory with optional additional arguments.
fn execute(work_dir: &Path, command: Command, extra_args: Vec<String>) -> Result<String> {
    let config = CONFIG.merge(&command.default_config(), &command.to_string());

    // Execute pre-checks in order.
    for pre_check in &config.pre_checks {
        Command::map_from_str(pre_check)
            .map(|command| execute(work_dir, command, vec![]))
            // Stop if any pre-check fails.
            .transpose()?;
    }

    // Prepare command arguments.
    let mut args = vec![config.command.clone()];
    args.extend(config.args.clone());
    args.extend(extra_args);

    // Prepare environment variables for execution.
    let env_vars = if !config.env_vars.is_empty() {
        Some(
            config
                .env_vars
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        )
    } else {
        None
    };

    // Execute the main command using the `exec` module.
    exec::npx(work_dir, args, env_vars).map_err(|source| Error::Execution {
        command: command.to_string(),
        source,
    })
}

/// Macro to define shorthand functions for each command.
macro_rules! generate_command_fns {
    ($($name:ident => $snake_case:ident),*) => {
        $(
            /// Executes the specific command in the given work directory.
            pub fn $snake_case(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
                execute(work_dir, Command::$name, extra_args)
            }
        )*
    };
}

// Generate shorthand functions for all predefined commands.
generate_command_fns!(Format => format, Check => check, Build => build, Test => test, Release => release);
