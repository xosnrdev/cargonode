use std::{io, path::PathBuf, process::ExitStatus};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot create directory: {path}\n\nThe directory already contains files.\n\nSuggestion: Use `cargonode init` to initialize the project in an existing directory, or choose a different empty directory.")]
    DirectoryNotEmpty { path: PathBuf },

    #[error("Invalid directory: {path}\n\nThe specified path exists but is not a directory.\n\nSuggestion: Please provide a valid directory path or create the directory first.")]
    DirectoryExists { path: PathBuf },

    #[error("Invalid package name: {name}\n\nError: {reason}\n\nSuggestion: Package names must be lowercase, can contain hyphens and underscores, and may be scoped (e.g., @scope/name).")]
    InvalidPackageName { name: String, reason: String },

    #[error("Package already exists\n\nA package.json file already exists in {}\n\nSuggestion: To create a new project, either:\n1. Use a different directory\n2. Remove the existing package.json\n3. Use `cargonode new` to create a new project in a different directory", std::env::current_dir().unwrap_or_default().display())]
    PackageJsonExists,

    #[error("Git operation failed\n\nError: {message}\n\nDetails: {details}\n\nSuggestion: Ensure you have git installed and have appropriate permissions.")]
    Git { message: String, details: String },

    #[error("File system error: {0}\n\nSuggestion: Check file permissions and ensure you have write access to the directory.")]
    Io(#[from] io::Error),

    #[error("JSON parsing error: {0}\n\nSuggestion: Verify that your package.json is valid JSON and contains all required fields.")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Failed to create package.json\n\nError: {0}\n\nSuggestion: Ensure you have write permissions in the current directory and that no other process is using the file.")]
    PackageJsonCreation(String),

    #[error("Configuration error\n\nError: {message}\n\nSuggestion: Check your package.json configuration and ensure all required fields are properly set.")]
    Config { message: String },

    #[error("Input error\n\nError: {message}\n\nSuggestion: Verify that all required input files exist and match the specified patterns.")]
    Input { message: String },

    #[error("Command failed: {command}\n\nStatus: {status}\n\nSuggestion: Try the following:\n1. Run the command manually to see detailed output\n2. Check if all required dependencies are installed\n3. Verify the command arguments are correct")]
    CommandFailed { command: String, status: ExitStatus },

    #[error("Output error\n\nError: {message}\n\nSuggestion: Check if you have write permissions and sufficient disk space in the output directory.")]
    Output { message: String },

    #[error("Output verification failed\n\nError: {message}\n\nSuggestion: {suggestion}")]
    OutputVerificationFailed { message: String, suggestion: String },
}
