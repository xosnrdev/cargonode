use std::{fmt, result};

use crate::cargo_node::exec;

pub enum Error {
    VitestError(exec::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VitestError(err) => write!(f, "Failed to execute vitest: {}", err),
        }
    }
}

type Result<T> = result::Result<T, Error>;

pub fn test(work_dir: std::path::PathBuf, extra_args: Vec<String>) -> Result<String> {
    let mut args = vec!["vitest".to_string(), "run".to_string()];
    args.extend(extra_args);
    exec::npx(work_dir, args).map_err(Error::VitestError)
}
