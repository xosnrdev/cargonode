//! Core functionality for the cn (cargonode) CLI
//!
//! This crate provides the core types and traits used across the cn ecosystem.

pub mod dependency;
pub mod error;
pub mod fs;
pub mod package_manager;
pub mod path;
pub mod registry;
pub mod template;
pub use error::{Error, Result};

use serde::{Deserialize, Serialize};

/// Type of Node.js project to create
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    /// A Node.js application (default)
    Application,
    /// A Node.js library
    Library,
    /// A Node.js CLI application
    Cli,
}

impl Default for ProjectType {
    fn default() -> Self {
        Self::Application
    }
}

/// Version Control System to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vcs {
    /// Git (default)
    Git,
    /// No version control
    None,
}

impl Default for Vcs {
    fn default() -> Self {
        Self::Git
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_type_default() {
        assert_eq!(ProjectType::default(), ProjectType::Application);
    }

    #[test]
    fn test_vcs_default() {
        assert_eq!(Vcs::default(), Vcs::Git);
    }
}
