use std::{fs, path::Path, process::Command, sync::OnceLock};

use regex::Regex;

use crate::{Error, Result};

/// Represents the type of version control system to use
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VcsType {
    /// Git version control (default)
    #[default]
    Git,
    /// No version control
    None,
}

/// Configuration for package name validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageNameConfig<'a> {
    /// The package name to validate
    pub name: &'a str,
    /// Whether this is a scoped package (starts with @)
    pub is_scoped: bool,
}

/// NPM package name regex pattern
/// Follows the official npm specification:
/// - Can be scoped (@org/name) or unscoped (name)
/// - Must contain only lowercase letters, numbers, and special characters: -._
/// - Must start with @ (for scoped) or letter/number
/// - Cannot have consecutive dots, hyphens, or underscores
/// - Cannot end with a dot, hyphen, or underscore
static NPM_PACKAGE_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn validate_package_name(name: &str) -> Result<()> {
    let regex = NPM_PACKAGE_REGEX.get_or_init(|| {
        Regex::new(r"^(?:@[a-z][a-z0-9-]*\/)?[a-z][a-z0-9-._]*[a-z0-9]$")
            .expect("Invalid package name regex pattern")
    });

    if name.is_empty() {
        return Err(Error::InvalidPackageName {
            name: name.to_string(),
            reason: "Package name cannot be empty".to_string(),
        });
    }

    // Check for consecutive special characters
    if name.contains("..") || name.contains("--") || name.contains("__") {
        return Err(Error::InvalidPackageName {
            name: name.to_string(),
            reason: "Package name cannot contain consecutive dots, hyphens, or underscores"
                .to_string(),
        });
    }

    if !regex.is_match(name) {
        return Err(Error::InvalidPackageName {
            name: name.to_string(),
            reason: format!(
                "Invalid package name format. Package names must:\n\
                 - Start with a letter (or @ for scoped packages)\n\
                 - Contain only lowercase letters, numbers, and special characters: -._\n\
                 - Not end with a dot, hyphen, or underscore\n\
                 - Follow the pattern: {} or @scope/{}",
                "[a-z][a-z0-9-._]*[a-z0-9]", "[a-z][a-z0-9-._]*[a-z0-9]"
            ),
        });
    }

    Ok(())
}

/// Configuration for version control initialization
#[derive(Debug, Clone)]
pub struct VcsConfig {
    /// Type of version control system
    pub vcs_type: VcsType,
    /// Content of the ignore file
    pub ignore_content: String,
}

impl Default for VcsConfig {
    fn default() -> Self {
        Self {
            vcs_type: VcsType::Git,
            ignore_content: crate::template::GITIGNORE_CONTENT.to_string(),
        }
    }
}

fn check_git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}

fn init_git_repo(path: &Path) -> Result<()> {
    if !check_git_available() {
        return Err(Error::Git {
            message: "Git is not installed".to_string(),
            details: "Please install git to continue".to_string(),
        });
    }

    let output = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        return Err(Error::Git {
            message: "Failed to initialize git repository".to_string(),
            details: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(())
}

fn write_ignore_file(path: &Path, content: &str) -> Result<()> {
    let gitignore = path.join(".gitignore");
    let should_write = if !gitignore.exists() {
        true
    } else {
        fs::read_to_string(&gitignore)?.is_empty()
    };
    if should_write {
        fs::write(&gitignore, content)?;
    }
    Ok(())
}

pub fn init_vcs(path: &Path, config: &VcsConfig) -> Result<()> {
    match config.vcs_type {
        VcsType::Git => {
            if !is_git_repo(path) {
                init_git_repo(path)?;
            }
            write_ignore_file(path, &config.ignore_content)?;
        }
        VcsType::None => (),
    }
    Ok(())
}

pub fn is_directory_empty(path: &Path) -> Result<bool> {
    if !path.exists() {
        return Ok(true);
    }

    if !path.is_dir() {
        return Err(Error::DirectoryExists {
            path: path.to_path_buf(),
        });
    }

    Ok(fs::read_dir(path)?.count() == 0)
}

pub fn ensure_directory_empty(path: &Path) -> Result<()> {
    match is_directory_empty(path)? {
        true => Ok(()),
        false => Err(Error::DirectoryNotEmpty {
            path: path.to_path_buf(),
        }),
    }
}

#[derive(Debug, Clone)]
pub struct ProjectStructure {
    /// Root path of the project
    pub path: std::path::PathBuf,
    /// Whether this is a binary project
    pub is_binary: bool,
    /// Source file content
    pub source_content: String,
}

pub fn create_project_config(path: &Path, is_binary: bool) -> ProjectStructure {
    let source_content = if is_binary {
        crate::template::MAIN_JS_CONTENT.to_string()
    } else {
        crate::template::LIB_JS_CONTENT.to_string()
    };

    ProjectStructure {
        path: path.to_path_buf(),
        is_binary,
        source_content,
    }
}

pub fn create_project_structure(config: &ProjectStructure) -> Result<()> {
    fs::create_dir_all(&config.path)?;
    fs::create_dir_all(config.path.join("src"))?;

    let source_file = if config.is_binary {
        "main.js"
    } else {
        "lib.js"
    };
    fs::write(
        config.path.join("src").join(source_file),
        &config.source_content,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_package_name_valid() {
        let valid_names = [
            "valid-package",
            "@scope/package",
            "package.name",
            "@org/pkg-name",
            "my-package",
            "my.package",
            "my_package",
            "@scope/my.package",
            "@scope/my-package",
            "@scope/my_package",
            "ab1",
            "@abc/def2",
        ];

        for name in valid_names {
            assert!(
                validate_package_name(name).is_ok(),
                "Should accept {}",
                name
            );
        }
    }

    #[test]
    fn test_validate_package_name_invalid() {
        let invalid_names = [
            "",                     // empty
            "UPPERCASE",            // uppercase letters
            ".start-dot",           // starts with dot
            "_start-underscore",    // starts with underscore
            "end.",                 // ends with dot
            "end_",                 // ends with underscore
            "double..dot",          // consecutive dots
            "double__underscore",   // consecutive underscores
            "double--dash",         // consecutive dashes
            "@/package",            // empty scope
            "@scope/",              // empty package name
            "/package",             // no scope but starts with slash
            "@scope//pkg",          // double slash
            "pkg/",                 // ends with slash
            "@123/package",         // scope starts with number
            "~package",             // starts with tilde
            "package~",             // ends with tilde
            "@.org/package",        // scope starts with dot
            "@org/.package",        // package starts with dot
            "package name",         // contains space
            "@scope/package/extra", // extra segments
            "a",                    // single character
            "@a/",                  // scope with no package
            "@/a",                  // no scope name
            "1package",             // starts with number
            "@scope/1package",      // package starts with number
        ];

        for name in invalid_names {
            assert!(
                validate_package_name(name).is_err(),
                "Should reject {}",
                name
            );
        }
    }

    #[test]
    fn test_is_directory_empty() {
        let temp_dir = tempfile::tempdir().unwrap();
        assert!(is_directory_empty(temp_dir.path()).unwrap());

        fs::write(temp_dir.path().join("file.txt"), "content").unwrap();
        assert!(!is_directory_empty(temp_dir.path()).unwrap());
    }

    #[test]
    fn test_create_project_config() {
        let path = PathBuf::from("/test/path");
        let config = create_project_config(&path, true);
        assert!(config.is_binary);
        assert_eq!(config.path, path);
        assert_eq!(config.source_content, crate::template::MAIN_JS_CONTENT);

        let config = create_project_config(&path, false);
        assert!(!config.is_binary);
        assert_eq!(config.source_content, crate::template::LIB_JS_CONTENT);
    }

    #[test]
    fn test_vcs_config_default() {
        let config = VcsConfig::default();
        assert_eq!(config.vcs_type, VcsType::Git);
        assert_eq!(config.ignore_content, crate::template::GITIGNORE_CONTENT);
    }
}
