use std::path::{Path, PathBuf};

use glob::glob;

use crate::error::Error;
use crate::Result;

/// Output verifier for checking expected output files
pub struct OutputVerifier {
    /// Base path for resolving output patterns
    base_path: PathBuf,

    /// Output file patterns
    patterns: Vec<String>,
}

impl OutputVerifier {
    /// Create a new output verifier
    ///
    /// # Arguments
    ///
    /// * `base_path` - Base path for resolving output patterns
    /// * `patterns` - Output file patterns
    ///
    /// # Returns
    ///
    /// * `Self` - A new OutputVerifier instance
    pub fn new(base_path: &Path, patterns: Vec<String>) -> Self {
        Self {
            base_path: base_path.to_path_buf(),
            patterns,
        }
    }

    /// Verify that all expected output files exist
    ///
    /// # Returns
    ///
    /// * `Result<Vec<PathBuf>>` - List of output files that exist
    pub fn verify_outputs(&self) -> Result<Vec<PathBuf>> {
        // Check if patterns is empty
        if self.patterns.is_empty() {
            return Ok(Vec::new());
        }

        let mut output_files = Vec::new();
        let mut missing_files = Vec::new();

        // Process each pattern
        for pattern in &self.patterns {
            let pattern_str = self.base_path.join(pattern).to_string_lossy().to_string();
            let mut found = false;

            // Use glob to find matching files
            match glob(&pattern_str) {
                Ok(entries) => {
                    for entry in entries {
                        match entry {
                            Ok(path) => {
                                output_files.push(path);
                                found = true;
                            }
                            Err(e) => {
                                return Err(Error::Output {
                                    message: format!("Failed to read glob entry: {}", e),
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(Error::Output {
                        message: format!("Invalid glob pattern '{}': {}", pattern, e),
                    });
                }
            }

            // If no files were found for this pattern, add it to missing files
            if !found {
                missing_files.push(pattern.clone());
            }
        }

        // If any patterns had no matching files, return an error
        if !missing_files.is_empty() {
            return Err(Error::OutputNotFound {
                patterns: missing_files,
            });
        }

        Ok(output_files)
    }

    /// Get a list of expected output files
    ///
    /// # Returns
    ///
    /// * `Result<Vec<String>>` - List of expected output file patterns
    pub fn get_expected_outputs(&self) -> Vec<String> {
        self.patterns.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use tempfile::tempdir;

    use super::*;

    /// Test output verification with existing files
    #[test]
    fn test_verify_outputs_existing() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();

        // Create test files
        let file1_path = temp_path.join("test1.out");
        let file2_path = temp_path.join("test2.out");
        let _file1 = File::create(&file1_path)?;
        let _file2 = File::create(&file2_path)?;

        // Create output verifier
        let verifier = OutputVerifier::new(temp_path, vec!["*.out".to_string()]);

        // Verify outputs
        let outputs = verifier.verify_outputs()?;

        // Check that both files were found
        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains(&file1_path));
        assert!(outputs.contains(&file2_path));

        Ok(())
    }

    /// Test output verification with missing files
    #[test]
    fn test_verify_outputs_missing() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();

        // Create output verifier with non-existent pattern
        let verifier = OutputVerifier::new(temp_path, vec!["missing*.out".to_string()]);

        // Verify outputs (should fail)
        let result = verifier.verify_outputs();
        assert!(result.is_err());

        // Check error type
        match result {
            Err(Error::OutputNotFound { patterns }) => {
                assert_eq!(patterns.len(), 1);
                assert_eq!(patterns[0], "missing*.out");
            }
            _ => panic!("Expected OutputNotFound error"),
        }

        Ok(())
    }

    /// Test output verification with empty patterns
    #[test]
    fn test_verify_outputs_empty() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();

        // Create output verifier with empty patterns
        let verifier = OutputVerifier::new(temp_path, vec![]);

        // Verify outputs (should succeed with empty list)
        let outputs = verifier.verify_outputs()?;
        assert!(outputs.is_empty());

        Ok(())
    }

    /// Test output verification with subdirectories
    #[test]
    fn test_verify_outputs_subdirectories() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();

        // Create subdirectory
        let subdir_path = temp_path.join("subdir");
        fs::create_dir(&subdir_path)?;

        // Create test file in subdirectory
        let file_path = subdir_path.join("test.out");
        let _file = File::create(&file_path)?;

        // Create output verifier
        let verifier = OutputVerifier::new(temp_path, vec!["**/*.out".to_string()]);

        // Verify outputs
        let outputs = verifier.verify_outputs()?;

        // Check that the file was found
        assert_eq!(outputs.len(), 1);
        assert!(outputs.contains(&file_path));

        Ok(())
    }
}
