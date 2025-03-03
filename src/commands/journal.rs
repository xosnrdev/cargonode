use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};

use crate::cache::Cache;
use crate::journal::{Journal, JournalEntry};
use crate::progress;
use crate::Result;

/// Format a timestamp as a human-readable date and time
///
/// # Arguments
///
/// * `timestamp` - Unix timestamp in seconds
///
/// # Returns
///
/// * `String` - Formatted date and time
fn format_timestamp(timestamp: u64) -> String {
    let system_time = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = chrono::DateTime::<chrono::Utc>::from(system_time);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Format a journal entry for display
///
/// # Arguments
///
/// * `entry` - Journal entry to format
/// * `verbose` - Whether to show detailed information
///
/// # Returns
///
/// * `String` - Formatted entry
fn format_journal_entry(entry: &JournalEntry, verbose: bool) -> String {
    let timestamp = format_timestamp(entry.timestamp);
    let status = if entry.exit_code == 0 {
        progress::style_text("SUCCESS", progress::Color::Green, false)
    } else {
        progress::style_text("FAILED", progress::Color::Red, false)
    };

    let cache_status = if entry.from_cache {
        progress::style_text("(cached)", progress::Color::Blue, false)
    } else {
        String::new()
    };

    if verbose {
        format!(
            "{} | {} {} | Tool: {} | Command: {} {} | Hash: {}",
            timestamp,
            status,
            cache_status,
            entry.tool_name,
            entry.command,
            entry.args.join(" "),
            entry.input_hash
        )
    } else {
        format!(
            "{} | {} {} | Tool: {}",
            timestamp, status, cache_status, entry.tool_name
        )
    }
}

/// Show command execution history
///
/// # Arguments
///
/// * `tool` - Optional tool name to filter by
/// * `limit` - Maximum number of entries to show
/// * `journal_dir` - Path to the journal directory
/// * `verbose` - Whether to show detailed information
///
/// # Returns
///
/// * `Result<()>` - Whether the operation succeeded
pub fn show_history(
    tool: Option<&str>,
    limit: usize,
    journal_dir: &Path,
    verbose: bool,
) -> Result<()> {
    // Create journal
    let journal = Journal::new(journal_dir, 1000)?;

    // Get entries
    let entries = match tool {
        Some(tool_name) => journal.get_entries_for_tool(tool_name)?,
        None => journal.get_entries()?,
    };

    // Check if there are any entries
    if entries.is_empty() {
        let message = match tool {
            Some(tool_name) => format!("No history found for tool '{}'", tool_name),
            None => "No command history found".to_string(),
        };

        progress::write_message(&progress::format_note(&message))?;
        return Ok(());
    }

    // Print header
    let header = if verbose {
        "TIMESTAMP | STATUS | TOOL | COMMAND | HASH"
    } else {
        "TIMESTAMP | STATUS | TOOL"
    };

    progress::write_message(&progress::style_text(header, progress::Color::Blue, true))?;

    // Print entries (most recent first, limited by limit)
    let start_index = if entries.len() > limit {
        entries.len() - limit
    } else {
        0
    };

    for entry in &entries[start_index..] {
        let formatted = format_journal_entry(entry, verbose);
        progress::write_message(&formatted)?;
    }

    // Print summary
    let shown = entries.len().min(limit);
    let total = entries.len();

    let summary = format!("Showing {} of {} entries", shown, total);
    progress::write_message(&progress::format_note(&summary))?;

    Ok(())
}

/// Clear the cache
///
/// # Arguments
///
/// * `tool` - Optional tool name to clear cache for
/// * `cache_dir` - Path to the cache directory
/// * `verbose` - Whether to show verbose output
///
/// # Returns
///
/// * `Result<()>` - Whether the operation succeeded
pub fn clear_cache(tool: Option<&str>, cache_dir: &Path, verbose: bool) -> Result<()> {
    // Create cache
    let mut cache = Cache::new(cache_dir)?;

    // Clear cache
    match tool {
        Some(tool_name) => {
            // Clear cache for specific tool
            let count = cache.invalidate(tool_name)?;

            if verbose {
                let message = format!("Cleared {} cache entries for tool '{}'", count, tool_name);
                progress::write_message(&progress::format_status("Cleared", &message))?;
            }
        }
        None => {
            // Clear all cache
            let count = cache.clear()?;

            if verbose {
                let message = format!("Cleared {} cache entries", count);
                progress::write_message(&progress::format_status("Cleared", &message))?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_format_timestamp() {
        // Test with a known timestamp
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let formatted = format_timestamp(timestamp);
        assert_eq!(formatted, "2021-01-01 00:00:00");
    }

    #[test]
    fn test_show_history() -> Result<()> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let journal_dir = temp_dir.path();

        // Create journal
        let journal = Journal::new(journal_dir, 10)?;

        // Add some entries
        let entry1 = Journal::create_entry(
            "test-tool",
            "test-hash",
            "npm",
            &["run".to_string(), "test".to_string()],
            0,
            false,
        );

        let entry2 = Journal::create_entry(
            "build-tool",
            "build-hash",
            "yarn",
            &["build".to_string()],
            1,
            true,
        );

        journal.add_entry(entry1)?;
        journal.add_entry(entry2)?;

        // Show history
        show_history(None, 10, journal_dir, false)?;

        // Show history for specific tool
        show_history(Some("test-tool"), 10, journal_dir, true)?;

        Ok(())
    }

    #[test]
    fn test_clear_cache() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir)?;

        let cache = Cache::new(cache_dir.as_path())?;

        // Add some entries
        let entry1 = Cache::create_entry(
            "test-tool",
            "test-hash",
            "npm",
            &["run".to_string(), "test".to_string()],
            0,
        );

        let entry2 = Cache::create_entry(
            "build-tool",
            "build-hash",
            "yarn",
            &["build".to_string()],
            1,
        );

        cache.store_entry(&entry1)?;
        cache.store_entry(&entry2)?;

        // Clear cache for specific tool
        clear_cache(Some("test-tool"), cache_dir.as_path(), true)?;

        // Verify that only the specified tool's cache was cleared
        let cache = Cache::new(cache_dir.as_path())?;
        assert!(!cache.has_entry("test-tool", "test-hash"));
        assert!(cache.has_entry("build-tool", "build-hash"));

        // Clear all cache
        clear_cache(None, cache_dir.as_path(), true)?;

        // Verify that all cache was cleared
        let cache = Cache::new(cache_dir.as_path())?;
        assert!(!cache.has_entry("test-tool", "test-hash"));
        assert!(!cache.has_entry("build-tool", "build-hash"));

        Ok(())
    }
}
