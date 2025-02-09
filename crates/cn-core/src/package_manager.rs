//! Package manager detection and workspace functionality
//!
//! This module provides utilities for detecting and working with different
//! Node.js package managers (npm, yarn) and workspace configurations.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::{Error, Result};

/// Supported package managers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    /// npm (default)
    Npm,
    /// Yarn Classic (v1)
    Yarn,
    /// Yarn Berry (v2+)
    YarnBerry,
    /// pnpm
    Pnpm,
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::Npm
    }
}

/// Workspace configuration from package.json
#[derive(Debug, Deserialize)]
struct WorkspaceConfig {
    workspaces: Option<WorkspaceSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum WorkspaceSpec {
    /// Array of workspace glob patterns
    Patterns(Vec<String>),
    /// Yarn workspace configuration object
    YarnConfig {
        packages: Vec<String>,
        #[serde(default)]
        #[allow(dead_code)]
        nohoist: Vec<String>,
    },
}

/// Detects the package manager used in a directory
///
/// # Arguments
/// * `dir` - Directory to check
///
/// # Returns
/// The detected package manager or default (npm)
pub fn detect_package_manager(dir: &Path) -> PackageManager {
    if dir.join("yarn.lock").exists() {
        if dir.join(".yarnrc.yml").exists() {
            PackageManager::YarnBerry
        } else {
            PackageManager::Yarn
        }
    } else if dir.join("pnpm-lock.yaml").exists() {
        PackageManager::Pnpm
    } else {
        PackageManager::Npm
    }
}

/// Checks if a directory is part of a workspace
///
/// # Arguments
/// * `dir` - Directory to check
///
/// # Returns
/// Some(root_dir) if in a workspace, None if not
pub fn find_workspace_root(dir: &Path) -> Option<PathBuf> {
    let mut current = dir.to_path_buf();
    while let Some(parent) = current.parent() {
        let package_json = current.join("package.json");
        if package_json.exists() {
            if let Ok(content) = fs::read_to_string(&package_json) {
                if let Ok(config) = serde_json::from_str::<WorkspaceConfig>(&content) {
                    if config.workspaces.is_some() {
                        return Some(current);
                    }
                }
            }
        }
        current = parent.to_path_buf();
    }
    None
}

/// Gets workspace package locations based on workspace patterns
///
/// # Arguments
/// * `root` - Workspace root directory
///
/// # Returns
/// List of workspace package directories
pub fn get_workspace_packages(root: &Path) -> Result<Vec<PathBuf>> {
    let package_json = root.join("package.json");
    let content = fs::read_to_string(&package_json).map_err(Error::Io)?;

    let config: WorkspaceConfig = serde_json::from_str(&content).map_err(Error::Json)?;

    let patterns = match config.workspaces {
        Some(WorkspaceSpec::Patterns(patterns)) => patterns,
        Some(WorkspaceSpec::YarnConfig { packages, .. }) => packages,
        None => return Ok(vec![]),
    };

    let mut result = Vec::new();
    for pattern in patterns {
        let glob_pattern = root.join(pattern);
        for entry in glob::glob(glob_pattern.to_str().unwrap())
            .map_err(|e| Error::Template(format!("Invalid glob pattern: {}", e)))?
        {
            match entry {
                Ok(path) => {
                    if path.join("package.json").exists() {
                        result.push(path);
                    }
                }
                Err(e) => {
                    return Err(Error::Io(e.into_error()));
                }
            }
        }
    }

    Ok(result)
}

/// Adds workspace package dependencies to a package
pub fn add_workspace_dependencies(pkg_path: &Path, workspace_root: &Path) -> Result<()> {
    let pkg_json_path = pkg_path.join("package.json");
    let pkg_content = fs::read_to_string(&pkg_json_path).map_err(Error::Io)?;

    let mut pkg_json: serde_json::Value =
        serde_json::from_str(&pkg_content).map_err(Error::Json)?;

    // Get workspace packages
    let packages = get_workspace_packages(workspace_root)?;

    // Get package name
    let pkg_name = pkg_json["name"]
        .as_str()
        .ok_or_else(|| Error::Workspace("Package name not found".into()))?;

    // Add dependencies to other workspace packages
    let mut dependencies = pkg_json["dependencies"]
        .as_object()
        .cloned()
        .unwrap_or_default();

    for workspace_pkg in packages {
        let workspace_pkg_json_path = workspace_pkg.join("package.json");
        if workspace_pkg_json_path.exists() && workspace_pkg_json_path != pkg_json_path {
            let content = fs::read_to_string(&workspace_pkg_json_path).map_err(Error::Io)?;
            let other_pkg: serde_json::Value =
                serde_json::from_str(&content).map_err(Error::Json)?;

            if let Some(other_name) = other_pkg["name"].as_str() {
                if other_name != pkg_name {
                    dependencies.insert(
                        other_name.to_string(),
                        serde_json::Value::String("workspace:*".to_string()),
                    );
                }
            }
        }
    }

    pkg_json["dependencies"] = serde_json::Value::Object(dependencies);

    // Write back to package.json
    fs::write(
        &pkg_json_path,
        serde_json::to_string_pretty(&pkg_json).map_err(Error::Json)?,
    )
    .map_err(Error::Io)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_detect_package_manager() {
        let temp = tempdir().unwrap();

        // Default (npm)
        assert_eq!(detect_package_manager(temp.path()), PackageManager::Npm);

        // Yarn Classic
        File::create(temp.path().join("yarn.lock")).unwrap();
        assert_eq!(detect_package_manager(temp.path()), PackageManager::Yarn);

        // Yarn Berry
        File::create(temp.path().join(".yarnrc.yml")).unwrap();
        assert_eq!(
            detect_package_manager(temp.path()),
            PackageManager::YarnBerry
        );

        // Clean up
        fs::remove_file(temp.path().join("yarn.lock")).unwrap();
        fs::remove_file(temp.path().join(".yarnrc.yml")).unwrap();

        // pnpm
        File::create(temp.path().join("pnpm-lock.yaml")).unwrap();
        assert_eq!(detect_package_manager(temp.path()), PackageManager::Pnpm);
    }

    #[test]
    fn test_workspace_detection() {
        let temp = tempdir().unwrap();
        let workspace_config = r#"{
            "name": "workspace-root",
            "private": true,
            "workspaces": ["packages/*"]
        }"#;

        fs::write(temp.path().join("package.json"), workspace_config).unwrap();

        // Create a package directory
        let package_dir = temp.path().join("packages").join("test-pkg");
        fs::create_dir_all(&package_dir).unwrap();

        // Test workspace root detection
        assert_eq!(
            find_workspace_root(&package_dir),
            Some(temp.path().to_path_buf())
        );
    }

    #[test]
    fn test_workspace_packages() {
        let temp = tempdir().unwrap();

        // Create workspace root
        let workspace_config = r#"{
            "name": "workspace-root",
            "private": true,
            "workspaces": ["packages/*"]
        }"#;

        fs::write(temp.path().join("package.json"), workspace_config).unwrap();

        // Create some workspace packages
        let packages_dir = temp.path().join("packages");
        fs::create_dir_all(&packages_dir).unwrap();

        let pkg_names = ["pkg-a", "pkg-b", "pkg-c"];
        for pkg in &pkg_names {
            let pkg_dir = packages_dir.join(pkg);
            fs::create_dir_all(&pkg_dir).unwrap();
            fs::write(
                pkg_dir.join("package.json"),
                format!(r#"{{"name": "{}"}}"#, pkg),
            )
            .unwrap();
        }

        // Test package detection
        let packages = get_workspace_packages(temp.path()).unwrap();
        assert_eq!(packages.len(), 3);

        for pkg in &pkg_names {
            assert!(packages.contains(&packages_dir.join(pkg)));
        }
    }

    #[test]
    fn test_yarn_workspace_config() {
        let temp = tempdir().unwrap();

        // Create workspace with Yarn-specific config
        let workspace_config = r#"{
            "name": "workspace-root",
            "private": true,
            "workspaces": {
                "packages": ["packages/*"],
                "nohoist": ["**/react-native", "**/react-native/**"]
            }
        }"#;

        fs::write(temp.path().join("package.json"), workspace_config).unwrap();

        // Create a package
        let pkg_dir = temp.path().join("packages").join("test-pkg");
        fs::create_dir_all(&pkg_dir).unwrap();
        fs::write(pkg_dir.join("package.json"), r#"{"name": "test-pkg"}"#).unwrap();

        // Test package detection
        let packages = get_workspace_packages(temp.path()).unwrap();
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0], pkg_dir);
    }

    #[test]
    fn test_invalid_workspace_config() {
        let temp = tempdir().unwrap();

        // Create invalid workspace config
        let invalid_config = r#"{
            "name": "workspace-root",
            "workspaces": "invalid"
        }"#;

        fs::write(temp.path().join("package.json"), invalid_config).unwrap();

        // Test error handling
        let result = get_workspace_packages(temp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_workspace_dependencies() {
        let temp = tempdir().unwrap();

        // Create workspace root
        let workspace_config = r#"{
            "name": "test-workspace",
            "private": true,
            "workspaces": ["packages/*"]
        }"#;

        fs::write(temp.path().join("package.json"), workspace_config).unwrap();

        // Create packages
        let packages_dir = temp.path().join("packages");
        fs::create_dir_all(&packages_dir).unwrap();

        // Create package A
        let pkg_a_dir = packages_dir.join("pkg-a");
        fs::create_dir_all(&pkg_a_dir).unwrap();
        fs::write(
            pkg_a_dir.join("package.json"),
            r#"{"name": "@test/pkg-a", "version": "1.0.0"}"#,
        )
        .unwrap();

        // Create package B
        let pkg_b_dir = packages_dir.join("pkg-b");
        fs::create_dir_all(&pkg_b_dir).unwrap();
        fs::write(
            pkg_b_dir.join("package.json"),
            r#"{"name": "@test/pkg-b", "version": "1.0.0"}"#,
        )
        .unwrap();

        // Add workspace dependencies
        add_workspace_dependencies(&pkg_a_dir, temp.path()).unwrap();

        // Verify dependencies were added
        let pkg_a_json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(pkg_a_dir.join("package.json")).unwrap())
                .unwrap();

        assert_eq!(pkg_a_json["dependencies"]["@test/pkg-b"], "workspace:*");
    }
}
