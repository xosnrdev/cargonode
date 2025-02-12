use std::{env, path::Path};

use crate::{
    progress,
    template::{self, ProjectType},
    utils::{self, VcsConfig, VcsType},
    Result,
};

use super::config;

fn create_package_config(config: &config::ProjectConfig) -> template::PackageConfig {
    template::PackageConfig {
        name: config.name.clone(),
        project_type: if config.is_binary {
            ProjectType::Binary
        } else {
            ProjectType::Library
        },
        version: None,
    }
}

fn should_use_vcs(vcs_config: &Option<VcsConfig>) -> bool {
    vcs_config
        .as_ref()
        .map(|c| c.vcs_type == VcsType::Git)
        .unwrap_or(true)
}

pub fn create_project(
    path: &Path,
    lib: bool,
    vcs_config: Option<VcsConfig>,
    is_new: bool,
) -> Result<()> {
    let has_vcs = should_use_vcs(&vcs_config);

    // Validate configuration first
    let config = if is_new {
        // For new projects, ensure directory is empty first
        utils::ensure_directory_empty(path)?;
        config::validate_project_config(path, lib, vcs_config)?
    } else {
        config::validate_init_config(path, lib, vcs_config)?
    };

    // Create project structure
    let project_type = if lib { "library" } else { "binary" };
    let action = if is_new { "Creating" } else { "Initializing" };
    progress::write_message(&progress::format_status(
        action,
        &format!("{} package `{}`", project_type, config.name),
    ))?;

    let project_config = utils::create_project_config(&config.path, config.is_binary);
    utils::create_project_structure(&project_config)?;

    // Generate package.json
    let package_config = create_package_config(&config);
    let package_json = template::create_package_json(package_config);
    template::write_package_json(&package_json, &config.path)?;

    // Initialize version control if needed
    if has_vcs {
        if let Some(vcs_config) = config.vcs_config.as_ref() {
            utils::init_vcs(&config.path, vcs_config)?;
        }
    }

    // Show completion message
    progress::write_message(&progress::format_note(
        "See package.json for available scripts and configuration options",
    ))?;

    Ok(())
}

pub fn create_new_project(path: &Path, lib: bool, vcs_config: Option<VcsConfig>) -> Result<()> {
    create_project(path, lib, vcs_config, true)
}

pub fn init_project(lib: bool, vcs_config: Option<VcsConfig>) -> Result<()> {
    let current_dir = env::current_dir()?;
    create_project(&current_dir, lib, vcs_config, false)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_should_use_vcs() {
        assert!(should_use_vcs(&None));
        assert!(should_use_vcs(&Some(VcsConfig::default())));
        assert!(!should_use_vcs(&Some(VcsConfig {
            vcs_type: VcsType::None,
            ..Default::default()
        })));
    }

    #[test]
    fn test_create_package_config() {
        let temp_dir = TempDir::new().unwrap();
        let config = config::ProjectConfig {
            name: "test-pkg".to_string(),
            path: temp_dir.path().to_path_buf(),
            is_binary: true,
            vcs_config: None,
        };

        let pkg_config = create_package_config(&config);
        assert_eq!(pkg_config.name, "test-pkg");
        assert!(matches!(pkg_config.project_type, ProjectType::Binary));
        assert!(pkg_config.version.is_none());
    }

    #[test]
    fn test_create_project_new() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("new-project");

        // Create project with VCS disabled
        let vcs_config = Some(VcsConfig {
            vcs_type: VcsType::None,
            ignore_content: String::new(),
        });

        assert!(create_project(&path, false, vcs_config, true).is_ok());
        assert!(path.exists());
        assert!(path.join("package.json").exists());
        assert!(path.join("src").exists());
    }

    #[test]
    fn test_create_project_init() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("init-project");
        std::fs::create_dir(&path).unwrap();

        // Create project with VCS disabled
        let vcs_config = Some(VcsConfig {
            vcs_type: VcsType::None,
            ignore_content: String::new(),
        });

        assert!(create_project(&path, true, vcs_config, false).is_ok());
        assert!(path.exists());
        assert!(path.join("package.json").exists());
        assert!(path.join("src").exists());
    }
}
