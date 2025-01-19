use std::{
    borrow::Cow,
    env, fs,
    io::{BufReader, Cursor, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;
use flate2::bufread::GzDecoder;
use tar::{Archive, EntryType};

use crate::{
    error::CliError,
    ops::{
        replace::{replace_placeholders, Replacements},
        runner::Runner,
        validate::validate_pkg_name,
    },
    source::{EmbeddedTemplateSource, TemplateSource},
};

struct NewProject<'s> {
    name: &'s Path,
}

impl Runner for NewProject<'_> {
    fn run(&self) -> Result<(), CliError> {
        log::debug!("Creating `{}` package", self.name.display());
        println!("Creating `{}` package", self.name.display());
        validate_project_name(self.name)?;
        create_project_dir(self.name)?;

        let placeholders = prepare_placeholders(self.name);
        let data = load_embedded_template().context("Failed to load embedded template")?;

        let gz = GzDecoder::new(Cursor::new(data));
        let mut archive = Archive::new(gz);

        for entry_res in archive.entries().context("Failed to read tar entries")? {
            let entry = entry_res?;
            let header_type = entry.header().entry_type();
            let path_in_archive = entry.path().context("No entry path")?;
            let out_path = self.name.join(path_in_archive);

            match header_type {
                EntryType::Directory => {
                    log::debug!("Creating `{}` directory", out_path.display());
                    fs::create_dir_all(&out_path).with_context(|| {
                        format!("Failed to create directory: {}", out_path.display())
                    })?;
                }
                EntryType::Regular => {
                    let mut content = String::with_capacity(entry.size() as usize);
                    let mut buf_reader = BufReader::new(entry);
                    log::debug!("Creating `{}` file", out_path.display());
                    buf_reader.read_to_string(&mut content)?;
                    log::debug!("Replacing placeholders in `{}`", out_path.display());
                    let replaced = replace_placeholders(&content, &placeholders);

                    if let Some(parent) = out_path.parent() {
                        log::debug!("Creating `{}` directory", parent.display());
                        fs::create_dir_all(parent).with_context(|| {
                            format!("Failed to create directory: {}", parent.display())
                        })?;
                    }

                    log::debug!("Writing `{}` file", out_path.display());
                    fs::write(&out_path, replaced.as_bytes())
                        .with_context(|| format!("Failed to write file: {}", out_path.display()))?;
                }
                _ => {
                    log::warn!("Unsupported entry type; skipping: {}", out_path.display());
                }
            }
        }
        Ok(())
    }
}

struct InitProject {
    path: PathBuf,
}

impl Runner for InitProject {
    fn run(&self) -> Result<(), CliError> {
        log::debug!("Initializing `{}` package", self.path.display());
        println!("Initializing `{}` package", self.path.display());
        validate_project_name(&self.path)?;
        validate_dir_state(&self.path)?;

        let placeholders = prepare_placeholders(&self.path);
        let data = load_embedded_template().context("Failed to load embedded template")?;

        let gz = GzDecoder::new(Cursor::new(data));
        let mut archive = Archive::new(gz);

        for entry_res in archive.entries().context("Failed to read tar entries")? {
            let entry = entry_res?;
            let header_type = entry.header().entry_type();
            let path_in_archive = entry.path().context("No entry path")?;
            let out_path = self.path.join(path_in_archive);

            match header_type {
                EntryType::Directory => {
                    log::debug!("Creating `{}` directory", out_path.display());
                    fs::create_dir_all(&out_path).with_context(|| {
                        format!("Failed to create directory: {}", out_path.display())
                    })?;
                }
                EntryType::Regular => {
                    let mut content = String::with_capacity(entry.size() as usize);
                    let mut buf_reader = BufReader::new(entry);
                    buf_reader.read_to_string(&mut content)?;
                    log::debug!("Replacing placeholders in `{}`", out_path.display());
                    let replaced = replace_placeholders(&content, &placeholders);

                    if let Some(parent) = out_path.parent() {
                        log::debug!("Creating `{}` directory", parent.display());
                        fs::create_dir_all(parent).with_context(|| {
                            format!("Failed to create directory: {}", parent.display())
                        })?;
                    }

                    if !out_path.exists() {
                        log::debug!("Writing `{}` file", out_path.display());
                        fs::write(&out_path, replaced.as_bytes()).with_context(|| {
                            format!("Failed to write file: {}", out_path.display())
                        })?;
                    } else {
                        log::warn!("File already exists; skipping: {}", out_path.display());
                    }
                }
                _ => {
                    log::warn!("Unsupported entry type; skipping: {}", out_path.display());
                }
            }
        }
        Ok(())
    }
}

fn validate_project_name(path: &Path) -> Result<(), CliError> {
    let dir_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| CliError::message(anyhow::anyhow!("Invalid directory name")))?;

    log::debug!("Validating `{}` package name", dir_name);

    validate_pkg_name(dir_name).map_err(CliError::from)
}

fn validate_dir_state(path: &Path) -> Result<(), CliError> {
    log::debug!("Validating `{}` directory state", path.display());
    const MANIFEST: &str = "package.json";
    if path.read_dir()?.next().is_some() {
        log::warn!("Directory is not empty; existing files will be preserved.");
    }
    if path.join(MANIFEST).exists() {
        return Err(CliError::message(anyhow::anyhow!(
            "package.json manifest already exists in the current directory"
        )));
    }
    Ok(())
}

fn create_project_dir(path: &Path) -> Result<(), CliError> {
    log::debug!("Creating `{}` directory", path.display());
    fs::create_dir(path)
        .map_err(|err| CliError::message(anyhow::format_err!(
            "destination `{}` already exists: {}\n\nUse `cargonode init` to initialize in the current directory",
            path.display(),
            err
        )))
}

fn prepare_placeholders(path: &Path) -> Replacements<'_> {
    log::debug!("Preparing placeholders for `{}`", path.display());
    let mut rep = Replacements::new();
    let base = path.file_name().unwrap_or(path.as_os_str());
    let final_name = base.to_string_lossy();
    rep.add("NAME", final_name);
    rep
}

fn load_embedded_template() -> Result<Cow<'static, [u8]>, CliError> {
    log::debug!("Loading embedded template");
    let source = EmbeddedTemplateSource;
    source.load_template()
}

pub fn with_name(name: &Path) -> Result<(), CliError> {
    let project = NewProject { name };
    project.run()
}

pub fn as_init() -> Result<(), CliError> {
    let current_dir = env::current_dir().context("Failed to get current directory")?;

    let project = InitProject { path: current_dir };

    project.run()
}
