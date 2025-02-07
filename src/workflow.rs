use std::{
    collections::{hash_map, HashMap},
    path::PathBuf,
};

use anyhow::Context;
use clap::{ArgGroup, Args};

use crate::{cmd::CommandContext, config::Config, error::AppResult, job::Job};

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
    #[arg(long, value_name = "WORKING DIRECTORY", global = true)]
    pub working_dir: Option<PathBuf>,

    /// Extra jobs to run before the main job.
    #[arg(
        long,
        value_name = "STEPS",
        num_args = 0..,
        global = true
    )]
    pub steps: Option<Vec<Job>>,
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
