use std::{
    borrow::Cow,
    env, fs,
    io::{BufReader, Cursor, Read},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Context;
use flate2::bufread::GzDecoder;
use regex::Regex;
use tar::{Archive, EntryType};

use crate::{
    error::{AppResult, CliError},
    pkgmgr::PackageManager,
    replace::Replacer,
    shell,
};

// Generated at build time by build.rs script
include!(concat!(env!("OUT_DIR"), "/embedding.rs"));

#[derive(Debug, PartialEq)]
enum ProjectKind {
    New,
    Init,
}

struct Project<'s> {
    path: &'s Path,
    kind: ProjectKind,
}

impl Project<'_> {
    fn scaffold(&self) -> AppResult<()> {
        shell::status("Creating", format!("`{}` package", self.path.display()))?;
        validate_dir_name(self.path)?;
        if self.kind == ProjectKind::New {
            create_project_dir(self.path)?;
        } else {
            validate_dir_state(self.path)?;
        }

        let replacer = prepare_replacement(self.path);
        let data = load_template();
        let gz = GzDecoder::new(Cursor::new(data));
        let mut archive = Archive::new(gz);

        for entry_res in archive.entries()? {
            let entry = entry_res?;
            let header_type = entry.header().entry_type();
            let path_in_archive = entry.path()?;
            let out_path = self.path.join(path_in_archive);

            match header_type {
                EntryType::Directory => {
                    shell::log(
                        log::Level::Debug,
                        format!("Creating {:?} directory", out_path.display()),
                    )?;
                    fs::create_dir_all(&out_path).with_context(|| {
                        format!("Failed to create directory: {}", out_path.display())
                    })?;
                }
                EntryType::Regular => {
                    let mut content = String::with_capacity(entry.size() as usize);
                    let mut buf_reader = BufReader::new(entry);
                    shell::log(
                        log::Level::Debug,
                        format!("Processing file: {:?}", out_path.display()),
                    )?;
                    buf_reader.read_to_string(&mut content)?;
                    shell::log(
                        log::Level::Debug,
                        format!("Replacing placeholders in {:?}", out_path.display()),
                    )?;
                    let replaced = replacer.with_haystack(&content);

                    if let Some(parent) = out_path.parent() {
                        shell::log(
                            log::Level::Debug,
                            format!("Creating {:?} directory", parent.display()),
                        )?;
                        fs::create_dir_all(parent).with_context(|| {
                            format!("Failed to create directory: {}", parent.display())
                        })?;
                    }

                    match self.kind {
                        ProjectKind::New => {
                            shell::log(
                                log::Level::Debug,
                                format!("Writing new file: {:?}", out_path.display()),
                            )?;
                            fs::write(&out_path, replaced.as_bytes()).with_context(|| {
                                format!("Failed to write file: {}", out_path.display())
                            })?;
                        }
                        ProjectKind::Init => {
                            if !out_path.exists() {
                                shell::log(
                                    log::Level::Debug,
                                    format!("Writing new file: {:?}", out_path.display()),
                                )?;
                                fs::write(&out_path, replaced.as_bytes()).with_context(|| {
                                    format!("Failed to write file: {}", out_path.display())
                                })?;
                            } else {
                                shell::warn(format!(
                                    "File already exists; skipping: {}",
                                    out_path.display()
                                ))?;
                            }
                        }
                    }
                }
                _ => {
                    shell::warn(format!(
                        "Unsupported entry type; skipping: {}",
                        out_path.display()
                    ))?;
                }
            }
        }
        Ok(())
    }
}

fn validate_dir_name(path: &Path) -> AppResult<()> {
    let dir_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Invalid directory name (could not extract a valid UTF-8 name from path {:?})",
                path.display()
            )
        })?;
    validate_pkg_name(dir_name)
}

fn validate_pkg_name(name: &str) -> AppResult<()> {
    const MAX_PKG_NAME_LENGTH: usize = 214;
    // Reference: <https://docs.npmjs.com/cli/v7/configuring-npm/package-json#name>
    static PKG_NAME_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$",
        )
        .expect("Failed to compile package name regex")
    });

    if name.trim().is_empty() {
        anyhow::bail!("Package name cannot be empty.");
    }
    if name.len() > MAX_PKG_NAME_LENGTH {
        anyhow::bail!(
            "Package name length exceeds {max} characters (found length: {}). \
            The maximum allowed is {max}.",
            name.len(),
            max = MAX_PKG_NAME_LENGTH
        );
    }
    if name.starts_with(".") || name.starts_with("_") {
        anyhow::bail!("Package name cannot start with a period (.) or underscore (_).");
    }
    if name.contains("..") || (name.contains('@') && !name.contains('/')) {
        anyhow::bail!("Package name cannot contain double periods (..) or '@' without a scope.");
    }

    if !PKG_NAME_PATTERN.is_match(name) {
        anyhow::bail!("Package name '{name}' contains invalid characters.");
    }

    Ok(())
}

fn validate_dir_state(path: &Path) -> AppResult<()> {
    const MANIFEST: &str = "package.json";
    if path.read_dir()?.next().is_some() {
        shell::warn(format!(
            "Directory '{}' is not empty. Existing files will be preserved.",
            path.display()
        ))?;
    }
    if path.join(MANIFEST).exists() {
        anyhow::bail!(
            "A '{}' manifest already exists in '{}'.",
            MANIFEST,
            path.display()
        );
    }
    Ok(())
}

fn load_template() -> Cow<'static, [u8]> {
    Cow::Borrowed(EMBEDDED_TEMPLATE)
}

fn create_project_dir(path: &Path) -> AppResult<()> {
    shell::log(
        log::Level::Debug,
        format!("Creating {:?} directory", path.display()),
    )?;
    fs::create_dir(path).map_err(|err| {
        anyhow::format_err!(
            "Destination '{}' already exists or cannot be created: {}\n\
         \nHint: Use `cargonode init` to initialize in the current directory.",
            path.display(),
            err
        )
    })
}

fn prepare_replacement(path: &Path) -> Replacer<'_> {
    let mut rep = Replacer::new();
    let base = path.file_name().unwrap_or(path.as_os_str());
    let final_name = base.to_string_lossy();
    rep.add("NAME", final_name);
    rep
}

pub fn new_pkg(dir_name: PathBuf, pm: Option<PackageManager>) -> Result<(), CliError> {
    let project = Project {
        path: &dir_name,
        kind: ProjectKind::New,
    };
    project.scaffold()?;
    install_deps(dir_name, pm)
}

pub fn init_pkg(pm: Option<PackageManager>) -> Result<(), CliError> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project = Project {
        path: &current_dir,
        kind: ProjectKind::Init,
    };
    project.scaffold()?;
    install_deps(current_dir, pm)
}

fn install_deps(dir_name: PathBuf, pm: Option<PackageManager>) -> Result<(), CliError> {
    match pm {
        Some(pm) => {
            pm.call(dir_name)?;
        }
        None => {
            shell::note("No package manager was specified. If you don't have a preferred manager, run `npm install` to install dependencies.")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};
    use tempfile::TempDir;

    use super::*;

    fn create_temp_dir() -> TempDir {
        tempfile::tempdir().expect("Failed to create temporary directory")
    }

    fn create_temp_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let file_path = dir.join(name);
        let mut file = File::create(&file_path).expect("Failed to create temporary file");
        writeln!(file, "{}", content).expect("Failed to write to temporary file");
        file_path
    }

    // #[test]
    // fn test_project_scaffold_new() {
    //     // Arrange
    //     let temp_dir = create_temp_dir();
    //     let project = Project {
    //         path: temp_dir.path(),
    //         kind: ProjectKind::New,
    //     };
    //     // Act
    //     let result = project.scaffold();
    //     // Assert
    //     assert!(result.is_ok());
    //     // Act
    //     let package_json = temp_dir.path().join("package.json");
    //     // Assert
    //     assert!(package_json.exists());
    // }

    // #[test]
    // fn test_project_scaffold_init() {
    //     // Arrange
    //     let temp_dir = create_temp_dir();
    //     let project = Project {
    //         path: temp_dir.path(),
    //         kind: ProjectKind::Init,
    //     };
    //     // Act
    //     let result = project.scaffold();
    //     if let Err(ref err) = result {
    //         dbg!(err);
    //     }
    //     // Assert
    //     assert!(result.is_ok());
    //     // Act
    //     let package_json = temp_dir.path().join("package.json");
    //     // Assert
    //     assert!(package_json.exists());
    // }

    #[test]
    fn test_project_scaffold_init_existing_file() {
        // Arrange
        let temp_dir = create_temp_dir();
        create_temp_file(temp_dir.path(), "package.json", "{}");
        let project = Project {
            path: temp_dir.path(),
            kind: ProjectKind::Init,
        };
        // Act
        let result = project.scaffold();
        // Assert
        assert!(result.is_err());
    }

    // #[test]
    // fn test_validate_dir_name_valid() {
    //     // Arrange
    //     let temp_dir = create_temp_dir();
    //     // Act
    //     let result = validate_dir_name(temp_dir.path());
    //     if let Err(ref err) = result {
    //         dbg!(err);
    //     }
    //     // Assert
    //     assert!(result.is_ok());
    // }

    #[test]
    fn test_validate_dir_name_invalid() {
        // Arrange
        let invalid_path = Path::new("");
        // Act
        let result = validate_dir_name(invalid_path);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pkg_name_valid() {
        // Arrange
        let valid_name = "valid-package-name";
        // Act
        let result = validate_pkg_name(valid_name);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_pkg_name_invalid() {
        // Arrange
        let invalid_name = "invalid/package/name";
        // Act
        let result = validate_pkg_name(invalid_name);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_dir_state_empty() {
        // Arrange
        let temp_dir = create_temp_dir();
        // Act
        let result = validate_dir_state(temp_dir.path());
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dir_state_non_empty() {
        // Arrange
        let temp_dir = create_temp_dir();
        create_temp_file(temp_dir.path(), "test.txt", "content");
        // Act
        let result = validate_dir_state(temp_dir.path());
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_dir_state_existing_package_json() {
        // Arrange
        let temp_dir = create_temp_dir();
        create_temp_file(temp_dir.path(), "package.json", "{}");
        // Act
        let result = validate_dir_state(temp_dir.path());
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_create_project_dir_valid() {
        // Arrange
        let temp_dir = create_temp_dir();
        let new_dir = temp_dir.path().join("new_dir");
        // Act
        let result = create_project_dir(&new_dir);
        // Assert
        assert!(result.is_ok());
        assert!(new_dir.exists());
    }

    #[test]
    fn test_create_project_dir_existing() {
        // Arrange
        let temp_dir = create_temp_dir();
        // Act
        let result = create_project_dir(temp_dir.path());
        // Assert
        assert!(result.is_err());
    }
}
