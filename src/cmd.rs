use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{AppResult, CliError},
    job::Job,
    shell,
};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, default)]
#[serde(rename_all = "kebab-case")]
pub struct CommandContext {
    pub executable: PathBuf,
    pub subcommand: String,
    pub args: Vec<String>,
    pub envs: HashMap<String, String>,
    pub working_dir: PathBuf,
    pub steps: Vec<Job>,
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
            self.args.extend(other.args);
        }
        self.envs.extend(other.envs);
        if !other.working_dir.as_os_str().is_empty() {
            self.working_dir = validate_working_dir(&other.working_dir)?;
        }
        if !other.steps.is_empty() {
            self.steps = other.steps;
        }
        Ok(())
    }
}

pub fn from_default(
    executable: &str,
    subcommand: impl Into<String>,
    args: &[&str],
    working_dir: Option<&str>,
    steps: Vec<Job>,
) -> CommandContext {
    CommandContext {
        executable: PathBuf::from(executable),
        subcommand: subcommand.into(),
        args: args.iter().map(|s| s.to_string()).collect(),
        working_dir: if let Some(dir) = working_dir {
            PathBuf::from(dir)
        } else {
            env::current_dir().expect("Failed to get current directory")
        },
        steps,
        ..Default::default()
    }
}

pub fn do_call(ctx: &CommandContext) -> Result<(), CliError> {
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
    use tempfile::TempDir;

    use super::*;

    fn create_temp_dir() -> TempDir {
        tempfile::tempdir().expect("Failed to create temporary directory")
    }

    #[test]
    fn test_from_default() {
        // Act
        let ctx = from_default("npx", "tsup", &["src/main.js"], None, vec![Job::Check]);
        // Assert
        assert_eq!(ctx.executable, PathBuf::from("npx"));
        assert_eq!(ctx.subcommand, "tsup");
        assert_eq!(ctx.args, vec!["src/main.js"]);
        assert_eq!(ctx.working_dir, env::current_dir().unwrap());
        assert_eq!(ctx.steps, vec![Job::Check]);
    }

    #[test]
    fn test_validate_working_dir_valid() {
        // Arrange
        let temp_dir = create_temp_dir();
        // Act
        let result = validate_working_dir(temp_dir.path());
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_validate_working_dir_invalid() {
        // Arrange
        let invalid_path = Path::new("nonexistent_dir");
        // Act
        let result = validate_working_dir(invalid_path);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_executable_valid() {
        // Act
        let result = validate_executable("cargo");
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_executable_invalid() {
        // Act
        let result = validate_executable("nonexistent_executable");
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_do_call_success() {
        // Arrange
        let ctx = CommandContext {
            executable: PathBuf::from("echo"),
            subcommand: "hello".to_string(),
            args: vec![],
            working_dir: PathBuf::from("."),
            ..Default::default()
        };
        // Act
        let result = do_call(&ctx);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_do_call_failure() {
        // Arrange
        let ctx = CommandContext {
            executable: PathBuf::from("false"),
            subcommand: "".to_string(),
            args: vec![],
            working_dir: PathBuf::from("."),
            ..Default::default()
        };
        // Act
        let result = do_call(&ctx);
        // Assert
        assert!(result.is_err());
    }
}
