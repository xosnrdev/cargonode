use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{bail, Context, Result};

// Compile-time platform detection
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

// Compile-time platform constants
#[allow(dead_code)]
pub const CURRENT_PLATFORM: Platform = {
    #[cfg(windows)]
    {
        Platform::Windows
    }
    #[cfg(target_os = "macos")]
    {
        Platform::MacOS
    }
    #[cfg(target_os = "linux")]
    {
        Platform::Linux
    }
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    {
        Platform::Unknown
    }
};

/// Platform-specific path separator (const evaluation)
pub const PATH_SEPARATOR: char = if cfg!(windows) { '\\' } else { '/' };

/// Platform-specific line ending (const evaluation)
pub const LINE_ENDING: &str = if cfg!(windows) { "\r\n" } else { "\n" };

/// Platform-specific executable extension (const evaluation)
pub const EXECUTABLE_EXTENSION: &str = if cfg!(windows) { ".exe" } else { "" };

/// Security flag to prevent path traversal
static ALLOW_SYMLINKS: AtomicBool = AtomicBool::new(false);

/// Set whether to allow symlink traversal
pub fn set_allow_symlinks(allow: bool) {
    ALLOW_SYMLINKS.store(allow, Ordering::SeqCst);
}

/// Check if a path is safe to access
fn is_safe_path(path: &Path) -> Result<()> {
    // Check for path traversal attempts
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        bail!("Path traversal detected: {}", path.display());
    }

    // Check for symlinks if not allowed
    if !ALLOW_SYMLINKS.load(Ordering::SeqCst) {
        // Check if the path exists and is a symlink
        if path.exists() && path.is_symlink() {
            bail!("Symlinks are not allowed: {}", path.display());
        }

        // If the path doesn't exist, check its parent directory
        if let Some(parent) = path.parent() {
            if parent.exists() && parent.is_symlink() {
                bail!(
                    "Symlinks are not allowed in parent directory: {}",
                    parent.display()
                );
            }
        }
    }

    Ok(())
}

/// Sanitize a path string
fn sanitize_path(path: &str) -> String {
    // Remove any null bytes and normalize separators
    path.chars().filter(|&c| c != '\0').collect::<String>()
}

/// Trait for platform-specific operations
pub trait Ops {
    /// Get platform-specific path separator
    fn path_separator(&self) -> char;

    /// Get platform-specific line ending
    fn line_ending(&self) -> &'static str;

    /// Get platform-specific executable extension
    fn executable_extension(&self) -> &'static str;

    /// Set executable permissions
    ///
    /// # Errors
    /// - If the file does not exist
    /// - If there are insufficient permissions
    /// - If the operation is not supported on the current platform
    fn set_executable(&self, path: &Path) -> Result<()>;

    /// Normalize path separators
    fn normalize_path(&self, path: &str) -> String;

    /// Normalize line endings
    fn normalize_line_endings(&self, content: &str) -> String;
}

impl Ops for Platform {
    fn path_separator(&self) -> char {
        PATH_SEPARATOR
    }

    fn line_ending(&self) -> &'static str {
        LINE_ENDING
    }

    fn executable_extension(&self) -> &'static str {
        EXECUTABLE_EXTENSION
    }

    fn set_executable(&self, path: &Path) -> Result<()> {
        set_executable(path)
    }

    fn normalize_path(&self, path: &str) -> String {
        normalize_path(path)
    }

    fn normalize_line_endings(&self, content: &str) -> String {
        normalize_line_endings(content)
    }
}

/// Set executable permissions in a cross-platform way.
///
/// # Arguments
/// * `path` - The path to the file to make executable
///
/// # Platform-specific behavior
/// * Unix: Sets the executable bit (0o755)
/// * Windows: No-op (executability is determined by file extension)
///
/// # Errors
/// * If the file doesn't exist
/// * If the current user lacks permissions to modify the file
/// * If the filesystem doesn't support permission bits (Unix-only)
/// * If path traversal is detected
/// * If symlinks are not allowed but detected
pub fn set_executable(path: &Path) -> Result<()> {
    is_safe_path(path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path)
            .with_context(|| format!("Failed to get metadata for {}", path.display()))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(path, perms)
            .with_context(|| format!("Failed to set permissions for {}", path.display()))?;
    }
    #[cfg(windows)]
    {
        if !path.exists() {
            bail!("File does not exist: {}", path.display());
        }
    }
    Ok(())
}

/// Normalize path separators for the current platform.
///
/// # Arguments
/// * `path` - The path string to normalize
///
/// # Returns
/// A new string with platform-appropriate path separators
#[must_use]
pub fn normalize_path(path: &str) -> String {
    let sanitized = sanitize_path(path);
    #[cfg(windows)]
    {
        sanitized.replace('/', "\\")
    }
    #[cfg(not(windows))]
    {
        sanitized.replace('\\', "/")
    }
}

/// Convert line endings to platform-specific format.
///
/// # Arguments
/// * `content` - The string content to normalize
///
/// # Returns
/// A new string with platform-appropriate line endings
#[must_use]
pub fn normalize_line_endings(content: &str) -> String {
    #[cfg(windows)]
    {
        if !content.contains('\r') {
            // Fast path for Unix line endings
            return content.replace('\n', "\r\n");
        }
        // First normalize to Unix line endings, then convert to Windows
        content.replace("\r\n", "\n").replace('\n', "\r\n")
    }
    #[cfg(not(windows))]
    {
        if !content.contains('\r') {
            // Fast path for already Unix line endings
            return content.to_string();
        }
        content.replace("\r\n", "\n")
    }
}

/// Check if running on Windows.
///
/// # Returns
/// `true` if running on Windows, `false` otherwise
#[must_use]
pub const fn is_windows() -> bool {
    cfg!(windows)
}

/// Get the current user's home directory.
///
/// # Returns
/// * `Some(PathBuf)` - The path to the user's home directory
/// * `None` - If the home directory cannot be determined
#[must_use]
pub fn get_home_dir() -> Option<std::path::PathBuf> {
    static HOME: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
    HOME.get_or_init(dirs::home_dir).clone()
}

/// Get the system's temporary directory.
///
/// # Returns
/// The path to the system's temporary directory
#[must_use]
pub fn get_temp_dir() -> std::path::PathBuf {
    std::env::temp_dir()
}

/// Get the current platform's name.
///
/// # Returns
/// One of: "windows", "macos", "linux", or "unknown"
#[must_use]
pub const fn get_platform_name() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

/// Check if the current platform is Unix-like (Linux, macOS, etc.).
///
/// # Returns
/// `true` if running on a Unix-like platform, `false` otherwise
#[must_use]
pub const fn is_unix_like() -> bool {
    cfg!(unix)
}

/// Validate package name for security and npm compatibility
///
/// # Errors
/// * If the package name contains invalid characters
/// * If the package name is too long (>214 characters)
/// * If the package name has an invalid format for scoped packages
/// * If the package name starts with . or _
/// * If the package name contains non-URL-safe characters
pub fn validate_package_name(name: &str) -> Result<()> {
    // Check for invalid characters
    let invalid_chars = ['<', '>', '|', ':', '"', '?', '*', '\0'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        bail!("Package name contains invalid characters: {}", name);
    }

    // Check length
    if name.len() > 214 {
        bail!("Package name is too long (max 214 characters): {}", name);
    }

    // Handle scoped packages
    if name.starts_with('@') {
        let parts: Vec<&str> = name.split('/').collect();
        if parts.len() != 2 {
            bail!("Invalid scoped package name format: {}", name);
        }
        let scope = &parts[0][1..]; // Remove @ prefix
        let package = parts[1];

        // Validate scope
        if scope.is_empty()
            || !scope
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            bail!("Invalid scope name: {}", scope);
        }

        // Validate package name
        if package.is_empty()
            || !package
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        {
            bail!("Invalid package name: {}", package);
        }
    } else {
        // Regular package name validation
        if name.starts_with('.') || name.starts_with('_') {
            bail!("Package name cannot start with . or _: {}", name);
        }

        // Check for URL-safe characters only
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        {
            bail!("Package name contains non-URL-safe characters: {}", name);
        }
    }

    Ok(())
}

/// Validate file path for security
///
/// # Errors
/// * If path traversal is detected
/// * If absolute paths are used
/// * If symlinks are not allowed but detected
/// * If the path is too long (>260 characters)
pub fn validate_file_path(path: &Path) -> Result<()> {
    // Check for path traversal
    if path.components().any(|c| c.as_os_str() == "..") {
        bail!("Path traversal detected: {}", path.display());
    }

    // Check for absolute paths
    if path.is_absolute() {
        bail!("Absolute paths are not allowed: {}", path.display());
    }

    // Check for symlinks if not allowed
    if !ALLOW_SYMLINKS.load(Ordering::SeqCst) && path.is_symlink() {
        bail!("Symlinks are not allowed: {}", path.display());
    }

    // Check path length
    let path_str = path.to_string_lossy();
    if path_str.len() > 260 {
        bail!("Path is too long (max 260 characters): {}", path.display());
    }

    Ok(())
}

/// Validate workspace pattern for security
///
/// # Errors
/// * If the pattern contains path traversal (..)
/// * If absolute paths are used
/// * If the pattern contains invalid characters
pub fn validate_workspace_pattern(pattern: &str) -> Result<()> {
    // Check for invalid glob patterns
    if pattern.contains("..") {
        bail!("Invalid workspace pattern (contains ..): {}", pattern);
    }

    // Check for absolute paths
    if Path::new(pattern).is_absolute() {
        bail!(
            "Absolute paths not allowed in workspace patterns: {}",
            pattern
        );
    }

    // Check for invalid characters
    let invalid_chars = ['<', '>', '|', ':', '"', '?', '*'];
    if pattern
        .chars()
        .any(|c| invalid_chars.contains(&c) && c != '*')
    {
        bail!("Invalid characters in workspace pattern: {}", pattern);
    }

    Ok(())
}
