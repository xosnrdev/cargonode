use std::{
    convert::identity,
    fs,
    io::{self, Cursor},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use super::file_util;

pub struct Config {
    pub package_name: String,
    pub current_dir: PathBuf,
    pub template: Template,
}

pub struct Package {
    config: Config,
}

#[derive(Debug)]
pub enum Error {
    InvalidPathName,
    GetUrl(ureq::Error),
    ReadResponse(io::Error),
    ZipExtract(zip_extract::ZipExtractError),
    ReadFile(io::Error),
    WriteFile(io::Error),
    RenameDir(io::Error),
    CopyToDestination(fs_extra::error::Error),
    RenameTemplateDir(io::Error),
    TempDir(io::Error),
}

impl Package {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn create(&self) -> Result<(), Error> {
        validate_package_name(&self.config.package_name)?;
        let temp_dir = tempfile::tempdir().map_err(Error::TempDir)?;
        let template_dir = self.prepare_template(&temp_dir)?;
        copy_to_dest(
            &self.config.package_name,
            &template_dir,
            &self.config.current_dir,
        )?;

        Ok(())
    }

    fn prepare_template(&self, temp_dir: &tempfile::TempDir) -> Result<PathBuf, Error> {
        let template_info = self.config.template.info();
        let temp_dir_path = temp_dir.path();
        let template_dir = temp_dir_path.join(&template_info.path);

        let bytes = download_file(&template_info)?;
        extract_zip(bytes, temp_dir_path)?;
        replace_placeholders(&self.config.package_name, &template_info, &template_dir)?;

        Ok(template_dir)
    }
}

#[derive(Clone)]
pub struct TemplateInfo {
    url: String,
    path: String,
    placeholder: String,
}

pub enum Template {
    CNTS,
    Custom(TemplateInfo),
}

impl Template {
    pub fn info(&self) -> TemplateInfo {
        match self {
            Template::CNTS => TemplateInfo {
                url: "https://github.com/xosnrdev/cargo-node/archive/refs/heads/master.zip"
                    .to_string(),
                path: "templates".to_string(),
                placeholder: "cn-ts".to_string(),
            },
            Template::Custom(info) => info.clone(),
        }
    }
}

fn download_file(template_info: &TemplateInfo) -> Result<Vec<u8>, Error> {
    let response = ureq::get(&template_info.url)
        .call()
        .map_err(Error::GetUrl)?;

    let mut buffer = Vec::new();

    response
        .into_reader()
        .read_to_end(&mut buffer)
        .map_err(Error::ReadResponse)?;

    Ok(buffer)
}

fn extract_zip(bytes: Vec<u8>, _path: &Path) -> Result<(), Error> {
    let mut cursor = Cursor::new(bytes);
    zip_extract::extract(&mut cursor, _path, true).map_err(Error::ZipExtract)?;
    Ok(())
}

fn replace_placeholders(
    package_name: &str,
    template_info: &TemplateInfo,
    template_dir: &PathBuf,
) -> Result<(), Error> {
    let paths = collect_dir_entries(template_dir);

    paths
        .files
        .iter()
        .map(|path| replace_placeholder_in_file(package_name, template_info, path))
        .collect::<Result<(), Error>>()?;

    paths
        .dirs
        .iter()
        .map(|path| replace_placeholder_in_dir(package_name, template_info, path))
        .collect::<Result<(), Error>>()?;

    Ok(())
}

struct Paths {
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

fn collect_dir_entries(template_dir: &PathBuf) -> Paths {
    let entries = WalkDir::new(template_dir)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(err) => {
                eprintln!("Warning: Can't access file: {:?}", err);
                None
            }
        });

    let mut files: Vec<PathBuf> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let file_type = entry.file_type();
        if file_type.is_file() {
            files.push(entry.path().to_path_buf());
        } else if file_type.is_dir() {
            dirs.push(entry.path().to_path_buf());
        }
    }

    Paths { files, dirs }
}

fn replace_placeholder_in_file(
    package_name: &str,
    template_info: &TemplateInfo,
    file_path: &PathBuf,
) -> Result<(), Error> {
    let old_file = file_util::read(file_path).map_err(Error::ReadFile)?;

    let new_content = old_file
        .content
        .replace(&template_info.placeholder, package_name);

    let new_file = file_util::FileData {
        content: new_content,
        permissions: old_file.permissions,
    };

    file_util::write(file_path, new_file).map_err(Error::WriteFile)?;
    Ok(())
}

fn replace_placeholder_in_dir(
    package_name: &str,
    template_info: &TemplateInfo,
    dir_path: &PathBuf,
) -> Result<(), Error> {
    let dir_name = dir_path.file_name().and_then(|name| name.to_str());

    if let Some(old_dir_name) = dir_name {
        let new_dir_name = old_dir_name.replace(&template_info.placeholder, package_name);
        let new_dir_path = dir_path.with_file_name(&new_dir_name);

        if new_dir_name != old_dir_name {
            fs::rename(dir_path, new_dir_path).map_err(Error::RenameDir)?;
        }
    }

    Ok(())
}

fn copy_to_dest(package_name: &str, template_dir: &PathBuf, dest: &PathBuf) -> Result<(), Error> {
    let tmp_project_path = template_dir.with_file_name(package_name);
    fs::rename(&template_dir, &tmp_project_path).map_err(Error::RenameTemplateDir)?;

    fs_extra::dir::copy(tmp_project_path, dest, &fs_extra::dir::CopyOptions::new())
        .map_err(Error::CopyToDestination)?;

    Ok(())
}

fn validate_package_name(package_name: &str) -> Result<(), Error> {
    let not_empty = !package_name.is_empty();
    let has_valid_chars = package_name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c == '-' || c == '_');
    let first_char_is_ascii = package_name
        .chars()
        .nth(0)
        .map_or(false, |c| c.is_ascii_lowercase());

    [not_empty, has_valid_chars, first_char_is_ascii]
        .into_iter()
        .all(identity)
        .then_some(())
        .ok_or(Error::InvalidPathName)
}
