use std::path::{Path, PathBuf};

use crate::{
    utils::{self, VcsConfig},
    Error, Result,
};

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    /// Package name (can be scoped)
    pub name: String,
    /// Project root path
    pub path: PathBuf,
    /// Whether this is a binary project
    pub is_binary: bool,
    /// Version control configuration
    pub vcs_config: Option<VcsConfig>,
}

pub fn validate_init_config(
    path: &Path,
    lib: bool,
    vcs_config: Option<VcsConfig>,
) -> Result<ProjectConfig> {
    // Check if package.json already exists
    if path.join("package.json").exists() {
        return Err(Error::PackageJsonExists);
    }

    // Extract and validate package name
    let package_name = super::extract_package_name(path)?;
    utils::validate_package_name(&package_name)?;

    Ok(ProjectConfig {
        name: package_name,
        path: path.to_path_buf(),
        is_binary: !lib,
        vcs_config,
    })
}

pub fn validate_project_config(
    path: &Path,
    lib: bool,
    vcs_config: Option<VcsConfig>,
) -> Result<ProjectConfig> {
    // Extract and validate package name
    let package_name = super::extract_package_name(path)?;
    utils::validate_package_name(&package_name)?;

    Ok(ProjectConfig {
        name: package_name,
        path: path.to_path_buf(),
        is_binary: !lib,
        vcs_config,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_validate_init_config() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("valid-package");
        std::fs::create_dir(&path).unwrap();

        // Test valid configuration (library)
        let config = validate_init_config(&path, true, None).unwrap();
        assert!(!config.is_binary);
        assert_eq!(config.path, path);
        assert_eq!(config.name, "valid-package");

        // Test valid configuration (binary)
        let config = validate_init_config(&path, false, None).unwrap();
        assert!(config.is_binary);
        assert_eq!(config.path, path);
        assert_eq!(config.name, "valid-package");

        // Test package.json exists
        fs::write(path.join("package.json"), "{}").unwrap();
        assert!(validate_init_config(&path, false, None).is_err());
    }

    #[test]
    fn test_validate_project_config() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("@scope").join("my-pkg");

        let config = validate_project_config(&path, true, None).unwrap();
        assert!(!config.is_binary);
        assert_eq!(config.path, path);
        assert_eq!(config.name, "@scope/my-pkg");
    }
}
