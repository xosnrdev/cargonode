use anyhow::Context;
use clap::{ArgGroup, Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::{
    cmd,
    config::Config,
    error::{AppResult, CliError},
    WorkflowSteps,
};

#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about,
    visible_alias = "cn",
    propagate_version = true,
    styles = clap_cargo::style::CLAP_STYLING,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub workflow: Option<Workflow>,

    #[command(flatten)]
    pub workflow_config: WorkflowConfig,
}

impl Cli {
    pub fn run(self) -> Result<(), CliError> {
        let config = self.workflow_config.from_args()?;
        let mut builder = get_logging(*config.get_global_scope().get_verbose());
        builder.init();

        #[allow(unused_variables)]
        match self.workflow {
            Some(Workflow::New { name }) => cmd::project::with_name(name),
            Some(Workflow::Init) => unimplemented!(),
            Some(Workflow::Run { executable }) => unimplemented!(),
            Some(Workflow::Fmt { args }) => unimplemented!(),
            Some(Workflow::Check { args }) => unimplemented!(),
            Some(Workflow::Build { args }) => unimplemented!(),
            Some(Workflow::Test { args }) => unimplemented!(),
            Some(Workflow::Release { args }) => unimplemented!(),
            None => cmd::do_call(&config),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Workflow {
    /// Create a new project at the specified path.
    New {
        /// Name or path for the new project.
        #[arg(value_name = "NAME")]
        name: PathBuf,
    },
    /// Initialize a project in the current directory.
    Init,
    /// Run a custom script or command (default: dist/main.cjs).
    #[command(disable_help_flag = true, visible_alias = "r")]
    Run {
        /// Script or binary to run.
        #[arg(
            default_value = "dist/main.cjs",
            value_name = "EXECUTABLE",
            allow_hyphen_values = true,
            trailing_var_arg = true
        )]
        executable: Option<PathBuf>,
    },
    /// Format files (default: biome format).
    #[command(disable_help_flag = true)]
    Fmt {
        /// Arguments for the formatter.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Check code (default: biome check).
    #[command(disable_help_flag = true, visible_alias = "c")]
    Check {
        /// Arguments for the checker.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Build or bundle (default: tsup).
    #[command(disable_help_flag = true, visible_alias = "b")]
    Build {
        /// Arguments for the builder.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Run tests (default: vitest).
    #[command(disable_help_flag = true, visible_alias = "t")]
    Test {
        /// Arguments for the test runner.
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Release project (default: release-it).
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
    )
)]
pub struct WorkflowConfig {
    /// Path to a JSON config file (default: package.json).
    #[arg(
        short,
        long,
        value_name = "CONFIG FILE",
        global = true,
        group = "config_source",
        //default_value = "package.json"
        //default_value = "package.json"
    )]
    pub config_file: Option<PathBuf>,

    /// Override the configured executable.
    #[arg(short = 'x', long, value_name = "EXECUTABLE")]
    pub executable: Option<PathBuf>,

    /// Additional arguments passed to the executable.
    #[arg(short, long, value_name = "ARGS", allow_hyphen_values = true)]
    pub args: Option<Vec<String>>,

    /// Environment variables (KEY=VALUE).
    #[arg(
        short,
        long,
        value_name = "ENV_VARS",
        num_args = 0..,
    )]
    pub env_vars: Option<Vec<String>>,

    /// Working directory (default: .).
    #[arg(
        short = 'w',
        long,
        value_name = "WORKING DIRECTORY",
        global = true,
        default_value = "."
    )]
    pub working_dir: PathBuf,

    /// Extra steps to run before the main executable.
    #[arg(
        long,
        value_name = "STEPS",
        num_args = 0..,
        global = true
    )]
    pub workflow_step: Option<Vec<WorkflowSteps>>,

    /// Time limit in seconds (default: 60).
    #[arg(
        short,
        long,
        value_name = "SECONDS",
        global = true,
        default_value = "60"
    )]
    pub timeout: u64,

    /// Increase logging verbosity (use multiple times for more detail).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

impl WorkflowConfig {
    pub fn from_args(self) -> AppResult<Config> {
        let mut config = Config::default();

        if let Some(config_path) = self.config_file {
            let canonical_path = config_path.canonicalize().with_context(|| {
                format!(
                    "Failed to canonicalize config file path: {}",
                    config_path.display()
                )
            })?;

            let file_config = Config::from_file(&canonical_path)?;
            config.merge(file_config)?;
        }

        *config.get_global_scope_mut().get_executable_mut() = self.executable;

        if let Some(args) = self.args {
            *config.get_global_scope_mut().get_args_mut() = args;
        }

        if let Some(env_vars) = self.env_vars {
            for var in env_vars {
                let parts: Vec<&str> = var.splitn(2, '=').collect();
                if parts.len() != 2 {
                    anyhow::bail!(
                        "Invalid environment variable: '{}'. Must be KEY=VALUE.",
                        var
                    );
                }
                config
                    .get_global_scope_mut()
                    .get_env_vars_mut()
                    .insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        *config.get_global_scope_mut().get_working_dir_mut() = self.working_dir;

        if let Some(steps) = self.workflow_step {
            *config.get_global_scope_mut().get_workflow_steps_mut() = steps;
        }

        *config.get_global_scope_mut().get_timeout_mut() = self.timeout;

        if self.verbose > *config.get_global_scope().get_verbose() {
            *config.get_global_scope_mut().get_verbose_mut() = self.verbose;
        }

        config.validate()?;
        Ok(config)
    }
}

fn get_logging(verbosity: u8) -> env_logger::Builder {
    use log::LevelFilter;

    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let mut builder = env_logger::Builder::new();
    builder.filter(None, level);
    builder.format_module_path(false);

    if level == LevelFilter::Trace || level == LevelFilter::Debug {
        builder.format_timestamp_secs();
    } else {
        builder.format_target(false);
        builder.format_timestamp(None);
    }
    builder
}

#[cfg(test)]
mod tests {
    use crate::MAX_TIMEOUT;

    use super::*;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_cli_parse_new() {
        let args = ["cn", "new", "my_project"];
        let cli = Cli::parse_from(args);
        match cli.workflow {
            Some(Workflow::New { name }) => assert_eq!(name, PathBuf::from("my_project")),
            _ => panic!("invalid workflow"),
        }
    }

    #[test]
    fn test_cli_parse_init() {
        let args = ["cn", "init"];
        let cli = Cli::parse_from(args);
        match cli.workflow {
            Some(Workflow::Init) => {}
            _ => panic!("invalid workflow"),
        }
    }

    #[test]
    fn test_cli_parse_none() {
        let args = ["cn"];
        let cli = Cli::parse_from(args);
        assert!(cli.workflow.is_none());
    }

    #[test]
    fn test_config_with_file_not_found() {
        let config = WorkflowConfig {
            config_file: Some(PathBuf::from("no_such_file.json")),
            ..Default::default()
        };
        let result = config.from_args();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .to_string()
            .contains("Failed to canonicalize config file path: no_such_file.json"));
    }

    #[test]
    fn test_config_with_too_high_timeout() {
        let config = WorkflowConfig {
            executable: Some(PathBuf::from("test")),
            timeout: MAX_TIMEOUT + 100,
            ..Default::default()
        };
        let result = config.from_args();
        let err = result.unwrap_err();
        dbg!(&err);
        assert!(err.to_string().contains(
            "Failed to validate global scope: Timeout 160 exceeds the maximum of 60 seconds."
        ));
    }

    #[test]
    fn test_config_env_var_format_error() {
        let config = WorkflowConfig {
            env_vars: Some(vec!["NOTVALID".into()]),
            ..Default::default()
        };
        let result = config.from_args();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid environment variable"));
    }
}
