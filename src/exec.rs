use std::{
    fmt, io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    string::FromUtf8Error,
};

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

#[derive(Debug)]
pub enum Error {
    Execute(io::Error),
    ReadOutput(FromUtf8Error),
    ExitFailure {
        stdout: String,
        stderr: String,
        exit_status: Option<ExitStatus>,
    },
}

impl fmt::Display for Error {
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

#[derive(Clone)]
pub struct Config {
    pub work_dir: PathBuf,
    pub program: &'static str,
    pub args: Vec<String>,
    pub env_vars: Option<Vec<(String, String)>>,
}

pub fn run(config: &Config) -> Result<String, Error> {
    exec_command!(config, |stdout| {
        String::from_utf8(stdout).map_err(Error::ReadOutput)
    })
}

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
