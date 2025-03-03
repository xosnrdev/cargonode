use std::{io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not create directory `{}`\n\nDirectory already contains files.\nTry using `cn init` to initialize the project in an existing directory.", path.display())]
    DirectoryNotEmpty { path: PathBuf },

    #[error("Destination `{}` is not a directory", path.display())]
    DirectoryExists { path: PathBuf },

    #[error("Invalid package name: {}\n\n{}", name, reason)]
    InvalidPackageName { name: String, reason: String },

    #[error("Destination `{}` already contains a package.json file\n\nUse `cn new` to create a new project in a different directory.", std::env::current_dir().unwrap_or_default().display())]
    PackageJsonExists,

    #[error("{}\n\n{}", message, details)]
    Git { message: String, details: String },

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("Failed to create package.json\n\n{0}\n\nEnsure you have write permissions in the current directory.")]
    PackageJsonCreation(String),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Input error: {message}")]
    Input { message: String },

    #[error("Cache error: {message}")]
    Cache { message: String },
}
