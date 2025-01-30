use std::{
    collections::{hash_map, HashMap},
    path::PathBuf,
};

use anyhow::Context;
use clap::{ArgGroup, Args, Subcommand};

use crate::{
    cmd::CommandContext, config::Config, error::AppResult, job::Job, pkgmgr::PackageManager,
};

#[derive(Debug, Subcommand)]
pub enum Workflow {
    /// Create a new project at the specified path.
    New {
        /// Name or path for the new project.
        #[arg(value_name = "NAME")]
        name: PathBuf,
        /// Package manager to use.
        #[arg(short, long, value_name = "PACKAGE MANAGER")]
        package_manager: Option<PackageManager>,
    },
    /// Initialize a project in the current directory.
    Init {
        /// Package manager to use.
        #[arg(short, long, value_name = "PACKAGE MANAGER")]
        package_manager: Option<PackageManager>,
    },
    /// Run a custom script or command.
    #[command(disable_help_flag = true, visible_alias = "r")]
    Run {
        /// Arguments for the runner.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Format code.
    #[command(disable_help_flag = true)]
    Fmt {
        /// Arguments for the formatter.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check code.
    #[command(disable_help_flag = true, visible_alias = "c")]
    Check {
        /// Arguments for the checker.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Build or bundle.
    #[command(disable_help_flag = true, visible_alias = "b")]
    Build {
        /// Arguments for the builder.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run tests.
    #[command(disable_help_flag = true, visible_alias = "t")]
    Test {
        /// Arguments for the test runner.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Release project.
    #[command(disable_help_flag = true)]
    Release {
        /// Arguments for the release command.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

#[derive(Debug, Args, Default)]
#[command(
    group(
        ArgGroup::new("config_source")
            .required(false)
            .multiple(false)
            .args(&["config_file"])
    ),
)]
pub struct WorkflowConfig {
    /// Path to a JSON config file.
    #[arg(
        short,
        long,
        value_name = "CONFIG FILE",
        global = true,
        group = "config_source"
    )]
    pub config_file: Option<PathBuf>,

    /// Override the configured executable.
    #[arg(short = 'x', long, value_name = "EXECUTABLE", global = true)]
    pub executable: Option<PathBuf>,

    /// Single argument to pass to the executable.
    #[arg(short, long, value_name = "SUBCOMMAND", global = true)]
    pub subcommand: Option<String>,

    /// Additional arguments passed to the executable.
    #[arg(
        short,
        long,
        value_name = "ARGS",
        allow_hyphen_values = true,
        global = true
    )]
    pub args: Option<Vec<String>>,

    /// Environment variables (KEY=VALUE).
    #[arg(
        short,
        long,
        value_name = "ENVS",
        num_args = 0..,
        global = true
    )]
    pub envs: Option<Vec<String>>,

    /// Working directory.
    #[arg(short = 'w', long, value_name = "WORKING DIRECTORY", global = true)]
    pub working_dir: Option<PathBuf>,

    /// Extra jobs to run before the main job.
    #[arg(
        long,
        value_name = "STEPS",
        num_args = 0..,
        global = true
    )]
    pub steps: Option<Vec<Job>>,

    /// Increase logging verbosity (-v, -vv, -vvv).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbosity: u8,
}

impl WorkflowConfig {
    pub fn from_args(self, job: &Job) -> AppResult<Config> {
        let mut config = Config::from_default();
        let mut ctx = CommandContext::default();

        if let Some(config_path) = self.config_file {
            let canonical_path = config_path
                .canonicalize()
                .with_context(|| format!("Invalid path `{}`", config_path.display()))?;
            let file_config = Config::with_file(&canonical_path)?;
            config.merge(file_config);
        }
        if let Some(executable) = self.executable {
            ctx.executable = executable;
        }
        if let Some(subcommand) = self.subcommand {
            ctx.subcommand = subcommand;
        }
        if let Some(args) = self.args {
            ctx.args = args;
        }
        if let Some(env_vars) = self.envs {
            let vars: Result<HashMap<String, String>, _> = env_vars
                .into_iter()
                .map(|var| {
                    let parts: Vec<&str> = var.splitn(2, '=').collect();
                    match parts.len() {
                        2 => Ok((parts[0].to_string(), parts[1].to_string())),
                        _ => Err(anyhow::format_err!(
                            "Invalid environment variable: '{}'",
                            var
                        )),
                    }
                })
                .collect();
            ctx.envs = vars?;
        }
        if let Some(working_dir) = self.working_dir {
            ctx.working_dir = working_dir;
        }
        if let Some(steps) = self.steps {
            ctx.steps = steps;
        }
        if self.verbosity > ctx.verbosity {
            ctx.verbosity = self.verbosity;
        }
        match config.cargonode.entry(*job) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().merge(ctx)?;
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(ctx);
            }
        }
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::job::Job;

    fn default_job() -> Job {
        Job::Build
    }

    // Test Workflow::New
    #[test]
    fn test_workflow_new() {
        // Arrange
        let workflow = Workflow::New {
            name: PathBuf::from("test_project"),
            package_manager: Some(PackageManager::Npm),
        };
        // Act
        match workflow {
            Workflow::New {
                name,
                package_manager,
            } => {
                // Assert
                assert_eq!(name, PathBuf::from("test_project"));
                assert_eq!(package_manager, Some(PackageManager::Npm));
            }
            _ => panic!("Unexpected workflow variant"),
        }
    }

    // Test Workflow::Init
    #[test]
    fn test_workflow_init() {
        // Arrange
        let workflow = Workflow::Init {
            package_manager: Some(PackageManager::Yarn),
        };
        // Act
        match workflow {
            Workflow::Init { package_manager } => {
                // Assert
                assert_eq!(package_manager, Some(PackageManager::Yarn));
            }
            _ => panic!("Unexpected workflow variant"),
        }
    }

    // Test Workflow::Run
    #[test]
    fn test_workflow_run() {
        // Arrange
        let workflow = Workflow::Run {
            args: vec!["arg1".to_string(), "arg2".to_string()],
        };
        // Act
        match workflow {
            Workflow::Run { args } => {
                // Assert
                assert_eq!(args, vec!["arg1".to_string(), "arg2".to_string()]);
            }
            _ => panic!("Unexpected workflow variant"),
        }
    }

    // Test WorkflowConfig::from_args with all fields populated
    #[test]
    fn test_workflow_config_from_args_full() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Arrange
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"{{"key": "value"}}"#).unwrap();
        let config = WorkflowConfig {
            config_file: Some(file.path().to_path_buf()),
            executable: Some(PathBuf::from("echo")),
            subcommand: Some("hello".to_string()),
            args: Some(vec!["arg1".to_string(), "arg2".to_string()]),
            envs: Some(vec!["KEY1=value1".to_string(), "KEY2=value2".to_string()]),
            working_dir: Some(PathBuf::from("/tmp")),
            steps: Some(vec![default_job()]),
            verbosity: 2,
        };
        // Act
        let job = default_job();
        let result = config.from_args(&job);
        // Assert
        assert!(result.is_ok());
    }

    // Test WorkflowConfig::from_args with only required fields
    #[test]
    fn test_workflow_config_from_args_minimal() {
        // Arrange
        let config = WorkflowConfig {
            config_file: None,
            executable: None,
            subcommand: None,
            args: None,
            envs: None,
            working_dir: None,
            steps: None,
            verbosity: 0,
        };
        // Act
        let job = default_job();
        let result = config.from_args(&job);
        // Assert
        assert!(result.is_ok());
    }

    // Test WorkflowConfig::from_args with invalid config file path
    #[test]
    fn test_workflow_config_from_args_invalid_config_file() {
        // Arrange
        let config = WorkflowConfig {
            config_file: Some(PathBuf::from("nonexistent_config.json")),
            ..Default::default()
        };
        // Act
        let job = default_job();
        let result = config.from_args(&job);
        // Assert
        assert!(result.is_err());
    }

    // Test WorkflowConfig::from_args with malformed environment variables
    #[test]
    fn test_workflow_config_from_args_malformed_envs() {
        // Arrange
        let config = WorkflowConfig {
            envs: Some(vec!["MALFORMED_ENV".to_string()]),
            ..Default::default()
        };
        // Act
        let job = default_job();
        let result = config.from_args(&job);
        // Assert
        assert!(result.is_err());
    }

    // Test WorkflowConfig::from_args with verbosity
    #[test]
    fn test_workflow_config_from_args_verbosity() {
        // Arrange
        let config = WorkflowConfig {
            verbosity: 3,
            ..Default::default()
        };
        // Act
        let job = default_job();
        let result = config.from_args(&job);
        // Assert
        assert!(result.is_ok());
    }

    // Test WorkflowConfig::from_args with steps
    #[test]
    fn test_workflow_config_from_args_steps() {
        // Arrange
        let config = WorkflowConfig {
            steps: Some(vec![default_job()]),
            ..Default::default()
        };
        // Act
        let job = default_job();
        let result = config.from_args(&job);
        // Assert
        assert!(result.is_ok());
    }

    // Test WorkflowConfig::from_args with empty steps
    #[test]
    fn test_workflow_config_from_args_empty_steps() {
        // Arrange
        let config = WorkflowConfig {
            steps: Some(vec![]),
            ..Default::default()
        };
        // Act
        let job = default_job();
        let result = config.from_args(&job);
        // Assert
        assert!(result.is_ok());
    }
}
