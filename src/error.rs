use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid package name '{name}'\n\nCause: {reason}\n\nPackage names must:\n  - contain only lowercase letters, numbers, and hyphens/dots\n  - start with a letter\n  - not contain consecutive dots or hyphens\n  - not end with a dot or hyphen")]
    InvalidPackageName { name: String, reason: String },

    #[error("Reserved package name '{name}'\n\nCause: {reason}\n\nSuggestion: {suggestion}")]
    ReservedPackageName {
        name: String,
        reason: String,
        suggestion: String,
    },

    #[error("Directory already exists: {path}\n\nUse a different name or remove the existing directory.")]
    DirectoryExists { path: PathBuf },

    #[error("Directory not empty: {path}\n\nThe target directory `{path}` already contains files.\nUse `cn init` to initialize a project in an existing directory\nor choose a different directory name.")]
    DirectoryNotEmpty { path: PathBuf },

    #[error("Git error: {message}\n\nCause: {details}")]
    Git { message: String, details: String },

    #[error("package.json already exists in the current directory\n\nUse `cn new` to create a project in a new directory\nor remove the existing package.json file.")]
    PackageJsonExists,

    #[error("Failed to create package.json: {0}\n\nMake sure you have write permissions in the current directory.")]
    PackageJsonCreation(String),
}
