//! CargoNode integration module for command execution and configuration management
//!
//! # Overview
//!
//! This module provides a robust system for managing and executing development commands
//! with flexible configuration, pre-check support, and advanced error handling.
//!
//! # Features
//!
//! - Dynamic command configuration via `cargonode.toml`
//! - Pre-check support (run dependent commands before main command)
//! - Timeout management
//! - Environment variable injection
//! - Custom and built-in command support
//!
//! # Configuration Example
//!
//! ```toml
//! # cargonode.toml
//! default_timeout_secs = 180  # Global 3-minute timeout
//!
//! [commands.format]
//! command = "prettier"  # Override default Biome formatter
//! args = ["--write", "."]
//!
//! [commands.build]
//! pre_checks = ["check", "format"]
//! env_vars = { NODE_ENV = "production" }
//! ```
//!
//! # Usage
//!
//! ```rust
//! use std::path::Path;
//!
//! async fn run_build() -> Result<(), cargonode::Error> {
//!     let project_dir = Path::new("./");
//!     let output = cargonode::build(project_dir, vec!["--verbose"]).await?;
//!     println!("Build output: {}", output);
//!     Ok(())
//! }
//! ```
//!
//! # Error Handling
//!
//! The module provides a comprehensive [`Error`] enum to handle various failure scenarios,
//! including command execution, configuration parsing, and pre-check resolution.

use async_recursion::async_recursion;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs, io, path::Path, result, time::Duration};

use crate::exec;

/// Represents errors that can occur during command execution
///
/// # Variants
///
/// - `Command`: Failure in executing a specific command
/// - `Config`: Issues with configuration loading or parsing
/// - `Io`: Standard input/output related errors
/// - `Toml`: Errors parsing TOML configuration
/// - `PreCheck`: Problems resolving pre-check commands
///
/// # Examples
///
/// ```
/// match result {
///     Err(Error::Command { command, error }) => {
///         eprintln!("Command {} failed: {}", command, error);
///     },
///     Err(Error::Config { message }) => {
///         eprintln!("Configuration problem: {}", message);
///     },
///     // ... handle other error types
///     Ok(_) => println!("Operation successful"),
/// }
/// ```
#[derive(Debug)]
pub enum Error {
    /// Indicates a failure in command execution
    Command {
        /// The command that failed
        command: String,
        /// Specific execution error
        error: exec::Error,
    },

    /// Represents configuration-related errors
    Config {
        /// Detailed error message
        message: String,
    },

    /// Wraps standard IO errors
    Io(io::Error),

    /// Represents TOML parsing errors
    Toml(toml::de::Error),

    /// Represents pre-check resolution errors
    PreCheck {
        /// The pre-check command that could not be resolved
        pre_check: String,
    },
}

/// A comprehensive result type for command operations
///
/// Simplifies error handling by using the custom [`Error`] type
///
/// # Examples
///
/// ```
/// fn example_operation() -> Result<String> {
///     // Operations that might fail
///     let result = some_fallible_operation()?;
///     Ok(result)
/// }
/// ```
type Result<T> = result::Result<T, Error>;

/// Configuration for individual commands
///
/// Defines how a specific command should be executed, including
/// arguments, pre-checks, environment variables, and timeout.
///
/// # Fields
///
/// - `command`: The executable command
/// - `args`: Command-line arguments
/// - `pre_checks`: Commands to run before the main command
/// - `env_vars`: Environment variables to set
/// - `timeout_secs`: Maximum execution time
///
/// # Examples
///
/// ```
/// let config = CommandConfig {
///     command: "biome".to_string(),
///     args: vec!["format".to_string()],
///     pre_checks: vec!["check".to_string()],
///     env_vars: HashMap::new(),
///     timeout_secs: Some(60),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    /// The command to execute (e.g., "biome", "tsup")
    command: String,
    /// Arguments to pass to the command
    args: Vec<String>,
    /// Commands to run before this command
    #[serde(default)]
    pre_checks: Vec<String>,
    /// Environment variables for this command
    #[serde(default)]
    env_vars: HashMap<String, String>,
    /// Command-specific timeout in seconds
    #[serde(default)]
    timeout_secs: Option<u64>,
}

/// Global configuration structure for command management
///
/// Provides a centralized way to configure command behaviors
/// and set global defaults.
///
/// # Fields
///
/// - `commands`: Mapping of command names to their configurations
/// - `default_timeout_secs`: Global timeout for all commands
///
/// # Configuration Precedence
/// 1. Command-specific configuration
/// 2. Global default timeout
/// 3. Command default timeout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Map of command names to their configurations
    commands: HashMap<String, CommandConfig>,
    /// Default timeout for all commands
    #[serde(default)]
    default_timeout_secs: Option<u64>,
}

/// Represents available commands in the system
///
/// Provides a type-safe enum of supported commands with
/// built-in default configurations.
///
/// # Variants
///
/// - `Format`: Code formatting using Biome
/// - `Check`: Code checking using Biome
/// - `Build`: Project build using tsup
/// - `Test`: Test execution using Vitest
/// - `Release`: Release creation using release-it
/// - `Custom`: User-defined custom command
///
/// # Examples
///
/// ```
/// let command = Command::Build;
/// let config = command.get_default_config();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Format code using Biome
    Format,
    /// Run code checks using Biome
    Check,
    /// Build the project using tsup
    Build,
    /// Run tests using vitest
    Test,
    /// Create a release using release-it
    Release,
    /// Custom command defined in configuration
    #[allow(dead_code)]
    Custom(String),
}

impl fmt::Display for Error {
    /// Formats the error for human-readable display
    ///
    /// # Examples
    ///
    /// ```
    /// let error = Error::Command {
    ///     command: "build".to_string(),
    ///     error: exec::Error::Timeout
    /// };
    /// println!("{}", error);  // Prints a readable error message
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Command { command, error } => {
                write!(f, "Failed to execute {}: {}", command, error)
            }
            Error::Config { message } => write!(f, "Configuration error: {}", message),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Toml(err) => write!(f, "TOML parsing error: {}", err),
            Error::PreCheck { pre_check } => {
                write!(f, "Unable to resolve pre-check command: {}", pre_check)
            }
        }
    }
}

impl From<io::Error> for Error {
    /// Converts a standard IO error into the custom Error type
    ///
    /// # Examples
    ///
    /// ```
    /// fn read_file() -> Result<()> {
    ///     let file = fs::File::open("nonexistent.txt")?;  // Automatically converts
    ///     Ok(())
    /// }
    /// ```
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    /// Converts a TOML deserialization error into the custom Error type
    ///
    /// # Examples
    ///
    /// ```
    /// let toml_str = "invalid toml content";
    /// let config: Result<Config> = toml::from_str(toml_str).map_err(Error::from);
    /// ```
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl Config {
    /// Merges command configuration with default settings
    ///
    /// Provides a flexible configuration resolution strategy:
    /// 1. Prioritize file-specific configurations
    /// 2. Fall back to default configurations
    ///
    /// # Arguments
    ///
    /// * `default_config` - The default configuration for the command
    /// * `command_name` - Name of the command being configured
    ///
    /// # Returns
    ///
    /// A merged `CommandConfig` with resolved settings
    ///
    /// # Examples
    ///
    /// ```
    /// // Typically used internally during command resolution
    /// let merged_config = config.merge_command_config(&default_config, "build");
    /// ```
    fn merge_command_config(
        &self,
        default_config: &CommandConfig,
        command_name: &str,
    ) -> CommandConfig {
        // Retrieve file config if exists
        let file_config = self.commands.get(command_name);

        CommandConfig {
            // Prioritize file config, fallback to default
            command: file_config
                .map(|fc| fc.command.clone())
                .unwrap_or_else(|| default_config.command.clone()),

            // Args: replace entirely from file config or use default
            args: file_config
                .map(|fc| fc.args.clone())
                .unwrap_or_else(|| default_config.args.clone()),

            // Pre-checks: prefer file config, use default if empty
            pre_checks: file_config
                .map(|fc| fc.pre_checks.clone())
                .filter(|pc| !pc.is_empty())
                .unwrap_or_else(|| default_config.pre_checks.clone()),

            // Environment variables: merge with file config taking precedence
            env_vars: {
                let mut merged_vars = default_config.env_vars.clone();
                if let Some(file_config) = file_config {
                    merged_vars.extend(file_config.env_vars.clone());
                }
                merged_vars
            },

            // Timeout: prioritize file config, then global default, then command default
            timeout_secs: file_config
                .and_then(|fc| fc.timeout_secs)
                .or(self.default_timeout_secs)
                .or(default_config.timeout_secs),
        }
    }
}

/// Lazy-loaded global configuration
///
/// Handles configuration loading with the following strategy:
/// 1. Attempt to read 'cargonode.toml'
/// 2. Return default configuration if file not found
/// 3. Handle other IO errors
///
/// # Configuration Loading
///
/// - Uses `once_cell::sync::Lazy` for thread-safe, one-time loading
/// - Supports graceful fallback to default configuration
static CONFIG: Lazy<Result<Config>> = Lazy::new(|| {
    match fs::read_to_string("cargonode.toml") {
        Ok(config_content) => {
            // Successfully read config file
            toml::from_str(&config_content).map_err(Error::from)
        }
        Err(io_err) if io_err.kind() == io::ErrorKind::NotFound => {
            // Config file not found, return default config
            Ok(Config {
                commands: HashMap::new(),
                default_timeout_secs: None,
            })
        }
        Err(other_err) => {
            // Other IO errors
            Err(Error::Io(other_err))
        }
    }
});

impl Command {
    /// Converts a pre-check string to an appropriate Command
    ///
    /// Provides a safe mapping from pre-check strings to known commands
    ///
    /// # Arguments
    ///
    /// * `pre_check` - A string representing a pre-check command
    ///
    /// # Returns
    ///
    /// An optional `Command` if a matching pre-check is found
    ///
    /// # Examples
    ///
    /// ```
    /// let cmd = Command::from_pre_check("build");  // Returns Some(Command::Build)
    /// let custom = Command::from_pre_check("custom");  // Returns None
    /// ```
    fn from_pre_check(pre_check: &str) -> Option<Self> {
        match pre_check {
            "check" => Some(Command::Check),
            "build" => Some(Command::Build),
            "format" => Some(Command::Format),
            "test" => Some(Command::Test),
            // Fallback for custom commands
            _ => None,
        }
    }

    /// Returns the default configuration for built-in commands
    ///
    /// Provides predefined configurations for standard commands
    ///
    /// # Returns
    ///
    /// A `CommandConfig` with default settings for the specific command
    ///
    /// # Examples
    ///
    /// ```
    /// let build_config = Command::Build.get_default_config();
    /// assert_eq!(build_config.command, "tsup");
    /// ```
    fn get_default_config(&self) -> CommandConfig {
        match self {
            Command::Format => CommandConfig {
                command: "biome".into(),
                args: vec!["format".into()],
                pre_checks: vec![],
                env_vars: HashMap::new(),
                timeout_secs: Some(60),
            },
            Command::Check => CommandConfig {
                command: "biome".into(),
                args: vec!["check".into()],
                pre_checks: vec![],
                env_vars: HashMap::new(),
                timeout_secs: Some(60),
            },
            Command::Build => CommandConfig {
                command: "tsup".into(),
                args: vec![],
                pre_checks: vec!["check".into()],
                env_vars: HashMap::new(),
                timeout_secs: Some(300),
            },
            Command::Test => CommandConfig {
                command: "vitest".into(),
                args: vec!["run".into()],
                pre_checks: vec!["check".into()],
                env_vars: HashMap::new(),
                timeout_secs: Some(300),
            },
            Command::Release => CommandConfig {
                command: "release-it".into(),
                args: vec![],
                pre_checks: vec!["build".into()],
                env_vars: HashMap::new(),
                timeout_secs: Some(600),
            },
            Command::Custom(cmd) => CommandConfig {
                command: cmd.clone(),
                args: vec![],
                pre_checks: vec![],
                env_vars: HashMap::new(),
                timeout_secs: None,
            },
        }
    }
}

impl fmt::Display for Command {
    /// Converts a Command to its string representation
    ///
    /// Provides a consistent string mapping for commands
    ///
    /// # Examples
    ///
    /// ```
    /// let cmd = Command::Build;
    /// assert_eq!(cmd.to_string(), "build");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Format => write!(f, "format"),
            Command::Check => write!(f, "check"),
            Command::Build => write!(f, "build"),
            Command::Test => write!(f, "test"),
            Command::Release => write!(f, "release"),
            Command::Custom(cmd) => write!(f, "{}", cmd),
        }
    }
}

/// Resolves a command to its configuration
///
/// # Arguments
///
/// * `command` - The command to resolve configuration for
///
/// # Returns
///
/// A `Result` containing the resolved `CommandConfig`
///
/// # Errors
///
/// Returns a `Config` if the global configuration cannot be loaded
///
/// # Examples
///
/// ```
/// let config = resolve_command(&Command::Build)?;
/// ```
fn resolve_command(command: &Command) -> Result<CommandConfig> {
    let config = CONFIG.as_ref().map_err(|e| Error::Config {
        message: format!("Failed to load config: {}", e),
    })?;

    let cmd_str = command.to_string();
    Ok(config.merge_command_config(&command.get_default_config(), &cmd_str))
}

/// Executes a command with pre-checks and configuration
///
/// # Arguments
///
/// * `work_dir` - The working directory for command execution
/// * `command` - The command to execute
/// * `extra_args` - Additional arguments to pass to the command
///
/// # Returns
///
/// A `Result` containing the command output as a string
///
/// # Errors
///
/// - `PreCheck` if a pre-check command cannot be resolved
/// - `Command` for execution failures
/// - `Config` for configuration loading issues
///
/// # Examples
///
/// ```
/// let output = execute(
///     Path::new("./project"),
///     Command::Build,
///     vec!["--verbose"]
/// ).await?;
/// ```
#[async_recursion]
async fn execute(work_dir: &Path, command: Command, extra_args: Vec<String>) -> Result<String> {
    let config = resolve_command(&command)?;

    // Execute pre-checks with proper Command resolution
    for pre_check in &config.pre_checks {
        match Command::from_pre_check(pre_check) {
            Some(pre_check_command) => {
                // Execute the mapped pre-check command
                Box::pin(execute(work_dir, pre_check_command, vec![])).await?;
            }
            None => {
                // Fallback to custom command execution if no direct mapping
                return Err(Error::PreCheck {
                    pre_check: pre_check.clone(),
                });
            }
        }
    }

    // Prepare arguments
    let mut full_args = vec![config.command.clone()];
    full_args.extend(config.args.clone());
    full_args.extend(extra_args);

    // Convert environment variables
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

    // Execute command with timeout handling
    let result = tokio::time::timeout(
        Duration::from_secs(config.timeout_secs.unwrap_or(300)),
        exec::npx(work_dir, full_args, env_vars),
    )
    .await
    .map_err(|_| Error::Command {
        command: config.command.clone(),
        error: exec::Error::Timeout,
    })?
    .map_err(|e| Error::Command {
        command: config.command.clone(),
        error: e,
    })?;

    Ok(result)
}

/// Runs the format command
///
/// # Arguments
///
/// * `work_dir` - The working directory for command execution
/// * `extra_args` - Additional arguments to pass to the formatter
///
/// # Examples
///
/// ```
/// let output = format(Path::new("./"), vec!["--check"]).await?;
/// ```
pub async fn format(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    execute(work_dir, Command::Format, extra_args).await
}

/// Runs the code check command
///
/// # Arguments
///
/// * `work_dir` - The working directory for command execution
/// * `extra_args` - Additional arguments to pass to the checker
///
/// # Examples
///
/// ```
/// let output = check(Path::new("./"), vec!["--verbose"]).await?;
/// ```
pub async fn check(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    execute(work_dir, Command::Check, extra_args).await
}

/// Runs the build command
///
/// # Arguments
///
/// * `work_dir` - The working directory for command execution
/// * `extra_args` - Additional arguments to pass to the build tool
///
/// # Examples
///
/// ```
/// let output = build(Path::new("./"), vec!["--production"]).await?;
/// ```
pub async fn build(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    execute(work_dir, Command::Build, extra_args).await
}

/// Runs the test command
///
/// # Arguments
///
/// * `work_dir` - The working directory for command execution
/// * `extra_args` - Additional arguments to pass to the test runner
///
/// # Examples
///
/// ```
/// let output = test(Path::new("./"), vec!["--watch"]).await?;
/// ```
pub async fn test(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    execute(work_dir, Command::Test, extra_args).await
}

/// Runs the release command
///
/// # Arguments
///
/// * `work_dir` - The working directory for command execution
/// * `extra_args` - Additional arguments to pass to the release tool
///
/// # Examples
///
/// ```
/// let output = release(Path::new("./"), vec!["--dry-run"]).await?;
/// ```
pub async fn release(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    execute(work_dir, Command::Release, extra_args).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Helper function to create a mock Config
    fn create_mock_config() -> Config {
        Config {
            commands: HashMap::from([(
                "format".into(),
                CommandConfig {
                    command: "custom-biome".into(),
                    args: vec!["custom-format".into()],
                    pre_checks: vec!["pre-format-check".into()],
                    env_vars: HashMap::from([("FORMAT_VAR".into(), "custom".into())]),
                    timeout_secs: Some(120),
                },
            )]),
            default_timeout_secs: Some(180),
        }
    }

    #[test]
    fn test_merge_command_config_full_override() {
        let default_config = Command::Format.get_default_config();
        let mock_config = create_mock_config();

        let merged_config = mock_config.merge_command_config(&default_config, "format");

        assert_eq!(merged_config.command, "custom-biome");
        assert_eq!(merged_config.args, vec!["custom-format"]);
        assert_eq!(merged_config.pre_checks, vec!["pre-format-check"]);
        assert_eq!(merged_config.timeout_secs, Some(120));

        // Verify env vars merge
        assert!(merged_config.env_vars.contains_key("FORMAT_VAR"));
    }

    #[test]
    fn test_merge_command_config_partial_override() {
        let default_config = Command::Build.get_default_config();
        let partial_config = Config {
            commands: HashMap::from([(
                "build".into(),
                CommandConfig {
                    command: "custom-tsup".into(),
                    args: vec![],
                    pre_checks: vec![],
                    env_vars: HashMap::new(),
                    timeout_secs: None,
                },
            )]),
            default_timeout_secs: Some(400),
        };

        let merged_config = partial_config.merge_command_config(&default_config, "build");

        assert_eq!(merged_config.command, "custom-tsup");
        assert_eq!(merged_config.args, vec![] as Vec<String>);
        assert_eq!(merged_config.pre_checks, vec!["check"]);
        assert_eq!(merged_config.timeout_secs, Some(400));
    }

    #[test]
    fn test_resolve_command_with_custom_config() {
        // This would require mocking the CONFIG lazy static
        // Simulating config resolution for a specific command
        let format_command = Command::Format;
        let config = create_mock_config();

        let merged_config = config.merge_command_config(
            &format_command.get_default_config(),
            &format_command.to_string(),
        );

        assert_eq!(merged_config.command, "custom-biome");
        assert_eq!(merged_config.timeout_secs, Some(120));
    }

    #[test]
    fn test_environment_variable_merging() {
        let default_config = CommandConfig {
            command: "default-cmd".into(),
            args: vec![],
            pre_checks: vec![],
            env_vars: HashMap::from([("DEFAULT_VAR".into(), "default".into())]),
            timeout_secs: None,
        };

        let config = Config {
            commands: HashMap::from([(
                "test".into(),
                CommandConfig {
                    command: "test-cmd".into(),
                    args: vec![],
                    pre_checks: vec![],
                    env_vars: HashMap::from([("TEST_VAR".into(), "custom".into())]),
                    timeout_secs: None,
                },
            )]),
            default_timeout_secs: None,
        };

        let merged_config = config.merge_command_config(&default_config, "test");

        assert_eq!(merged_config.env_vars.len(), 2);
        assert_eq!(
            merged_config.env_vars.get("DEFAULT_VAR"),
            Some(&"default".into())
        );
        assert_eq!(
            merged_config.env_vars.get("TEST_VAR"),
            Some(&"custom".into())
        );
    }

    #[test]
    fn test_pre_check_command_mapping() {
        assert_eq!(Command::from_pre_check("check"), Some(Command::Check));
        assert_eq!(Command::from_pre_check("build"), Some(Command::Build));
        assert_eq!(Command::from_pre_check("unknown"), None);
    }

    #[test]
    fn test_release_pre_checks() {
        let release_config = Command::Release.get_default_config();
        assert_eq!(release_config.pre_checks, vec!["build"]);
    }

    #[test]
    fn test_default_command_pre_checks() {
        // Test that Build command has Check as pre-check
        let build_config = Command::Build.get_default_config();
        assert_eq!(build_config.pre_checks, vec!["check"]);

        // Test that Test command has Check as pre-check
        let test_config = Command::Test.get_default_config();
        assert_eq!(test_config.pre_checks, vec!["check"]);

        // Test that Release command has Build as pre-check
        let release_config = Command::Release.get_default_config();
        assert_eq!(release_config.pre_checks, vec!["build"]);
    }
}
