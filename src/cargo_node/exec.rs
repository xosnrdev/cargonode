use std::{
    fmt::{self, Display, Formatter},
    io,
    path::PathBuf,
    process::Command,
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum Error {
    FailedToExecute(io::Error),
    FailedToReadOutput(FromUtf8Error),
    ExitFailure {
        stdout: String,
        stderr: String,
        exit_status: Option<i32>,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::FailedToExecute(err) => write!(f, "Failed to execute command: {err}"),
            Self::FailedToReadOutput(err) => write!(f, "Failed to read command output: {err}"),
            Self::ExitFailure {
                stdout,
                stderr,
                exit_status,
            } => {
                writeln!(
                    f,
                    "Command failed{}",
                    exit_status
                        .map(|code| format!(" with status: {code}"))
                        .unwrap_or_default()
                )?;

                if !stdout.is_empty() {
                    write!(f, "\n[stdout]\n{stdout}\n")?;
                }
                if !stderr.is_empty() {
                    write!(f, "\n[stderr]\n{stderr}\n")?;
                }
                Ok(())
            }
        }
    }
}

pub struct Config {
    pub work_dir: PathBuf,
    pub cmd: String,
    pub args: Vec<String>,
}

pub fn run(config: &Config) -> Result<String, Error> {
    log(config);

    let output = Command::new(&config.cmd)
        .current_dir(&config.work_dir)
        .args(&config.args)
        .output()
        .map_err(Error::FailedToExecute)?;

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
        println!("Executing: {}", config.cmd);
    } else {
        let cmd_string = format!("{} {}", config.cmd, config.args.join(" "));
        println!("Executing: {}", cmd_string);
    }
}

pub fn npx<T: From<Error>>(work_dir: PathBuf, args: Vec<String>) -> Result<String, T> {
    run(&Config {
        work_dir,
        cmd: "npx".to_string(),
        args,
    })
    .map_err(From::from)
}
