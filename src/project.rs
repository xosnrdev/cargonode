use std::{
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
    pkgmgr::{call_with_pm, PackageManager},
    replace::Replacer,
    shell,
    template::load_template,
};

#[derive(Debug, PartialEq)]
pub enum ProjectKind {
    New,
    Init,
}

struct Project<'s> {
    path: &'s Path,
    kind: ProjectKind,
}

impl Project<'_> {
    fn scaffold(&self) -> AppResult<()> {
        shell::log(
            log::Level::Info,
            format!(
                "Scaffolding a {:?} package in '{}'",
                self.kind,
                self.path.display()
            ),
        )?;
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
                        format!("Creating directory: {}", out_path.display()),
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
                        format!("Processing file: {}", out_path.display()),
                    )?;
                    buf_reader.read_to_string(&mut content)?;
                    log::debug!("Replacing placeholders in {:?}", out_path.display());
                    let replaced = replacer.with_haystack(&content);

                    if let Some(parent) = out_path.parent() {
                        log::debug!("Creating {:?} directory", parent.display());
                        fs::create_dir_all(parent).with_context(|| {
                            format!("Failed to create directory: {}", parent.display())
                        })?;
                    }

                    match self.kind {
                        ProjectKind::New => {
                            shell::log(
                                log::Level::Debug,
                                format!("Writing new file: {}", out_path.display()),
                            )?;
                            fs::write(&out_path, replaced.as_bytes()).with_context(|| {
                                format!("Failed to write file: {}", out_path.display())
                            })?;
                        }
                        ProjectKind::Init => {
                            if !out_path.exists() {
                                shell::log(
                                    log::Level::Debug,
                                    format!("Writing new file: {}", out_path.display()),
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
                "Invalid directory name (could not extract a valid UTF-8 name from path '{}')",
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

fn create_project_dir(path: &Path) -> AppResult<()> {
    log::debug!("Attempting to create directory: '{}'", path.display());
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

pub fn new_pkg(dir_name: PathBuf, pm: PackageManager) -> Result<(), CliError> {
    let project = Project {
        path: &dir_name,
        kind: ProjectKind::New,
    };
    project.scaffold()?;
    call_with_pm(pm, dir_name)
}

pub fn init_pkg(pm: PackageManager) -> Result<(), CliError> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;
    let project = Project {
        path: &current_dir,
        kind: ProjectKind::Init,
    };
    project.scaffold()?;
    call_with_pm(pm, current_dir)
}
