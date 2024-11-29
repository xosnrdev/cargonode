use std::env;
use std::fs;
use std::io;

use tempfile::tempdir;

use cargonode::package::{
    download_file, extract_zip, replace_placeholders, validate_package_name, Config, Error,
    Package, Template, TemplateInfo,
};

/// Tests for package name validation
#[test]
fn test_valid_package_names() {
    // Should pass
    assert!(validate_package_name("my-package").is_ok());
    assert!(validate_package_name("my_package").is_ok());
    assert!(validate_package_name("mypackage").is_ok());
}

#[test]
fn test_invalid_package_names() {
    // Should fail
    assert!(
        validate_package_name("").is_err(),
        "Empty name should be invalid"
    );
    assert!(
        validate_package_name("My-Package").is_err(),
        "Uppercase letters should be invalid"
    );
    assert!(
        validate_package_name("-package").is_err(),
        "Starting with hyphen should be invalid"
    );
    assert!(
        validate_package_name("package-").is_err(),
        "Ending with hyphen should be invalid"
    );
    assert!(
        validate_package_name("package name").is_err(),
        "Spaces should be invalid"
    );
    assert!(
        validate_package_name("package!").is_err(),
        "Special characters should be invalid"
    );
}

/// Tests for template preparation and download
#[test]
fn test_download_file() {
    let template_info = Template::NodeTypeScript.info();
    let result = download_file(&template_info);

    assert!(result.is_ok(), "Failed to download template");
    let bytes = result.unwrap();
    assert!(!bytes.is_empty(), "Downloaded file is empty");
}

#[test]
fn test_extract_zip() {
    let template_info = Template::NodeTypeScript.info();
    let temp_dir = tempdir().expect("Failed to create temp directory");

    let bytes = download_file(&template_info).expect("Failed to download file");
    let result = extract_zip(bytes, temp_dir.path());

    assert!(result.is_ok(), "Failed to extract zip");

    // Verify extraction contents
    let entries: Vec<_> = fs::read_dir(temp_dir.path())
        .expect("Failed to read temp directory")
        .collect();
    assert!(!entries.is_empty(), "No files extracted");
}

/// Tests for placeholder replacement
#[test]
fn test_replace_placeholders() {
    // Create a temporary directory with test files
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_file_path = temp_dir.path().join("node_typescript_test.txt");

    // Create a test file with placeholder
    fs::write(&test_file_path, "project: node_typescript").expect("Failed to write test file");

    let template_info = TemplateInfo {
        url: "",
        path: "",
        placeholder: "node_typescript",
    };

    // Replace placeholders
    let result = replace_placeholders("my-new-package", &template_info, temp_dir.path(), true);

    assert!(result.is_ok(), "Placeholder replacement failed");

    // Verify file content
    let content = fs::read_to_string(&test_file_path).expect("Failed to read test file");
    assert_eq!(content, "project: my-new-package");
}

#[test]
fn test_directory_placeholder_replacement() {
    // Create a temporary directory with test subdirectory
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let node_typescript_dir = temp_dir.path().join("node_typescript_template");
    fs::create_dir_all(&node_typescript_dir).expect("Failed to create test directory");

    let template_info = TemplateInfo {
        url: "",
        path: "",
        placeholder: "node_typescript",
    };

    // Replace placeholders
    let result = replace_placeholders("my-new-package", &template_info, temp_dir.path(), true);

    assert!(result.is_ok(), "Directory placeholder replacement failed");

    // Check if directory was renamed
    let renamed_dir = temp_dir.path().join("my-new-package_template");
    assert!(renamed_dir.exists(), "Directory was not renamed");
}

/// Integration tests for package creation
#[test]
fn test_create_package() {
    let temp_dir = tempdir().expect("Failed to create temp directory");

    let config = Config {
        package_name: "test-package".to_string(),
        current_dir: temp_dir.path().to_path_buf(),
        template: Template::NodeTypeScript,
    };

    let package = Package::new(config);
    let result = package.create_package();

    assert!(
        result.is_ok(),
        "Package creation failed: {:?}",
        result.err()
    );

    // Verify package directory exists
    let package_dir = temp_dir.path().join("test-package");
    assert!(package_dir.exists(), "Package directory not created");

    // Verify key files exist
    assert!(
        package_dir.join("package.json").exists(),
        "package.json missing"
    );
    assert!(
        package_dir.join(".git").exists(),
        "Git repository not initialized"
    );
}

#[test]
fn test_init_package() {
    let temp_dir = tempdir().expect("Failed to create temp directory");

    // Create a directory with a valid name inside the temp directory
    let project_dir = temp_dir.path().join("valid-project");
    fs::create_dir_all(&project_dir).expect("Failed to create project directory");

    let config = Config {
        // Explicitly set a valid name
        package_name: "valid-project".to_string(),
        // Use the directory with a valid name
        current_dir: project_dir.clone(),
        template: Template::NodeTypeScript,
    };

    // Temporarily change the current working directory
    let original_dir = std::env::current_dir().expect("Failed to get current directory");
    env::set_current_dir(&project_dir).expect("Failed to change current directory");

    let package = Package::new(config);
    let result = package.init_package();

    // Restore the original working directory
    env::set_current_dir(original_dir).expect("Failed to restore original directory");

    assert!(
        result.is_ok(),
        "Package initialization failed: {:?}",
        result.err()
    );

    // Verify files exist in current directory
    assert!(
        project_dir.join("package.json").exists(),
        "package.json missing"
    );
    assert!(
        project_dir.join(".git").exists(),
        "Git repository not initialized"
    );
}

/// Error handling tests
#[test]
fn test_error_display() {
    let io_error = Error::Io {
        context: "Test context",
        error: io::Error::new(io::ErrorKind::NotFound, "File not found"),
    };

    let error_string = format!("{}", io_error);
    assert!(
        error_string.contains("Test context"),
        "Error display should include context"
    );

    let invalid_name_error = Error::InvalidPackageName;
    assert_eq!(format!("{}", invalid_name_error), "Invalid package name");
}

#[test]
fn test_invalid_download_url() {
    let invalid_template_info = TemplateInfo {
        url: "https://invalid-url-that-does-not-exist.com/nonexistent.zip",
        path: "templates",
        placeholder: "node_typescript",
    };

    let result = download_file(&invalid_template_info);
    assert!(result.is_err(), "Invalid URL should result in an error");
}
