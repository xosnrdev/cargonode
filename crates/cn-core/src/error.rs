use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("❌ Project already exists at: {0}\nUse --force flag to override existing project.")]
    ProjectExists(String),

    #[error("❌ Invalid project name: {0}\nProject names must only contain alphanumeric characters, hyphens (-), and underscores (_).")]
    InvalidProjectName(String),

    #[error("❌ Invalid scope name: {0}\nScope names must only contain alphanumeric characters, hyphens (-), and underscores (_).")]
    InvalidScopeName(String),

    #[error("❌ Invalid package name: {0}\nPackage names must only contain alphanumeric characters, hyphens (-), and underscores (_).")]
    InvalidPackageName(String),

    #[error("❌ Reserved npm package name: {0}\nThis name is reserved by npm and cannot be used. Please choose a different name.")]
    ReservedPackageName(String),

    #[error("❌ Template error: {0}\nPlease check the template configuration and try again.")]
    Template(String),

    #[error("❌ Version control error: {0}\nPlease ensure git is installed and you have proper permissions.")]
    Vcs(String),

    #[error("❌ Permission denied: {0}\nPlease check your file system permissions and try again.")]
    Permission(String),

    #[error("❌ Invalid path: {0}\nThe specified path is invalid or contains illegal characters.")]
    InvalidPath(String),

    #[error("❌ Registry error: {0}\nPlease check your internet connection and try again.")]
    Registry(String),

    #[error("❌ Network error: {0}\nPlease check your internet connection and try again later.")]
    Network(String),

    #[error("❌ Workspace error: {0}\nPlease check your workspace configuration.")]
    Workspace(String),
}

pub type Result<T> = std::result::Result<T, Error>;
