# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2024-11-30

### 🚀 Features

- Add create new package command
- Implement display trait for error
- Add new command init
- Integrating biome
- Should execute npm install on package creation
- Add tsup integration for building and bundling the current package
- Add vitest integration for running tests
- Add release-it integration for automating release
- Add .cargo/config.toml and .clippy.toml files
- _(ci)_ Add script to build and publish cargonode release for aarch64-apple-darwin target
- _(build)_ Add build.rs to set Windows executable options and embed manifest
- _(pkg)_ Add Windows application manifest file
- _(bin)_ Add jemalloc global allocator for musl target on 64-bit systems
- _(workflows)_ Add release workflow for creating cross-binary releases
- _(pkg)_ Add Homebrew formula for Cargo Node binary
- _(Changelog)_ Add project changelog

### 🐛 Bug Fixes

- Set copy options to content only
- Param typo
- _(ci)_ Make ci happy
- _(integration)_ Add async_recursion to execute function resolve recursive `async fn` error
- _(ci)_ Remove pcre2
- _(brandname)_ Replace cargo-node with cargonode
- _(tests)_ Skip tests if required commands are missing in cross-docker environment
- _(tests)_ Resolve "No such file or directory" error in GitHub Actions
- _(tests)_ Resolve "No such file or directory" error in GitHub Actions
- _(workflows)_ Moves the leading v from $VERSION if it exists

### ⚙️ Miscellaneous Tasks

- Add cn template for bootstrapping new typescript project
- Command exec module for handling command execution with child process in an isolated fashion
- Add file_util module for handling reading and writing files
- Add bootstrap module for handling and managing package creation
- Expose cargo_node modules
- Resolve cargo
- Add "sample" to .gitignore
- Later things
- Add GitHub Actions CI workflow
- _(ci)_ Add script to install required packages on Ubuntu
- _(dependency)_ Update yanked "url" crate
- _(pkg)_ Update Rust version requirement to 1.80+
- _(changelog)_ Update changelog
- _(package)_ Command typo
