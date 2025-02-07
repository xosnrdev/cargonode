use std::{fs as std_fs, path::Path};

use anyhow::{anyhow, Result};
use serde_json::json;

use crate::{
    core::package::PackageOptions,
    ui::Status,
    util::fs::{
        find_workspace_root, get_package_name, init_git_repository, set_executable_permissions,
        write_with_line_endings, FsCache,
    },
};

const PACKAGE_MANIFEST: &str = "package.json";

#[derive(Debug, Copy, Clone)]
pub enum TemplateType {
    Binary,
    Library,
    TypeScriptBinary,
    TypeScriptLibrary,
}

const BIN_TEMPLATE: &str = "#!/usr/bin/env node\n'use strict';\n\nconsole.log('Hello, world!');";
const BIN_TS_TEMPLATE: &str = "#!/usr/bin/env node\n'use strict';\n\nconsole.log('Hello, world!');";
const LIB_TEMPLATE: &str = "'use strict';\n\n/**\n * @module my-package\n */\n\nexport default {};\n\n// Basic test included\nimport { test } from 'node:test';\nimport assert from 'node:assert';\n\ntest('my-package', (t) => {\n    assert.ok(true, 'should pass');\n});";
const LIB_TS_TEMPLATE: &str = "/**\n * @module my-package\n */\n\nexport interface PackageOptions {\n    name: string;\n    version: string;\n}\n\nexport default class Package {\n    constructor(options: PackageOptions) {\n        // Implementation\n    }\n}\n\n// Basic test included\nimport { test } from 'node:test';\nimport assert from 'node:assert';\n\ntest('my-package', async (t) => {\n    assert.ok(true, 'should pass');\n});";

const fn get_template(template_type: TemplateType) -> &'static str {
    match template_type {
        TemplateType::Binary => BIN_TEMPLATE,
        TemplateType::Library => LIB_TEMPLATE,
        TemplateType::TypeScriptBinary => BIN_TS_TEMPLATE,
        TemplateType::TypeScriptLibrary => LIB_TS_TEMPLATE,
    }
}

const fn get_package_template(is_typescript: bool, is_lib: bool) -> TemplateType {
    match (is_typescript, is_lib) {
        (true, true) => TemplateType::TypeScriptLibrary,
        (true, false) => TemplateType::TypeScriptBinary,
        (false, true) => TemplateType::Library,
        (false, false) => TemplateType::Binary,
    }
}

const TSCONFIG_TEMPLATE: &str = r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "./dist",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}"#;

const GITIGNORE_TEMPLATE: &str = concat!(
    "# Dependencies\n",
    "node_modules/\n",
    "package-lock.json\n",
    "yarn.lock\n",
    "pnpm-lock.yaml\n\n",
    "# Build output\n",
    "dist/\n",
    "build/\n",
    "*.tsbuildinfo\n\n",
    "# Logs\n",
    "logs/\n",
    "*.log\n",
    "npm-debug.log*\n",
    "yarn-debug.log*\n",
    "yarn-error.log*\n\n",
    "# Test coverage\n",
    "coverage/\n",
    ".nyc_output/\n\n",
    "# IDE\n",
    ".idea/\n",
    ".vscode/\n",
    "*.swp\n",
    "*.swo\n\n",
    "# OS\n",
    ".DS_Store\n",
    "Thumbs.db\n",
);

const NPMIGNORE_TEMPLATE: &str = concat!(
    "# Source\n",
    "src/\n",
    "tests/\n",
    "**/*.test.ts\n",
    "**/*.test.js\n\n",
    "# Config files\n",
    "tsconfig.json\n",
    ".eslintrc*\n",
    ".prettier*\n",
    ".editorconfig\n",
    ".github/\n",
    ".vscode/\n\n",
    "# Logs\n",
    "logs/\n",
    "*.log\n\n",
    "# Dependencies\n",
    "node_modules/\n\n",
    "# Build artifacts\n",
    "coverage/\n",
    ".nyc_output/\n",
);

const README_TEMPLATE: &str = r"# {name}

## Description

{description}

## Installation

```bash
npm install {name}
```

## Usage

```javascript
// For libraries
import pkg from '{name}';

// For binaries
npx {name}
```

## License

ISC
";

/// Create a new package at the given path with the specified options.
///
/// # Errors
/// - If the path contains traversal attempts
/// - If symlinks are detected and not allowed
/// - If the destination directory is not empty
/// - If package.json already exists
/// - If filesystem operations fail
///
/// # Panics
/// - If JSON manipulation operations fail
pub fn create_package(opts: &PackageOptions) -> Result<()> {
    let status = Status::new(opts.is_bin(), opts.is_lib(), true);

    // Check for path traversal before any operations
    if opts
        .path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(anyhow!("Path traversal detected: {}", opts.path.display()));
    }

    // Check for symlinks
    if opts.path.is_symlink() {
        return Err(anyhow!("Symlinks are not allowed: {}", opts.path.display()));
    }
    if let Some(parent) = opts.path.parent() {
        if parent.exists() && parent.is_symlink() {
            return Err(anyhow!(
                "Symlinks are not allowed in parent directory: {}",
                parent.display()
            ));
        }
    }

    status.start(&opts.path);

    // Create directory if it doesn't exist
    if !opts.path.exists() {
        std_fs::create_dir_all(&opts.path)?;
    } else if std_fs::read_dir(&opts.path)?.next().is_some() {
        return Err(anyhow!(
            "Destination `{}` already exists and is not empty",
            opts.path.display()
        ));
    }

    let package_json_path = opts.path.join(PACKAGE_MANIFEST);
    if package_json_path.exists() {
        return Err(anyhow!("package.json already exists"));
    }

    // Create initial package.json
    let mut json = json!({
        "name": opts.name.clone().unwrap_or_else(|| get_package_name(&opts.path)),
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
        create_package_structure_in(&opts.path, opts, &mut FsCache::new())?;
        status.created_source_files();
    }

    // Initialize Git only if needed
    if opts.vcs_enabled() && !FsCache::new().is_git_repo(&opts.path)? {
        init_git_repository(&opts.path)?;
        status.initialized_git();
    }

    status.created_package();
    status.finish(
        &opts
            .name
            .clone()
            .unwrap_or_else(|| get_package_name(&opts.path)),
    );
    Ok(())
}

/// Create package.json with basic configuration
fn create_package_json(path: &Path, opts: &PackageOptions, template_content: &str) -> Result<()> {
    let mut package_json = json!({
        "name": opts.package_name().replace(' ', "-"),
        "version": "0.1.0",
        "description": opts.description.as_deref().unwrap_or(""),
        "main": if opts.is_lib() { "lib.js" } else { "index.js" },
        "type": "module",
        "scripts": {
            "test": "node --test"
        },
        "keywords": [],
        "author": opts.author.as_deref().unwrap_or(""),
        "license": "ISC"
    });

    if opts.is_typescript() {
        add_typescript_config(&mut package_json);
    }

    if !opts.is_lib() {
        package_json.as_object_mut().unwrap().insert(
            "bin".to_string(),
            json!({ opts.package_name(): template_content }),
        );
    }

    add_main_exports(&mut package_json, opts, template_content);
    add_workspace_config(&mut package_json, path)?;

    write_with_line_endings(
        &path.join(PACKAGE_MANIFEST),
        &(serde_json::to_string_pretty(&package_json)? + "\n"),
    )?;

    Ok(())
}

/// Add TypeScript-specific configuration to package.json
fn add_typescript_config(package_json: &mut serde_json::Value) {
    let scripts = package_json["scripts"].as_object_mut().unwrap();
    scripts.insert("build".to_string(), json!("tsc"));
    scripts.insert("dev".to_string(), json!("tsc --watch"));
    scripts.insert("clean".to_string(), json!("rimraf dist"));
    scripts.insert(
        "prepublishOnly".to_string(),
        json!("npm run clean && npm run build"),
    );

    let dev_deps = json!({
        "typescript": "^5.0.0",
        "@types/node": "^20.0.0",
        "rimraf": "^5.0.0"
    });

    package_json
        .as_object_mut()
        .unwrap()
        .insert("devDependencies".to_string(), dev_deps);

    package_json
        .as_object_mut()
        .unwrap()
        .insert("types".to_string(), json!("dist/lib.d.ts"));
}

/// Add main and exports fields to package.json
fn add_main_exports(
    package_json: &mut serde_json::Value,
    opts: &PackageOptions,
    template_content: &str,
) {
    package_json
        .as_object_mut()
        .unwrap()
        .insert("main".to_string(), json!(template_content));

    if opts.is_lib() {
        let exports = if opts.is_typescript() {
            json!({
                ".": {
                    "import": "./dist/lib.js",
                    "types": "./dist/lib.d.ts"
                }
            })
        } else {
            json!({
                ".": {
                    "import": template_content,
                    "types": "./types/lib.d.ts"
                }
            })
        };

        package_json
            .as_object_mut()
            .unwrap()
            .insert("exports".to_string(), exports);
    }
}

/// Add workspace-specific configuration to package.json
fn add_workspace_config(package_json: &mut serde_json::Value, path: &Path) -> Result<()> {
    if let Some(workspace_root) = find_workspace_root(path) {
        let root_pkg_json = std_fs::read_to_string(workspace_root.join(PACKAGE_MANIFEST))?;
        if let Ok(root_json) = serde_json::from_str::<serde_json::Value>(&root_pkg_json) {
            let inherit_scripts = root_json
                .get("workspaceConfig")
                .and_then(|c| c.get("inheritScripts"))
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(true);

            if inherit_scripts {
                if let Some(root_scripts) = root_json.get("scripts").and_then(|s| s.as_object()) {
                    let pkg_scripts = package_json["scripts"].as_object_mut().unwrap();
                    for (key, value) in root_scripts {
                        if !pkg_scripts.contains_key(key) {
                            pkg_scripts.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Create source files and set permissions
fn create_source_files(path: &Path, opts: &PackageOptions) -> Result<()> {
    let src_dir = path.join("src");
    std_fs::create_dir_all(&src_dir)?;

    let template_type = get_package_template(opts.is_typescript(), opts.is_lib());
    let source_content = get_template(template_type);
    let main_ext = if opts.is_typescript() { "ts" } else { "js" };
    let main_path = src_dir.join(if opts.is_lib() {
        format!("lib.{main_ext}")
    } else {
        format!("main.{main_ext}")
    });

    write_with_line_endings(&main_path, source_content)?;

    if !opts.is_lib() {
        set_executable_permissions(&main_path)?;
    }

    Ok(())
}

/// Create the package structure in the given directory
pub(super) fn create_package_structure_in(
    path: &Path,
    opts: &PackageOptions,
    _cache: &mut FsCache,
) -> Result<()> {
    let template_type = get_package_template(opts.is_typescript(), opts.is_lib());
    let template_content = get_template(template_type);

    create_package_json(path, opts, template_content)?;
    create_source_files(path, opts)?;

    // Create .gitignore with consistent line endings
    write_with_line_endings(&path.join(".gitignore"), GITIGNORE_TEMPLATE)?;

    // Create .npmignore with consistent line endings
    write_with_line_endings(&path.join(".npmignore"), NPMIGNORE_TEMPLATE)?;

    // Create tsconfig.json for TypeScript projects
    if opts.is_typescript() {
        write_with_line_endings(&path.join("tsconfig.json"), TSCONFIG_TEMPLATE)?;
    }

    // Create README.md with consistent line endings
    let readme_content = README_TEMPLATE
        .replace("{name}", &opts.package_name())
        .replace(
            "{description}",
            opts.description
                .as_deref()
                .unwrap_or("A new Node.js package"),
        );

    write_with_line_endings(&path.join("README.md"), &readme_content)?;

    Ok(())
}
