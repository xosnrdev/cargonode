# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [Unreleased]


### Added

- Add create new package command by @xosnrdev
- Implement display trait for error by @xosnrdev
- Add new command init by @xosnrdev
- Integrating biome by @xosnrdev
- Should execute npm install on package creation by @xosnrdev
- Add tsup integration for building and bundling the current package by @xosnrdev
- Add vitest integration for running tests by @xosnrdev
- Add release-it integration for automating release by @xosnrdev
- Add .cargo/config.toml and .clippy.toml files by @xosnrdev
- Add script to build and publish cargonode release for aarch64-apple-darwin target by @xosnrdev
- Add build.rs to set Windows executable options and embed manifest by @xosnrdev
- Add Windows application manifest file by @xosnrdev
- Add jemalloc global allocator for musl target on 64-bit systems by @xosnrdev
- Add release workflow for creating cross-binary releases by @xosnrdev
- Add Homebrew formula for Cargo Node binary by @xosnrdev
- Add project changelog by @xosnrdev
- Add script to generate SHA256 hashes for releases by @xosnrdev
- Add Nix configuration for building cargonode by @xosnrdev
- 4c79687 follow up by @xosnrdev
- Add install script for cargonode with platform detection and installation process by @xosnrdev
- Add GitHub Actions workflow for installation tests across multiple OS by @xosnrdev
- Enhance install_cargonode.sh with force option and checksum utility detection by @xosnrdev
- Update README and assets for cargonode by @xosnrdev
- Add centralized error module with `anyhow` crate ([#27](https://github.com/xosnrdev/cargonode/issues/27)) by @Success Kingsley
- Implement step execution with cycle detection by @xosnrdev
- Add generic command execution and CLI support for run, check, build, and test commands ([#65](https://github.com/xosnrdev/cargonode/issues/65)) by @Success Kingsley
- Add history and cache management commands by @Success Kingsley
- Add output verification and error handling for command execution by @Success Kingsley


### Changed

- Rename to package by @xosnrdev
- Rename template placeholder by @xosnrdev
- Improve package creation logging by @xosnrdev
- Std alread imported by @xosnrdev
- Remove sample by @xosnrdev
- Update cargo by @xosnrdev
- Remove redundant crate by @xosnrdev
- Make clippy happy by @xosnrdev
- Updated package.rs by @xosnrdev
- Revamp main by @xosnrdev
- Update README.md with improved installation and usage instructions by @xosnrdev
- Update CargoNode module description by @xosnrdev
- Add Apache License to the repository by @xosnrdev
- Update .cargo/config.toml for Windows and MUSL targets by @xosnrdev
- Add jemallocator crate for memory allocation optimization, update metadata by @xosnrdev
- Update assets path in Cargo.toml by @xosnrdev
- Update Ubuntu package installation script path by @xosnrdev
- Update Ubuntu package installation script path by @xosnrdev
- Remove prec2 by @xosnrdev
- Add executable permission to Ubuntu package installation script by @xosnrdev
- Update Cargo.toml to include build.rs by @xosnrdev
- Update keywords in Cargo.toml by @xosnrdev
- Update release.yml to match version tag pattern by @xosnrdev
- Removed async runtime related by @xosnrdev
- Handled edge and test cases better by @xosnrdev
- Use macro to generate command handling by @xosnrdev
- Added cargonode.toml boilerplate by @xosnrdev
- Added rust doc and move test related to the test directory by @xosnrdev
- Moved test related to the test dir, use macro for redundant processes and unified docs by @xosnrdev
- Use default path by @xosnrdev
- Reorganize imports and document process by @xosnrdev
- Ignore test_init_package by @xosnrdev
- Fix changelog path by @xosnrdev
- Update release.yml to correctly extract version from tag by @xosnrdev
- Follow up fix to cbe4350 by @xosnrdev
- Update version in CHANGELOG.md to 0.1.0 - 2024-11-30 by @xosnrdev
- Removed v prefix in version by @xosnrdev
- Pre-release preparation by @xosnrdev
- Remove unnecessary heading in README.md by @xosnrdev
- Remove complete directory by @xosnrdev
- Update sha256_releases script by @xosnrdev
- Update description by @xosnrdev
- Update CLI tool description and features by @xosnrdev
- Update CLI tool description and features by @xosnrdev
- Update keywords and description by @xosnrdev
- Update formula by @xosnrdev
- Update Cargonode formula by @xosnrdev
- Update Rust version in CI workflow to stable by @xosnrdev
- Update changelog format for version 0.1.1 by @xosnrdev
- Update how to install brew by @xosnrdev
- Update changelog with patch information by @xosnrdev
- Remove unused build and native build inputs by @xosnrdev
- Use nixfmt by @xosnrdev
- Format with nixfmt-rfc-style by @xosnrdev
- Streamline Nix configuration and add formatter by @xosnrdev
- Remove unnecessary documentation comments from source files by @xosnrdev
- Drop once_cell's crate for std sync LazyLock by @xosnrdev
- Replace installation_tests.yml with test_install.yml for improved testing structure by @xosnrdev
- Enhance platform detection and mapping in install_cargonode.sh by @xosnrdev
- Update README docs ([#35](https://github.com/xosnrdev/cargonode/issues/35)) by @Success Kingsley
- Simplify command and workflow execution handling ([#47](https://github.com/xosnrdev/cargonode/issues/47)) by @Success Kingsley
- Simplify job and workflow execution by delegates argument passing directly by @xosnrdev
- Make project functionality even better by @Success Kingsley


### Documentation

- Add cargonode logo by @xosnrdev
- Update README.md by @xosnrdev
- Added rust doc by @xosnrdev
- Add NixOS installation instructions for cargonode by @xosnrdev
- Revamp README with comprehensive project overview and usage guide by @xosnrdev
- Update README with development status and build instructions by @xosnrdev
- Update README with development status and build instructions by @Success Kingsley


### Fixed

- Set copy options to content only by @xosnrdev
- Param typo by @xosnrdev
- Make ci happy by @xosnrdev
- Add async_recursion to execute function resolve recursive `async fn` error by @xosnrdev
- Remove pcre2 by @xosnrdev
- Replace cargo-node with cargonode by @xosnrdev
- Skip tests if required commands are missing in cross-docker environment by @xosnrdev
- Resolve "No such file or directory" error in GitHub Actions by @xosnrdev
- Resolve "No such file or directory" error in GitHub Actions by @xosnrdev
- Moves the leading v from $VERSION if it exists by @xosnrdev
- Bright as the sun by @xosnrdev
- Delegate help to external tools ([#2](https://github.com/xosnrdev/cargonode/issues/2)) by @xosnrdev
- Update script path in installation tests to use relative path by @xosnrdev
- Change shebang to bash and update BASE_URL construction in install_cargonode.sh by @xosnrdev
- CI wasm32-wasip1 error ([#23](https://github.com/xosnrdev/cargonode/issues/23)) by @Success Kingsley
- Message display abnormalities and formatting by @Success Kingsley


### Miscellaneous Tasks

- Add cn template for bootstrapping new typescript project by @xosnrdev
- Command exec module for handling command execution with child process in an isolated fashion by @xosnrdev
- Add file_util module for handling reading and writing files by @xosnrdev
- Add bootstrap module for handling and managing package creation by @xosnrdev
- Expose cargo_node modules by @xosnrdev
- Resolve cargo by @xosnrdev
- Add "sample" to .gitignore by @xosnrdev
- Later things by @xosnrdev
- Add script to install required packages on Ubuntu by @xosnrdev
- Update yanked "url" crate by @xosnrdev
- Update Rust version requirement to 1.80+ by @xosnrdev
- Update changelog by @xosnrdev
- Command typo by @xosnrdev
- Add git-cliff configuration for refined changelog generation by @xosnrdev
- Rename to snakecase by @xosnrdev
- Prepare homebrew tap by @xosnrdev
- Symbolic link by @xosnrdev
- Fix symbolic linking issue by @xosnrdev
- Release cargonode version 0.1.1 by @xosnrdev
- Release cargonode version 0.1.2 by @xosnrdev
- Prepare for publish on nixpkgs by @xosnrdev
- Add package.nix by @xosnrdev
- Remove unused build configuration from Cargo.toml by @xosnrdev
- Drop the shell script for nixfmt-rfc-style by @xosnrdev
- Add FUNDING.yml to specify GitHub sponsorship details by @xosnrdev
- Add Dependabot configuration and security audit workflow by @xosnrdev
- Add CODEOWNERS file to define repository ownership by @xosnrdev
- Add pull request template for improved contribution guidelines by @xosnrdev
- Update CI and release workflow names with emojis for better visibility by @xosnrdev
- Update installation tests workflow triggers and add workflow dispatch by @xosnrdev
- Bump the deps group with 3 updates by @dependabot[bot]
- Bump the dev-deps group by @dependabot[bot]
- Bump serde from 1.0.216 to 1.0.217 in the deps group ([#15](https://github.com/xosnrdev/cargonode/issues/15)) by @dependabot[bot]
- Bump tempfile from 3.14.0 to 3.15.0 in the deps group ([#16](https://github.com/xosnrdev/cargonode/issues/16)) by @dependabot[bot]
- Bump release-it in /templates/node_typescript ([#17](https://github.com/xosnrdev/cargonode/issues/17)) by @dependabot[bot]
- Bump clap from 4.5.23 to 4.5.24 in the deps group ([#18](https://github.com/xosnrdev/cargonode/issues/18)) by @dependabot[bot]
- Bump version to 0.1.3 by @xosnrdev
- Resolves post release by @xosnrdev
- Bump the dev-deps group ([#22](https://github.com/xosnrdev/cargonode/issues/22)) by @dependabot[bot]
- Bump clap from 4.5.24 to 4.5.26 in the deps group ([#21](https://github.com/xosnrdev/cargonode/issues/21)) by @dependabot[bot]
- Bump the deps group with 3 updates ([#29](https://github.com/xosnrdev/cargonode/issues/29)) by @dependabot[bot]
- Bump the deps group across 1 directory with 2 updates ([#34](https://github.com/xosnrdev/cargonode/issues/34)) by @dependabot[bot]
- Bump serde_json from 1.0.137 to 1.0.138 in the deps group ([#40](https://github.com/xosnrdev/cargonode/issues/40)) by @dependabot[bot]
- Bump the dev-deps group ([#41](https://github.com/xosnrdev/cargonode/issues/41)) by @dependabot[bot]
- Bump tempfile from 3.15.0 to 3.16.0 in the deps group ([#44](https://github.com/xosnrdev/cargonode/issues/44)) by @dependabot[bot]
- Bump vitest from 2.1.8 to 3.0.4 in /assets/template ([#43](https://github.com/xosnrdev/cargonode/issues/43)) by @dependabot[bot]
- Bump @vitest/coverage-v8 in /assets/template ([#42](https://github.com/xosnrdev/cargonode/issues/42)) by @dependabot[bot]
- Bump the dev-deps group ([#50](https://github.com/xosnrdev/cargonode/issues/50)) by @dependabot[bot]
- Bump clap from 4.5.27 to 4.5.28 in the deps group ([#52](https://github.com/xosnrdev/cargonode/issues/52)) by @dependabot[bot]
- Bump which from 7.0.1 to 7.0.2 in the deps group ([#53](https://github.com/xosnrdev/cargonode/issues/53)) by @dependabot[bot]
- Bump clap from 4.5.28 to 4.5.29 in the deps group ([#57](https://github.com/xosnrdev/cargonode/issues/57)) by @dependabot[bot]
- Prevent overwriting non-empty .gitignore and improve project structure creation ([#59](https://github.com/xosnrdev/cargonode/issues/59)) by @Success Kingsley
- Bump the deps group with 2 updates by @dependabot[bot]
- Bump the deps group with 2 updates ([#61](https://github.com/xosnrdev/cargonode/issues/61)) by @dependabot[bot]
- Bump clap from 4.5.30 to 4.5.31 in the deps group by @dependabot[bot]


### Refactor

- Not necessary by @xosnrdev
- Move npm install expression to a function call by @xosnrdev
- "fmt" script not needed by @xosnrdev
- Reduce boilerplate by @xosnrdev
- Update command descriptions and arguments in main.rs by @xosnrdev
- Std already imported by @xosnrdev
- Work_dir now generic type AsRef of Path by @xosnrdev
- Update params by @xosnrdev
- Use tokio::process::Command for async support by @xosnrdev
- Revamping modules by @xosnrdev
- Should execute synchronously no async deadlocks by @xosnrdev
- Implement project handler and template embedding functionalities by @Success Kingsley
- Cleanup redundant and obsolete implementations ([#38](https://github.com/xosnrdev/cargonode/issues/38)) by @Success Kingsley
- Extract dependency installation logic into separate function by @xosnrdev


### Testing

- Add comprehensive test suites and improve config handling ([#46](https://github.com/xosnrdev/cargonode/issues/46)) by @Success Kingsley


### Ci

- Add GitHub Actions CI workflow by @xosnrdev


### Recfactor

- Keep things in check ([#68](https://github.com/xosnrdev/cargonode/issues/68)) by @Success Kingsley

