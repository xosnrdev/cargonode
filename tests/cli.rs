use std::{fs, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup_temp_project() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let package_json = r#"{
        "name": "test-project",
        "version": "1.0.0",
        "description": "Test project",
        "main": "index.js",
        "scripts": {
            "test": "echo \"Error: no test specified\" && exit 1"
        }
    }"#;
    fs::write(temp_dir.path().join("package.json"), package_json)
        .expect("Failed to write package.json");
    temp_dir
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("cargonode").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("cargonode").unwrap();
    cmd.arg("invalid-command");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn test_config_file() {
    let temp_dir = setup_temp_project();
    let config_content = r#"{
        "cargonode": {
            "build": {
                "executable": "echo",
                "subcommand": "test",
                "args": [],
                "working-dir": ".",
                "steps": []
            }
        }
    }"#;
    fs::write(temp_dir.path().join("config.json"), config_content)
        .expect("Failed to write config file");
    let mut cmd = Command::cargo_bin("cargonode").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("build")
        .arg("-c")
        .arg("config.json");
    cmd.assert().success();
}
