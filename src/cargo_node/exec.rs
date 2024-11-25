use std::{
    fmt, io,
    path::{Path, PathBuf},
    string::FromUtf8Error,
};
use tokio::process::Command;

#[derive(Debug)]
pub enum Error {
    FailedToExecute(io::Error),

    FailedToReadOutput(FromUtf8Error),

    ExitFailure {
        stdout: String,
        stderr: String,
        exit_status: Option<i32>,
    },

    Timeout,
}

impl fmt::Display for Error {
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

#[derive(Clone)]
pub struct Config {
    pub work_dir: PathBuf,
    pub program: &'static str,
    pub args: Vec<String>,
    pub env_vars: Option<Vec<(String, String)>>,
}

pub async fn run(config: &Config) -> Result<String, Error> {
    log(config);

    let mut command = Command::new(config.program);
    command.current_dir(&config.work_dir);
    command.args(&config.args);

    if let Some(env_vars) = &config.env_vars {
        for (key, value) in env_vars {
            command.env(key, value);
        }
    }

    let output = command.output().await.map_err(Error::FailedToExecute)?;

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

fn log(config: &Config) {
    if config.args.is_empty() {
        println!("Executing: {}", config.program);
    } else {
        let cmd_string = format!("{} {}", config.program, config.args.join(" "));
        println!("Executing: {}", cmd_string);
    }
}

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
