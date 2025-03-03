use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use crate::cache::{Cache, CacheEntry};
use crate::config::{get_tool_config, ToolConfig};
use crate::error::Error;
use crate::inputs::InputTracker;
use crate::journal::Journal;
use crate::outputs::OutputVerifier;
use crate::progress;
use crate::Result;

/// Options for running a tool
pub struct RunOptions {
    /// Project directory
    pub project_dir: PathBuf,

    /// Cache directory
    pub cache_dir: PathBuf,

    /// Journal directory
    pub journal_dir: PathBuf,

    /// Whether to force execution even if cached
    pub force: bool,

    /// Whether to print verbose output
    pub verbose: bool,

    /// Maximum number of journal entries to keep
    pub max_journal_entries: usize,
}

/// Result of running a tool
pub struct RunResult {
    /// Exit status of the command
    pub status: ExitStatus,

    /// Whether the result was from cache
    pub from_cache: bool,

    /// Input hash used for caching
    pub input_hash: String,
}

/// Run a tool with the given options
///
/// # Arguments
///
/// * `tool_name` - Name of the tool to run
/// * `config` - Cargonode configuration
/// * `options` - Run options
///
/// # Returns
///
/// * `Result<RunResult>` - Result of running the tool
pub fn run_tool(
    tool_name: &str,
    config: &crate::config::CargonodeConfig,
    options: &RunOptions,
) -> Result<RunResult> {
    // Get tool configuration
    let tool_config = get_tool_config(config, tool_name).ok_or_else(|| Error::Config {
        message: format!("Tool '{}' not found in configuration", tool_name),
    })?;

    // Validate tool configuration
    crate::config::validate_tool_config(tool_name, tool_config)?;

    // Calculate input hash
    let input_hash = calculate_input_hash(tool_name, tool_config, &options.project_dir)?;

    // Create journal
    let journal = Journal::new(&options.journal_dir, options.max_journal_entries)?;

    // Check cache if not forced
    let mut from_cache = false;
    let status = if !options.force {
        if let Some(result) = check_cache(tool_name, &input_hash, &options.cache_dir)? {
            if options.verbose {
                progress::write_message(&progress::format_note(&format!(
                    "Using cached result for tool '{}' with input hash '{}'",
                    tool_name, input_hash
                )))?;
            }

            from_cache = true;

            #[cfg(unix)]
            let status = ExitStatus::from_raw(result.exit_code);

            #[cfg(windows)]
            let status = {
                // On Windows, we need to create a fake command to get an ExitStatus
                // This is a workaround since Windows doesn't have from_raw
                if result.exit_code == 0 {
                    // For success case, use a simple echo command
                    Command::new("cmd").args(&["/C", "exit", "0"]).status()?
                } else {
                    // For failure case, use a simple echo command with non-zero exit code
                    Command::new("cmd").args(&["/C", "exit", "1"]).status()?
                }
            };

            status
        } else {
            // Execute command
            execute_command(
                tool_name,
                tool_config,
                &options.project_dir,
                options.verbose,
            )?
        }
    } else {
        // Force execution
        execute_command(
            tool_name,
            tool_config,
            &options.project_dir,
            options.verbose,
        )?
    };

    // Verify outputs if command succeeded and not from cache
    if status.success() && !from_cache && !tool_config.outputs.is_empty() {
        if options.verbose {
            progress::write_message(&progress::format_note(&format!(
                "Verifying outputs for tool '{}'",
                tool_name
            )))?;
        }

        // Create output verifier
        let verifier = OutputVerifier::new(&options.project_dir, tool_config.outputs.clone());

        // Verify outputs
        match verifier.verify_outputs() {
            Ok(outputs) => {
                if options.verbose {
                    progress::write_message(&progress::format_note(&format!(
                        "Found {} output files for tool '{}'",
                        outputs.len(),
                        tool_name
                    )))?;
                }
            }
            Err(e) => {
                // Log error to journal
                let entry = crate::journal::Journal::create_entry(
                    tool_name,
                    &input_hash,
                    &tool_config.command,
                    &tool_config.args,
                    1, // Force non-zero exit code for output verification failure
                    false,
                );
                journal.add_entry(entry)?;

                return Err(e);
            }
        }
    }

    // Cache result if not from cache
    if !from_cache {
        cache_result(
            tool_name,
            &input_hash,
            tool_config,
            status,
            &options.cache_dir,
        )?;
    }

    // Create journal entry
    let entry = crate::journal::Journal::create_entry(
        tool_name,
        &input_hash,
        &tool_config.command,
        &tool_config.args,
        status.code().unwrap_or(1),
        from_cache,
    );

    // Add entry to journal
    journal.add_entry(entry)?;

    // Return result
    Ok(RunResult {
        status,
        from_cache,
        input_hash,
    })
}

/// Calculate hash of input files for a tool
///
/// # Arguments
///
/// * `tool_name` - Name of the tool
/// * `config` - Tool configuration
/// * `project_dir` - Project directory
///
/// # Returns
///
/// * `Result<String>` - Hash of input files
fn calculate_input_hash(
    _tool_name: &str,
    config: &ToolConfig,
    project_dir: &Path,
) -> Result<String> {
    // Create input tracker
    let tracker = InputTracker::new(project_dir, config.inputs.clone());

    // Calculate hash
    let hash = tracker.calculate_hash()?;

    Ok(hash)
}

/// Check if a cached result exists for a tool
///
/// # Arguments
///
/// * `tool_name` - Name of the tool
/// * `input_hash` - Hash of input files
/// * `cache_dir` - Cache directory
///
/// # Returns
///
/// * `Result<Option<CacheEntry>>` - Cached entry if found
fn check_cache(tool_name: &str, input_hash: &str, cache_dir: &Path) -> Result<Option<CacheEntry>> {
    // Create cache
    let cache = Cache::new(cache_dir)?;

    // Check if entry exists
    if !cache.has_entry(tool_name, input_hash) {
        return Ok(None);
    }

    // Get entry
    let entry = cache.get_entry(tool_name, input_hash)?;

    Ok(entry)
}

/// Execute a command
///
/// # Arguments
///
/// * `tool_name` - Name of the tool
/// * `config` - Tool configuration
/// * `project_dir` - Project directory
/// * `verbose` - Whether to print verbose output
///
/// # Returns
///
/// * `Result<ExitStatus>` - Exit status of the command
fn execute_command(
    tool_name: &str,
    config: &ToolConfig,
    project_dir: &Path,
    verbose: bool,
) -> Result<ExitStatus> {
    // Determine working directory
    let working_dir = if let Some(dir) = &config.working_dir {
        project_dir.join(dir)
    } else {
        project_dir.to_path_buf()
    };

    // Verify working directory exists
    if !working_dir.exists() {
        return Err(Error::Config {
            message: format!(
                "Working directory '{}' for tool '{}' does not exist",
                working_dir.display(),
                tool_name
            ),
        });
    }

    // Verify working directory is a directory
    if !working_dir.is_dir() {
        return Err(Error::Config {
            message: format!(
                "Working directory '{}' for tool '{}' is not a directory",
                working_dir.display(),
                tool_name
            ),
        });
    }

    // Build command
    let mut command = Command::new(&config.command);

    // Set working directory
    command.current_dir(&working_dir);

    // Add arguments
    command.args(&config.args);

    // Add environment variables
    for (key, value) in &config.env {
        command.env(key, value);
    }

    // Print command if verbose
    if verbose {
        println!("Executing: {} {}", config.command, config.args.join(" "));
    }

    // Execute command
    let output = command.output()?;

    // Print output if verbose
    if verbose {
        if !output.stdout.is_empty() {
            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        }

        if !output.stderr.is_empty() {
            eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(output.status)
}

/// Cache the result of a command
///
/// # Arguments
///
/// * `tool_name` - Name of the tool
/// * `input_hash` - Hash of input files
/// * `config` - Tool configuration
/// * `status` - Exit status of the command
/// * `cache_dir` - Cache directory
///
/// # Returns
///
/// * `Result<()>` - Whether the operation succeeded
fn cache_result(
    tool_name: &str,
    input_hash: &str,
    config: &ToolConfig,
    status: ExitStatus,
    cache_dir: &Path,
) -> Result<()> {
    // Skip caching if disabled
    if !config.cache {
        return Ok(());
    }

    // Create cache
    let cache = Cache::new(cache_dir)?;

    // Create entry
    let entry = Cache::create_entry(
        tool_name,
        input_hash,
        &config.command,
        &config.args,
        status.code().unwrap_or(0),
    );

    // Store entry
    cache.store_entry(&entry)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    use crate::config::CargonodeConfig;

    use super::*;

    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> Result<PathBuf> {
        let file_path = dir.join(name);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content)?;
        Ok(file_path)
    }

    fn create_test_config() -> (ToolConfig, CargonodeConfig) {
        let mut tools = HashMap::new();

        // Create a tool config that runs "echo" command
        let tool_config = ToolConfig {
            command: if cfg!(windows) {
                "cmd".to_string()
            } else {
                "echo".to_string()
            },
            args: if cfg!(windows) {
                vec!["/C".to_string(), "echo".to_string(), "test".to_string()]
            } else {
                vec!["test".to_string()]
            },
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["*.txt".to_string()],
            outputs: vec!["*.out".to_string()],
            cache: true,
        };

        tools.insert("test-tool".to_string(), tool_config.clone());

        let config = CargonodeConfig { tools };

        (tool_config, config)
    }

    #[test]
    fn test_calculate_input_hash() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        // Create test files
        create_test_file(dir_path, "file1.txt", b"content1")?;
        create_test_file(dir_path, "file2.txt", b"content2")?;

        let (tool_config, _) = create_test_config();

        // Calculate hash
        let hash = calculate_input_hash("test-tool", &tool_config, dir_path)?;

        // Hash should not be empty
        assert!(!hash.is_empty());

        // Calculate hash again (should be the same)
        let hash2 = calculate_input_hash("test-tool", &tool_config, dir_path)?;
        assert_eq!(hash, hash2);

        // Modify a file and check that hash changes
        create_test_file(dir_path, "file1.txt", b"modified content")?;
        let hash3 = calculate_input_hash("test-tool", &tool_config, dir_path)?;
        assert_ne!(hash, hash3);

        Ok(())
    }

    #[test]
    fn test_check_cache() -> Result<()> {
        let temp_dir = tempdir()?;
        let cache_dir = temp_dir.path();

        // Create cache
        let cache = Cache::new(cache_dir)?;

        // Check that entry doesn't exist
        let result = check_cache("test-tool", "test-hash", cache_dir)?;
        assert!(result.is_none());

        // Create and store entry
        let entry = Cache::create_entry("test-tool", "test-hash", "echo", &["test".to_string()], 0);

        cache.store_entry(&entry)?;

        // Check that entry exists
        let result = check_cache("test-tool", "test-hash", cache_dir)?;
        assert!(result.is_some());

        let retrieved = result.unwrap();
        assert_eq!(retrieved.tool_name, "test-tool");
        assert_eq!(retrieved.input_hash, "test-hash");

        Ok(())
    }

    #[test]
    fn test_execute_command() -> Result<()> {
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        let (tool_config, _) = create_test_config();

        // Execute command
        let status = execute_command("test-tool", &tool_config, dir_path, false)?;

        // Command should succeed
        assert!(status.success());

        Ok(())
    }

    #[test]
    fn test_cache_result() -> Result<()> {
        let temp_dir = tempdir()?;
        let cache_dir = temp_dir.path();

        let (tool_config, _) = create_test_config();

        // Create a mock exit status
        let status = if cfg!(windows) {
            ExitStatus::from_raw(0)
        } else {
            // On Unix, we can use the raw exit status
            ExitStatus::from_raw(0)
        };

        // Cache result
        cache_result("test-tool", "test-hash", &tool_config, status, cache_dir)?;

        // Check that entry exists
        let cache = Cache::new(cache_dir)?;
        assert!(cache.has_entry("test-tool", "test-hash"));

        Ok(())
    }

    #[test]
    fn test_run_tool() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        // Create cache directory
        let cache_dir = dir_path.join("cache");
        fs::create_dir_all(&cache_dir)?;

        // Create journal directory
        let journal_dir = dir_path.join("journal");
        fs::create_dir_all(&journal_dir)?;

        // Create a test file
        let test_file = dir_path.join("test.txt");
        let mut file = File::create(&test_file)?;
        file.write_all(b"test content")?;

        // Create expected output file
        let output_file = dir_path.join("test.out");
        let mut file = File::create(&output_file)?;
        file.write_all(b"test output")?;

        // Create a test tool configuration
        let tool_config = ToolConfig {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["*.txt".to_string()],
            outputs: vec!["*.out".to_string()],
            cache: true,
        };

        // Create a test configuration
        let mut tools = HashMap::new();
        tools.insert("test-tool".to_string(), tool_config);
        let config = CargonodeConfig { tools };

        // Create run options
        let options = RunOptions {
            project_dir: dir_path.to_path_buf(),
            cache_dir: cache_dir.clone(),
            journal_dir: journal_dir.clone(),
            force: false,
            verbose: false,
            max_journal_entries: 100,
        };

        // Run the tool
        let result = run_tool("test-tool", &config, &options)?;

        // Check result
        assert!(result.status.success());
        assert!(!result.from_cache);

        // Run the tool again (should use cache)
        let result = run_tool("test-tool", &config, &options)?;
        assert!(result.status.success());
        assert!(result.from_cache);

        // Run the tool with force (should not use cache)
        let options = RunOptions {
            project_dir: dir_path.to_path_buf(),
            cache_dir,
            journal_dir,
            force: true,
            verbose: false,
            max_journal_entries: 100,
        };

        let result = run_tool("test-tool", &config, &options)?;
        assert!(result.status.success());
        assert!(!result.from_cache);

        Ok(())
    }

    /// Test output verification
    #[test]
    fn test_output_verification() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        // Create cache directory
        let cache_dir = dir_path.join("cache");
        fs::create_dir_all(&cache_dir)?;

        // Create journal directory
        let journal_dir = dir_path.join("journal");
        fs::create_dir_all(&journal_dir)?;

        // Create a test tool configuration with output verification
        let tool_config = crate::config::ToolConfig {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["*.txt".to_string()],
            outputs: vec!["test-output.txt".to_string()],
            cache: true,
        };

        // Create a test configuration
        let mut tools = HashMap::new();
        tools.insert("test-tool".to_string(), tool_config);
        let config = crate::config::CargonodeConfig { tools };

        // Create run options
        let options = RunOptions {
            project_dir: dir_path.to_path_buf(),
            cache_dir: cache_dir.clone(),
            journal_dir: journal_dir.clone(),
            force: false,
            verbose: false,
            max_journal_entries: 100,
        };

        // Create a test input file
        let input_file = dir_path.join("test.txt");
        let _file = File::create(&input_file)?;

        // Run the tool (should fail due to missing output)
        let result = run_tool("test-tool", &config, &options);
        assert!(result.is_err());

        // Check error type
        match result {
            Err(Error::OutputNotFound { patterns }) => {
                assert_eq!(patterns.len(), 1);
                assert_eq!(patterns[0], "test-output.txt");
            }
            _ => panic!("Expected OutputNotFound error"),
        }

        // Create the expected output file
        let output_file = dir_path.join("test-output.txt");
        let _file = File::create(&output_file)?;

        // Run the tool again (should succeed)
        let result = run_tool("test-tool", &config, &options)?;
        assert!(result.status.success());
        assert!(!result.from_cache);

        // Run the tool again (should use cache)
        let result = run_tool("test-tool", &config, &options)?;
        assert!(result.status.success());
        assert!(result.from_cache);

        Ok(())
    }
}
