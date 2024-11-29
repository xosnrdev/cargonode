//! Provides utilities for running shell commands and handling their output.

use std::{
    fmt, io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    string::FromUtf8Error,
};

/// Executes a command based on the given configuration and applies a transformation on the output.
macro_rules! exec_command {
    ($config:expr, $transform:expr) => {{
        log($config);

        let mut command = Command::new($config.program);
        command
            .current_dir(&$config.work_dir)
            .args(&$config.args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        if let Some(env_vars) = $config.env_vars.clone() {
            command.envs(env_vars);
        }

        let output = command.output().map_err(Error::Execute)?;

        match output.status.success() {
            true => $transform(output.stdout),
            false => {
                let stdout = String::from_utf8(output.stdout).map_err(Error::ReadOutput)?;
                let stderr = String::from_utf8(output.stderr).map_err(Error::ReadOutput)?;
                Err(Error::ExitFailure {
                    stdout,
                    stderr,
                    exit_status: Some(output.status),
                })
            }
        }
    }};
}

/// Represents errors that can occur during command execution.
#[derive(Debug)]
pub enum Error {
    /// Represents an error that occurs while executing a command.
    Execute(io::Error),
    /// Represents an error that occurs while reading command output.
    ReadOutput(FromUtf8Error),
    /// Represents a command failure with details on the stdout, stderr, and exit status.
    ExitFailure {
        stdout: String,
        stderr: String,
        exit_status: Option<ExitStatus>,
    },
}

impl fmt::Display for Error {
    /// Formats the error for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Execute(err) => write!(f, "Failed to execute command: {}", err),
            Error::ReadOutput(err) => write!(f, "Failed to read command output: {}", err),
            Error::ExitFailure {
                stdout,
                stderr,
                exit_status,
            } => {
                write!(f, "Command failed")?;
                if let Some(exit_status) = exit_status {
                    write!(f, " with exit status: {}", exit_status)?;
                }
                if !stdout.is_empty() {
                    write!(f, "\n\nstdout:\n{}", stdout)?;
                }
                if !stderr.is_empty() {
                    write!(f, "\n\nstderr:\n{}", stderr)?;
                }
                Ok(())
            }
        }
    }
}

/// Represents the configuration required to run a command.
#[derive(Clone)]
pub struct Config {
    /// Specifies the working directory for the command.
    pub work_dir: PathBuf,
    /// Specifies the program to execute.
    pub program: &'static str,
    /// Specifies the arguments to pass to the command.
    pub args: Vec<String>,
    /// Specifies optional environment variables for the command.
    pub env_vars: Option<Vec<(String, String)>>,
}

/// Runs a shell command using the provided configuration and returns its output.
pub fn run(config: &Config) -> Result<String, Error> {
    exec_command!(config, |stdout| {
        String::from_utf8(stdout).map_err(Error::ReadOutput)
    })
}

/// Logs the execution details of a command.
fn log(config: &Config) {
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    let cmd_string = if config.args.is_empty() {
        config.program.to_string()
    } else {
        format!("{} {}", config.program, config.args.join(" "))
    };
    println!("{}Executing:{} {}", GREEN, RESET, cmd_string);
}

/// Executes an `npx` command with the specified working directory, arguments, and environment variables.
pub fn npx<P: AsRef<Path>>(
    work_dir: P,
    args: Vec<String>,
    env_vars: Option<Vec<(String, String)>>,
) -> Result<String, Error> {
    run(&Config {
        work_dir: work_dir.as_ref().to_path_buf(),
        program: "npx",
        args,
        env_vars,
    })
}
