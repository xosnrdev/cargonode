use std::{fmt, path::PathBuf};

use crate::cargo_node::exec;

pub enum Error {
    BiomeError(exec::Error, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BiomeError(err, operation) => write!(f, "Failed to {}: {}", operation, err),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

fn run_biome_command(work_dir: PathBuf, command: &str, extra_args: Vec<String>) -> Result<String> {
    let mut args = vec!["biome".to_string(), command.to_string()];
    args.extend(extra_args);
    exec::npx(work_dir, args).map_err(|err| Error::BiomeError(err, command.to_string()))
}

pub fn format(work_dir: PathBuf, extra_args: Vec<String>) -> Result<String> {
    run_biome_command(work_dir, "format", extra_args)
}

pub fn check(work_dir: PathBuf, extra_args: Vec<String>) -> Result<String> {
    run_biome_command(work_dir, "check", extra_args)
}
