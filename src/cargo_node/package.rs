use crate::cargo_node::exec;
use std::{
    fmt::{self, Display},
    fs,
    io::{self, Cursor},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
pub enum Error {
    Io {
        context: &'static str,
        error: io::Error,
    },
    InvalidPackageName,
    GetUrl(ureq::Error),
    ZipExtract(zip_extract::ZipExtractError),
    CopyToDestination(fs_extra::error::Error),
    NpmInstall(exec::Error),
}

impl Error {
    fn io(context: &'static str, error: io::Error) -> Self {
        Self::Io { context, error }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io { context, error } => write!(f, "{}: {}", context, error),
            Self::InvalidPackageName => write!(f, "Invalid package name"),
            Self::GetUrl(err) => write!(f, "Failed to get URL: {}", err),
            Self::ZipExtract(err) => write!(f, "Failed to extract zip: {}", err),
            Self::CopyToDestination(err) => write!(f, "Failed to copy to destination: {}", err),
            Self::NpmInstall(err) => write!(f, "Failed to run npm install: {}", err),
        }
    }
}

#[derive(Clone)]
pub struct TemplateInfo {
    url: String,
    path: String,
    placeholder: String,
}

pub enum Template {
    NodeTypeScript,
    Custom(TemplateInfo),
}

impl Template {
    pub fn info(&self) -> TemplateInfo {
        match self {
            Self::NodeTypeScript => TemplateInfo {
                url: "https://github.com/xosnrdev/cargo-node/archive/refs/heads/master.zip"
                    .to_string(),
                path: "templates".to_string(),
                placeholder: "node_typescript".to_string(),
            },
            Self::Custom(info) => info.clone(),
        }
    }
}

pub struct Config {
    pub package_name: String,
    pub current_dir: PathBuf,
    pub template: Template,
}

pub struct Package {
    config: Config,
}

impl Package {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn create(&self) -> Result<String, Error> {
        println!("Creating package: `{}`", self.config.package_name);
        validate_package_name(&self.config.package_name)?;

        let temp_dir = tempfile::tempdir()
            .map_err(|e| Error::io("Failed to create temporary directory", e))?;
        let template_dir = self.prepare_template(&temp_dir, true)?;

        copy_to_dest(
            &self.config.package_name,
            &template_dir,
            &self.config.current_dir,
        )?;

        exec_npm_install(self.config.current_dir.join(&self.config.package_name))
    }

    pub fn create_as_init(&self) -> Result<Option<String>, Error> {
        let dir_name = self.current_dir_name();
        println!("Creating package: {}", dir_name);
        validate_package_name(&dir_name)?;

        if self.has_node_package() {
            eprintln!("Error: `purr init` cannot be run on existing node packages");
            return Ok(None);
        }

        // TODO: should unpack the template contents into the current directory
        let temp_dir = tempfile::tempdir()
            .map_err(|e| Error::io("Failed to create temporary directory", e))?;
        let template_dir = self.prepare_template(&temp_dir, false)?;

        copy_to_dest(&dir_name, &template_dir, &self.config.current_dir)?;
        Ok(Some(exec_npm_install(self.config.current_dir.clone())?))
    }

    fn has_node_package(&self) -> bool {
        self.config.current_dir.join("package.json").exists()
    }

    fn current_dir_name(&self) -> String {
        self.config
            .current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    }

    fn prepare_template(
        &self,
        temp_dir: &tempfile::TempDir,
        is_new: bool,
    ) -> Result<PathBuf, Error> {
        let template_info = self.config.template.info();
        let temp_dir_path = temp_dir.path();
        let template_dir = temp_dir_path.join(&template_info.path);

        let bytes = download_file(&template_info)?;
        extract_zip(bytes, temp_dir_path)?;

        let package_name = if is_new {
            &self.config.package_name
        } else {
            &self.current_dir_name()
        };

        replace_placeholders(package_name, &template_info, &template_dir, is_new)?;
        Ok(template_dir)
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
        .map_err(|e| Error::io("Failed to read response", e))?;

    Ok(buffer)
}

fn extract_zip(bytes: Vec<u8>, path: &Path) -> Result<(), Error> {
    zip_extract::extract(&mut Cursor::new(bytes), path, true).map_err(Error::ZipExtract)
}

struct Paths {
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

fn collect_dir_entries(dir: &Path) -> Paths {
    let mut paths = Paths {
        files: Vec::new(),
        dirs: Vec::new(),
    };

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            paths.files.push(entry.path().to_path_buf());
        } else if entry.file_type().is_dir() {
            paths.dirs.push(entry.path().to_path_buf());
        }
    }

    paths
}

fn replace_placeholders(
    package_name: &str,
    template_info: &TemplateInfo,
    template_dir: &Path,
    process_dirs: bool,
) -> Result<(), Error> {
    let paths = collect_dir_entries(template_dir);

    // Process files
    for path in &paths.files {
        let content = fs::read_to_string(path).map_err(|e| Error::io("Failed to read file", e))?;

        let new_content = content.replace(&template_info.placeholder, package_name);

        fs::write(path, new_content).map_err(|e| Error::io("Failed to write file", e))?;
    }

    // Process directories if needed
    if process_dirs {
        for dir_path in &paths.dirs {
            if let Some(dir_name) = dir_path.file_name().and_then(|n| n.to_str()) {
                let new_name = dir_name.replace(&template_info.placeholder, package_name);
                if new_name != dir_name {
                    let new_path = dir_path.with_file_name(new_name);
                    fs::rename(dir_path, new_path)
                        .map_err(|e| Error::io("Failed to rename directory", e))?;
                }
            }
        }
    }

    Ok(())
}

fn copy_to_dest(package_name: &str, template_dir: &Path, dest: &Path) -> Result<(), Error> {
    let package_path = template_dir.with_file_name(package_name);
    fs::rename(template_dir, &package_path)
        .map_err(|e| Error::io("Failed to rename template directory", e))?;

    fs_extra::dir::copy(
        package_path,
        dest,
        &fs_extra::dir::CopyOptions::new().content_only(true),
    )
    .map_err(Error::CopyToDestination)?;

    Ok(())
}

fn validate_package_name(name: &str) -> Result<(), Error> {
    let is_valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c == '_')
        && name
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_lowercase());

    if is_valid {
        Ok(())
    } else {
        Err(Error::InvalidPackageName)
    }
}

fn exec_npm_install(work_dir: PathBuf) -> Result<String, Error> {
    exec::run(&exec::Config {
        work_dir,
        cmd: "npm".to_string(),
        args: vec!["install".to_string()],
    })
    .map_err(Error::NpmInstall)
}
