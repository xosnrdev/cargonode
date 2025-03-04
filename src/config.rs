use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::{utils, Result};

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    /// Package name (can be scoped)
    pub name: String,
    /// Project root path
    pub path: PathBuf,
    /// Whether this is a binary project
    pub is_binary: bool,
    /// Version control configuration
    pub vcs_config: Option<utils::VcsConfig>,
}

pub fn validate_init_config(
    path: &Path,
    lib: bool,
    vcs_config: Option<utils::VcsConfig>,
) -> Result<ProjectConfig> {
    // Check if package.json already exists
    if path.join("package.json").exists() {
        return Err(Error::PackageJsonExists);
    }

    // Extract and validate package name
    let package_name = utils::extract_package_name(path)?;
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
    vcs_config: Option<utils::VcsConfig>,
) -> Result<ProjectConfig> {
    // Extract and validate package name
    let package_name = utils::extract_package_name(path)?;
    utils::validate_package_name(&package_name)?;

    Ok(ProjectConfig {
        name: package_name,
        path: path.to_path_buf(),
        is_binary: !lib,
        vcs_config,
    })
}

/// Configuration for a tool
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolConfig {
    /// Command to run
    pub command: String,

    /// Arguments to pass to the command
    #[serde(default)]
    pub args: Vec<String>,

    /// Environment variables to set
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Working directory for the command
    #[serde(default)]
    pub working_dir: Option<String>,

    /// Input file patterns
    #[serde(default)]
    pub inputs: Vec<String>,

    /// Output file patterns (optional)
    /// Only required for commands that generate files (e.g., build)
    #[serde(default)]
    pub outputs: Vec<String>,
}

/// Configuration for cargonode
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CargonodeConfig {
    /// Tool configurations
    #[serde(default)]
    pub tools: HashMap<String, ToolConfig>,
}

/// Load the cargonode configuration from package.json
///
/// # Arguments
///
/// * `project_dir` - Path to the project directory
///
/// # Returns
///
/// * `Result<CargonodeConfig>` - The loaded configuration
pub fn load_config(project_dir: &Path) -> Result<CargonodeConfig> {
    let package_json_path = project_dir.join("package.json");

    // Check if package.json exists
    if !package_json_path.exists() {
        return Err(Error::Config {
            message: format!("package.json not found in {}", project_dir.display()),
        });
    }

    // Check if project_dir is a directory
    if !project_dir.is_dir() {
        return Err(Error::Config {
            message: format!("{} is not a directory", project_dir.display()),
        });
    }

    // Read package.json
    let package_json_content = fs::read_to_string(package_json_path)?;

    // Parse package.json
    let package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;

    // Extract cargonode configuration
    let config = if let Some(cargonode_config) = package_json.get("cargonode") {
        // Parse cargonode configuration
        serde_json::from_value(cargonode_config.clone())?
    } else {
        // No cargonode configuration found, use default
        CargonodeConfig {
            tools: HashMap::new(),
        }
    };

    Ok(config)
}

/// Get a tool configuration by name
///
/// # Arguments
///
/// * `config` - The cargonode configuration
/// * `tool_name` - Name of the tool
///
/// # Returns
///
/// * `Result<ToolConfig>` - The tool configuration
pub fn get_tool_config<'a>(config: &'a CargonodeConfig, tool_name: &str) -> Option<&'a ToolConfig> {
    config.tools.get(tool_name)
}

/// Validate a tool configuration
///
/// # Arguments
///
/// * `tool_name` - Name of the tool
/// * `config` - The tool configuration
///
/// # Returns
///
/// * `Result<()>` - Whether the configuration is valid
pub fn validate_tool_config(tool_name: &str, config: &ToolConfig) -> Result<()> {
    // Check if command is empty
    if config.command.is_empty() {
        return Err(Error::Config {
            message: format!("Tool '{}' has an empty command", tool_name),
        });
    }

    // Check if inputs is empty
    if config.inputs.is_empty() {
        return Err(Error::Config {
            message: format!("Tool '{}' has no input patterns", tool_name),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::TempDir;

    use super::*;

    fn create_package_json(dir: &Path, content: &str) -> Result<()> {
        let file_path = dir.join("package.json");
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_load_config() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create package.json with cargonode configuration
        let package_json = r#"
        {
            "name": "test-project",
            "version": "1.0.0",
            "cargonode": {
                "tools": {
                    "test": {
                        "command": "npm",
                        "args": ["test"],
                        "inputs": ["src/**/*.js"],
                        "outputs": ["coverage/**/*"]
                    }
                }
            }
        }
        "#;

        create_package_json(dir_path, package_json)?;

        // Load configuration
        let config = load_config(dir_path)?;

        // Check if configuration was loaded correctly
        assert_eq!(config.tools.len(), 1);
        assert!(config.tools.contains_key("test"));

        let test_tool = &config.tools["test"];
        assert_eq!(test_tool.command, "npm");
        assert_eq!(test_tool.args, vec!["test"]);
        assert_eq!(test_tool.inputs, vec!["src/**/*.js"]);
        assert_eq!(test_tool.outputs, vec!["coverage/**/*"]);

        Ok(())
    }

    #[test]
    fn test_get_tool_config() -> Result<()> {
        // Create a configuration
        let mut tools = HashMap::new();
        tools.insert(
            "test".to_string(),
            ToolConfig {
                command: "npm".to_string(),
                args: vec!["test".to_string()],
                env: HashMap::new(),
                working_dir: None,
                inputs: vec!["src/**/*.js".to_string()],
                outputs: vec!["coverage/**/*".to_string()],
            },
        );

        let config = CargonodeConfig { tools };

        // Get existing tool
        let test_tool = get_tool_config(&config, "test").unwrap();
        assert_eq!(test_tool.command, "npm");

        // Get non-existing tool
        let result = get_tool_config(&config, "build");
        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_validate_tool_config() -> Result<()> {
        // Valid configuration with outputs
        let valid_config = ToolConfig {
            command: "npm".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["src/**/*.js".to_string()],
            outputs: vec!["coverage/**/*".to_string()],
        };
        assert!(validate_tool_config("test", &valid_config).is_ok());

        // Valid configuration without outputs
        let valid_no_outputs = ToolConfig {
            command: "npm".to_string(),
            args: vec!["start".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["src/**/*.js".to_string()],
            outputs: vec![],
        };
        assert!(validate_tool_config("start", &valid_no_outputs).is_ok());

        // Invalid configuration - empty command
        let invalid_command = ToolConfig {
            command: "".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["src/**/*.js".to_string()],
            outputs: vec!["coverage/**/*".to_string()],
        };
        assert!(validate_tool_config("test", &invalid_command).is_err());

        // Invalid configuration - empty inputs
        let invalid_inputs = ToolConfig {
            command: "npm".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec![],
            outputs: vec!["coverage/**/*".to_string()],
        };
        assert!(validate_tool_config("test", &invalid_inputs).is_err());

        Ok(())
    }

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
