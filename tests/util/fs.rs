use std::fs;
use tempfile::tempdir;

use cargonode::util::fs::{
    find_workspace_packages, find_workspace_root, get_package_name, init_git_repository,
    set_executable_permissions, write_with_line_endings, FsCache,
};

#[test]
fn test_fs_cache() {
    let temp = tempdir().unwrap();
    let mut cache = FsCache::new();

    // Test git repo caching
    let result1 = cache.is_git_repo(temp.path()).unwrap();
    let result2 = cache.is_git_repo(temp.path()).unwrap();
    assert_eq!(result1, result2);
}

#[test]
fn test_line_endings() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("test.txt");
    let content = "line1\nline2\n";
    write_with_line_endings(&path, content).unwrap();

    let written = fs::read_to_string(&path).unwrap();
    if cfg!(windows) {
        assert!(written.contains("\r\n"));
    } else {
        assert!(!written.contains("\r\n"));
    }
}

#[test]
fn test_executable_permissions() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("test.sh");
    fs::write(&path, "#!/bin/sh\necho test").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        set_executable_permissions(&path).unwrap();
        let perms = fs::metadata(&path).unwrap().permissions();
        assert!(perms.mode() & 0o111 != 0);
    }
}

#[test]
fn test_git_repository() {
    let temp = tempdir().unwrap();
    init_git_repository(temp.path()).unwrap();
    assert!(temp.path().join(".git").exists());
}

#[test]
fn test_find_workspace_root() {
    let temp = tempdir().unwrap();
    let root = temp.path();

    // Create workspace package.json
    fs::write(
        root.join("package.json"),
        r#"{"workspaces": ["packages/*"]}"#,
    )
    .unwrap();

    // Create a subdirectory
    let subdir = root.join("packages/test");
    fs::create_dir_all(&subdir).unwrap();

    // Test finding workspace root from subdirectory
    let found_root = find_workspace_root(&subdir).unwrap();
    assert_eq!(found_root, root);
}

#[test]
fn test_find_workspace_packages() {
    let temp = tempdir().unwrap();
    let root = temp.path();

    // Create workspace structure
    fs::create_dir_all(root.join("packages/pkg1")).unwrap();
    fs::create_dir_all(root.join("packages/pkg2")).unwrap();

    // Create package.json files
    fs::write(
        root.join("packages/pkg1/package.json"),
        r#"{"name": "pkg1"}"#,
    )
    .unwrap();
    fs::write(
        root.join("packages/pkg2/package.json"),
        r#"{"name": "pkg2"}"#,
    )
    .unwrap();

    let packages = find_workspace_packages(root).unwrap();
    assert_eq!(packages.len(), 2);
    assert!(packages.iter().any(|p| p.name == "pkg1"));
    assert!(packages.iter().any(|p| p.name == "pkg2"));
}

#[test]
fn test_get_package_name() {
    assert_eq!(get_package_name("my-package".as_ref()), "my_package");
    assert_eq!(get_package_name("my package".as_ref()), "my_package");
    assert_eq!(get_package_name("my_package".as_ref()), "my_package");
}
