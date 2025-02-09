//! Dependency management and version resolution
//!
//! This module provides functionality for managing project dependencies,
//! including version resolution, peer dependency handling, and validation.

use std::collections::HashMap;

use async_trait::async_trait;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Dependency specification with version requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySpec {
    /// Package name
    pub name: String,
    /// Version requirement
    pub version_req: String,
    /// Whether this is a peer dependency
    #[serde(default)]
    pub peer: bool,
    /// Whether this is an optional dependency
    #[serde(default)]
    pub optional: bool,
}

/// Resolved dependency with exact version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    /// Package name
    pub name: String,
    /// Resolved version
    pub version: Version,
    /// Whether this is a peer dependency
    pub peer: bool,
    /// Whether this is an optional dependency
    pub optional: bool,
}

/// Dependency resolution result
#[derive(Debug, Clone, Default)]
pub struct ResolutionResult {
    /// Successfully resolved dependencies
    pub resolved: Vec<ResolvedDependency>,
    /// Dependencies that couldn't be resolved
    pub unresolved: Vec<DependencySpec>,
    /// Conflicting dependencies
    pub conflicts: Vec<DependencyConflict>,
}

/// Dependency version conflict
#[derive(Debug, Clone)]
pub struct DependencyConflict {
    /// Package name
    pub name: String,
    /// Conflicting requirements
    pub requirements: Vec<String>,
    /// Available versions
    pub available_versions: Vec<Version>,
}

/// Validates a dependency specification
pub fn validate_dependency(spec: &DependencySpec) -> Result<()> {
    // Validate package name
    if spec.name.is_empty() {
        return Err(Error::Template("Dependency name cannot be empty".into()));
    }

    // Validate version requirement
    VersionReq::parse(&spec.version_req).map_err(|e| {
        Error::Template(format!(
            "Invalid version requirement for '{}': {}",
            spec.name, e
        ))
    })?;

    Ok(())
}

#[async_trait]
pub trait RegistryClient {
    async fn get_package_info(&self, name: &str) -> Result<crate::registry::RegistryResponse>;
}

#[async_trait]
impl RegistryClient for crate::registry::Client {
    async fn get_package_info(&self, name: &str) -> Result<crate::registry::RegistryResponse> {
        self.get_package_info(name).await
    }
}

/// Resolves dependencies to exact versions
pub async fn resolve_dependencies<C>(
    specs: &[DependencySpec],
    registry_client: &C,
) -> Result<ResolutionResult>
where
    C: RegistryClient + Send + Sync,
{
    let mut result = ResolutionResult::default();
    let mut version_cache: HashMap<String, Vec<Version>> = HashMap::new();

    // First pass: collect all versions
    for spec in specs {
        validate_dependency(spec)?;

        if !version_cache.contains_key(&spec.name) {
            let package_info = registry_client.get_package_info(&spec.name).await?;
            let versions: Vec<Version> = package_info
                .versions
                .iter()
                .filter_map(|v| Version::parse(v).ok())
                .collect();
            version_cache.insert(spec.name.clone(), versions);
        }
    }

    // Second pass: resolve dependencies and detect conflicts
    let mut seen_packages = HashMap::new();
    for spec in specs {
        let versions = version_cache.get(&spec.name).unwrap();
        let req = match VersionReq::parse(&spec.version_req) {
            Ok(req) => req,
            Err(_) => {
                result.unresolved.push(spec.clone());
                continue;
            }
        };

        // Find matching version
        match versions.iter().filter(|v| req.matches(v)).max() {
            Some(version) => {
                if let Some(existing_version) = seen_packages.get(&spec.name) {
                    if version != existing_version {
                        // We have a conflict
                        result.conflicts.push(DependencyConflict {
                            name: spec.name.clone(),
                            requirements: specs
                                .iter()
                                .filter(|s| s.name == spec.name)
                                .map(|s| s.version_req.clone())
                                .collect(),
                            available_versions: versions.clone(),
                        });
                        continue;
                    }
                }

                seen_packages.insert(spec.name.clone(), version.clone());
                result.resolved.push(ResolvedDependency {
                    name: spec.name.clone(),
                    version: version.clone(),
                    peer: spec.peer,
                    optional: spec.optional,
                });
            }
            None => {
                // Check for conflicts
                if seen_packages.contains_key(&spec.name) {
                    result.conflicts.push(DependencyConflict {
                        name: spec.name.clone(),
                        requirements: specs
                            .iter()
                            .filter(|s| s.name == spec.name)
                            .map(|s| s.version_req.clone())
                            .collect(),
                        available_versions: versions.clone(),
                    });
                } else {
                    result.unresolved.push(spec.clone());
                }
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct MockClient {
        packages: HashMap<String, Vec<String>>,
    }

    impl MockClient {
        fn new() -> Self {
            Self::default()
        }

        fn add_package(&mut self, name: &str, versions: Vec<String>) {
            self.packages.insert(name.to_string(), versions);
        }
    }

    #[async_trait]
    impl RegistryClient for MockClient {
        async fn get_package_info(&self, name: &str) -> Result<crate::registry::RegistryResponse> {
            match self.packages.get(name) {
                Some(versions) => Ok(crate::registry::RegistryResponse {
                    id: name.to_string(),
                    name: name.to_string(),
                    dist_tags: HashMap::new(),
                    versions: versions.clone(),
                }),
                None => Err(Error::Registry(format!("Package not found: {}", name))),
            }
        }
    }

    #[test]
    fn test_dependency_validation() {
        // Valid dependency
        let valid_dep = DependencySpec {
            name: "express".into(),
            version_req: "^4.17.1".into(),
            peer: false,
            optional: false,
        };
        assert!(validate_dependency(&valid_dep).is_ok());

        // Empty name
        let invalid_name = DependencySpec {
            name: "".into(),
            version_req: "^4.17.1".into(),
            peer: false,
            optional: false,
        };
        assert!(validate_dependency(&invalid_name).is_err());

        // Invalid version requirement
        let invalid_version = DependencySpec {
            name: "express".into(),
            version_req: "invalid".into(),
            peer: false,
            optional: false,
        };
        assert!(validate_dependency(&invalid_version).is_err());
    }

    #[tokio::test]
    async fn test_dependency_resolution() {
        let mut mock_client = MockClient::new();

        // Mock express package info
        mock_client.add_package(
            "express",
            vec!["4.17.1".into(), "4.17.2".into(), "4.18.0".into()],
        );

        // Test successful resolution
        let specs = vec![DependencySpec {
            name: "express".into(),
            version_req: "^4.17.1".into(),
            peer: false,
            optional: false,
        }];

        let result = resolve_dependencies(&specs, &mock_client).await.unwrap();
        assert_eq!(result.resolved.len(), 1);
        assert_eq!(result.resolved[0].name, "express");
        assert_eq!(result.resolved[0].version, Version::new(4, 18, 0));
    }

    #[tokio::test]
    async fn test_dependency_conflicts() {
        let mut mock_client = MockClient::new();

        // Mock package info
        mock_client.add_package(
            "react",
            vec!["16.0.0".into(), "17.0.0".into(), "18.0.0".into()],
        );

        // Test conflicting requirements
        let specs = vec![
            DependencySpec {
                name: "react".into(),
                version_req: "^16.0.0".into(),
                peer: false,
                optional: false,
            },
            DependencySpec {
                name: "react".into(),
                version_req: "^18.0.0".into(),
                peer: true,
                optional: false,
            },
        ];

        let result = resolve_dependencies(&specs, &mock_client).await.unwrap();
        assert_eq!(result.conflicts.len(), 1);
        assert_eq!(result.conflicts[0].name, "react");
        assert_eq!(result.conflicts[0].requirements.len(), 2);
    }

    #[tokio::test]
    async fn test_peer_dependencies() {
        let mut mock_client = MockClient::new();

        // Mock package info
        mock_client.add_package("react", vec!["18.0.0".into()]);
        mock_client.add_package("react-dom", vec!["18.0.0".into()]);

        let specs = vec![
            DependencySpec {
                name: "react".into(),
                version_req: "^18.0.0".into(),
                peer: false,
                optional: false,
            },
            DependencySpec {
                name: "react-dom".into(),
                version_req: "^18.0.0".into(),
                peer: true,
                optional: false,
            },
        ];

        let result = resolve_dependencies(&specs, &mock_client).await.unwrap();
        assert_eq!(result.resolved.len(), 2);
        assert!(result.resolved.iter().any(|d| d.peer));
    }

    #[tokio::test]
    async fn test_optional_dependencies() {
        let mut mock_client = MockClient::new();

        // Mock package info
        mock_client.add_package("typescript", vec!["4.5.0".into()]);

        let specs = vec![DependencySpec {
            name: "typescript".into(),
            version_req: "^4.5.0".into(),
            peer: false,
            optional: true,
        }];

        let result = resolve_dependencies(&specs, &mock_client).await.unwrap();
        assert_eq!(result.resolved.len(), 1);
        assert!(result.resolved[0].optional);
    }

    #[tokio::test]
    async fn test_unresolvable_dependencies() {
        let mut mock_client = MockClient::new();

        // Mock package info with no matching versions
        mock_client.add_package("nonexistent", vec![]);

        let specs = vec![DependencySpec {
            name: "nonexistent".into(),
            version_req: "^1.0.0".into(),
            peer: false,
            optional: false,
        }];

        let result = resolve_dependencies(&specs, &mock_client).await.unwrap();
        assert_eq!(result.unresolved.len(), 1);
        assert_eq!(result.unresolved[0].name, "nonexistent");
    }
}
