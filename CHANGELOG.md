# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2025-01-09

### Added

- Add pull request template for improved contribution guidelines by @xosnrdev
- Add CODEOWNERS file to define repository ownership by @xosnrdev
- Add Dependabot configuration and security audit workflow by @xosnrdev
- Add GitHub Actions workflow for installation tests across multiple OS by @xosnrdev
- Add install script for cargonode with platform detection and installation process by @xosnrdev
- Add FUNDING.yml to specify GitHub sponsorship details by @xosnrdev
- Add NixOS installation instructions for cargonode by @xosnrdev
- Add package.nix by @xosnrdev

### Changed

- Bump version to 0.1.3 by @xosnrdev
- Drop legacy release pipeline for distribution with cargo-dist by @xosnrdev
  in [#20](https://github.com/xosnrdev/cargo-node/pull/20)
- Improved user interface experience by @xosnrdev in [#19](https://github.com/xosnrdev/cargo-node/pull/19)
- Bump clap from 4.5.23 to 4.5.24 in the deps group by @dependabot[bot]
  in [#18](https://github.com/xosnrdev/cargo-node/pull/18)
- Bump release-it in /templates/node_typescript by @dependabot[bot]
  in [#17](https://github.com/xosnrdev/cargo-node/pull/17)
- Bump tempfile from 3.14.0 to 3.15.0 in the deps group by @dependabot[bot]
  in [#16](https://github.com/xosnrdev/cargo-node/pull/16)
- Bump serde from 1.0.216 to 1.0.217 in the deps group by @dependabot[bot]
  in [#15](https://github.com/xosnrdev/cargo-node/pull/15)
- Merge pull request #14 from xosnrdev/dependabot/npm_and_yarn/templates/node_typescript/dev-deps-b3604ced80 by
  @xosnrdev in [#14](https://github.com/xosnrdev/cargo-node/pull/14)
- Bump the dev-deps group by @dependabot[bot]
- Merge pull request #13 from xosnrdev/dependabot/cargo/deps-b39bdc87dc by @xosnrdev
  in [#13](https://github.com/xosnrdev/cargo-node/pull/13)
- Bump the deps group with 3 updates by @dependabot[bot]
- Refactor installation script and CI workflow for improved clarity and compatibility by @xosnrdev
  in [#12](https://github.com/xosnrdev/cargo-node/pull/12)
- Refactor installation script and CI workflow by @xosnrdev in [#11](https://github.com/xosnrdev/cargo-node/pull/11)
- Merge pull request #10 from xosnrdev/dev by @xosnrdev in [#10](https://github.com/xosnrdev/cargo-node/pull/10)
- Update README and assets for cargonode by @xosnrdev
- Enhance platform detection and mapping in install_cargonode.sh by @xosnrdev
- Merge pull request #6 from xosnrdev/dev by @xosnrdev in [#6](https://github.com/xosnrdev/cargo-node/pull/6)
- Enhance install_cargonode.sh with force option and checksum utility detection by @xosnrdev
- Replace installation_tests.yml with test_install.yml for improved testing structure by @xosnrdev
- Update installation tests workflow triggers and add workflow dispatch by @xosnrdev
- Update CI and release workflow names with emojis for better visibility by @xosnrdev
- Merge pull request #5 from xosnrdev/dev by @xosnrdev in [#5](https://github.com/xosnrdev/cargo-node/pull/5)
- Merge pull request #4 from xosnrdev/dev by @xosnrdev in [#4](https://github.com/xosnrdev/cargo-node/pull/4)
- Drop once_cell's crate for std sync LazyLock by @xosnrdev
- Streamline Nix configuration and add formatter by @xosnrdev
- Format with nixfmt-rfc-style by @xosnrdev
- Drop the shell script for nixfmt-rfc-style by @xosnrdev
- Use nixfmt by @xosnrdev
- Prepare for publish on nixpkgs by @xosnrdev
- Remove unused build and native build inputs by @xosnrdev
- 4c79687 follow up by @xosnrdev
- Add Nix configuration for building cargonode by @xosnrdev
- Update changelog with patch information by @xosnrdev

### Fixed

- Testing by @xosnrdev
- Change shebang to bash and update BASE_URL construction in install_cargonode.sh by @xosnrdev
- Update script path in installation tests to use relative path by @xosnrdev

### Removed

- Remove unnecessary documentation comments from source files by @xosnrdev
- Remove unused build configuration from Cargo.toml by @xosnrdev

## New Contributors

* @dependabot[bot] made their first contribution in [#18](https://github.com/xosnrdev/cargo-node/pull/18)

## [0.1.2] - 2024-12-02

### Changed

- Release cargonode version 0.1.2 by @xosnrdev
- Merge pull request #3 from xosnrdev/issue-2/external-tools-help by @xosnrdev
  in [#3](https://github.com/xosnrdev/cargo-node/pull/3)
- Update how to install brew by @xosnrdev
- Update changelog format for version 0.1.1 by @xosnrdev
- Update Rust version in CI workflow to stable by @xosnrdev

### Fixed

- Delegate help to external tools by @xosnrdev

## [0.1.1] - 2024-12-01

### Changed

- Release cargonode version 0.1.1 by @xosnrdev
- Update Cargonode formula by @xosnrdev
- Update formula by @xosnrdev

## [0.1.0] - 2024-12-01

### Added

- Added rust doc by @xosnrdev
- Added cargonode.toml boilerplate by @xosnrdev
- Add cn template for bootstrapping new typescript project by @xosnrdev
- Added description metadata and author--email typo by @xosnrdev
- Add basic README file by @xosnrdev
- Add initial project files by @xosnrdev

### Changed

- Update keywords and description by @xosnrdev
- Update CLI tool description and features by @xosnrdev
- Update CLI tool description and features by @xosnrdev
- Update description by @xosnrdev
- Update sha256_releases script by @xosnrdev
- Symbolic link by @xosnrdev
- Merge pull request #1 from xosnrdev/dev by @xosnrdev in [#1](https://github.com/xosnrdev/cargo-node/pull/1)
- Prepare homebrew tap by @xosnrdev
- Pre-release preparation by @xosnrdev
- Rename to snakecase by @xosnrdev
- Add script to generate SHA256 hashes for releases by @xosnrdev
- Update version in CHANGELOG.md to 0.1.0 - 2024-11-30 by @xosnrdev
- Add git-cliff configuration for refined changelog generation by @xosnrdev
- Follow up fix to cbe4350 by @xosnrdev
- Update release.yml to correctly extract version from tag by @xosnrdev
- Merge branch 'master' of github.com:xosnrdev/cargo-node by @xosnrdev
- Rename CHANGELOG.MD to CHANGELOG.md by @xosnrdev
- Merge branch 'master' of github.com:xosnrdev/cargo-node by @xosnrdev
- Merge branch 'master' of github.com:xosnrdev/cargo-node by @xosnrdev
- Ignore test_init_package by @xosnrdev
- Reorganize imports and document process by @xosnrdev
- Use default path by @xosnrdev
- Moved test related to the test dir, use macro for redundant processes and unified docs by @xosnrdev
- Added rust doc and move test related to the test directory by @xosnrdev
- Use macro to generate command handling by @xosnrdev
- Handled edge and test cases better by @xosnrdev
- Should execute synchronously no async deadlocks by @xosnrdev
- Command typo by @xosnrdev
- Update release.yml to match version tag pattern by @xosnrdev
- Update changelog by @xosnrdev
- Update Rust version requirement to 1.80+ by @xosnrdev
- Update yanked "url" crate by @xosnrdev
- Update keywords in Cargo.toml by @xosnrdev
- Add project changelog by @xosnrdev
- Add Homebrew formula for Cargo Node binary by @xosnrdev
- Add release workflow for creating cross-binary releases by @xosnrdev
- Add jemalloc global allocator for musl target on 64-bit systems by @xosnrdev
- Add Windows application manifest file by @xosnrdev
- Add build.rs to set Windows executable options and embed manifest by @xosnrdev
- Update Cargo.toml to include build.rs by @xosnrdev
- Add executable permission to Ubuntu package installation script by @xosnrdev
- Merge branch 'master' of github.com:xosnrdev/cargo-node by @xosnrdev
- Update Ubuntu package installation script path by @xosnrdev
- Update Ubuntu package installation script path by @xosnrdev
- Add script to build and publish cargonode release for aarch64-apple-darwin target by @xosnrdev
- Add script to install required packages on Ubuntu by @xosnrdev
- Update assets path in Cargo.toml by @xosnrdev
- Add GitHub Actions CI workflow by @xosnrdev
- Add jemallocator crate for memory allocation optimization, update metadata by @xosnrdev
- Update .cargo/config.toml for Windows and MUSL targets by @xosnrdev
- Add Apache License to the repository by @xosnrdev
- Update CargoNode module description by @xosnrdev
- Update README.md by @xosnrdev
- Update README.md with improved installation and usage instructions by @xosnrdev
- Add cargonode logo by @xosnrdev
- Revamp main by @xosnrdev
- Updated package.rs by @xosnrdev
- Make clippy happy by @xosnrdev
- Add .cargo/config.toml and .clippy.toml files by @xosnrdev
- Update cargo by @xosnrdev
- Revamping modules by @xosnrdev
- Use tokio::process::Command for async support by @xosnrdev
- Update params by @xosnrdev
- Work_dir now generic type AsRef of Path by @xosnrdev
- Add release-it integration for automating release by @xosnrdev
- Std alread imported by @xosnrdev
- Add vitest integration for running tests by @xosnrdev
- Std already imported by @xosnrdev
- Add tsup integration for building and bundling the current package by @xosnrdev
- Later things by @xosnrdev
- Update command descriptions and arguments in main.rs by @xosnrdev
- Reduce boilerplate by @xosnrdev
- Add "sample" to .gitignore by @xosnrdev
- "fmt" script not needed by @xosnrdev
- Move npm install expression to a function call by @xosnrdev
- Should execute npm install on package creation by @xosnrdev
- Not necessary by @xosnrdev
- Integrating biome by @xosnrdev
- Add new command init by @xosnrdev
- Improve package creation logging by @xosnrdev
- Implement display trait for error by @xosnrdev
- Add create new package command by @xosnrdev
- Rename template placeholder by @xosnrdev
- Rename to package by @xosnrdev
- Resolve cargo by @xosnrdev
- Expose cargo_node modules by @xosnrdev
- Add bootstrap module for handling and managing package creation by @xosnrdev
- Add file_util module for handling reading and writing files by @xosnrdev
- Command exec module for handling command execution with child process in an isolated fashion by @xosnrdev
- Update Cargo.toml with project metadata by @xosnrdev
- Use MIT License by @xosnrdev

### Fixed

- Bright as the sun by @xosnrdev
- Fix symbolic linking issue by @xosnrdev
- Fix changelog path by @xosnrdev
- Moves the leading v from $VERSION if it exists by @xosnrdev
- Resolve "No such file or directory" error in GitHub Actions by @xosnrdev
- Resolve "No such file or directory" error in GitHub Actions by @xosnrdev
- Skip tests if required commands are missing in cross-docker environment by @xosnrdev
- Replace cargo-node with cargonode by @xosnrdev
- Add async_recursion to execute function resolve recursive `async fn` error by @xosnrdev
- Make ci happy by @xosnrdev
- Param typo by @xosnrdev
- Set copy options to content only by @xosnrdev

### Removed

- Remove complete directory by @xosnrdev
- Remove unnecessary heading in README.md by @xosnrdev
- Removed v prefix in version by @xosnrdev
- Removed async runtime related by @xosnrdev
- Remove pcre2 by @xosnrdev
- Remove prec2 by @xosnrdev
- Remove redundant crate by @xosnrdev
- Remove sample by @xosnrdev

## New Contributors

* @xosnrdev made their first contribution
  [0.1.3]: https://github.com/xosnrdev/cargo-node/compare/0.1.2..0.1.3
  [0.1.2]: https://github.com/xosnrdev/cargo-node/compare/0.1.1..0.1.2
  [0.1.1]: https://github.com/xosnrdev/cargo-node/compare/0.1.0..0.1.1

