use anyhow::Ok;
use regex::Regex;
use std::sync::LazyLock;

use crate::error::AppResult;

const PKG_NAME_LENGTH: usize = 214;

/// Reference: <https://docs.npmjs.com/cli/v7/configuring-npm/package-json#name>
pub static PKG_NAME_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$")
        .expect("Failed to compile package name regex pattern")
});

pub fn validate_pkg_name(name: &str) -> AppResult<()> {
    log::debug!("Validating package name: {}", name);

    if name.trim().is_empty() {
        anyhow::bail!("Package name cannot be empty");
    }

    if name.len() > PKG_NAME_LENGTH {
        anyhow::bail!("Package name length exceeds {} characters", PKG_NAME_LENGTH);
    }

    if name.starts_with(".") || name.starts_with("_") {
        anyhow::bail!("Package name cannot start with a period or underscore");
    }

    if name.contains("..") || (name.contains('@') && !name.contains('/')) {
        anyhow::bail!("Package name cannot contain double periods or '@' without a scope");
    }

    if !PKG_NAME_PATTERN.is_match(name) {
        anyhow::bail!("Package name contains invalid characters");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_valid_pkg_names() {
        let pkg_names = vec![
            "package",
            "my-package",
            "@scope/package",
            "package123",
            "package.name",
        ];

        for name in pkg_names {
            assert!(validate_pkg_name(name).is_ok(), "Should accept {}", name);
        }
    }

    #[test]
    fn test_with_invalid_pkg_names() {
        let binding = "a".repeat(215);
        let pkg_names = vec![
            "",
            " ",
            ".package",
            "_package",
            "Package",
            "@invalid",
            "package..name",
            &binding,
        ];

        for name in pkg_names {
            assert!(validate_pkg_name(name).is_err(), "Should reject {}", name);
        }
    }

    #[test]
    fn test_with_edge_cases() {
        assert!(
            validate_pkg_name("a").is_ok(),
            "Single char should be valid"
        );
        assert!(
            validate_pkg_name(&"a".repeat(214)).is_ok(),
            "Max length should be valid"
        );
        assert!(
            validate_pkg_name("@a/b").is_ok(),
            "Minimal scoped name should be valid"
        );
    }
}
