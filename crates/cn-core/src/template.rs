//! Project template handling and validation
//!
//! This module provides functionality for managing and validating project templates,
//! including custom templates and built-in templates.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Error, ProjectType, Result};

/// Template manifest file name
const TEMPLATE_MANIFEST: &str = "template.json";

/// Workspace configuration file templates
pub const WORKSPACE_CONFIG_FILES: &[(&str, &str, bool)] =
    &[(".gitignore", r#"node_modules/"#, false)];

/// Template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template version
    pub version: String,
    /// Supported project types
    #[serde(default)]
    pub project_types: Vec<ProjectType>,
    /// Template files
    pub files: HashMap<String, TemplateFile>,
    /// Template dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    /// Template dev dependencies
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
    /// Template scripts
    #[serde(default)]
    pub scripts: HashMap<String, String>,
}

/// Template file configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    /// File path relative to project root
    pub path: String,
    /// Whether the file is executable
    #[serde(default)]
    pub executable: bool,
    /// File content template
    pub content: String,
}

impl TemplateConfig {
    /// Validates the template configuration
    pub fn validate(&self) -> Result<()> {
        // Validate name
        if self.name.is_empty() {
            return Err(Error::Template("Template name cannot be empty".into()));
        }

        // Validate version format
        if !is_valid_semver(&self.version) {
            return Err(Error::Template(format!(
                "Invalid template version: {}",
                self.version
            )));
        }

        // Validate file paths
        for file in self.files.values() {
            if file.path.is_empty() {
                return Err(Error::Template(format!(
                    "Empty path for file '{}'",
                    file.path
                )));
            }

            // Check for path traversal attempts
            let path = PathBuf::from(&file.path);
            if path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                return Err(Error::Template(format!(
                    "Path traversal not allowed: {}",
                    file.path
                )));
            }
        }

        // Validate dependencies
        for (name, version) in &self.dependencies {
            if name.is_empty() {
                return Err(Error::Template("Dependency name cannot be empty".into()));
            }
            if !is_valid_npm_version(version) {
                return Err(Error::Template(format!(
                    "Invalid version for dependency '{}': {}",
                    name, version
                )));
            }
        }

        // Validate dev dependencies
        for (name, version) in &self.dev_dependencies {
            if name.is_empty() {
                return Err(Error::Template(
                    "Dev dependency name cannot be empty".into(),
                ));
            }
            if !is_valid_npm_version(version) {
                return Err(Error::Template(format!(
                    "Invalid version for dev dependency '{}': {}",
                    name, version
                )));
            }
        }

        Ok(())
    }

    /// Loads a template from a directory
    pub fn load(path: &Path) -> Result<Self> {
        let manifest_path = path.join(TEMPLATE_MANIFEST);
        if !manifest_path.exists() {
            return Err(Error::Template(format!(
                "Template manifest not found at {}",
                manifest_path.display()
            )));
        }

        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| Error::Template(format!("Failed to read template manifest: {}", e)))?;

        let config: TemplateConfig = serde_json::from_str(&content)
            .map_err(|e| Error::Template(format!("Invalid template manifest: {}", e)))?;

        config.validate()?;
        Ok(config)
    }

    /// Renders template files for a project
    pub fn render(
        &self,
        project_name: &str,
        project_type: ProjectType,
    ) -> Result<HashMap<String, String>> {
        // Validate project type support
        if !self.project_types.is_empty() && !self.project_types.contains(&project_type) {
            return Err(Error::Template(format!(
                "Template does not support project type {:?}",
                project_type
            )));
        }

        let mut rendered = HashMap::new();
        for file in self.files.values() {
            let content = render_template(&file.content, project_name)?;
            rendered.insert(file.path.clone(), content);
        }

        Ok(rendered)
    }

    /// Creates workspace configuration files
    pub fn create_workspace_files(&self, workspace_root: &Path) -> Result<()> {
        for (filename, content, executable) in WORKSPACE_CONFIG_FILES {
            crate::fs::write_file(&workspace_root.join(filename), content, *executable)?;
        }
        Ok(())
    }
}

/// Validates a semantic version string
fn is_valid_semver(version: &str) -> bool {
    // Basic semver validation (x.y.z)
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    parts.iter().all(|part| part.parse::<u32>().is_ok())
}

/// Validates an npm version specifier
fn is_valid_npm_version(version: &str) -> bool {
    // Allow common npm version patterns
    if version.is_empty() {
        return false;
    }

    // Allow semver
    if is_valid_semver(version) {
        return true;
    }

    // Allow version ranges (^1.0.0, ~1.0.0, >=1.0.0)
    if version.starts_with('^') || version.starts_with('~') || version.starts_with(">=") {
        let v = if let Some(stripped) = version.strip_prefix(">=") {
            stripped
        } else {
            &version[1..]
        };
        return is_valid_semver(v);
    }

    // Allow wildcards (*)
    if version == "*" {
        return true;
    }

    // Allow x-ranges (1.x, 1.2.x)
    if let Some(base) = version.strip_suffix(".x") {
        let parts: Vec<&str> = base.split('.').collect();
        return (parts.len() == 1 || parts.len() == 2)
            && parts.iter().all(|part| part.parse::<u32>().is_ok());
    }

    false
}

/// Renders a template string with variables
fn render_template(template: &str, project_name: &str) -> Result<String> {
    let mut result = template.to_string();

    // Replace project name
    result = result.replace("{{project-name}}", project_name);

    // Replace other variables as needed
    // TODO: Add more variable substitutions

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_template_validation() {
        let valid_template = TemplateConfig {
            name: "test".into(),
            description: "Test template".into(),
            version: "1.0.0".into(),
            project_types: vec![ProjectType::Application],
            files: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            scripts: HashMap::new(),
        };
        assert!(valid_template.validate().is_ok());

        let invalid_name = TemplateConfig {
            name: "".into(),
            description: "Test template".into(),
            version: "1.0.0".into(),
            project_types: vec![ProjectType::Application],
            files: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            scripts: HashMap::new(),
        };
        assert!(invalid_name.validate().is_err());

        let invalid_version = TemplateConfig {
            name: "test".into(),
            description: "Test template".into(),
            version: "invalid".into(),
            project_types: vec![ProjectType::Application],
            files: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            scripts: HashMap::new(),
        };
        assert!(invalid_version.validate().is_err());
    }

    #[test]
    fn test_template_loading() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join(TEMPLATE_MANIFEST);

        let template_json = r#"{
            "name": "test",
            "description": "Test template",
            "version": "1.0.0",
            "project_types": ["application"],
            "files": {
                "package.json": {
                    "path": "package.json",
                    "content": "{\"name\":\"{{project-name}}\",\"version\":\"1.0.0\"}",
                    "executable": false
                }
            }
        }"#;

        fs::write(&template_path, template_json).unwrap();
        let template = TemplateConfig::load(temp_dir.path()).unwrap();
        assert_eq!(template.name, "test");
        assert_eq!(template.project_types, vec![ProjectType::Application]);
    }

    #[test]
    fn test_invalid_template() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join(TEMPLATE_MANIFEST);

        let invalid_json = r#"{
            "name": "test",
            "description": "Test template",
            "version": "1.0.0",
            "project_types": ["invalid"],
            "files": {}
        }"#;

        fs::write(&template_path, invalid_json).unwrap();
        assert!(TemplateConfig::load(temp_dir.path()).is_err());
    }

    #[test]
    fn test_template_rendering() {
        let mut files = HashMap::new();
        files.insert(
            "package.json".into(),
            TemplateFile {
                path: "package.json".into(),
                content: r#"{"name":"{{project-name}}","version":"1.0.0"}"#.into(),
                executable: false,
            },
        );

        let template = TemplateConfig {
            name: "test".into(),
            description: "Test template".into(),
            version: "1.0.0".into(),
            project_types: vec![ProjectType::Application],
            files,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            scripts: HashMap::new(),
        };

        let rendered = template
            .render("my-project", ProjectType::Application)
            .unwrap();
        assert_eq!(
            rendered.get("package.json").unwrap(),
            r#"{"name":"my-project","version":"1.0.0"}"#
        );
    }

    #[test]
    fn test_version_validation() {
        assert!(is_valid_semver("1.0.0"));
        assert!(!is_valid_semver("1.0.0-alpha.1"));
        assert!(!is_valid_semver("invalid"));
    }

    #[test]
    fn test_npm_version_validation() {
        // Test valid versions
        assert!(is_valid_npm_version("1.0.0"));
        assert!(is_valid_npm_version("^1.0.0"));
        assert!(is_valid_npm_version("~1.0.0"));
        assert!(is_valid_npm_version(">=1.0.0"));
        assert!(is_valid_npm_version("*"));
        assert!(is_valid_npm_version("1.x"));
        assert!(is_valid_npm_version("1.2.x"));

        // Test invalid versions
        assert!(!is_valid_npm_version(""));
        assert!(!is_valid_npm_version("invalid"));
        assert!(!is_valid_npm_version("1.0"));
        assert!(!is_valid_npm_version("1.0.0.0"));
    }
}
