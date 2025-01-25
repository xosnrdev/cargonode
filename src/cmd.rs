use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{AppResult, CliError},
    job::Job,
    shell,
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct CommandContext {
    pub executable: PathBuf,
    pub subcommand: String,
    pub args: Vec<String>,
    pub envs: HashMap<String, String>,
    pub working_dir: PathBuf,
    pub steps: Vec<Job>,
    pub verbosity: u8,
}

impl CommandContext {
    pub fn merge(&mut self, other: CommandContext) -> AppResult<()> {
        if !other.executable.as_os_str().is_empty() {
            self.executable = validate_executable(other.executable)?;
        }
        if !other.subcommand.is_empty() {
            self.subcommand = other.subcommand;
        }
        if !other.args.is_empty() {
            self.args = other.args;
        }
        self.envs.extend(other.envs);
        if !other.working_dir.as_os_str().is_empty() {
            self.working_dir = validate_working_dir(&other.working_dir)?;
        }
        if !other.steps.is_empty() {
            self.steps = other.steps;
        }
        if other.verbosity > self.verbosity {
            self.verbosity = other.verbosity;
        }
        Ok(())
    }
}

pub fn from_default(
    executable: &str,
    subcommand: impl Into<String>,
    args: &[&str],
    working_dir: &str,
    steps: &[&str],
) -> CommandContext {
    let steps = steps.iter().map(|s| Job::from_str(s).unwrap()).collect();
    CommandContext {
        executable: PathBuf::from(executable),
        subcommand: subcommand.into(),
        args: args.iter().map(|s| s.to_string()).collect(),
        working_dir: PathBuf::from(working_dir),
        steps,
        ..Default::default()
    }
}

pub(crate) fn do_call(ctx: &CommandContext) -> Result<(), CliError> {
    shell::status(
        "Running",
        format!(
            "{} {} {}",
            ctx.executable.display(),
            ctx.subcommand,
            ctx.args.join(" ")
        ),
    )?;
    let mut cmd = Command::new(&ctx.executable);
    cmd.arg(&ctx.subcommand)
        .args(&ctx.args)
        .envs(&ctx.envs)
        .current_dir(&ctx.working_dir);
    let mut child = cmd.spawn().map_err(CliError::from)?;
    let status = child.wait().map_err(CliError::from)?;
    if !status.success() {
        return Err(CliError::from(status.code().unwrap_or(1)));
    }
    Ok(())
}

fn validate_working_dir(path: &Path) -> AppResult<PathBuf> {
    let canonical_dir = path.canonicalize()?;
    if !canonical_dir.is_dir() {
        anyhow::bail!("The path {} is not a directory.", canonical_dir.display());
    }
    Ok(canonical_dir)
}

pub(crate) fn validate_executable<P: AsRef<Path>>(executable: P) -> AppResult<PathBuf> {
    match which::which(executable.as_ref()) {
        Ok(path) => Ok(path),
        Err(err) => Err(anyhow::format_err!(
            "Executable '{}' not found in PATH: {}",
            executable.as_ref().display(),
            err
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_validate_working_dir() {
        let dir = tempdir().unwrap();
        let result = validate_working_dir(dir.path());
        assert!(result.is_ok());

        let file = dir.path().join("file");
        fs::write(&file, "content").unwrap();
        let result = validate_working_dir(&file);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_executable() {
        let result = validate_executable("echo");
        assert!(result.is_ok());

        let result = validate_executable("unknown");
        assert!(result.is_err());
    }
}
