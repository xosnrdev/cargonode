use std::{
    env,
    fmt::{self, Display},
    fs,
    io::{self, Cursor, Read},
    path::{Path, PathBuf},
};

use crate::exec;

/// Comprehensive error handling for package creation operations
#[derive(Debug)]
pub enum Error {
    /// IO-related errors with contextual information
    Io {
        context: &'static str,
        error: io::Error,
    },
    /// Invalid package name error
    InvalidPackageName,
    /// URL retrieval error
    GetUrl(Box<ureq::Error>),
    /// ZIP extraction error
    ZipExtract(zip_extract::ZipExtractError),
    /// File/directory copy error
    CopyToDestination(fs_extra::error::Error),
    /// Command execution error
    Command(exec::Error),
}

impl Error {
    /// Convenience method for creating IO-related errors
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
            Self::Command(err) => write!(f, "Failed to run npm install: {}", err),
        }
    }
}

/// Represents detailed information about a project template
#[derive(Clone)]
pub struct TemplateInfo {
    /// URL of the template source
    url: &'static str,
    /// Path within the downloaded template
    path: &'static str,
    /// Placeholder to be replaced in files and directories
    placeholder: &'static str,
}

/// Represents different template options for package initialization
pub enum Template {
    /// A pre-defined Node.js TypeScript template
    NodeTypeScript,
    /// A custom template with specific configuration
    #[allow(dead_code)]
    Custom(TemplateInfo),
}

impl Template {
    /// Retrieve the template information
    pub fn info(&self) -> TemplateInfo {
        match self {
            Self::NodeTypeScript => TemplateInfo {
                url: "https://github.com/xosnrdev/cargonode/archive/refs/heads/master.zip",
                path: "templates",
                placeholder: "node_typescript",
            },
            Self::Custom(info) => info.clone(),
        }
    }
}

/// Configuration for package creation
pub struct Config {
    /// Name of the package to be created
    pub package_name: String,
    /// Current working directory
    pub current_dir: PathBuf,
    /// Template to use for package creation
    pub template: Template,
}

/// Package creation and initialization struct
pub struct Package {
    /// Configuration for package creation
    config: Config,
}

impl Package {
    /// Create a new Package instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Create a new package in a separate directory
    pub async fn create(&self) -> Result<String, Error> {
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

        init_git(
            self.config
                .current_dir
                .clone()
                .join(&self.config.package_name),
        )
        .await?;

        exec_npm_install(self.config.current_dir.join(&self.config.package_name)).await
    }

    /// Create a package in the current directory (init mode)
    pub async fn create_as_init(&self) -> Result<Option<String>, Error> {
        let dir_name = self.current_dir_name();
        println!("Creating package: {}", dir_name);
        validate_package_name(&dir_name)?;

        if self.has_node_package() {
            eprintln!("Error: `cargonode init` cannot be run on existing node packages");
            return Ok(None);
        }

        let temp_dir = tempfile::tempdir()
            .map_err(|e| Error::io("Failed to create temporary directory", e))?;
        let template_dir = self.prepare_template(&temp_dir, false)?;

        flatten_extracted_template(&template_dir, &self.config.current_dir)?;

        init_git(self.config.current_dir.clone()).await?;
        Ok(Some(
            exec_npm_install(self.config.current_dir.clone()).await?,
        ))
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

/// Download a file from a given URL
fn download_file(template_info: &TemplateInfo) -> Result<Vec<u8>, Error> {
    let response = ureq::get(template_info.url)
        .call()
        .map_err(|err| Error::GetUrl(Box::new(err)))?;

    let mut buffer = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut buffer)
        .map_err(|e| Error::io("Failed to read response", e))?;

    Ok(buffer)
}

/// Extract a ZIP file to a specified path
fn extract_zip(bytes: Vec<u8>, path: &Path) -> Result<(), Error> {
    zip_extract::extract(&mut Cursor::new(bytes), path, true).map_err(Error::ZipExtract)
}

/// Struct to collect file and directory paths
struct Paths {
    /// List of file paths
    files: Vec<PathBuf>,
    /// List of directory paths
    dirs: Vec<PathBuf>,
}

/// Collect all files and directories in a given directory
fn collect_dir_entries(dir: &Path) -> Result<Paths, io::Error> {
    let mut paths = Paths {
        files: Vec::new(),
        dirs: Vec::new(),
    };

    let mut stack = vec![dir.to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                paths.dirs.push(path.clone());
                stack.push(path);
            } else if path.is_file() {
                paths.files.push(path);
            }
        }
    }

    Ok(paths)
}

/// Replace placeholders in template files and directories
fn replace_placeholders(
    package_name: &str,
    template_info: &TemplateInfo,
    template_dir: &Path,
    process_dirs: bool,
) -> Result<(), Error> {
    let paths =
        collect_dir_entries(template_dir).map_err(|e| Error::io("Failed to read directory", e))?;

    // Process files
    for path in &paths.files {
        let content = fs::read_to_string(path).map_err(|e| Error::io("Failed to read file", e))?;

        let new_content = content.replace(template_info.placeholder, package_name);

        fs::write(path, new_content).map_err(|e| Error::io("Failed to write file", e))?;
    }

    // Process directories if needed
    if process_dirs {
        for dir_path in &paths.dirs {
            if let Some(dir_name) = dir_path.file_name().and_then(|n| n.to_str()) {
                let new_name = dir_name.replace(template_info.placeholder, package_name);
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

/// Copy template to destination directory
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

/// Validate package name against naming conventions
fn validate_package_name(name: &str) -> Result<(), Error> {
    let is_valid = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c == '_')
        && name
            .chars()
            .next()
            .map(|c| c.is_ascii_lowercase())
            .unwrap_or(false);

    if is_valid {
        Ok(())
    } else {
        Err(Error::InvalidPackageName)
    }
}

/// Run npm install in the project directory
async fn exec_npm_install(work_dir: PathBuf) -> Result<String, Error> {
    exec::run(&exec::Config {
        work_dir,
        program: "npm",
        args: vec!["install".to_string()],
        env_vars: None,
    })
    .await
    .map_err(Error::Command)
}

/// Initialize git repository
async fn init_git(work_dir: PathBuf) -> Result<String, Error> {
    exec::run(&exec::Config {
        work_dir,
        program: "git",
        args: vec!["init".to_string()],
        env_vars: None,
    })
    .await
    .map_err(Error::Command)
}

/// Flatten extracted template to current directory
fn flatten_extracted_template(temp_dir: &Path, current_dir: &Path) -> Result<(), Error> {
    // List all entries in the temporary directory
    let entries: Vec<PathBuf> = fs::read_dir(temp_dir)
        .map_err(|e| Error::io("Failed to read extracted template directory", e))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Ensure there's only one top-level directory (the template root)
    if entries.len() != 1 || !entries[0].is_dir() {
        return Err(Error::io(
            "Unexpected structure in extracted template",
            io::Error::new(io::ErrorKind::Other, "Invalid template format"),
        ));
    }

    let root_dir = &entries[0];

    // Move all contents from the root directory into the current directory
    for entry in fs::read_dir(root_dir)
        .map_err(|e| Error::io("Failed to read root directory contents", e))?
    {
        let entry = entry.map_err(|e| Error::io("Failed to read entry", e))?;
        let source_path = entry.path();
        let dest_path = current_dir.join(entry.file_name());

        if source_path.is_dir() {
            fs::create_dir_all(&dest_path)
                .map_err(|e| Error::io("Failed to create directory", e))?;
            fs_extra::dir::copy(
                &source_path,
                &dest_path,
                &fs_extra::dir::CopyOptions::new().content_only(true),
            )
            .map_err(Error::CopyToDestination)?;
        } else if source_path.is_file() {
            fs::copy(&source_path, &dest_path).map_err(|e| Error::io("Failed to copy file", e))?;
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
