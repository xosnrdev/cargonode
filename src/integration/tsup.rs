use std::{fmt, path::Path, result};

use crate::cargo_node::exec;

use super::biome;

pub enum Error {
    TsupError(exec::Error),
    BiomeError(biome::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TsupError(err) => write!(f, "Failed to execute tsup: {}", err),
            Error::BiomeError(err) => write!(f, "{}", err),
        }
    }
}

type Result<T> = result::Result<T, Error>;

pub fn build(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    let mut args = vec!["tsup".to_string()];
    args.extend(extra_args);
    biome::check(work_dir, Vec::new()).map_err(Error::BiomeError)?;
    exec::npx(work_dir, args).map_err(Error::TsupError)
}
