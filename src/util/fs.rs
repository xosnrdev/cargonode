use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde_json::Value;

use super::platform;

/// Cache for filesystem checks
#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct FsCache {
    is_git_repo: Option<bool>,
}

impl FsCache {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a path is inside a Git repository
    ///
    /// # Errors
    /// - If the git command fails to execute
    /// - If the path does not exist
    /// - If there are permission issues
    pub fn is_git_repo(&mut self, path: &Path) -> Result<bool> {
        if let Some(cached) = self.is_git_repo {
            return Ok(cached);
        }
        let result = is_in_git_repo(path)?;
        self.is_git_repo = Some(result);
        Ok(result)
    }
}

/// Set executable permissions for binary files
///
/// # Errors
/// - If the file does not exist
/// - If there are insufficient permissions
/// - If the operation is not supported on the current platform
pub fn set_executable_permissions(path: &Path) -> Result<()> {
    platform::set_executable(path)
}

/// Ensure consistent line endings
///
/// # Errors
/// - If the file cannot be written to
/// - If there are insufficient permissions
/// - If the parent directory does not exist
pub fn write_with_line_endings(path: &Path, content: &str) -> Result<()> {
    let content = platform::normalize_line_endings(content);
    fs::write(path, content)?;
    Ok(())
}

/// Check if a directory is inside a Git repository
///
/// # Errors
/// - If the git command fails to execute
/// - If the path does not exist
/// - If there are permission issues
pub fn is_in_git_repo(path: &Path) -> Result<bool> {
    use std::process::Command;
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(path)
        .output()?;
    Ok(output.status.success())
}

/// Initialize a Git repository
///
/// # Errors
/// - If git is not installed
/// - If the directory already contains a git repository
/// - If there are insufficient permissions
/// - If the git command fails
pub fn init_git_repository(path: &Path) -> Result<()> {
    use std::process::Command;
    Command::new("git").arg("init").current_dir(path).output()?;
    Ok(())
}

/// Find the workspace root by looking for a package.json file with workspaces
#[must_use]
pub fn find_workspace_root(path: &Path) -> Option<PathBuf> {
    let mut current = path;
    while let Some(parent) = current.parent() {
        let package_json = current.join("package.json");
        if package_json.exists() {
            if let Ok(content) = fs::read_to_string(&package_json) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if json.get("workspaces").is_some() {
                        return Some(current.to_path_buf());
                    }
                }
            }
        }
        current = parent;
    }
    None
}

#[derive(Debug)]
pub struct PackageInfo {
    pub name: String,
}

/// Find all packages in a workspace
///
/// # Errors
/// - If the directory cannot be read
/// - If there are permission issues
/// - If package.json files are invalid
pub fn find_workspace_packages(root: &Path) -> Result<Vec<PackageInfo>> {
    let mut packages = Vec::new();
    let packages_dir = root.join("packages");

    if packages_dir.exists() {
        for entry in fs::read_dir(packages_dir)? {
            let entry = entry?;
            let pkg_json_path = entry.path().join("package.json");
            if pkg_json_path.exists() {
                if let Ok(content) = fs::read_to_string(&pkg_json_path) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                            packages.push(PackageInfo {
                                name: name.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(packages)
}

#[must_use]
pub fn get_package_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.replace(['-', ' '], "_"))
        .unwrap_or_else(|| "package".to_string())
}
