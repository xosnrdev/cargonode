use std::path::Path;

use crate::{utils, Result};

mod config;
mod project;

pub use config::{validate_init_config, validate_project_config, ProjectConfig};
pub use project::{create_new_project, create_project, init_project};

pub(crate) fn extract_package_name(path: &Path) -> Result<String> {
    // Get the base name
    let name = path.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
        crate::Error::InvalidPackageName {
            name: path.display().to_string(),
            reason: "Invalid path name".to_string(),
        }
    })?;

    // Count @ occurrences in the path
    let at_count = path
        .components()
        .filter(|c| {
            c.as_os_str()
                .to_str()
                .map(|s| s.starts_with('@'))
                .unwrap_or(false)
        })
        .count();

    // More than one @ in path is invalid
    if at_count > 1 {
        return Err(crate::Error::InvalidPackageName {
            name: path.display().to_string(),
            reason: "Multiple scopes are not allowed".to_string(),
        });
    }

    // Handle scoped packages
    let package_name = path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|scope| scope.to_str())
        .filter(|scope| scope.starts_with('@'))
        .map(|scope| format!("{}/{}", scope, name))
        .unwrap_or_else(|| name.to_string());

    // Validate the extracted package name
    utils::validate_package_name(&package_name)?;

    Ok(package_name)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_extract_package_name_simple() {
        let path = PathBuf::from("/some/path/package-name");
        assert_eq!(extract_package_name(&path).unwrap(), "package-name");
    }

    #[test]
    fn test_extract_package_name_scoped() {
        let path = PathBuf::from("/some/path/@scope/package-name");
        assert_eq!(extract_package_name(&path).unwrap(), "@scope/package-name");
    }

    #[test]
    fn test_extract_package_name_invalid() {
        let path = PathBuf::from("");
        assert!(extract_package_name(&path).is_err());
    }

    #[test]
    fn test_extract_package_name_invalid_chars() {
        let path = PathBuf::from("/some/path/INVALID_NAME");
        assert!(extract_package_name(&path).is_err());
    }

    #[test]
    fn test_extract_package_name_invalid_scope() {
        let path = PathBuf::from("/some/path/@/package-name");
        assert!(extract_package_name(&path).is_err());
    }

    #[test]
    fn test_extract_package_name_empty_scope() {
        let path = PathBuf::from("/some/path/@scope/");
        assert!(extract_package_name(&path).is_err());
    }

    #[test]
    fn test_extract_package_name_double_scope() {
        let path = PathBuf::from("/some/path/@scope/@other/package-name");
        assert!(extract_package_name(&path).is_err());
    }

    #[test]
    fn test_extract_package_name_with_dots() {
        let path = PathBuf::from("/some/path/valid.package");
        assert_eq!(extract_package_name(&path).unwrap(), "valid.package");
    }

    #[test]
    fn test_extract_package_name_with_hyphens() {
        let path = PathBuf::from("/some/path/valid-package");
        assert_eq!(extract_package_name(&path).unwrap(), "valid-package");
    }
}
