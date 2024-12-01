# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1](https://github.com/xosnrdev/cargonode/compare/0.1.0...0.1.1) - 2024-12-01
### Highlights
- Patch release with minor bug fixes and improvements.

## [0.1.0] - 2024-11-30

### Highlights

- Initial release of cargonode with essential features and integrations.

### Features

- **Package Management**: Added command to create a new package and initialize existing projects.
- **Error Handling**: Implemented display trait for improved error reporting.
- **Integration**:
  - Integrated Biome for enhanced development workflows.
  - Added Tsup for building and bundling packages.
  - Included Vitest for running tests.
  - Integrated Release-It for automating the release process.
- **Configuration Files**: Added `.cargo/config.toml` and `.clippy.toml` for project configuration.
- **Cross-Platform Support**:
  - Added scripts for building and publishing releases for various platforms, including aarch64-apple-darwin.
  - Included Windows application manifest file for better compatibility.

### Bug Fixes

- Resolved issues with missing files in GitHub Actions.
- Fixed parameter typos and improved CI processes.
- Addressed async recursion errors in integration tests.

### Miscellaneous Improvements

- Added a template for bootstrapping new TypeScript projects.
- Created utility modules for file handling and command execution.
- Updated Rust version requirement to 1.80+ for better performance and features.

### Refactor

- Streamlined codebase by removing unnecessary scripts and reducing boilerplate.
- Enhanced command descriptions and arguments for clarity.
