use std::fs;
use std::thread;
use tempfile::tempdir;

use cargonode::{
    core::package::{PackageOptions, WorkspaceConfig},
    ops::{init::init, new::create_package},
    util::platform,
};

#[test]
fn test_end_to_end_binary_creation() {
    let temp = tempdir().unwrap();
    let mut opts = PackageOptions::new(temp.path());
    opts.set_bin(true).set_typescript(true).set_vcs(true);

    // Create package
    create_package(&opts).unwrap();

    // Verify structure
    let package_json = temp.path().join("package.json");
    assert!(package_json.exists());
    let content = fs::read_to_string(package_json).unwrap();
    assert!(content.contains("\"bin\""));
    assert!(content.contains("\"typescript\""));

    // Verify executable permissions
    let main_file = temp.path().join("src").join("main.ts");
    assert!(main_file.exists());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::metadata(&main_file).unwrap().permissions();
        assert!(perms.mode() & 0o111 != 0);
    }

    // Verify Git initialization
    assert!(temp.path().join(".git").exists());
}

#[test]
fn test_end_to_end_workspace_creation() {
    let temp = tempdir().unwrap();
    let temp_path = temp.path().to_path_buf(); // Keep a reference to the path
    let mut opts = PackageOptions::new(&temp_path);
    opts.workspace = true;
    opts.workspace_config = Some(WorkspaceConfig {
        patterns: vec!["packages/*".to_string(), "apps/*".to_string()],
        inherit_scripts: true,
        hoist_dependencies: true,
    });

    // Create workspace
    create_package(&opts).unwrap();

    // Verify workspace structure
    assert!(temp_path.join("packages").exists());
    assert!(temp_path.join("apps").exists());

    // Create a package in the workspace
    let pkg_path = temp_path.join("packages").join("test-pkg");
    let mut pkg_opts = PackageOptions::new(&pkg_path);
    pkg_opts.set_lib(true).set_typescript(true);
    create_package(&pkg_opts).unwrap();

    // Verify package structure
    assert!(pkg_path.join("package.json").exists());

    // Debug: Print the contents of the src directory
    if let Ok(entries) = std::fs::read_dir(pkg_path.join("src")) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("Found file: {:?}", entry.path());
            }
        }
    }

    assert!(pkg_path.join("src").join("lib.ts").exists());

    // Keep the directory around for inspection
    std::mem::forget(temp);
}

#[test]
fn test_cross_platform_compatibility() {
    let temp = tempdir().unwrap();
    let mut opts = PackageOptions::new(temp.path());
    opts.set_bin(true);

    // Create package
    create_package(&opts).unwrap();

    // Test path handling
    let main_path = temp.path().join("src").join("main.js");
    assert!(main_path.exists());
    let normalized = platform::normalize_path(&main_path.to_string_lossy());
    #[cfg(windows)]
    assert!(normalized.contains('\\'));
    #[cfg(not(windows))]
    assert!(normalized.contains('/'));

    // Test line endings
    let content = fs::read_to_string(main_path).unwrap();
    #[cfg(windows)]
    assert!(content.contains("\r\n"));
    #[cfg(not(windows))]
    assert!(!content.contains("\r\n"));
}

#[test]
fn test_error_handling() {
    let temp = tempdir().unwrap();

    // Test duplicate package creation
    let opts = PackageOptions::new(temp.path());
    create_package(&opts).unwrap();
    let result = create_package(&opts);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already exists"));

    // Test invalid path
    let invalid_opts = PackageOptions::new("/nonexistent/path");
    let result = create_package(&invalid_opts);
    assert!(result.is_err());
    #[cfg(unix)]
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Read-only file system"));

    // Test workspace package conflict
    let mut ws_opts = PackageOptions::new(temp.path());
    ws_opts.workspace = true;
    let result = init(&ws_opts);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already exists"));
}

#[test]
fn test_security_features() {
    let temp = tempdir().unwrap();

    // Test path traversal prevention
    let path = temp.path().join("test-pkg");
    fs::create_dir_all(&path).unwrap();
    let opts = PackageOptions::new(path.join("../outside"));
    platform::set_allow_symlinks(false); // Ensure symlinks are not allowed
    let result = create_package(&opts);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("traversal"));

    // Test symlink handling
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let target = temp.path().join("target");
        let link = temp.path().join("link");
        fs::create_dir(&target).unwrap();
        symlink(&target, &link).unwrap();
        platform::set_allow_symlinks(false); // Ensure symlinks are not allowed

        let opts = PackageOptions::new(link);
        let result = create_package(&opts);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Symlinks"));
    }
}

#[test]
fn test_concurrent_package_creation() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let temp = tempdir().unwrap();
                let mut opts = PackageOptions::new(temp.path().join(format!("pkg-{}", i)));
                opts.set_bin(true).set_typescript(true).set_vcs(true);
                create_package(&opts)
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap().unwrap();
    }
}
