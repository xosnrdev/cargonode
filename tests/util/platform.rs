use std::path::Path;
use tempfile::tempdir;

use cargonode::util::platform::{self, Platform};

#[cfg(test)]
mod platform_detection {
    use super::*;

    #[test]
    fn test_current_platform() {
        let platform = platform::CURRENT_PLATFORM;
        #[cfg(windows)]
        assert_eq!(platform, Platform::Windows);
        #[cfg(target_os = "macos")]
        assert_eq!(platform, Platform::MacOS);
        #[cfg(target_os = "linux")]
        assert_eq!(platform, Platform::Linux);
    }

    #[test]
    fn test_platform_consistency() {
        let is_windows = platform::is_windows();
        let is_unix = platform::is_unix_like();
        assert_ne!(
            is_windows, is_unix,
            "Platform detection should be mutually exclusive"
        );
    }
}

#[cfg(test)]
mod constants {
    use super::*;

    #[test]
    fn test_path_separator() {
        #[cfg(windows)]
        assert_eq!(platform::PATH_SEPARATOR, '\\');
        #[cfg(not(windows))]
        assert_eq!(platform::PATH_SEPARATOR, '/');
    }

    #[test]
    fn test_line_ending() {
        #[cfg(windows)]
        assert_eq!(platform::LINE_ENDING, "\r\n");
        #[cfg(not(windows))]
        assert_eq!(platform::LINE_ENDING, "\n");
    }

    #[test]
    fn test_executable_extension() {
        #[cfg(windows)]
        assert_eq!(platform::EXECUTABLE_EXTENSION, ".exe");
        #[cfg(not(windows))]
        assert_eq!(platform::EXECUTABLE_EXTENSION, "");
    }
}

#[cfg(test)]
mod path_handling {
    use super::*;

    #[test]
    fn test_normalize_path() {
        let unix_path = "path/to/file";
        let windows_path = "path\\to\\file";
        let mixed_path = "path\\to/file";

        #[cfg(windows)]
        {
            assert_eq!(platform::normalize_path(unix_path), windows_path);
            assert_eq!(platform::normalize_path(windows_path), windows_path);
            assert_eq!(platform::normalize_path(mixed_path), windows_path);
        }
        #[cfg(not(windows))]
        {
            assert_eq!(platform::normalize_path(unix_path), unix_path);
            assert_eq!(platform::normalize_path(windows_path), unix_path);
            assert_eq!(platform::normalize_path(mixed_path), unix_path);
        }
    }

    #[test]
    fn test_normalize_line_endings() {
        let unix_content = "line1\nline2\n";
        let windows_content = "line1\r\nline2\r\n";
        let mixed_content = "line1\r\nline2\nline3\r\n";

        #[cfg(windows)]
        {
            assert_eq!(
                platform::normalize_line_endings(unix_content),
                windows_content
            );
            assert_eq!(
                platform::normalize_line_endings(windows_content),
                windows_content
            );
            assert_eq!(
                platform::normalize_line_endings(mixed_content),
                "line1\r\nline2\r\nline3\r\n"
            );
        }
        #[cfg(not(windows))]
        {
            assert_eq!(platform::normalize_line_endings(unix_content), unix_content);
            assert_eq!(
                platform::normalize_line_endings(windows_content),
                "line1\nline2\n"
            );
            assert_eq!(
                platform::normalize_line_endings(mixed_content),
                "line1\nline2\nline3\n"
            );
        }
    }
}

#[cfg(test)]
mod file_operations {
    use super::*;

    #[test]
    fn test_set_executable() {
        let temp = tempdir().unwrap();
        let test_file = temp.path().join("test.sh");
        std::fs::write(&test_file, "#!/bin/sh\necho test").unwrap();

        platform::set_executable(&test_file).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::metadata(&test_file).unwrap().permissions();
            assert!(perms.mode() & 0o111 != 0);
        }
    }

    #[test]
    fn test_set_executable_nonexistent() {
        let temp = tempdir().unwrap();
        let nonexistent = temp.path().join("nonexistent");
        assert!(platform::set_executable(&nonexistent).is_err());
    }
}

#[cfg(test)]
mod system_info {
    use super::*;

    #[test]
    fn test_get_home_dir() {
        let home = platform::get_home_dir();
        assert!(home.is_some());
        let home_path = home.unwrap();
        assert!(home_path.exists());
        assert!(home_path.is_absolute());
    }

    #[test]
    fn test_get_temp_dir() {
        let temp = platform::get_temp_dir();
        assert!(temp.exists());
        assert!(temp.is_absolute());
        assert!(temp.is_dir());
    }

    #[test]
    fn test_get_platform_name() {
        let platform_name = platform::get_platform_name();
        assert!(["windows", "macos", "linux", "unknown"].contains(&platform_name));

        #[cfg(windows)]
        assert_eq!(platform_name, "windows");
        #[cfg(target_os = "macos")]
        assert_eq!(platform_name, "macos");
        #[cfg(target_os = "linux")]
        assert_eq!(platform_name, "linux");
    }
}

#[cfg(test)]
mod security_validation {
    use super::*;

    #[test]
    fn test_package_name_validation() {
        // Valid names
        assert!(platform::validate_package_name("valid-package").is_ok());
        assert!(platform::validate_package_name("@scope/package").is_ok());
        assert!(platform::validate_package_name("package.js").is_ok());

        // Invalid names
        assert!(platform::validate_package_name("../package").is_err());
        assert!(platform::validate_package_name("/root/package").is_err());
        assert!(platform::validate_package_name("package<name>").is_err());
        assert!(platform::validate_package_name(".hidden").is_err());
        assert!(platform::validate_package_name("_private").is_err());
    }

    #[test]
    fn test_file_path_validation() {
        // Valid paths
        assert!(platform::validate_file_path(Path::new("valid/path")).is_ok());
        assert!(platform::validate_file_path(Path::new("package/src")).is_ok());

        // Invalid paths
        assert!(platform::validate_file_path(Path::new("../invalid")).is_err());
        assert!(platform::validate_file_path(Path::new("/root/path")).is_err());

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let temp = tempdir().unwrap();
            let link = temp.path().join("link");
            symlink("/target", &link).unwrap();
            assert!(platform::validate_file_path(&link).is_err());
        }
    }

    #[test]
    fn test_workspace_pattern_validation() {
        // Valid patterns
        assert!(platform::validate_workspace_pattern("packages/*").is_ok());
        assert!(platform::validate_workspace_pattern("apps/*").is_ok());

        // Invalid patterns
        assert!(platform::validate_workspace_pattern("../packages/*").is_err());
        assert!(platform::validate_workspace_pattern("/root/packages/*").is_err());
        assert!(platform::validate_workspace_pattern("packages/<invalid>/*").is_err());
    }
}
