use std::{fmt, path::PathBuf};

use crate::cargo_node::exec;

pub enum Error {
    FormatError(exec::Error),
    CheckError(exec::Error),
    FixError(exec::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FormatError(err) => write!(f, "Failed to format: {}", err),
            Error::CheckError(err) => write!(f, "Failed to check: {}", err),
            Error::FixError(err) => write!(f, "Failed to fix: {}", err),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn format(work_dir: PathBuf, biome_args: Vec<String>) -> Result<String> {
    let mut args = vec!["biome".to_string(), "format".to_string()];
    args.extend(biome_args);
    let config = exec::Config {
        work_dir,
        cmd: "npx".to_string(),
        args,
    };
    exec::run(&config).map_err(Error::FormatError)
}

pub fn check() -> Result<()> {
    unimplemented!()
}

pub fn fix() -> Result<()> {
    unimplemented!()
}
