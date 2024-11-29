use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs, path::Path, result};

use crate::exec;

#[derive(Debug)]
pub enum Error {
    Execution {
        command: String,
        source: exec::Error,
    },
    Toml(toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Execution { command, source } => {
                write!(f, "Failed to execute command '{}': {}", command, source)
            }
            Error::Toml(err) => write!(f, "Failed to parse TOML configuration: {}", err),
        }
    }
}

#[derive(Debug)]
enum Command {
    Format,
    Check,
    Build,
    Test,
    Release,
}

impl fmt::Display for Command {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
struct CommandConfig {
    command: String,
    args: Vec<String>,
    #[serde(default)]
    pre_checks: Vec<String>,
    #[serde(default)]
    env_vars: HashMap<String, String>,
}

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
    /// Parse a command from its string representation.
    fn from_str(cmd: &str) -> Option<Self> {
        match cmd {
            "format" => Some(Self::Format),
            "check" => Some(Self::Check),
            "build" => Some(Self::Build),
            "test" => Some(Self::Test),
            "release" => Some(Self::Release),
            _ => None,
        }
    }

    fn default_config(&self) -> CommandConfig {
        match self {
            Self::Format => command_config!("biome", ["format"], [""]),
            Self::Check => command_config!("biome", ["check"], [""]),
            Self::Build => command_config!("tsup", [""], ["check"]),
            Self::Test => command_config!("vitest", [""], ["check"]),
            Self::Release => command_config!("release-it", [""], ["build"]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    #[serde(default)]
    commands: HashMap<String, CommandConfig>,
}

impl Config {
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

    fn merge(&self, default: &CommandConfig, command_name: &str) -> CommandConfig {
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

type Result<T> = result::Result<T, Error>;

static CONFIG: Lazy<Config> = Lazy::new(Config::load);

fn execute(work_dir: &Path, command: Command, extra_args: Vec<String>) -> Result<String> {
    let config = CONFIG.merge(&command.default_config(), &command.to_string());

    // Execute pre-checks in order.
    for pre_check in &config.pre_checks {
        Command::from_str(pre_check)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn default_config() -> CommandConfig {
        CommandConfig {
            command: "biome".to_string(),
            args: vec!["check".to_string()],
            pre_checks: vec!["validate".to_string()],
            env_vars: HashMap::from([("ENV".to_string(), "production".to_string())]),
        }
    }

    #[test]
    fn test_command_parsing() {
        let valid_commands = ["format", "check", "build", "test", "release"];
        for cmd in valid_commands {
            assert!(Command::from_str(cmd).is_some(), "Failed to parse: {}", cmd);
        }
        assert!(
            Command::from_str("invalid").is_none(),
            "Invalid command parsed unexpectedly"
        );
    }

    #[test]
    fn test_merge_with_full_file_config() {
        let mut config = Config::default();
        config.commands.insert(
            "build".to_string(),
            CommandConfig {
                command: "custom_command".to_string(),
                args: vec!["custom_arg".to_string()],
                pre_checks: vec!["custom_check".to_string()],
                env_vars: HashMap::from([
                    ("LOG_LEVEL".to_string(), "debug".to_string()),
                    // Overrides default
                    ("ENV".to_string(), "development".to_string()),
                ]),
            },
        );

        let default = default_config();
        let merged = config.merge(&default, "build");

        assert_eq!(merged.command, "custom_command");
        assert_eq!(merged.args, vec!["custom_arg"]);
        assert_eq!(merged.pre_checks, vec!["custom_check"]);
        assert_eq!(
            merged.env_vars,
            HashMap::from([
                ("LOG_LEVEL".to_string(), "debug".to_string()),
                ("ENV".to_string(), "development".to_string()),
            ])
        );
    }

    #[test]
    fn test_merge_with_partial_file_config() {
        let mut config = Config::default();
        config.commands.insert(
            "test".to_string(),
            CommandConfig {
                // Should fallback to default
                command: "".to_string(),
                // Override
                args: vec!["test_arg".to_string()],
                // Fallback to default
                pre_checks: vec![],
                env_vars: HashMap::from([("LOG_LEVEL".to_string(), "debug".to_string())]),
            },
        );

        let default = default_config();
        let merged = config.merge(&default, "test");

        assert_eq!(merged.command, "biome");
        assert_eq!(merged.args, vec!["test_arg"]);
        assert_eq!(merged.pre_checks, vec!["validate"]);
        assert_eq!(
            merged.env_vars,
            HashMap::from([
                // Default retained
                ("ENV".to_string(), "production".to_string()),
                // New added
                ("LOG_LEVEL".to_string(), "debug".to_string()),
            ])
        );
    }

    #[test]
    fn test_merge_with_no_file_config() {
        let config = Config::default();
        let default = default_config();
        let merged = config.merge(&default, "nonexistent");

        assert_eq!(merged, default, "Should default to default_config");
    }

    #[test]
    fn test_merge_with_empty_file_config() {
        let mut config = Config::default();
        config
            .commands
            .insert("build".to_string(), CommandConfig::default());

        let default = default_config();
        let merged = config.merge(&default, "build");

        assert_eq!(
            merged, default,
            "Should fallback entirely to default_config"
        );
    }

    #[test]
    fn test_environment_variable_merging() {
        let default_config = CommandConfig {
            command: "default-cmd".to_string(),
            args: vec![],
            pre_checks: vec![],
            env_vars: HashMap::from([("DEFAULT_VAR".to_string(), "default".to_string())]),
        };

        let mut config = Config::default();
        config.commands.insert(
            "test".to_string(),
            CommandConfig {
                command: "test-cmd".to_string(),
                env_vars: HashMap::from([("TEST_VAR".to_string(), "custom".to_string())]),
                ..Default::default()
            },
        );

        let merged_config = config.merge(&default_config, "test");

        assert_eq!(
            merged_config.env_vars,
            HashMap::from([
                ("DEFAULT_VAR".to_string(), "default".to_string()),
                ("TEST_VAR".to_string(), "custom".to_string()),
            ])
        );
    }

    #[test]
    fn test_default_command_configurations() {
        let default_tests = [
            (Command::Format, "biome", vec!["format"]),
            (Command::Check, "biome", vec!["check"]),
            (Command::Build, "tsup", vec![""]),
            (Command::Test, "vitest", vec![""]),
            (Command::Release, "release-it", vec![""]),
        ];

        for (cmd, expected_cmd, expected_args) in default_tests {
            let default_config = cmd.default_config();
            assert_eq!(default_config.command, expected_cmd);
            assert_eq!(default_config.args, expected_args);
        }
    }

    #[test]
    fn test_pre_check_configurations() {
        let pre_check_tests = [
            (Command::Build, vec!["check"]),
            (Command::Test, vec!["check"]),
            (Command::Release, vec!["build"]),
        ];

        for (cmd, expected_checks) in pre_check_tests {
            let config = cmd.default_config();
            assert_eq!(config.pre_checks, expected_checks);
        }
    }

    #[test]
    fn test_merge_with_empty_or_whitespace_vectors() {
        let default = CommandConfig {
            command: "biome".to_string(),
            args: vec!["check".to_string()],
            pre_checks: vec!["validate".to_string()],
            env_vars: HashMap::from([("ENV".to_string(), "production".to_string())]),
        };

        let mut config = Config::default();
        config.commands.insert(
            "test".to_string(),
            CommandConfig {
                // Should fallback to default
                command: "  ".to_string(),
                // Should fallback to default
                args: vec!["  ".to_string()],
                // Should fallback to default
                pre_checks: vec![],
                env_vars: HashMap::new(),
            },
        );

        let merged = config.merge(&default, "test");

        assert_eq!(merged.command, "biome");
        assert_eq!(merged.args, vec!["check"]);
        assert_eq!(merged.pre_checks, vec!["validate"]);
        assert_eq!(
            merged.env_vars,
            HashMap::from([("ENV".to_string(), "production".to_string())])
        );
    }

    #[test]
    fn test_merge_with_partial_whitespace_vectors() {
        let default = CommandConfig {
            command: "biome".to_string(),
            args: vec!["check".to_string()],
            pre_checks: vec!["validate".to_string()],
            env_vars: HashMap::from([("ENV".to_string(), "production".to_string())]),
        };

        let mut config = Config::default();
        config.commands.insert(
            "build".to_string(),
            CommandConfig {
                command: "custom-cmd".to_string(),
                // Mixed, only valid-arg retained
                args: vec!["valid-arg".to_string(), "  ".to_string()],
                // Mixed, only custom-check retained
                pre_checks: vec!["".to_string(), "custom-check".to_string()],
                env_vars: HashMap::new(),
            },
        );

        let merged = config.merge(&default, "build");

        assert_eq!(merged.command, "custom-cmd");
        assert_eq!(merged.args, vec!["valid-arg"]);
        assert_eq!(merged.pre_checks, vec!["custom-check"]);
        assert_eq!(
            merged.env_vars,
            HashMap::from([("ENV".to_string(), "production".to_string())])
        );
    }

    #[test]
    fn test_merge_with_empty_whitespace_and_non_empty_vectors() {
        let default = CommandConfig {
            command: "default-cmd".to_string(),
            args: vec!["default-arg".to_string()],
            pre_checks: vec!["default-check".to_string()],
            env_vars: HashMap::from([("DEFAULT_ENV".to_string(), "default".to_string())]),
        };

        let mut config = Config::default();
        config.commands.insert(
            "release".to_string(),
            CommandConfig {
                command: "release-cmd".to_string(),
                // Mixed
                args: vec!["   ".to_string(), "release-arg".to_string()],
                // Mixed
                pre_checks: vec!["release-check".to_string(), "  ".to_string()],
                env_vars: HashMap::new(),
            },
        );

        let merged = config.merge(&default, "release");

        assert_eq!(merged.command, "release-cmd");
        assert_eq!(merged.args, vec!["release-arg"]);
        assert_eq!(merged.pre_checks, vec!["release-check"]);
        assert_eq!(
            merged.env_vars,
            HashMap::from([("DEFAULT_ENV".to_string(), "default".to_string())])
        );
    }
}
