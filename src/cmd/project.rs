use std::{
    borrow::Cow,
    fs,
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

struct Project {
    name: PathBuf,
}

impl Runner for Project {
    fn run(&self) -> Result<(), CliError> {
        validate_project_path(&self.name)?;
        create_project_dir(&self.name)?;

        let placeholders = prepare_placeholders(&self.name);
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
                    fs::create_dir_all(&out_path)
                        .with_context(|| format!("Could not create dir: {}", out_path.display()))?;
                }
                EntryType::Regular => {
                    let mut content = String::with_capacity(entry.size() as usize);
                    let mut buf_reader = BufReader::new(entry);
                    buf_reader.read_to_string(&mut content)?;
                    let replaced = replace_placeholders(&content, &placeholders);

                    if let Some(parent) = out_path.parent() {
                        fs::create_dir_all(parent).with_context(|| {
                            format!("Could not create dir: {}", parent.display())
                        })?;
                    }

                    fs::write(&out_path, replaced.as_bytes())
                        .with_context(|| format!("Could not write file: {}", out_path.display()))?;
                }
                _ => {
                    log::warn!("Skipping unsupported entry type: {}", out_path.display());
                }
            }
        }
        Ok(())
    }
}

fn validate_project_path(path: &Path) -> Result<(), CliError> {
    let text_path = path
        .to_str()
        .ok_or_else(|| CliError::message(anyhow::anyhow!("Invalid UTF-8 path")))?;
    validate_pkg_name(text_path).map_err(CliError::from)
}

fn create_project_dir(path: &Path) -> Result<(), CliError> {
    fs::create_dir(path)
        .with_context(|| {
            format!(
                "Path already exists or cannot be created: {}",
                path.display()
            )
        })
        .map_err(CliError::from)
}

fn prepare_placeholders(path: &Path) -> Replacements<'_> {
    let mut rep = Replacements::new();
    let base = path.file_name().unwrap_or(path.as_os_str());
    let final_name = base.to_string_lossy();
    rep.add("NAME", final_name);
    rep
}

fn load_embedded_template() -> Result<Cow<'static, [u8]>, CliError> {
    let source = EmbeddedTemplateSource;
    source.load_template()
}

pub fn with_name(name: PathBuf) -> Result<(), CliError> {
    let project = Project { name };
    project.run()
}
