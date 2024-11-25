use std::{fmt, path::Path, result};

use crate::cargo_node::exec;

use super::tsup;

pub enum Error {
    ReleaseItError(exec::Error),
    TsupError(tsup::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReleaseItError(err) => write!(f, "{}", err),
            Error::TsupError(err) => write!(f, "{}", err),
        }
    }
}

type Result<T> = result::Result<T, Error>;

pub fn release(work_dir: &Path, extra_args: Vec<String>) -> Result<String> {
    let mut args = vec!["release-it".to_string()];
    args.extend(extra_args.clone());
    tsup::build(work_dir, Vec::new()).map_err(Error::TsupError)?;
    exec::npx(work_dir, args).map_err(Error::ReleaseItError)
}
