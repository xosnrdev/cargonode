//! Path handling utilities for project creation
//!
//! This module provides utilities for handling and validating paths
//! in a cross-platform way.

use std::fs;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// Maximum path length for the current platform
#[cfg(windows)]
const MAX_PATH_LENGTH: usize = 260;

/// Maximum path length for the current platform
#[cfg(unix)]
const MAX_PATH_LENGTH: usize = 4096;

/// Characters that are problematic in paths across platforms
const INVALID_PATH_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*'];

/// Validates path characters
fn validate_path_chars(path: &Path) -> Result<()> {
    let path_string = path.to_string_lossy().into_owned();
    for c in path_string.chars() {
        if INVALID_PATH_CHARS.contains(&c) {
            return Err(Error::InvalidPath(format!(
                "Path contains invalid character: '{}'",
                c
            )));
        }
    }
    Ok(())
}

/// Validates path length
fn validate_path_length(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy();
    if path_str.len() > MAX_PATH_LENGTH {
        return Err(Error::InvalidPath(format!(
            "Path length exceeds maximum of {} characters",
            MAX_PATH_LENGTH
        )));
    }
    Ok(())
}

/// Validates and normalizes a path for project creation
///
/// # Arguments
/// * `path` - The path to validate and normalize
///
/// # Returns
/// A normalized PathBuf if valid, Error if invalid
pub fn validate_and_normalize_path(path: &Path) -> Result<PathBuf> {
    // Validate path characters and length
    validate_path_chars(path)?;
    validate_path_length(path)?;

    // For validation, we don't require the path to exist
    validate_directory_state(path, false)?;

    // Normalize the path
    Ok(normalize_path(path))
}

/// Normalizes a path by resolving . and .. components
///
/// # Arguments
/// * `path` - The path to normalize
///
/// # Returns
/// A normalized PathBuf
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::Prefix(p) => components.push(p.as_os_str().to_owned()),
            std::path::Component::RootDir => components.push(std::ffi::OsString::from("/")),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::Normal(x) => components.push(x.to_owned()),
        }
    }
    components.iter().collect::<PathBuf>()
}

pub fn validate_directory_state(path: &Path, must_exist: bool) -> Result<()> {
    if must_exist && !path.exists() {
        return Err(Error::InvalidPath(format!(
            "Path not found: {}",
            path.display()
        )));
    }

    if path.exists() && !path.is_dir() {
        return Err(Error::InvalidPath(format!(
            "Not a directory: {}",
            path.display()
        )));
    }

    // Get the current working directory to use as a base for validation
    let current_dir = std::env::current_dir()?;

    // If path doesn't exist and must_exist is false, we need to check if we can create it
    let parent = path.parent().unwrap_or(Path::new("."));
    let absolute_parent = if parent.is_absolute() {
        parent.to_path_buf()
    } else {
        current_dir.join(parent)
    };

    // Check if we can write to the parent directory or create it
    if absolute_parent.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let metadata = fs::metadata(&absolute_parent)?;
            let mode = metadata.mode();
            if mode & 0o200 == 0 {
                return Err(Error::Permission(format!(
                    "Write permission denied: {}",
                    absolute_parent.display()
                )));
            }
        }
        #[cfg(windows)]
        {
            let metadata = fs::metadata(&absolute_parent)?;
            if metadata.permissions().readonly() {
                return Err(Error::Permission(format!(
                    "Write permission denied: {}",
                    absolute_parent.display()
                )));
            }
        }
    } else {
        // Check if we can create the parent directory by checking permissions on the first existing ancestor
        let mut current = absolute_parent.as_path();
        while let Some(ancestor) = current.parent() {
            if ancestor.exists() {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    let metadata = fs::metadata(ancestor)?;
                    let mode = metadata.mode();
                    if mode & 0o200 == 0 {
                        return Err(Error::Permission(format!(
                            "Write permission denied: {}",
                            ancestor.display()
                        )));
                    }
                }
                #[cfg(windows)]
                {
                    let metadata = fs::metadata(ancestor)?;
                    if metadata.permissions().readonly() {
                        return Err(Error::Permission(format!(
                            "Write permission denied: {}",
                            ancestor.display()
                        )));
                    }
                }
                return Ok(());
            }
            current = ancestor;
        }
        // If we get here, we found no existing ancestor - this is only valid if we're dealing with an absolute path
        if !path.is_absolute() {
            return Err(Error::InvalidPath(format!(
                "Cannot determine write permissions for path: {}",
                path.display()
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rstest::rstest;
    use tempfile::tempdir;

    use super::*;

    // Pure test: Tests path character validation without side effects
    #[rstest]
    #[case("normal_path", true)]
    #[case("path/with/invalid/*/char", false)]
    #[case("path/with/invalid/:/char", false)]
    fn test_validate_path_chars(#[case] path: &str, #[case] should_be_valid: bool) {
        let result = validate_path_chars(Path::new(path));
        assert_eq!(result.is_ok(), should_be_valid);
    }

    // Pure test: Tests path length validation without side effects
    #[test]
    fn test_validate_path_length() {
        let short_path = Path::new("short/path");
        assert!(validate_path_length(short_path).is_ok());

        let long_string = "a".repeat(MAX_PATH_LENGTH + 1);
        let long_path = Path::new(&long_string);
        assert!(validate_path_length(long_path).is_err());
    }

    // Pure test: Tests path normalization without side effects
    #[test]
    fn test_normalize_path() {
        let test_cases = vec![
            ("a/b/c/../../b", "a/b"),
            ("a/./b", "a/b"),
            ("a/b/c/..", "a/b"),
        ];

        for (input, expected) in test_cases {
            let normalized = normalize_path(Path::new(input));
            assert_eq!(normalized, PathBuf::from(expected));
        }
    }

    // Pure test: Tests path validation with special characters
    #[rstest]
    #[case("normal_path", true)]
    #[case("path with spaces", true)]
    #[case("path/with/unicode/⚡", true)]
    #[case("path/with/invalid/*/char", false)]
    #[case("path/with/invalid/:/char", false)]
    fn test_validate_path(#[case] path: &str, #[case] should_be_valid: bool) {
        let result = validate_and_normalize_path(Path::new(path));
        assert_eq!(
            result.is_ok(),
            should_be_valid,
            "Path validation failed for '{}': expected valid = {}, got result = {:?}",
            path,
            should_be_valid,
            result
        );
    }

    // Pure test: Tests long path handling
    #[test]
    fn test_long_path() {
        let long_string = "a".repeat(MAX_PATH_LENGTH + 1);
        let test_path = Path::new(&long_string);

        let result = validate_path_length(test_path);

        #[cfg(windows)]
        assert!(result.is_err());

        #[cfg(unix)]
        assert!(result.is_err());
    }

    // Pure test: Tests non-existent parent directory handling
    #[test]
    fn test_non_existent_parent() {
        let test_path = Path::new("non_existent_dir/project");
        let result = validate_directory_state(test_path, true);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidPath(_))));
    }

    // Pure test: Tests permission handling
    #[test]
    fn test_permission_handling() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("readonly_dir");
        fs::create_dir(&test_dir).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};
            // Make directory read-only with no write permissions
            fs::set_permissions(&test_dir, std::fs::Permissions::from_mode(0o444)).unwrap();

            // Test that we detect read-only parent directory
            let test_path = test_dir.join("project");
            let metadata = fs::metadata(&test_dir).unwrap();
            assert_eq!(metadata.mode() & 0o200, 0);

            // Test that validate_directory_state detects the permission issue
            let result = validate_directory_state(&test_path, false);
            assert!(result.is_err());
            assert!(matches!(result, Err(Error::Permission(_))));
        }
    }
}
