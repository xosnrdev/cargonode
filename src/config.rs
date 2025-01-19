use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;
use getset::{Getters, MutGetters};
use log::{trace, warn};
use serde::{Deserialize, Serialize};
use which::which;

use crate::error::AppResult;

pub mod source;

pub const MAX_TIMEOUT: u64 = 60;
const MAX_ARGS_LEN: usize = 1024;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigKey {
    cargonode: Config,
}

#[derive(Debug, Default, Serialize, Deserialize, Getters, MutGetters, PartialEq)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    global_scope: StepCommand,
    local_scope: HashMap<WorkflowSteps, StepCommand>,
}

impl Config {
    pub fn merge(&mut self, other: Config) -> AppResult<()> {
        trace!("Merging configurations");
        self.global_scope.merge(other.global_scope)?;
        for (k, v) in other.local_scope {
            self.local_scope.insert(k, v);
        }
        Ok(())
    }

    pub fn validate(&self) -> AppResult<()> {
        self.global_scope
            .validate()
            .map_err(|err| anyhow::format_err!("Failed to validate global scope: {}", err))?;
        for (step, cmd) in &self.local_scope {
            cmd.validate()
                .with_context(|| format!("Failed to validate local scope: {}", step))?;
        }
        Ok(())
    }

    pub fn from_file(path: &Path) -> AppResult<Config> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open configuration file: {:?}", path))?;
        let reader = BufReader::new(file);
        Self::from_reader(reader).with_context(|| {
            format!(
                "Failed while reading and parsing configuration file: {:?}",
                path
            )
        })
    }

    pub fn from_reader<R: Read>(mut reader: R) -> AppResult<Config> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let config_key: ConfigKey = serde_json::from_str(&contents)?;
        config_key.cargonode.validate()?;
        Ok(config_key.cargonode)
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, MutGetters, PartialEq)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct StepCommand {
    executable: Option<PathBuf>,
    args: Vec<String>,
    env_vars: HashMap<String, String>,
    working_dir: PathBuf,
    workflow_steps: Vec<WorkflowSteps>,
    timeout: u64,
    verbose: u8,
}

impl Default for StepCommand {
    fn default() -> Self {
        StepCommand {
            executable: None,
            args: Vec::new(),
            env_vars: HashMap::new(),
            working_dir: PathBuf::new(),
            workflow_steps: Vec::new(),
            timeout: 30,
            verbose: 0,
        }
    }
}

impl StepCommand {
    pub fn merge(&mut self, other: StepCommand) -> AppResult<()> {
        self.validate()?;
        if other.executable.is_some() {
            self.executable = other.executable;
        }
        if !other.args.is_empty() {
            self.args = other.args;
        }
        if !other.env_vars.is_empty() {
            self.env_vars = other.env_vars;
        }
        if !other.working_dir.as_os_str().is_empty() {
            self.working_dir = other.working_dir.canonicalize().with_context(|| {
                format!(
                    "Failed to canonicalize working directory: {}",
                    other.working_dir.display()
                )
            })?;
        }
        if !other.workflow_steps.is_empty() {
            self.workflow_steps = other.workflow_steps;
        }
        if other.timeout > 0 && other.timeout <= MAX_TIMEOUT {
            self.timeout = other.timeout;
        } else if other.timeout > MAX_TIMEOUT {
            warn!(
                "Timeout {} exceeds the maximum of {} seconds. Using default {}.",
                other.timeout, MAX_TIMEOUT, MAX_TIMEOUT
            );
            self.timeout = MAX_TIMEOUT;
        }
        if other.verbose > self.verbose {
            self.verbose = other.verbose;
        }
        Ok(())
    }

    pub fn validate(&self) -> AppResult<()> {
        if let Some(ref binary_name) = self.executable {
            if which(binary_name).is_err() {
                anyhow::bail!("Executable '{}' not found in PATH.", binary_name.display());
            }
        }
        if self.args.len() > MAX_ARGS_LEN {
            anyhow::bail!(
                "Number of arguments {} exceeds maximum allowed {}.",
                self.args.len(),
                MAX_ARGS_LEN
            );
        }
        if self.env_vars.len() > MAX_ARGS_LEN {
            anyhow::bail!(
                "Number of environment variables {} exceeds maximum allowed {}.",
                self.env_vars.len(),
                MAX_ARGS_LEN
            );
        }
        if self.timeout > MAX_TIMEOUT {
            anyhow::bail!(
                "Timeout {} exceeds the maximum of {} seconds.",
                self.timeout,
                MAX_TIMEOUT
            );
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, clap::ValueEnum, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowSteps {
    Build,
    Check,
    Fmt,
    Release,
    Run,
    Test,
}

impl fmt::Display for WorkflowSteps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowSteps::Build => write!(f, "build"),
            WorkflowSteps::Check => write!(f, "check"),
            WorkflowSteps::Fmt => write!(f, "fmt"),
            WorkflowSteps::Release => write!(f, "release"),
            WorkflowSteps::Run => write!(f, "run"),
            WorkflowSteps::Test => write!(f, "test"),
        }
    }
}

macro_rules! default_cfg {
    ($executable:literal, $args:expr, $working_dir:expr, $workflow_steps:expr) => {{
        let args: &[&str] = $args;
        let workflow_steps: &[&str] = $workflow_steps;
        let working_dir: &str = $working_dir;
        StepCommand {
            executable: Some(PathBuf::from($executable)),
            args: args.iter().map(ToString::to_string).collect(),
            working_dir: PathBuf::from(working_dir),
            workflow_steps: workflow_steps
                .iter()
                .map(|s| WorkflowSteps::from_str(s))
                .collect(),
            ..Default::default()
        }
    }};
}

impl WorkflowSteps {
    pub fn from_default() -> HashMap<WorkflowSteps, StepCommand> {
        let mut local_scope = HashMap::with_capacity(6);
        local_scope.insert(
            WorkflowSteps::Build,
            default_cfg!("tsup", &[""], "src", &["check"]),
        );
        local_scope.insert(
            WorkflowSteps::Check,
            default_cfg!("biome", &["check"], "", &[]),
        );
        local_scope.insert(
            WorkflowSteps::Fmt,
            default_cfg!("biome", &["format"], "", &[]),
        );
        local_scope.insert(
            WorkflowSteps::Release,
            default_cfg!("release-it", &[], "", &["build"]),
        );
        local_scope.insert(
            WorkflowSteps::Run,
            default_cfg!("node", &[], "dist", &["build"]),
        );
        local_scope.insert(
            WorkflowSteps::Test,
            default_cfg!("vitest", &[""], "", &["check"]),
        );
        local_scope
    }

    fn from_str(s: &str) -> WorkflowSteps {
        match s {
            "build" => WorkflowSteps::Build,
            "check" => WorkflowSteps::Check,
            "fmt" => WorkflowSteps::Fmt,
            "release" => WorkflowSteps::Release,
            "run" => WorkflowSteps::Run,
            "test" => WorkflowSteps::Test,
            _ => unreachable!("Invalid workflow step: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_stepcommand_merge() {
        let mut cmd1 = StepCommand::default();
        cmd1.executable = Some(PathBuf::from("echo"));
        cmd1.args = vec!["run".into()];
        cmd1.timeout = 30;
        cmd1.verbose = 1;

        let mut cmd2 = StepCommand::default();
        cmd2.executable = Some(PathBuf::from("true"));
        cmd2.args = vec!["install".into()];
        cmd2.timeout = 60;
        cmd2.verbose = 2;
        cmd2.env_vars.insert("KEY".into(), "VALUE".into());

        cmd1.merge(cmd2).expect("Failed to merge StepCommands");
        assert_eq!(cmd1.executable, Some(PathBuf::from("true")));
        assert_eq!(cmd1.args, vec!["install"]);
        assert_eq!(cmd1.timeout, 60);
        assert_eq!(cmd1.verbose, 2);
        assert_eq!(cmd1.env_vars.get("KEY").unwrap(), "VALUE");
    }

    #[test]
    fn test_stepcommand_merge_maintains_fields() {
        let mut cmd1 = StepCommand::default();
        cmd1.executable = Some(PathBuf::from("echo"));
        cmd1.args = vec!["check".into()];
        cmd1.timeout = 15;

        let mut cmd2 = StepCommand::default();
        cmd2.working_dir = PathBuf::from(".");

        cmd1.merge(cmd2).expect("Failed to merge StepCommands");
        assert_eq!(cmd1.executable, Some(PathBuf::from("echo")));
        assert_eq!(cmd1.args, vec!["check"]);
        assert_eq!(cmd1.timeout, 30);
        assert_eq!(
            cmd1.working_dir,
            PathBuf::from(".")
                .canonicalize()
                .expect("Failed to canonicalize working directory")
        );
    }

    #[test]
    fn test_stepcommand_validation() {
        let mut cmd = StepCommand::default();
        cmd.executable = Some(PathBuf::from("true"));
        cmd.working_dir = PathBuf::from(".");
        cmd.timeout = 10;
        cmd.validate().unwrap();

        cmd.timeout = MAX_TIMEOUT + 1;
        let err = cmd.validate().unwrap_err();
        assert!(err
            .to_string()
            .contains("Timeout 61 exceeds the maximum of 60 seconds"));
    }

    #[test]
    fn test_config_merge_and_validation() {
        let mut cfg1 = Config::default();
        cfg1.global_scope = StepCommand {
            executable: Some(PathBuf::from("true")),
            timeout: 30,
            ..Default::default()
        };

        let mut cfg2 = Config::default();
        cfg2.global_scope = StepCommand {
            executable: Some(PathBuf::from("true")),
            timeout: 60,
            ..Default::default()
        };
        cfg2.local_scope.insert(
            WorkflowSteps::Build,
            StepCommand {
                executable: Some(PathBuf::from("echo")),
                ..Default::default()
            },
        );

        cfg1.merge(cfg2).expect("Failed to merge configurations");
        assert_eq!(cfg1.global_scope.executable, Some(PathBuf::from("true")));
        assert_eq!(cfg1.global_scope.timeout, 60);
        let sub = cfg1.local_scope.get(&WorkflowSteps::Build).unwrap();
        assert_eq!(sub.executable, Some(PathBuf::from("echo")));

        cfg1.validate().unwrap();
    }
}
