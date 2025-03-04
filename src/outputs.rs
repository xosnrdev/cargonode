use std::path::{Path, PathBuf};

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

    /// Verify that all expected output directories exist and create them if needed
    ///
    /// # Returns
    ///
    /// * `Result<Vec<PathBuf>>` - List of output file paths that were verified
    pub fn verify_outputs(&self) -> Result<Vec<PathBuf>> {
        // Check if patterns is empty
        if self.patterns.is_empty() {
            return Ok(Vec::new());
        }

        let mut output_paths = Vec::new();

        // Process each pattern
        for pattern in &self.patterns {
            let pattern_path = self.base_path.join(pattern);

            // Get the parent directory of the pattern
            if let Some(parent) = pattern_path.parent() {
                // Create parent directories if they don't exist
                if !parent.exists() {
                    std::fs::create_dir_all(parent).map_err(|e| Error::Output {
                        message: format!(
                            "Failed to create directory '{}': {}",
                            parent.display(),
                            e
                        ),
                    })?;
                }
            }

            // Add the expected output path
            output_paths.push(pattern_path);
        }

        Ok(output_paths)
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
        let verifier = OutputVerifier::new(
            temp_path,
            vec!["test1.out".to_string(), "test2.out".to_string()],
        );

        // Verify outputs
        let outputs = verifier.verify_outputs()?;

        // Check that both file paths were returned
        assert_eq!(outputs.len(), 2);
        assert!(outputs.contains(&file1_path));
        assert!(outputs.contains(&file2_path));

        Ok(())
    }

    /// Test output verification with non-existent files (should succeed and create directories)
    #[test]
    fn test_verify_outputs_missing() -> Result<()> {
        // Create temporary directory
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();

        // Create output verifier with non-existent pattern
        let verifier = OutputVerifier::new(temp_path, vec!["subdir/missing.out".to_string()]);

        // Verify outputs (should succeed and create directory)
        let outputs = verifier.verify_outputs()?;

        // Check that the directory was created
        assert!(temp_path.join("subdir").exists());

        // Check that the expected path was returned
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0], temp_path.join("subdir/missing.out"));

        Ok(())
    }

    /// Test output verification with empty patterns
    #[test]
    fn test_verify_outputs_empty() -> Result<()> {
        let temp_dir = tempdir()?;
        let verifier = OutputVerifier::new(temp_dir.path(), vec![]);
        let outputs = verifier.verify_outputs()?;
        assert!(outputs.is_empty());
        Ok(())
    }

    /// Test output verification with subdirectories
    #[test]
    fn test_verify_outputs_subdirectories() -> Result<()> {
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();

        // Create subdirectory and file
        let subdir_path = temp_path.join("subdir");
        fs::create_dir(&subdir_path)?;
        let file_path = subdir_path.join("test.out");
        let _file = File::create(&file_path)?;

        // Create output verifier
        let verifier = OutputVerifier::new(temp_path, vec!["subdir/test.out".to_string()]);

        // Verify outputs
        let outputs = verifier.verify_outputs()?;

        // Check that the file path was returned
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0], file_path);

        Ok(())
    }

    #[test]
    fn test_verify_outputs_creates_directories() -> Result<()> {
        let temp_dir = tempdir()?;
        let base_path = temp_dir.path();

        // Create patterns with nested directories
        let patterns = vec![
            "output/dir1/file1.txt".to_string(),
            "output/dir2/subdir/file2.txt".to_string(),
        ];

        let verifier = OutputVerifier::new(base_path, patterns.clone());
        let output_paths = verifier.verify_outputs()?;

        // Verify directories were created
        assert!(base_path.join("output/dir1").exists());
        assert!(base_path.join("output/dir2/subdir").exists());

        // Verify returned paths match expected
        assert_eq!(output_paths.len(), 2);
        assert_eq!(output_paths[0], base_path.join("output/dir1/file1.txt"));
        assert_eq!(
            output_paths[1],
            base_path.join("output/dir2/subdir/file2.txt")
        );

        Ok(())
    }

    #[test]
    fn test_verify_outputs_existing_directories() -> Result<()> {
        let temp_dir = tempdir()?;
        let base_path = temp_dir.path();

        // Create directories first
        fs::create_dir_all(base_path.join("existing/dir1"))?;
        fs::create_dir_all(base_path.join("existing/dir2"))?;

        let patterns = vec![
            "existing/dir1/file1.txt".to_string(),
            "existing/dir2/file2.txt".to_string(),
        ];

        let verifier = OutputVerifier::new(base_path, patterns);
        let output_paths = verifier.verify_outputs()?;

        // Verify directories still exist
        assert!(base_path.join("existing/dir1").exists());
        assert!(base_path.join("existing/dir2").exists());

        // Verify returned paths match expected
        assert_eq!(output_paths.len(), 2);
        assert_eq!(output_paths[0], base_path.join("existing/dir1/file1.txt"));
        assert_eq!(output_paths[1], base_path.join("existing/dir2/file2.txt"));

        Ok(())
    }
}
