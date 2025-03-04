use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use crate::config::{self};
use crate::error::Error;
use crate::outputs::OutputVerifier;
use crate::progress;
use crate::Result;

/// Options for running a tool
pub struct RunOptions {
    /// Project directory
    pub project_dir: PathBuf,

    /// Whether to force execution even if cached
    pub force: bool,

    /// Whether to print verbose output
    pub verbose: bool,
}

/// Result of running a tool
pub struct RunResult {
    /// Exit status of the command
    pub status: ExitStatus,
}

/// Run a tool with the given options
pub fn run_tool(
    tool_name: &str,
    config: &config::CargonodeConfig,
    options: &RunOptions,
) -> Result<RunResult> {
    let tool_config = config::get_tool_config(config, tool_name).ok_or_else(|| Error::Config {
        message: format!("Tool '{}' not found in configuration", tool_name),
    })?;

    config::validate_tool_config(tool_name, tool_config)?;

    let status = execute_command(
        tool_name,
        tool_config,
        &options.project_dir,
        options.verbose,
    )?;

    // Only verify outputs if the command succeeded and has output patterns defined
    if status.success() && !tool_config.outputs.is_empty() {
        if options.verbose {
            progress::write_message(&progress::format_note(&format!(
                "Verifying outputs for tool '{}'",
                tool_name
            )))?;
        }

        let verifier = OutputVerifier::new(&options.project_dir, tool_config.outputs.clone());

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
            Err(e) => return Err(e),
        }
    }

    Ok(RunResult { status })
}

/// Execute a command
fn execute_command(
    tool_name: &str,
    config: &config::ToolConfig,
    project_dir: &Path,
    verbose: bool,
) -> Result<ExitStatus> {
    let working_dir = if let Some(dir) = &config.working_dir {
        project_dir.join(dir)
    } else {
        project_dir.to_path_buf()
    };

    if !working_dir.exists() {
        return Err(Error::Config {
            message: format!(
                "Working directory '{}' for tool '{}' does not exist",
                working_dir.display(),
                tool_name
            ),
        });
    }
    if !working_dir.is_dir() {
        return Err(Error::Config {
            message: format!(
                "Working directory '{}' for tool '{}' is not a directory",
                working_dir.display(),
                tool_name
            ),
        });
    }

    let mut command = Command::new(&config.command);
    command.current_dir(&working_dir);
    command.args(&config.args);

    for (key, value) in &config.env {
        command.env(key, value);
    }

    // Format command for display
    let command_str = format!("{} {}", config.command, config.args.join(" "));

    if verbose {
        progress::write_message(&progress::format_status("Running", &command_str))?;
    }

    let output = command.output()?;

    // Handle command output
    if verbose || !output.status.success() {
        if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let formatted = if verbose {
                format!("\n{}", stdout)
            } else {
                // When not verbose, only show last few lines
                stdout
                    .lines()
                    .rev()
                    .take(5)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            progress::write_message(&progress::format_note("Command output:"))?;
            println!("{}", formatted);
        }

        if !output.stderr.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let formatted = if verbose {
                format!("\n{}", stderr)
            } else {
                // When not verbose, only show last few lines
                stderr
                    .lines()
                    .rev()
                    .take(5)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            progress::write_message(&progress::format_error("Command error output:"))?;
            eprintln!("{}", formatted);
        }
    }

    if !output.status.success() {
        return Err(Error::CommandFailed {
            command: command_str,
            status: output.status,
        });
    }

    Ok(output.status)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_run_tool() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        // Create a test file
        let test_file = dir_path.join("test.txt");
        let mut file = File::create(&test_file)?;
        file.write_all(b"test content")?;

        // Create expected output file
        let output_file = dir_path.join("test.out");
        let mut file = File::create(&output_file)?;
        file.write_all(b"test output")?;

        // Create a test tool configuration
        let tool_config = config::ToolConfig {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["*.txt".to_string()],
            outputs: vec!["*.out".to_string()],
        };

        // Create a test configuration
        let mut tools = HashMap::new();
        tools.insert("test-tool".to_string(), tool_config);
        let config = config::CargonodeConfig { tools };

        // Create run options
        let options = RunOptions {
            project_dir: dir_path.to_path_buf(),
            force: false,
            verbose: false,
        };

        // Run the tool
        let result = run_tool("test-tool", &config, &options)?;

        // Check result
        assert!(result.status.success());

        Ok(())
    }

    /// Test output verification
    #[test]
    fn test_output_verification() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        // Create a test tool configuration with output verification
        let tool_config = config::ToolConfig {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            env: HashMap::new(),
            working_dir: None,
            inputs: vec!["*.txt".to_string()],
            outputs: vec!["subdir/test-output.txt".to_string()],
        };

        // Create a test configuration
        let mut tools = HashMap::new();
        tools.insert("test-tool".to_string(), tool_config);
        let config = config::CargonodeConfig { tools };

        // Create run options
        let options = RunOptions {
            project_dir: dir_path.to_path_buf(),
            force: false,
            verbose: false,
        };

        // Run the tool (should succeed and create directory)
        let result = run_tool("test-tool", &config, &options)?;
        assert!(result.status.success());

        // Verify directory was created
        assert!(dir_path.join("subdir").exists());

        Ok(())
    }
}
