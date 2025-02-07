use std::path::PathBuf;

/// Workspace configuration options
#[derive(Debug, Clone)]
pub struct WorkspaceConfig {
    /// Custom patterns for workspace packages
    pub patterns: Vec<String>,
    /// Whether to inherit scripts from root
    pub inherit_scripts: bool,
    /// Whether to hoist dependencies to root
    pub hoist_dependencies: bool,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            patterns: vec!["packages/*".to_string()],
            inherit_scripts: true,
            hoist_dependencies: true,
        }
    }
}

/// Package type enumeration
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageType {
    Binary,
    Library,
}

/// Version control system configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VcsConfig {
    Enabled,
    Disabled,
}

/// Language configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    JavaScript,
    TypeScript,
}

/// Options for creating a new package
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct PackageOptions {
    /// Path where the package will be created
    pub path: PathBuf,
    /// Package type (binary or library)
    pub package_type: PackageType,
    /// Version control system configuration
    pub vcs: VcsConfig,
    /// Package name (defaults to directory name)
    pub name: Option<String>,
    /// Package description
    pub description: Option<String>,
    /// Package author
    pub author: Option<String>,
    /// Whether this is a workspace package
    pub workspace: bool,
    /// Language configuration
    pub language: Language,
    /// Workspace configuration
    pub workspace_config: Option<WorkspaceConfig>,
}

impl PackageOptions {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            package_type: PackageType::Binary,
            vcs: VcsConfig::Enabled,
            name: None,
            description: None,
            author: None,
            workspace: false,
            language: Language::JavaScript,
            workspace_config: None,
        }
    }

    pub fn set_bin(&mut self, bin: bool) -> &mut Self {
        self.package_type = if bin {
            PackageType::Binary
        } else {
            PackageType::Library
        };
        self
    }

    pub fn set_lib(&mut self, lib: bool) -> &mut Self {
        self.package_type = if lib {
            PackageType::Library
        } else {
            PackageType::Binary
        };
        self
    }

    pub fn set_vcs(&mut self, vcs: bool) -> &mut Self {
        self.vcs = if vcs {
            VcsConfig::Enabled
        } else {
            VcsConfig::Disabled
        };
        self
    }

    pub fn set_typescript(&mut self, typescript: bool) -> &mut Self {
        self.language = if typescript {
            Language::TypeScript
        } else {
            Language::JavaScript
        };
        self
    }

    #[cfg(test)]
    pub fn set_workspace_config(&mut self, config: WorkspaceConfig) -> &mut Self {
        self.workspace_config = Some(config);
        self
    }

    #[must_use]
    pub fn package_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            self.path
                .file_name()
                .and_then(|name| name.to_str())
                .map_or_else(|| "package".to_string(), ToString::to_string)
        })
    }

    #[must_use]
    pub const fn is_bin(&self) -> bool {
        matches!(self.package_type, PackageType::Binary)
    }

    #[must_use]
    pub const fn is_lib(&self) -> bool {
        matches!(self.package_type, PackageType::Library)
    }

    #[must_use]
    pub const fn is_typescript(&self) -> bool {
        matches!(self.language, Language::TypeScript)
    }

    #[must_use]
    pub const fn vcs_enabled(&self) -> bool {
        matches!(self.vcs, VcsConfig::Enabled)
    }
}
