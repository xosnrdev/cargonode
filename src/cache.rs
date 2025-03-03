use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::Result;

/// Cache entry for a command execution
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheEntry {
    /// Tool name
    pub tool_name: String,

    /// Input hash
    pub input_hash: String,

    /// Command that was executed
    pub command: String,

    /// Arguments that were passed to the command
    pub args: Vec<String>,

    /// Exit code of the command
    pub exit_code: i32,

    /// Timestamp when the cache entry was created
    pub timestamp: u64,
}

/// Cache for command executions
pub struct Cache {
    /// Path to the cache directory
    cache_dir: PathBuf,
}

impl Cache {
    /// Create a new cache
    ///
    /// # Arguments
    ///
    /// * `cache_dir` - Path to the cache directory
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A new Cache instance
    pub fn new(cache_dir: &Path) -> Result<Self> {
        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir)?;
        } else if !cache_dir.is_dir() {
            return Err(Error::Cache {
                message: format!(
                    "Cache path exists but is not a directory: {}",
                    cache_dir.display()
                ),
            });
        }

        Ok(Self {
            cache_dir: cache_dir.to_path_buf(),
        })
    }

    /// Get the path to a cache entry
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool
    /// * `input_hash` - Hash of the inputs
    ///
    /// # Returns
    ///
    /// * `PathBuf` - Path to the cache entry
    fn get_cache_path(&self, tool_name: &str, input_hash: &str) -> PathBuf {
        self.cache_dir
            .join(format!("{}_{}.json", tool_name, input_hash))
    }

    /// Check if a cache entry exists
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool
    /// * `input_hash` - Hash of the inputs
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the cache entry exists
    pub fn has_entry(&self, tool_name: &str, input_hash: &str) -> bool {
        self.get_cache_path(tool_name, input_hash).exists()
    }

    /// Get a cache entry
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool
    /// * `input_hash` - Hash of the inputs
    ///
    /// # Returns
    ///
    /// * `Result<Option<CacheEntry>>` - The cache entry, if it exists
    pub fn get_entry(&self, tool_name: &str, input_hash: &str) -> Result<Option<CacheEntry>> {
        let path = self.get_cache_path(tool_name, input_hash);

        if !path.exists() {
            return Ok(None);
        }

        // Read cache file
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse JSON
        let entry: CacheEntry = serde_json::from_str(&contents)?;

        Ok(Some(entry))
    }

    /// Store a cache entry
    ///
    /// # Arguments
    ///
    /// * `entry` - The cache entry to store
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Whether the operation succeeded
    pub fn store_entry(&self, entry: &CacheEntry) -> Result<()> {
        let path = self.get_cache_path(&entry.tool_name, &entry.input_hash);

        // Serialize to JSON
        let json = serde_json::to_string_pretty(entry)?;

        // Write to file
        let mut file = fs::File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Create a new cache entry
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool
    /// * `input_hash` - Hash of the inputs
    /// * `command` - Command that was executed
    /// * `args` - Arguments that were passed to the command
    /// * `exit_code` - Exit code of the command
    ///
    /// # Returns
    ///
    /// * `CacheEntry` - The created cache entry
    pub fn create_entry(
        tool_name: &str,
        input_hash: &str,
        command: &str,
        args: &[String],
        exit_code: i32,
    ) -> CacheEntry {
        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        CacheEntry {
            tool_name: tool_name.to_string(),
            input_hash: input_hash.to_string(),
            command: command.to_string(),
            args: args.to_vec(),
            exit_code,
            timestamp,
        }
    }

    /// Clear the cache
    ///
    /// # Returns
    ///
    /// * `Result<usize>` - Number of entries cleared
    pub fn clear(&self) -> Result<usize> {
        // Maximum number of entries to delete
        const MAX_ENTRIES: usize = 10000;

        let mut count = 0;

        for entry in fs::read_dir(&self.cache_dir)? {
            // Check if we've reached the maximum entry limit
            if count >= MAX_ENTRIES {
                return Err(Error::Cache {
                    message: format!("Too many cache entries to clear (limit: {})", MAX_ENTRIES),
                });
            }

            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                fs::remove_file(path)?;
                count += 1;
            }
        }

        Ok(count)
    }

    /// Invalidate cache entries for a specific tool
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool
    ///
    /// # Returns
    ///
    /// * `Result<usize>` - Number of entries invalidated
    pub fn invalidate(&mut self, tool_name: &str) -> Result<usize> {
        // Maximum number of entries to delete
        const MAX_ENTRIES: usize = 10000;

        let mut count = 0;
        let prefix = format!("{}_", tool_name);

        for entry in fs::read_dir(&self.cache_dir)? {
            // Check if we've reached the maximum entry limit
            if count >= MAX_ENTRIES {
                return Err(Error::Cache {
                    message: format!(
                        "Too many cache entries to invalidate (limit: {})",
                        MAX_ENTRIES
                    ),
                });
            }

            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.starts_with(&prefix) {
                        fs::remove_file(path)?;
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_cache_operations() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let cache_dir = temp_dir.path();

        // Create cache
        let cache = Cache::new(cache_dir)?;

        // Check that entry doesn't exist
        assert!(!cache.has_entry("test-tool", "test-hash"));
        assert!(cache.get_entry("test-tool", "test-hash")?.is_none());

        // Create and store entry
        let entry = Cache::create_entry(
            "test-tool",
            "test-hash",
            "npm",
            &["run".to_string(), "test".to_string()],
            0,
        );

        cache.store_entry(&entry)?;

        // Check that entry exists
        assert!(cache.has_entry("test-tool", "test-hash"));

        // Retrieve entry
        let retrieved = cache.get_entry("test-tool", "test-hash")?.unwrap();
        assert_eq!(retrieved.tool_name, "test-tool");
        assert_eq!(retrieved.input_hash, "test-hash");
        assert_eq!(retrieved.command, "npm");
        assert_eq!(retrieved.args, vec!["run".to_string(), "test".to_string()]);
        assert_eq!(retrieved.exit_code, 0);

        // Clear cache
        cache.clear()?;

        // Check that entry no longer exists
        assert!(!cache.has_entry("test-tool", "test-hash"));

        Ok(())
    }

    #[test]
    fn test_multiple_entries() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let cache_dir = temp_dir.path();

        // Create cache
        let cache = Cache::new(cache_dir)?;

        // Create and store multiple entries
        let entry1 = Cache::create_entry(
            "tool1",
            "hash1",
            "npm",
            &["run".to_string(), "test".to_string()],
            0,
        );

        let entry2 = Cache::create_entry("tool2", "hash2", "yarn", &["test".to_string()], 1);

        cache.store_entry(&entry1)?;
        cache.store_entry(&entry2)?;

        // Check that both entries exist
        assert!(cache.has_entry("tool1", "hash1"));
        assert!(cache.has_entry("tool2", "hash2"));

        // Retrieve entries
        let retrieved1 = cache.get_entry("tool1", "hash1")?.unwrap();
        let retrieved2 = cache.get_entry("tool2", "hash2")?.unwrap();

        assert_eq!(retrieved1.tool_name, "tool1");
        assert_eq!(retrieved2.tool_name, "tool2");

        // Clear cache
        cache.clear()?;

        // Check that entries no longer exist
        assert!(!cache.has_entry("tool1", "hash1"));
        assert!(!cache.has_entry("tool2", "hash2"));

        Ok(())
    }
}
