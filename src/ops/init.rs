use anyhow::{anyhow, Result};
use serde_json::json;
use std::fs as std_fs;

use crate::{
    core::package::PackageOptions,
    ui::Status,
    util::fs::{get_package_name, init_git_repository, write_with_line_endings, FsCache},
};

const PACKAGE_MANIFEST: &str = "package.json";

/// Initialize a new package in an existing directory
///
/// # Errors
/// - If package.json already exists
/// - If filesystem operations fail
/// - If Git initialization fails
///
/// # Panics
/// - If JSON manipulation operations fail
pub fn init(opts: &PackageOptions) -> Result<()> {
    let status = Status::new(opts.is_bin(), opts.is_lib(), false);
    status.start(&opts.path);

    if !opts.path.exists() {
        std_fs::create_dir_all(&opts.path)?;
    }

    let package_json_path = opts.path.join(PACKAGE_MANIFEST);
    if package_json_path.exists() {
        // If initializing a workspace and package.json exists, check if it's already a workspace
        if opts.workspace {
            let content = std_fs::read_to_string(&package_json_path)?;
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if json.get("workspaces").is_some() {
                    status.warning("workspace already initialized");
                    return Ok(());
                }
            }
        }
        return Err(anyhow!("package.json already exists"));
    }

    let package_name = opts
        .name
        .clone()
        .unwrap_or_else(|| get_package_name(&opts.path));
    let mut json = json!({
        "name": package_name,
        "version": "0.1.0",
        "description": "A new Node.js package",
        "main": if opts.is_lib() { "lib.js" } else { "index.js" },
        "type": "module",
        "scripts": {
            "test": "node --test"
        }
    });

    if opts.workspace {
        let workspace_config = opts.workspace_config.clone().unwrap_or_default();

        // Always use array format for workspaces field
        json.as_object_mut()
            .unwrap()
            .insert("workspaces".to_string(), json!(workspace_config.patterns));

        // Create directories for all workspace patterns
        for pattern in &workspace_config.patterns {
            if let Some(dir) = pattern.strip_suffix("/*") {
                std_fs::create_dir_all(opts.path.join(dir))?;
            }
        }

        // Add workspace-specific fields in a separate field if needed
        if workspace_config.inherit_scripts || !workspace_config.hoist_dependencies {
            json.as_object_mut().unwrap().insert(
                "workspaceConfig".to_string(),
                json!({
                    "inheritScripts": workspace_config.inherit_scripts,
                    "nohoist": if workspace_config.hoist_dependencies { vec![] } else { vec!["**"] }
                }),
            );
        }
        status.created_workspace();
    }

    write_with_line_endings(
        &package_json_path,
        &(serde_json::to_string_pretty(&json).unwrap() + "\n"),
    )?;
    status.created_manifest();

    // Create package structure only if not a workspace
    if !opts.workspace {
        super::new::create_package_structure_in(&opts.path, opts, &mut FsCache::new())?;
        status.created_source_files();
    }

    // Initialize Git only if needed
    if opts.vcs_enabled() && !FsCache::new().is_git_repo(&opts.path)? {
        init_git_repository(&opts.path)?;
        status.initialized_git();
    }

    status.created_package();
    status.finish(&package_name);
    Ok(())
}
