# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

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

### 🐛 Bug Fixes

- Set copy options to content only
- Param typo
- _(ci)_ Make ci happy
- _(integration)_ Add async_recursion to execute function resolve recursive `async fn` error
- _(ci)_ Remove pcre2

### 💼 Other

- Not necessary
- Move npm install expression to a function call
- "fmt" script not needed
- Reduce boilerplate
- Update command descriptions and arguments in main.rs
- Std already imported
- Work_dir now generic type AsRef of Path
- Update params
- Use tokio::process::Command for async support
- Revamping modules

### 🚜 Refactor

- Rename to package
- Rename template placeholder
- Improve package creation logging
- Std alread imported
- Remove sample
- Update cargo
- Remove redundant crate
- Make clippy happy
- Updated package.rs
- Revamp main
- _(docs)_ Update README.md with improved installation and usage instructions
- _(integration)_ Update cargonode module description
- _(docs)_ Add Apache License to the repository
- _(config)_ Update .cargo/config.toml for Windows and MUSL targets
- _(deps)_ Add jemallocator crate for memory allocation optimization, update metadata
- _(docs)_ Update assets path in Cargo.toml
- _(ci)_ Update Ubuntu package installation script path
- _(ci)_ Update Ubuntu package installation script path
- _(docs)_ Remove prec2
- _(ci)_ Add executable permission to Ubuntu package installation script
- _(build)_ Update Cargo.toml to include build.rs

### 📚 Documentation

- _(logo)_ Add cargonode logo
- Update README.md

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
