use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::Result;

/// Entry in the execution journal
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalEntry {
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

    /// Whether the result was from cache
    pub from_cache: bool,

    /// Timestamp when the entry was created
    pub timestamp: u64,
}

/// Execution journal for tracking command history
pub struct Journal {
    /// Path to the journal file
    journal_path: PathBuf,

    /// Maximum number of entries to keep
    max_entries: usize,
}

impl Journal {
    /// Create a new journal
    ///
    /// # Arguments
    ///
    /// * `journal_dir` - Path to the journal directory
    /// * `max_entries` - Maximum number of entries to keep
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A new Journal instance
    pub fn new(journal_dir: &Path, max_entries: usize) -> Result<Self> {
        // Create journal directory if it doesn't exist
        if !journal_dir.exists() {
            fs::create_dir_all(journal_dir)?;
        } else if !journal_dir.is_dir() {
            return Err(Error::Config {
                message: format!(
                    "Journal path exists but is not a directory: {}",
                    journal_dir.display()
                ),
            });
        }

        // Set maximum entries
        let max_entries = if max_entries == 0 { 1000 } else { max_entries };

        // Create journal path
        let journal_path = journal_dir.join("journal.json");

        Ok(Self {
            journal_path,
            max_entries,
        })
    }

    /// Add an entry to the journal
    ///
    /// # Arguments
    ///
    /// * `entry` - The journal entry to add
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Whether the operation succeeded
    pub fn add_entry(&self, entry: JournalEntry) -> Result<()> {
        // Read existing entries
        let mut entries = self.read_entries()?;

        // Add new entry
        entries.push(entry);

        // Trim entries if needed
        if entries.len() > self.max_entries {
            // Keep only the most recent entries
            entries = entries.split_off(entries.len() - self.max_entries);
        }

        // Write entries back to file
        self.write_entries(&entries)?;

        Ok(())
    }

    /// Get all entries from the journal
    ///
    /// # Returns
    ///
    /// * `Result<Vec<JournalEntry>>` - The journal entries
    pub fn get_entries(&self) -> Result<Vec<JournalEntry>> {
        self.read_entries()
    }

    /// Get entries for a specific tool
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool
    ///
    /// # Returns
    ///
    /// * `Result<Vec<JournalEntry>>` - The journal entries for the tool
    pub fn get_entries_for_tool(&self, tool_name: &str) -> Result<Vec<JournalEntry>> {
        let entries = self.read_entries()?;

        // Filter entries by tool name
        let filtered = entries
            .into_iter()
            .filter(|entry| entry.tool_name == tool_name)
            .collect();

        Ok(filtered)
    }

    /// Create a new journal entry
    ///
    /// # Arguments
    ///
    /// * `tool_name` - Name of the tool
    /// * `input_hash` - Hash of the inputs
    /// * `command` - Command that was executed
    /// * `args` - Arguments that were passed to the command
    /// * `exit_code` - Exit code of the command
    /// * `from_cache` - Whether the result was from cache
    ///
    /// # Returns
    ///
    /// * `JournalEntry` - The created journal entry
    pub fn create_entry(
        tool_name: &str,
        input_hash: &str,
        command: &str,
        args: &[String],
        exit_code: i32,
        from_cache: bool,
    ) -> JournalEntry {
        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        JournalEntry {
            tool_name: tool_name.to_string(),
            input_hash: input_hash.to_string(),
            command: command.to_string(),
            args: args.to_vec(),
            exit_code,
            from_cache,
            timestamp,
        }
    }

    /// Clear the journal
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Whether the operation succeeded
    pub fn clear(&self) -> Result<()> {
        // Write empty entries list
        self.write_entries(&[])?;

        Ok(())
    }

    /// Read entries from the journal file
    ///
    /// # Returns
    ///
    /// * `Result<Vec<JournalEntry>>` - The journal entries
    fn read_entries(&self) -> Result<Vec<JournalEntry>> {
        // Check if journal file exists
        if !self.journal_path.exists() {
            return Ok(Vec::new());
        }

        // Read journal file
        let mut file = File::open(&self.journal_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse JSON
        if contents.trim().is_empty() {
            return Ok(Vec::new());
        }

        let entries: Vec<JournalEntry> = serde_json::from_str(&contents)?;

        Ok(entries)
    }

    /// Write entries to the journal file
    ///
    /// # Arguments
    ///
    /// * `entries` - The journal entries to write
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Whether the operation succeeded
    fn write_entries(&self, entries: &[JournalEntry]) -> Result<()> {
        // Create or truncate journal file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.journal_path)?;

        // Serialize to JSON
        let json = serde_json::to_string_pretty(entries)?;

        // Write to file
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_journal_operations() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let journal_dir = temp_dir.path();

        // Create journal
        let journal = Journal::new(journal_dir, 10)?;

        // Check that journal is empty
        let entries = journal.get_entries()?;
        assert_eq!(entries.len(), 0);

        // Create and add entry
        let entry = Journal::create_entry(
            "test-tool",
            "test-hash",
            "npm",
            &["run".to_string(), "test".to_string()],
            0,
            false,
        );

        journal.add_entry(entry)?;

        // Check that entry exists
        let entries = journal.get_entries()?;
        assert_eq!(entries.len(), 1);

        let retrieved = &entries[0];
        assert_eq!(retrieved.tool_name, "test-tool");
        assert_eq!(retrieved.input_hash, "test-hash");
        assert_eq!(retrieved.command, "npm");
        assert_eq!(retrieved.args, vec!["run".to_string(), "test".to_string()]);
        assert_eq!(retrieved.exit_code, 0);
        assert!(!retrieved.from_cache);

        // Add another entry
        let entry2 = Journal::create_entry(
            "build-tool",
            "build-hash",
            "yarn",
            &["build".to_string()],
            0,
            true,
        );

        journal.add_entry(entry2)?;

        // Check that both entries exist
        let entries = journal.get_entries()?;
        assert_eq!(entries.len(), 2);

        // Get entries for specific tool
        let test_entries = journal.get_entries_for_tool("test-tool")?;
        assert_eq!(test_entries.len(), 1);
        assert_eq!(test_entries[0].tool_name, "test-tool");

        let build_entries = journal.get_entries_for_tool("build-tool")?;
        assert_eq!(build_entries.len(), 1);
        assert_eq!(build_entries[0].tool_name, "build-tool");

        // Clear journal
        journal.clear()?;

        // Check that journal is empty
        let entries = journal.get_entries()?;
        assert_eq!(entries.len(), 0);

        Ok(())
    }

    #[test]
    fn test_journal_max_entries() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let journal_dir = temp_dir.path();

        // Create journal with max 3 entries
        let journal = Journal::new(journal_dir, 3)?;

        // Add 5 entries
        for i in 0..5 {
            let entry = Journal::create_entry(
                &format!("tool-{}", i),
                &format!("hash-{}", i),
                "cmd",
                &[format!("arg-{}", i)],
                0,
                false,
            );

            journal.add_entry(entry)?;
        }

        // Check that only the 3 most recent entries are kept
        let entries = journal.get_entries()?;
        assert_eq!(entries.len(), 3);

        // Check that the entries are the most recent ones
        assert_eq!(entries[0].tool_name, "tool-2");
        assert_eq!(entries[1].tool_name, "tool-3");
        assert_eq!(entries[2].tool_name, "tool-4");

        Ok(())
    }
}
