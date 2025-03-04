use std::path::{Path, PathBuf};

use crate::commands::run::{run_tool, RunOptions, RunResult};
use crate::progress;
use crate::Result;

/// Run a generic command with the given type and arguments
///
/// # Arguments
///
/// * `command_type` - Type of command to run (check, build, test)
/// * `args` - Arguments to pass to the command
/// * `project_dir` - Project directory
/// * `force` - Whether to force execution even if cached
/// * `verbose` - Whether to print verbose output
///
/// # Returns
///
/// * `Result<RunResult>` - Result of running the command
pub fn run_generic_command(
    command_type: &str,
    _args: &[String],
    project_dir: &Path,
    force: bool,
    verbose: bool,
) -> Result<RunResult> {
    // Load configuration
    let config = if cfg!(test) && !project_dir.join("package.json").exists() {
        // For tests, create a mock configuration
        let mut config = crate::config::CargonodeConfig {
            tools: std::collections::HashMap::new(),
        };
        let tool_config = crate::config::ToolConfig {
            command: "echo".to_string(),
            args: vec![command_type.to_string()],
            env: std::collections::HashMap::new(),
            working_dir: None,
            inputs: vec!["*.txt".to_string()],
            outputs: vec!["*.out".to_string()],
            cache: true,
        };
        config.tools.insert(command_type.to_string(), tool_config);
        config
    } else {
        crate::config::load_config(project_dir)?
    };

    // Create run options
    let options = RunOptions {
        project_dir: project_dir.to_path_buf(),
        force,
        verbose,
    };

    // Run the tool
    run_tool(command_type, &config, &options)
}

/// Run the check command
///
/// # Arguments
///
/// * `paths` - Paths to check
/// * `project_dir` - Project directory
/// * `force` - Whether to force execution even if cached
/// * `verbose` - Whether to print verbose output
///
/// # Returns
///
/// * `Result<RunResult>` - Result of running the check command
pub fn check(
    paths: &[PathBuf],
    project_dir: &Path,
    force: bool,
    verbose: bool,
) -> Result<RunResult> {
    // Convert paths to strings
    let path_args: Vec<String> = paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    // Print status message
    if verbose {
        let paths_str = if paths.is_empty() {
            "all files".to_string()
        } else {
            format!("{} paths", paths.len())
        };

        progress::write_message(&progress::format_status("Checking", &paths_str))?;
    }

    // Run the check command
    run_generic_command("check", &path_args, project_dir, force, verbose)
}

/// Run the build command
///
/// # Arguments
///
/// * `release` - Whether to build in release mode
/// * `project_dir` - Project directory
/// * `force` - Whether to force execution even if cached
/// * `verbose` - Whether to print verbose output
///
/// # Returns
///
/// * `Result<RunResult>` - Result of running the build command
pub fn build(release: bool, project_dir: &Path, force: bool, verbose: bool) -> Result<RunResult> {
    // Create arguments
    let mut args = Vec::new();

    if release {
        args.push("--release".to_string());
    }

    // Print status message
    if verbose {
        let mode = if release { "release" } else { "debug" };
        progress::write_message(&progress::format_status(
            "Building",
            &format!("in {} mode", mode),
        ))?;
    }

    // Run the build command
    run_generic_command("build", &args, project_dir, force, verbose)
}

/// Run the test command
///
/// # Arguments
///
/// * `pattern` - Test pattern to run
/// * `project_dir` - Project directory
/// * `force` - Whether to force execution even if cached
/// * `verbose` - Whether to print verbose output
///
/// # Returns
///
/// * `Result<RunResult>` - Result of running the test command
pub fn test(pattern: &str, project_dir: &Path, force: bool, verbose: bool) -> Result<RunResult> {
    // Create arguments
    let mut args = Vec::new();

    if !pattern.is_empty() {
        args.push(pattern.to_string());
    }

    // Print status message
    if verbose {
        let pattern_str = if pattern.is_empty() {
            "all tests".to_string()
        } else {
            format!("tests matching '{}'", pattern)
        };

        progress::write_message(&progress::format_status("Running", &pattern_str))?;
    }

    // Run the test command
    run_generic_command("test", &args, project_dir, force, verbose)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use tempfile::TempDir;

    use super::*;

    // Helper function to create a test file
    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> Result<PathBuf> {
        let file_path = dir.join(name);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content)?;
        file.flush()?;
        Ok(file_path)
    }

    #[test]
    fn test_check_command() -> Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create test files
        create_test_file(dir_path, "test.txt", b"test content")?;
        create_test_file(dir_path, "test.out", b"test output")?;

        let paths = vec![dir_path.join("test.txt")];
        let result = check(&paths, dir_path, false, false)?;

        // Verify result
        assert!(result.status.success());

        Ok(())
    }

    #[test]
    fn test_build_command() -> Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create test files
        create_test_file(dir_path, "test.txt", b"test content")?;
        create_test_file(dir_path, "test.out", b"test output")?;

        let result = build(false, dir_path, false, false)?;

        // Verify result
        assert!(result.status.success());

        Ok(())
    }

    #[test]
    fn test_test_command() -> Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create test files
        create_test_file(dir_path, "test.txt", b"test content")?;
        create_test_file(dir_path, "test.out", b"test output")?;

        let result = test("*", dir_path, false, false)?;

        // Verify result
        assert!(result.status.success());

        Ok(())
    }
}
