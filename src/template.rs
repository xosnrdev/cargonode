use std::{collections::HashMap, path::Path};

use serde::Serialize;

use crate::Result;

/// Represents the type of Node.js project
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    /// A binary project with a main entry point
    Binary,
    /// A library project that can be imported
    Library,
}

/// Configuration for package.json generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageConfig {
    /// Name of the package
    pub name: String,
    /// Type of the project (binary or library)
    pub project_type: ProjectType,
    /// Version of the package (defaults to "0.1.0")
    pub version: Option<String>,
}

/// Represents a package.json file structure
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PackageJson {
    name: String,
    version: String,
    private: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    main: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bin: Option<HashMap<String, String>>,
}

pub fn create_package_json(config: PackageConfig) -> PackageJson {
    let is_binary = matches!(config.project_type, ProjectType::Binary);
    let main_file = if is_binary {
        "src/main.js"
    } else {
        "src/lib.js"
    };

    let bin = if is_binary {
        let mut bin_map = HashMap::new();
        bin_map.insert(config.name.clone(), main_file.to_string());
        Some(bin_map)
    } else {
        None
    };

    PackageJson {
        name: config.name,
        version: config.version.unwrap_or_else(|| "0.1.0".to_string()),
        main: Some(main_file.to_string()),
        bin,
        private: is_binary,
    }
}

pub fn serialize_package_json(package: &PackageJson) -> Result<String> {
    Ok(serde_json::to_string_pretty(package)?)
}

pub fn write_package_json(package: &PackageJson, path: &Path) -> Result<()> {
    let content = serialize_package_json(package)?;
    std::fs::write(path.join("package.json"), content)?;
    Ok(())
}

/// Template content for .gitignore file
pub const GITIGNORE_CONTENT: &str = r#"node_modules/
.env.*
"#;

/// Template content for main.js file
pub const MAIN_JS_CONTENT: &str = r#"function main() {
    console.log("Hello, world!");
}

if (import.meta.url === new URL(import.meta.resolve(), import.meta.url).href) {
    main();
}
"#;

/// Template content for lib.js file
pub const LIB_JS_CONTENT: &str = r#"export function add(left, right) {
    return left + right;
}

// Run tests only if the file is executed directly
if (import.meta.url === new URL(import.meta.resolve(), import.meta.url).href) {
    import("node:assert/strict").then(assert => {
        function testAdd() {
            const result = add(2, 2);
            assert.strictEqual(result, 4);
            console.log("✅ testAdd passed!");
        }

        testAdd();
    });
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_package_json_binary() {
        let config = PackageConfig {
            name: "test-bin".to_string(),
            project_type: ProjectType::Binary,
            version: None,
        };

        let package = create_package_json(config);

        assert_eq!(package.name, "test-bin");
        assert_eq!(package.version, "0.1.0");
        assert_eq!(package.main, Some("src/main.js".to_string()));
        assert!(package.private);

        let bin = package.bin.expect("Binary should have bin field");
        assert_eq!(bin.get("test-bin"), Some(&"src/main.js".to_string()));
    }

    #[test]
    fn test_create_package_json_library() {
        let config = PackageConfig {
            name: "test-lib".to_string(),
            project_type: ProjectType::Library,
            version: Some("1.0.0".to_string()),
        };

        let package = create_package_json(config);

        assert_eq!(package.name, "test-lib");
        assert_eq!(package.version, "1.0.0");
        assert_eq!(package.main, Some("src/lib.js".to_string()));
        assert!(!package.private);
        assert!(package.bin.is_none());
    }

    #[test]
    fn test_serialize_package_json() {
        let config = PackageConfig {
            name: "test-pkg".to_string(),
            project_type: ProjectType::Library,
            version: None,
        };

        let package = create_package_json(config);
        let json = serialize_package_json(&package).expect("Should serialize successfully");

        assert!(json.contains(r#""name": "test-pkg""#));
        assert!(json.contains(r#""version": "0.1.0""#));
        assert!(json.contains(r#""main": "src/lib.js""#));
        assert!(!json.contains(r#""bin""#));
    }
}
