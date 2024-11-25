use std::{fmt, path::Path, result};

use crate::cargo_node::exec;

use super::biome;

pub enum Error {
    VitestError(exec::Error),
    BiomeError(biome::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VitestError(err) => write!(f, "Failed to execute vitest: {}", err),
            Error::BiomeError(err) => write!(f, "{}", err),
        }
    }
}

type Result<T> = result::Result<T, Error>;

pub fn test(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    let mut args = vec!["vitest".to_string(), "run".to_string()];
    args.extend(extra_args);
    biome::check(work_dir, Vec::new()).map_err(Error::BiomeError)?;
    exec::npx(work_dir, args).map_err(Error::VitestError)
}
