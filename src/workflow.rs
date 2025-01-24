use std::{
    collections::{hash_map, HashMap},
    path::PathBuf,
};

use clap::{ArgGroup, Args, Subcommand};

use crate::{
    cmd::{validate_executable, validate_working_dir, CommandContext},
    config::Config,
    error::AppResult,
    job::Job,
    pkgmgr::PackageManager,
};

#[derive(Debug, Subcommand)]
pub enum Workflow {
    /// Create a new project at the specified path.
    New {
        /// Name or path for the new project.
        #[arg(value_name = "NAME")]
        name: PathBuf,
        /// Package manager to use.
        #[arg(short, long, value_name = "PACKAGE MANAGER", default_value = "npm")]
        package_manager: Option<PackageManager>,
    },
    /// Initialize a project in the current directory.
    Init {
        /// Package manager to use.
        #[arg(short, long, value_name = "PACKAGE MANAGER", default_value = "npm")]
        package_manager: Option<PackageManager>,
    },
    /// Run a custom script or command.
    #[command(disable_help_flag = true, visible_alias = "r")]
    Run {
        /// Script or binary to run.
        #[arg(
            value_name = "SCRIPT",
            allow_hyphen_values = true,
            trailing_var_arg = true
        )]
        args: Vec<String>,
    },
    /// Format files.
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
    )
)]
pub struct WorkflowConfig {
    /// Path to a JSON config file.
    #[arg(
        short,
        long,
        value_name = "CONFIG FILE",
        global = true,
        group = "config_source",
        default_value = "package.json"
    )]
    pub config_file: Option<PathBuf>,

    /// Override the configured executable.
    #[arg(short = 'x', long, value_name = "EXECUTABLE")]
    pub executable: Option<PathBuf>,

    /// Single argument to pass to the executable.
    #[arg(short, long, value_name = "SUBCOMMAND")]
    pub subcommand: Option<String>,

    /// Additional arguments passed to the executable.
    #[arg(short, long, value_name = "ARGS", allow_hyphen_values = true)]
    pub args: Option<Vec<String>>,

    /// Environment variables (KEY=VALUE).
    #[arg(
        short,
        long,
        value_name = "ENVS",
        num_args = 0..,
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
            let canonical_path = config_path.canonicalize()?;
            let file_config = Config::with_file(&canonical_path)?;
            config.merge(file_config);
        }
        if let Some(executable) = self.executable {
            ctx.executable = validate_executable(executable)?;
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
            ctx.working_dir = validate_working_dir(&working_dir)?;
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
