//! Asynchronous Command Execution Module
//!
//! # Overview
//!
//! A robust, flexible system for executing shell commands with advanced error handling,
//! logging, and configuration management.
//!
//! # Features
//!
//! - Asynchronous command execution
//! - Detailed error reporting
//! - Environment variable injection
//! - Logging of executed commands
//! - Configurable working directory
//!
//! # Examples
//!
//! ```rust
//! let result = npx(
//!     Path::new("./project"),
//!     vec!["eslint".to_string(), "--fix".to_string()],
//!     Some(vec![("NODE_ENV".to_string(), "development".to_string())])
//! ).await?;
//! ```

use std::{
    fmt, io,
    path::{Path, PathBuf},
    string::FromUtf8Error,
};
use tokio::process::Command;

/// Represents potential errors during command execution
///
/// Provides comprehensive error variants to capture different
/// failure scenarios in command execution.
///
/// # Variants
///
/// - `FailedToExecute`: Command could not be launched
/// - `FailedToReadOutput`: Unable to decode command output
/// - `ExitFailure`: Command exited with non-zero status
/// - `Timeout`: Command exceeded allowed execution time
///
/// # Examples
///
/// ```
/// match result {
///     Err(Error::ExitFailure { stdout, stderr, .. }) => {
///         eprintln!("Command failed with output: {}", stdout);
///     },
///     Err(Error::FailedToExecute(io_err)) => {
///         eprintln!("Could not launch command: {}", io_err);
///     },
///     // ... handle other error cases
/// }
/// ```
#[derive(Debug)]
pub enum Error {
    /// Represents a failure to launch the command
    FailedToExecute(io::Error),

    /// Indicates an issue decoding command output
    FailedToReadOutput(FromUtf8Error),

    /// Represents a command that exited with a non-zero status
    ExitFailure {
        /// Standard output captured from the command
        stdout: String,
        /// Standard error captured from the command
        stderr: String,
        /// Exit status code, if available
        exit_status: Option<i32>,
    },

    /// Indicates the command exceeded its allowed execution time
    Timeout,
}

impl fmt::Display for Error {
    /// Formats the error for human-readable display
    ///
    /// Provides a detailed, context-rich error representation
    ///
    /// # Examples
    ///
    /// ```
    /// let error = Error::ExitFailure {
    ///     stdout: "some output".to_string(),
    ///     stderr: "error details".to_string(),
    ///     exit_status: Some(1)
    /// };
    /// println!("{}", error);  // Prints detailed error information
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FailedToExecute(err) => write!(f, "Failed to execute command: {}", err),
            Error::FailedToReadOutput(err) => write!(f, "Failed to read command output: {}", err),
            Error::ExitFailure {
                stdout,
                stderr,
                exit_status,
            } => {
                write!(f, "Command failed")?;
                if let Some(code) = exit_status {
                    write!(f, " with status: {}", code)?;
                }
                write!(f, "\nstdout:\n{}\nstderr:\n{}", stdout, stderr)
            }
            Error::Timeout => write!(f, "Command timed out"),
        }
    }
}

/// Configuration for command execution
///
/// Provides a comprehensive configuration structure for
/// defining how a command should be executed.
///
/// # Fields
///
/// - `work_dir`: Working directory for command execution
/// - `program`: Command/executable to run
/// - `args`: Command-line arguments
/// - `env_vars`: Optional environment variables
///
/// # Examples
///
/// ```
/// let config = Config {
///     work_dir: PathBuf::from("./project"),
///     program: "npm",
///     args: vec!["install".to_string()],
///     env_vars: Some(vec![
///         ("NODE_ENV".to_string(), "production".to_string())
///     ]),
/// };
/// ```
#[derive(Clone)]
pub struct Config {
    /// Working directory for command execution
    pub work_dir: PathBuf,
    /// Program or executable to run
    pub program: &'static str,
    /// Command-line arguments
    pub args: Vec<String>,
    /// Optional environment variables to set for the command
    pub env_vars: Option<Vec<(String, String)>>,
}

/// Executes a command based on the provided configuration
///
/// # Arguments
///
/// * `config` - Configuration specifying how to run the command
///
/// # Returns
///
/// A `Result` containing the command's standard output as a string
///
/// # Errors
///
/// - `FailedToExecute` if the command cannot be launched
/// - `FailedToReadOutput` if the output cannot be decoded
/// - `ExitFailure` if the command exits with a non-zero status
///
/// # Examples
///
/// ```rust
/// let result = run(&config).await?;
/// println!("Command output: {}", result);
/// ```
pub async fn run(config: &Config) -> Result<String, Error> {
    // Log the command being executed
    log(config);

    // Prepare the command
    let mut command = Command::new(config.program);
    command.current_dir(&config.work_dir);
    command.args(&config.args);

    // Set environment variables if provided
    if let Some(env_vars) = &config.env_vars {
        for (key, value) in env_vars {
            command.env(key, value);
        }
    }

    // Execute the command
    let output = command.output().await.map_err(Error::FailedToExecute)?;

    // Process the command output
    if output.status.success() {
        String::from_utf8(output.stdout).map_err(Error::FailedToReadOutput)
    } else {
        let stdout = String::from_utf8(output.stdout).map_err(Error::FailedToReadOutput)?;
        let stderr = String::from_utf8(output.stderr).map_err(Error::FailedToReadOutput)?;

        Err(Error::ExitFailure {
            stdout,
            stderr,
            exit_status: output.status.code(),
        })
    }
}

/// Logs the command being executed
///
/// Provides a simple logging mechanism to print the command
/// being run to the console.
///
/// # Arguments
///
/// * `config` - Configuration of the command to be logged
///
/// # Examples
///
/// ```
/// // Automatically called by run(), but can be used manually
/// log(&config);  // Prints "Executing: npm install"
/// ```
fn log(config: &Config) {
    if config.args.is_empty() {
        println!("Executing: {}", config.program);
    } else {
        let cmd_string = format!("{} {}", config.program, config.args.join(" "));
        println!("Executing: {}", cmd_string);
    }
}

/// Convenience function to execute NPX commands
///
/// Provides a simplified interface for running NPX-based commands
///
/// # Arguments
///
/// * `work_dir` - Working directory for the command
/// * `args` - Command-line arguments for NPX
/// * `env_vars` - Optional environment variables
///
/// # Returns
///
/// A `Result` containing the command's standard output
///
/// # Examples
///
/// ```rust
/// let output = npx(
///     Path::new("./project"),
///     vec!["tsup".to_string()],
///     None
/// ).await?;
/// ```
pub async fn npx<P: AsRef<Path>>(
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
    .await
}
