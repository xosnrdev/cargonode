//! File system operations with permission handling
//!
//! This module provides safe file system operations that handle permissions
//! and error cases properly.

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use crate::{Error, Result};

/// Default permissions for directories
#[cfg(unix)]
const DEFAULT_DIR_MODE: u32 = 0o755;

/// Default permissions for files
#[cfg(unix)]
const DEFAULT_FILE_MODE: u32 = 0o644;

/// Default permissions for executable files
#[cfg(unix)]
const DEFAULT_EXEC_MODE: u32 = 0o755;

/// Creates a directory and all its parent components with proper permissions
///
/// # Arguments
/// * `path` - The path to create
/// * `is_executable` - Whether the directory should be executable (affects Unix permissions)
///
/// # Returns
/// Result indicating success or failure
pub fn create_dir_all(path: &Path) -> Result<()> {
    // First try to create the directory
    match fs::create_dir_all(path) {
        Ok(_) => {
            // Set permissions on Unix systems
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = fs::Permissions::from_mode(DEFAULT_DIR_MODE);
                fs::set_permissions(path, perms)?;
            }
            Ok(())
        }
        Err(e) => match e.kind() {
            io::ErrorKind::PermissionDenied => Err(Error::Permission(format!(
                "Permission denied when creating directory: {}",
                path.display()
            ))),
            _ => Err(Error::Io(e)),
        },
    }
}

/// Writes content to a file with proper permissions
/// Creates parent directories if they don't exist
///
/// # Arguments
/// * `path` - The path to write to
/// * `content` - The content to write
/// * `is_executable` - Whether the file should be executable (affects Unix permissions)
///
/// # Returns
/// Result indicating success or failure
pub fn write_file(path: &Path, content: &str, is_executable: bool) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    // Open file with proper options
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => Error::Permission(format!(
                "Permission denied when creating file: {}",
                path.display()
            )),
            _ => Error::Io(e),
        })?;

    // Write content
    let mut writer = io::BufWriter::new(file);
    writer.write_all(content.as_bytes()).map_err(Error::Io)?;
    writer.flush().map_err(Error::Io)?;

    // Set permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = if is_executable {
            DEFAULT_EXEC_MODE
        } else {
            DEFAULT_FILE_MODE
        };
        let perms = fs::Permissions::from_mode(mode);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

/// Reads a file's contents as a string
///
/// # Arguments
/// * `path` - The path to read from
///
/// # Returns
/// The file contents as a string
pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|e| match e.kind() {
        io::ErrorKind::PermissionDenied => Error::Permission(format!(
            "Permission denied when reading file: {}",
            path.display()
        )),
        _ => Error::Io(e),
    })
}

/// Removes a file or directory and all its contents
///
/// # Arguments
/// * `path` - The path to remove
///
/// # Returns
/// Result indicating success or failure
pub fn remove_all(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => Error::Permission(format!(
                "Permission denied when removing directory: {}",
                path.display()
            )),
            _ => Error::Io(e),
        })
    } else {
        fs::remove_file(path).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => Error::Permission(format!(
                "Permission denied when removing file: {}",
                path.display()
            )),
            _ => Error::Io(e),
        })
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_create_dir_all() {
        let temp = tempdir().unwrap();
        let test_dir = temp.path().join("a/b/c");

        create_dir_all(&test_dir).unwrap();
        assert!(test_dir.exists());

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&test_dir).unwrap();
            assert_eq!(metadata.permissions().mode() & 0o777, DEFAULT_DIR_MODE);
        }
    }

    #[test]
    fn test_write_file() {
        let temp = tempdir().unwrap();
        let test_file = temp.path().join("test.txt");
        let content = "Hello, World!";

        write_file(&test_file, content, false).unwrap();
        assert!(test_file.exists());
        assert_eq!(fs::read_to_string(&test_file).unwrap(), content);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&test_file).unwrap();
            assert_eq!(metadata.permissions().mode() & 0o777, DEFAULT_FILE_MODE);
        }
    }

    #[test]
    fn test_write_executable() {
        let temp = tempdir().unwrap();
        let test_file = temp.path().join("test.sh");
        let content = "#!/bin/sh\necho 'Hello'";

        write_file(&test_file, content, true).unwrap();
        assert!(test_file.exists());

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&test_file).unwrap();
            assert_eq!(metadata.permissions().mode() & 0o777, DEFAULT_EXEC_MODE);
        }
    }

    #[test]
    fn test_read_file() {
        let temp = tempdir().unwrap();
        let test_file = temp.path().join("test.txt");
        let content = "Hello, World!";

        fs::write(&test_file, content).unwrap();
        assert_eq!(read_file(&test_file).unwrap(), content);
    }

    #[test]
    fn test_remove_all() {
        let temp = tempdir().unwrap();
        let test_dir = temp.path().join("test_dir");
        let test_file = test_dir.join("test.txt");

        fs::create_dir(&test_dir).unwrap();
        fs::write(&test_file, "content").unwrap();

        assert!(test_dir.exists());
        assert!(test_file.exists());

        remove_all(&test_dir).unwrap();
        assert!(!test_dir.exists());
    }

    #[test]
    fn test_permission_denied() {
        let temp = tempdir().unwrap();
        let test_dir = temp.path().join("readonly");
        let test_file = test_dir.join("test.txt");

        fs::create_dir(&test_dir).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // First create a file to ensure we're the owner
            fs::write(&test_file, "").unwrap();

            // Make both directory and file read-only
            fs::set_permissions(&test_dir, fs::Permissions::from_mode(0o555)).unwrap();
            fs::set_permissions(&test_file, fs::Permissions::from_mode(0o444)).unwrap();

            assert!(matches!(
                write_file(&test_file, "content", false),
                Err(Error::Permission(_))
            ));
        }

        #[cfg(windows)]
        {
            // On Windows, we need to use a different approach
            // First create a file to ensure proper ownership
            fs::write(&test_file, "").unwrap();

            // Set both directory and file to read-only
            let mut perms = fs::metadata(&test_dir).unwrap().permissions();
            perms.set_readonly(true);
            fs::set_permissions(&test_dir, perms).unwrap();

            let mut perms = fs::metadata(&test_file).unwrap().permissions();
            perms.set_readonly(true);
            fs::set_permissions(&test_file, perms).unwrap();

            assert!(matches!(
                write_file(&test_file, "content", false),
                Err(Error::Permission(_))
            ));
        }
    }
}
