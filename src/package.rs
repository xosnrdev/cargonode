use std::{
    env,
    fmt::{self, Display},
    fs,
    io::{self, Cursor, Read},
    path::{Path, PathBuf},
};

use crate::exec;

// Macro for error mapping and context
macro_rules! map_error {
    ($context:expr) => {
        |err| Error::Io {
            context: $context,
            error: err,
        }
    };
}

// Macro for command execution
macro_rules! exec_command {
    ($work_dir:expr, $program:expr, $($arg:expr),*) => {
        exec::run(&exec::Config {
            work_dir: $work_dir,
            program: $program,
            args: vec![$($arg.to_string()),*],
            env_vars: None,
        })
    };
}

// Macro for validation
macro_rules! validate {
    ($name:expr, $($check:expr),+) => {
        {
            let is_valid = $($check($name))&&+;
            if is_valid { Ok(()) } else { Err(Error::InvalidPackageName) }
        }
    };
}

// Macro for placeholder replacement
macro_rules! replace_content {
    ($content:expr, $placeholder:expr, $replacement:expr) => {
        $content.replace($placeholder, $replacement)
    };
}

/// Comprehensive error handling for package creation operations
#[derive(Debug)]
pub enum Error {
    Io {
        context: &'static str,
        error: io::Error,
    },
    InvalidPackageName,
    GetUrl(Box<ureq::Error>),
    ZipExtract(zip_extract::ZipExtractError),
    CopyToDestination(fs_extra::error::Error),
    Command(exec::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io { context, error } => write!(f, "{}: {}", context, error),
            Self::InvalidPackageName => write!(f, "Invalid package name"),
            Self::GetUrl(err) => write!(f, "Failed to get URL: {}", err),
            Self::ZipExtract(err) => write!(f, "Failed to extract zip: {}", err),
            Self::CopyToDestination(err) => write!(f, "Failed to copy to destination: {}", err),
            Self::Command(err) => write!(f, "Failed to run command: {}", err),
        }
    }
}

/// Represents detailed information about a project template
#[derive(Clone)]
pub struct TemplateInfo {
    pub url: &'static str,
    pub path: &'static str,
    pub placeholder: &'static str,
}

/// Represents different template options for package initialization
pub enum Template {
    NodeTypeScript,
}

impl Template {
    pub fn info(&self) -> TemplateInfo {
        match self {
            Self::NodeTypeScript => TemplateInfo {
                url: "https://github.com/xosnrdev/cargonode/archive/refs/heads/master.zip",
                path: "templates",
                placeholder: "node_typescript",
            },
        }
    }
}

/// Configuration for package creation
pub struct Config {
    pub package_name: String,
    pub current_dir: PathBuf,
    pub template: Template,
}

/// Package creation and initialization struct
pub struct Package {
    config: Config,
}

impl Package {
    /// Create a new Package instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Create a new package in a separate directory
    pub fn create_package(&self) -> Result<String, Error> {
        println!("Creating package: `{}`", self.config.package_name);

        // Use validation macro
        validate_package_name(&self.config.package_name)?;

        // Use error mapping macro
        let temp_dir =
            tempfile::tempdir().map_err(map_error!("Failed to create temporary directory"))?;

        let template_dir = self.prepare_template(&temp_dir, true)?;

        // Copy template to destination
        copy_to_dest(
            &self.config.package_name,
            &template_dir,
            &self.config.current_dir,
        )?;

        // Use command execution macro
        exec_command!(
            self.config
                .current_dir
                .clone()
                .join(&self.config.package_name),
            "git",
            "init"
        )
        .map_err(Error::Command)?;

        // Run npm install
        exec_command!(
            self.config
                .current_dir
                .clone()
                .join(&self.config.package_name),
            "npm",
            "install"
        )
        .map_err(Error::Command)
    }

    /// Create a package in the current directory (init mode)
    pub fn init_package(&self) -> Result<String, Error> {
        let dir_name = self.current_dir_name();
        println!("Creating package: {}", dir_name);

        validate_package_name(&dir_name)?;

        if self.has_node_package() {
            eprintln!("Error: `cargonode init` cannot be run on existing node packages");
            return Ok("".to_string());
        }

        let temp_dir =
            tempfile::tempdir().map_err(map_error!("Failed to create temporary directory"))?;

        let template_dir = self.prepare_template(&temp_dir, false)?;

        flatten_extracted_template(&template_dir, &self.config.current_dir)?;

        // Use command execution macro for git init
        exec_command!(self.config.current_dir.clone(), "git", "init").map_err(Error::Command)?;

        // Run npm install
        exec_command!(self.config.current_dir.clone(), "npm", "install").map_err(Error::Command)
    }

    /// Check if the current directory contains a Node.js package
    fn has_node_package(&self) -> bool {
        self.config.current_dir.join("package.json").exists()
    }

    /// Get the name of the current directory
    fn current_dir_name(&self) -> String {
        self.config
            .current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    }

    /// Prepare the template for package creation
    fn prepare_template(
        &self,
        temp_dir: &tempfile::TempDir,
        is_new: bool,
    ) -> Result<PathBuf, Error> {
        let template_info = self.config.template.info();
        let temp_dir_path = temp_dir.path();
        let template_dir = temp_dir_path.join(template_info.path);

        let bytes = download_file(&template_info)?;
        extract_zip(bytes, temp_dir_path)?;

        replace_placeholders(
            &self.config.package_name,
            &template_info,
            &template_dir,
            is_new,
        )?;
        Ok(template_dir)
    }
}

/// Validate package name against naming conventions
pub fn validate_package_name(name: &str) -> Result<(), Error> {
    validate!(
        name,
        |n: &str| !n.is_empty(),
        |n: &str| n
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c == '_'),
        |n: &str| n
            .chars()
            .next()
            .map(|c| c.is_ascii_lowercase())
            .unwrap_or(false),
        |n: &str| !n.ends_with('-') && !n.ends_with('_')
    )
}

/// Download a file from a given URL
pub fn download_file(template_info: &TemplateInfo) -> Result<Vec<u8>, Error> {
    let response = ureq::get(template_info.url)
        .call()
        .map_err(|err| Error::GetUrl(Box::new(err)))?;

    let mut buffer = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut buffer)
        .map_err(map_error!("Failed to read response"))?;

    Ok(buffer)
}

/// Extract a ZIP file to a specified path
pub fn extract_zip(bytes: Vec<u8>, path: &Path) -> Result<(), Error> {
    zip_extract::extract(&mut Cursor::new(bytes), path, true).map_err(Error::ZipExtract)
}

/// Collect all files and directories in a given directory
fn collect_dir_entries(dir: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), io::Error> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    let mut stack = vec![dir.to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path.clone());
                stack.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    Ok((files, dirs))
}

/// Replace placeholders in template files and directories
pub fn replace_placeholders(
    package_name: &str,
    template_info: &TemplateInfo,
    template_dir: &Path,
    process_dirs: bool,
) -> Result<(), Error> {
    let (files, dirs) =
        collect_dir_entries(template_dir).map_err(map_error!("Failed to read directory"))?;

    // Process files
    for path in files {
        let content = fs::read_to_string(&path).map_err(map_error!("Failed to read file"))?;

        let new_content = replace_content!(content, template_info.placeholder, package_name);

        fs::write(&path, new_content).map_err(map_error!("Failed to write file"))?;
    }

    // Process directories if needed
    if process_dirs {
        for dir_path in dirs {
            if let Some(dir_name) = dir_path.file_name().and_then(|n| n.to_str()) {
                let new_name = dir_name.replace(template_info.placeholder, package_name);
                if new_name != dir_name {
                    let new_path = dir_path.with_file_name(new_name);
                    fs::rename(&dir_path, &new_path)
                        .map_err(map_error!("Failed to rename directory"))?;
                }
            }
        }
    }

    Ok(())
}

/// Copy template to destination directory
fn copy_to_dest(package_name: &str, template_dir: &Path, dest: &Path) -> Result<(), Error> {
    let package_path = template_dir.with_file_name(package_name);
    fs::rename(template_dir, &package_path)
        .map_err(map_error!("Failed to rename template directory"))?;

    fs_extra::dir::copy(
        package_path,
        dest,
        &fs_extra::dir::CopyOptions::new().content_only(true),
    )
    .map_err(Error::CopyToDestination)?;

    Ok(())
}

/// Flatten extracted template to current directory
fn flatten_extracted_template(temp_dir: &Path, current_dir: &Path) -> Result<(), Error> {
    // List all entries in the temporary directory
    let entries: Vec<PathBuf> = fs::read_dir(temp_dir)
        .map_err(map_error!("Failed to read extracted template directory"))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Ensure there's only one top-level directory (the template root)
    if entries.len() != 1 || !entries[0].is_dir() {
        return Err(Error::Io {
            context: "Unexpected structure in extracted template",
            error: io::Error::new(io::ErrorKind::Other, "Invalid template format"),
        });
    }

    let root_dir = &entries[0];

    // Move all contents from the root directory into the current directory
    for entry in
        fs::read_dir(root_dir).map_err(map_error!("Failed to read root directory contents"))?
    {
        let entry = entry.map_err(map_error!("Failed to read entry"))?;
        let source_path = entry.path();
        let dest_path = current_dir.join(entry.file_name());

        if source_path.is_dir() {
            fs::create_dir_all(&dest_path).map_err(map_error!("Failed to create directory"))?;
            fs_extra::dir::copy(
                &source_path,
                &dest_path,
                &fs_extra::dir::CopyOptions::new().content_only(true),
            )
            .map_err(Error::CopyToDestination)?;
        } else if source_path.is_file() {
            fs::copy(&source_path, &dest_path).map_err(map_error!("Failed to copy file"))?;
        }
    }

    Ok(())
}

/// Get the current working directory
pub fn get_current_dir() -> PathBuf {
    env::current_dir().expect("Failed to get current directory")
}

/// Get the name of the current directory
pub fn get_current_dir_name() -> String {
    get_current_dir()
        .file_name()
        .expect("Invalid directory")
        .to_str()
        .expect("Invalid directory name")
        .to_string()
}
