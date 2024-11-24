use std::{fmt, path::PathBuf, result};

use crate::cargo_node::exec;

pub enum Error {
    TsupError(exec::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TsupError(err) => write!(f, "Failed to execute tsup: {}", err),
        }
    }
}

type Result<T> = result::Result<T, Error>;

pub fn build(work_dir: PathBuf, extra_args: Vec<String>) -> Result<String> {
    let mut args = vec!["tsup".to_string()];
    args.extend(extra_args);
    exec::npx(work_dir, args).map_err(Error::TsupError)
}
