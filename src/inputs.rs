use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use glob::glob;
use sha2::{Digest, Sha256};

use crate::error::Error;
use crate::Result;

/// Tracks input files for idempotency
pub struct InputTracker {
    /// Base path for resolving relative patterns
    base_path: PathBuf,

    /// Glob patterns for input files
    patterns: Vec<String>,
}

impl InputTracker {
    /// Create a new input tracker
    ///
    /// # Arguments
    ///
    /// * `base_path` - Base path for resolving relative patterns
    /// * `patterns` - Glob patterns for input files
    ///
    /// # Returns
    ///
    /// * `Self` - A new InputTracker instance
    pub fn new(base_path: &Path, patterns: Vec<String>) -> Self {
        Self {
            base_path: base_path.to_path_buf(),
            patterns,
        }
    }

    /// Get all input files matching the patterns
    ///
    /// # Returns
    ///
    /// * `Result<Vec<PathBuf>>` - List of matching file paths
    pub fn get_input_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let mut seen_paths = HashSet::new();

        // Maximum number of files to process
        const MAX_FILES: usize = 10000;

        for pattern in &self.patterns {
            // Construct absolute pattern
            let abs_pattern = if Path::new(pattern).is_absolute() {
                pattern.clone()
            } else {
                self.base_path.join(pattern).to_string_lossy().to_string()
            };

            // Use glob to find matching files
            let glob_result = glob(&abs_pattern);

            match glob_result {
                Ok(entries) => {
                    for entry_result in entries {
                        // Check if we've reached the maximum file limit
                        if files.len() >= MAX_FILES {
                            return Err(Error::Input {
                                message: format!("Too many input files (limit: {})", MAX_FILES),
                            });
                        }

                        match entry_result {
                            Ok(path) => {
                                if path.is_file() && !seen_paths.contains(&path) {
                                    seen_paths.insert(path.clone());
                                    files.push(path);
                                }
                            }
                            Err(err) => {
                                return Err(Error::Input {
                                    message: format!("Failed to process glob entry: {}", err),
                                });
                            }
                        }
                    }
                }
                Err(err) => {
                    return Err(Error::Input {
                        message: format!("Invalid glob pattern '{}': {}", pattern, err),
                    });
                }
            }
        }

        Ok(files)
    }

    /// Calculate a hash of all input files
    ///
    /// # Returns
    ///
    /// * `Result<String>` - Hex string of the hash
    pub fn calculate_hash(&self) -> Result<String> {
        let mut files = self.get_input_files()?;

        // Sort files for deterministic ordering
        files.sort();

        // Create hasher
        let mut hasher = Sha256::new();

        // Maximum file size to hash (10MB)
        const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

        for file in files {
            // Get file metadata
            let metadata = fs::metadata(&file)?;

            // Skip files that are too large
            if metadata.len() > MAX_FILE_SIZE {
                return Err(Error::Input {
                    message: format!(
                        "File too large to hash: {} ({} bytes, limit: {} bytes)",
                        file.display(),
                        metadata.len(),
                        MAX_FILE_SIZE
                    ),
                });
            }

            // Read file content
            let content = fs::read(&file)?;

            // Update hash with file path and content
            hasher.update(file.to_string_lossy().as_bytes());
            hasher.update(b":");
            hasher.update(&content);
            hasher.update(b"\n");
        }

        // Finalize hash
        let hash = hasher.finalize();

        Ok(format!("{:x}", hash))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::TempDir;

    use super::*;

    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> Result<PathBuf> {
        let file_path = dir.join(name);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content)?;
        Ok(file_path)
    }

    #[test]
    fn test_get_input_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create test files
        create_test_file(dir_path, "file1.txt", b"content1")?;
        create_test_file(dir_path, "file2.txt", b"content2")?;
        create_test_file(dir_path, "other.log", b"log content")?;

        // Create subdirectory
        let sub_dir = dir_path.join("subdir");
        fs::create_dir(&sub_dir)?;
        create_test_file(&sub_dir, "file3.txt", b"content3")?;

        // Test with single pattern
        let tracker = InputTracker::new(dir_path, vec!["*.txt".to_string()]);

        let files = tracker.get_input_files()?;
        assert_eq!(files.len(), 2);

        // Test with multiple patterns
        let tracker = InputTracker::new(dir_path, vec!["*.txt".to_string(), "*.log".to_string()]);

        let files = tracker.get_input_files()?;
        assert_eq!(files.len(), 3);

        // Test with recursive pattern
        let tracker = InputTracker::new(dir_path, vec!["**/*.txt".to_string()]);

        let files = tracker.get_input_files()?;
        assert_eq!(files.len(), 3);

        Ok(())
    }

    #[test]
    fn test_calculate_hash() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        // Create test files
        create_test_file(dir_path, "file1.txt", b"content1")?;
        create_test_file(dir_path, "file2.txt", b"content2")?;

        // Calculate hash
        let tracker = InputTracker::new(dir_path, vec!["*.txt".to_string()]);

        let hash1 = tracker.calculate_hash()?;

        // Calculate hash again (should be the same)
        let hash2 = tracker.calculate_hash()?;
        assert_eq!(hash1, hash2);

        // Modify a file and check that hash changes
        create_test_file(dir_path, "file1.txt", b"modified content")?;
        let hash3 = tracker.calculate_hash()?;
        assert_ne!(hash1, hash3);

        Ok(())
    }

    #[test]
    fn test_empty_patterns() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let dir_path = temp_dir.path();

        let tracker = InputTracker::new(dir_path, vec![]);

        let files = tracker.get_input_files()?;
        assert_eq!(files.len(), 0);

        Ok(())
    }
}
