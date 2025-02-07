use std::path::PathBuf;

use cargonode::core::package::{PackageOptions, WorkspaceConfig};

#[test]
fn test_package_options_new() {
    let path = PathBuf::from("test-pkg");
    let opts = PackageOptions::new(&path);
    assert_eq!(opts.path, path);
    assert!(opts.is_bin());
    assert!(!opts.is_lib());
    assert!(opts.vcs_enabled());
    assert!(!opts.workspace);
    assert!(!opts.is_typescript());
    assert!(opts.workspace_config.is_none());
}

#[test]
fn test_package_options_setters() {
    let path = PathBuf::from("test-pkg");
    let mut opts = PackageOptions::new(&path);

    opts.set_bin(true);
    assert!(opts.is_bin());

    opts.set_lib(true);
    assert!(opts.is_lib());

    opts.set_vcs(false);
    assert!(!opts.vcs_enabled());

    opts.set_typescript(true);
    assert!(opts.is_typescript());

    opts.workspace = true;
    assert!(opts.workspace);
}

#[test]
fn test_package_name_from_path() {
    let opts = PackageOptions::new("my-awesome-package");
    assert_eq!(opts.package_name(), "my-awesome-package");

    let mut opts = PackageOptions::new("test");
    opts.name = Some("custom-name".to_string());
    assert_eq!(opts.package_name(), "custom-name");
}

#[test]
fn test_workspace_config_default() {
    let config = WorkspaceConfig::default();
    assert_eq!(config.patterns, vec!["packages/*".to_string()]);
    assert!(config.inherit_scripts);
    assert!(config.hoist_dependencies);
}

#[test]
fn test_package_options_with_workspace() {
    let path = PathBuf::from("test-pkg");
    let mut opts = PackageOptions::new(&path);
    opts.workspace = true;
    opts.workspace_config = Some(WorkspaceConfig {
        patterns: vec!["packages/*".to_string(), "apps/*".to_string()],
        inherit_scripts: false,
        hoist_dependencies: false,
    });

    assert!(opts.workspace);
    let config = opts.workspace_config.unwrap();
    assert_eq!(config.patterns.len(), 2);
    assert!(!config.inherit_scripts);
    assert!(!config.hoist_dependencies);
}
