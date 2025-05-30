# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [Unreleased]


### Documentation

- Update changelog ([`41024c2`](https://github.com/xosnrdev/cargonode/commit/41024c2ed4007173d0cd0b1fd765c0245f4ec56f))
- Update changelog ([`3ab85e0`](https://github.com/xosnrdev/cargonode/commit/3ab85e086568fe97ca28cb6ea4a7f54ce995d461))
- Update changelog ([`0172948`](https://github.com/xosnrdev/cargonode/commit/017294801fb59a4a77dfc8b34073ec75829a607a))


### Fixed

- Changelog release version bug ([`ef873b9`](https://github.com/xosnrdev/cargonode/commit/ef873b95bb2f34662fa1362d958c646fecaa0cec))


### Miscellaneous Tasks

- Bump the deps group with 2 updates ([`7000ff8`](https://github.com/xosnrdev/cargonode/commit/7000ff837e5238fb718f7a0820443799edc113b2))
- Bump tempfile from 3.17.1 to 3.18.0 in the deps group ([`74fa53c`](https://github.com/xosnrdev/cargonode/commit/74fa53ca50c30cc1fc1bb81ddad2167f3ae04cac))
- Bump the deps group with 2 updates ([`76a11c6`](https://github.com/xosnrdev/cargonode/commit/76a11c6d5d84fac102fd0d3f3afbdf1c5ac28c0b))
- Bump tempfile from 3.18.0 to 3.19.0 in the deps group ([`333e679`](https://github.com/xosnrdev/cargonode/commit/333e6798c75469390f02b0da76dd87b2ebc06525))
- Bump the deps group across 1 directory with 2 updates ([`85d961a`](https://github.com/xosnrdev/cargonode/commit/85d961a51d094504770e83677b889bf67b1a0036))

## [1.0.0] - 2025-03-04


### Added

- Add centralized error module with `anyhow` crate ([#27](https://github.com/xosnrdev/cargonode/issues/27)) ([`07af321`](https://github.com/xosnrdev/cargonode/commit/07af321cd476f5aae223302692675e4600ac124a))
- Implement step execution with cycle detection ([`e4aa6c1`](https://github.com/xosnrdev/cargonode/commit/e4aa6c16184a89a5683087f53092ef85a2475cb4))
- Add generic command execution and CLI support for run, check, build, and test commands ([#65](https://github.com/xosnrdev/cargonode/issues/65)) ([`c7df096`](https://github.com/xosnrdev/cargonode/commit/c7df096243ce78a5baefc9a4493e8cc011c436c4))
- Add history and cache management commands ([`4780384`](https://github.com/xosnrdev/cargonode/commit/4780384057fd004e354bd47fb1ddff936da90d65))
- Add output verification and error handling for command execution ([`74ba6e5`](https://github.com/xosnrdev/cargonode/commit/74ba6e520e2010aecea020eb754317d2b1cdd078))


### Changed

- Update README docs ([#35](https://github.com/xosnrdev/cargonode/issues/35)) ([`98a536e`](https://github.com/xosnrdev/cargonode/commit/98a536e4e1245607cc2506166840a94ad2836b44))
- Simplify command and workflow execution handling ([#47](https://github.com/xosnrdev/cargonode/issues/47)) ([`73c4f05`](https://github.com/xosnrdev/cargonode/commit/73c4f05f105bcfe2c45d916ed8aa13f2f54c9977))
- Simplify job and workflow execution by delegates argument passing directly ([`e6bda19`](https://github.com/xosnrdev/cargonode/commit/e6bda196a2c5adc2c7db440708ce91e6046dd6cb))
- Make project functionality even better ([`0e942f2`](https://github.com/xosnrdev/cargonode/commit/0e942f223a949b5407e8265cb3626dcb6e73ebed))


### Documentation

- Revamp README with comprehensive project overview and usage guide ([`899e41c`](https://github.com/xosnrdev/cargonode/commit/899e41c626eb76eabacf3d122981fac4d2a02ab6))
- Update README with development status and build instructions ([`2f94aca`](https://github.com/xosnrdev/cargonode/commit/2f94acadd9da4c811020daf36cd58e9e0992d2b0))
- Update README with development status and build instructions ([`c1ac927`](https://github.com/xosnrdev/cargonode/commit/c1ac9273a37a5d1ad77a36204fd73ff9365b9926))
- Update changelog ([#71](https://github.com/xosnrdev/cargonode/issues/71)) ([`4bcd492`](https://github.com/xosnrdev/cargonode/commit/4bcd4926c903fd2331a10a67e343a78906de5dc1))
- Update changelog ([`ff81d33`](https://github.com/xosnrdev/cargonode/commit/ff81d338575bda09b2cbd6a3faefac011c55fe7e))
- Update with detailed configuration protocol ([`5536e05`](https://github.com/xosnrdev/cargonode/commit/5536e057a0b9af299bd6a4914d08fbe4e9701b18))
- Update changelog ([`552d83f`](https://github.com/xosnrdev/cargonode/commit/552d83f4cea96f721891c61533126beab4844fa5))
- Improve installation instructions and formatting ([`58b2f1c`](https://github.com/xosnrdev/cargonode/commit/58b2f1c9879d9bea47dd6f174088314554b9e3d7))


### Fixed

- CI wasm32-wasip1 error ([#23](https://github.com/xosnrdev/cargonode/issues/23)) ([`782098c`](https://github.com/xosnrdev/cargonode/commit/782098cd0ab459ed5fe5f7b3689aff7df6cc82b8))
- Message display abnormalities and formatting ([`9a8e2d3`](https://github.com/xosnrdev/cargonode/commit/9a8e2d3d94e84d75a307da1ca0773152b49af57b))
- Make outputs optional and improve error handling ([#74](https://github.com/xosnrdev/cargonode/issues/74)) ([`829c415`](https://github.com/xosnrdev/cargonode/commit/829c4154611039f1f323fca50fa121df91ac1b02))


### Miscellaneous Tasks

- Resolves post release ([`32e0a5a`](https://github.com/xosnrdev/cargonode/commit/32e0a5ab2a640fc4a611ab3472ddaa5c497eafcd))
- Bump the dev-deps group ([#22](https://github.com/xosnrdev/cargonode/issues/22)) ([`6382282`](https://github.com/xosnrdev/cargonode/commit/6382282b16a08ff38966f3ea3b1feb30c00afe1a))
- Bump clap from 4.5.24 to 4.5.26 in the deps group ([#21](https://github.com/xosnrdev/cargonode/issues/21)) ([`4cd39c2`](https://github.com/xosnrdev/cargonode/commit/4cd39c2746da3031f3fab43bcc86fc55065bd3ac))
- Bump the deps group with 3 updates ([#29](https://github.com/xosnrdev/cargonode/issues/29)) ([`588c98c`](https://github.com/xosnrdev/cargonode/commit/588c98c3a497bfb41eb02ea8e75b55bd861b4b61))
- Bump the deps group across 1 directory with 2 updates ([#34](https://github.com/xosnrdev/cargonode/issues/34)) ([`86940b2`](https://github.com/xosnrdev/cargonode/commit/86940b2838d352945f495fffe1b11df8f0c9a6a3))
- Bump serde_json from 1.0.137 to 1.0.138 in the deps group ([#40](https://github.com/xosnrdev/cargonode/issues/40)) ([`4181e08`](https://github.com/xosnrdev/cargonode/commit/4181e08340021d93e2c3f3ebc1011fa5293526cd))
- Bump the dev-deps group ([#41](https://github.com/xosnrdev/cargonode/issues/41)) ([`ce728bc`](https://github.com/xosnrdev/cargonode/commit/ce728bcc045d86de82657b3d3c71357fd4c88fc7))
- Bump tempfile from 3.15.0 to 3.16.0 in the deps group ([#44](https://github.com/xosnrdev/cargonode/issues/44)) ([`4021905`](https://github.com/xosnrdev/cargonode/commit/402190570d319e71d6d751b09845fe70011b9652))
- Bump vitest from 2.1.8 to 3.0.4 in /assets/template ([#43](https://github.com/xosnrdev/cargonode/issues/43)) ([`fef584a`](https://github.com/xosnrdev/cargonode/commit/fef584a1aef4d0b9936e98bece70acd0c49abe67))
- Bump @vitest/coverage-v8 in /assets/template ([#42](https://github.com/xosnrdev/cargonode/issues/42)) ([`03ed4a7`](https://github.com/xosnrdev/cargonode/commit/03ed4a798346405a1696f6f31911d63b4b6f7cd1))
- Bump the dev-deps group ([#50](https://github.com/xosnrdev/cargonode/issues/50)) ([`cae1d74`](https://github.com/xosnrdev/cargonode/commit/cae1d74ed390c4a6d6e0459e405745e8fde87dd5))
- Bump clap from 4.5.27 to 4.5.28 in the deps group ([#52](https://github.com/xosnrdev/cargonode/issues/52)) ([`16adb6f`](https://github.com/xosnrdev/cargonode/commit/16adb6f02c148ca40c0dd6cff50a00318b91572d))
- Bump which from 7.0.1 to 7.0.2 in the deps group ([#53](https://github.com/xosnrdev/cargonode/issues/53)) ([`c0a88bc`](https://github.com/xosnrdev/cargonode/commit/c0a88bced41c8bcc80b8abbd141fbb4e7aa3ecd4))
- Bump clap from 4.5.28 to 4.5.29 in the deps group ([#57](https://github.com/xosnrdev/cargonode/issues/57)) ([`27b8e09`](https://github.com/xosnrdev/cargonode/commit/27b8e091252618b373fd5bbe0df5b783ff65b758))
- Prevent overwriting non-empty .gitignore and improve project structure creation ([#59](https://github.com/xosnrdev/cargonode/issues/59)) ([`6613c87`](https://github.com/xosnrdev/cargonode/commit/6613c8733a8ea53f37cc38cb384fc20b87a1ebc9))
- Bump the deps group with 2 updates ([`6bef44b`](https://github.com/xosnrdev/cargonode/commit/6bef44b247a3eab37b366a2dbf814427ed13670c))
- Bump the deps group with 2 updates ([#61](https://github.com/xosnrdev/cargonode/issues/61)) ([`9a836a6`](https://github.com/xosnrdev/cargonode/commit/9a836a602481a16a46747476299f093f1ee34b76))
- Bump clap from 4.5.30 to 4.5.31 in the deps group ([`c552edc`](https://github.com/xosnrdev/cargonode/commit/c552edce02b3a3bf442214b46e223b64beb6899e))
- Update git-cliff changelog template ([`1b2721f`](https://github.com/xosnrdev/cargonode/commit/1b2721f3bdeb6bb56bb9372313a2c1aa79105a57))
- Bump version to 1.0.0 ([`62a7dc7`](https://github.com/xosnrdev/cargonode/commit/62a7dc7e5f11e5e876ccc2fb14dbd8eef981a2ce))


### Refactor

- Implement project handler and template embedding functionalities ([`71471af`](https://github.com/xosnrdev/cargonode/commit/71471afce570a0f7f2928803b3c1380944798ee3))
- Cleanup redundant and obsolete implementations ([#38](https://github.com/xosnrdev/cargonode/issues/38)) ([`d45e58f`](https://github.com/xosnrdev/cargonode/commit/d45e58ffdc31a6cb08b6ac18760709b04133429a))
- Extract dependency installation logic into separate function ([`6825937`](https://github.com/xosnrdev/cargonode/commit/68259379338d35285c47c3536c3330f6f2d34873))


### Testing

- Add comprehensive test suites and improve config handling ([#46](https://github.com/xosnrdev/cargonode/issues/46)) ([`686bcb9`](https://github.com/xosnrdev/cargonode/commit/686bcb9beaa20161533c8a96063fbecefc7be8bb))


### Recfactor

- Keep things in check ([#68](https://github.com/xosnrdev/cargonode/issues/68)) ([`a0156cb`](https://github.com/xosnrdev/cargonode/commit/a0156cb9b28a20292dd50f900c157f6d39a01cdf))

## [0.1.3] - 2025-01-09


### Added

- Add Nix configuration for building cargonode ([`4c79687`](https://github.com/xosnrdev/cargonode/commit/4c79687b1d7faa757ccbdc896dc09bfece1517af))
- 4c79687 follow up ([`26c74fd`](https://github.com/xosnrdev/cargonode/commit/26c74fd28fd71950d303fc52a4694d97a00ba712))
- Add install script for cargonode with platform detection and installation process ([`4ab9bd3`](https://github.com/xosnrdev/cargonode/commit/4ab9bd303a8ff0c9d9ce1fb202217fbd9142de63))
- Add GitHub Actions workflow for installation tests across multiple OS ([`b7ce3ec`](https://github.com/xosnrdev/cargonode/commit/b7ce3ecaa088518ef137dd243a0397bdc3c5d02b))
- Enhance install_cargonode.sh with force option and checksum utility detection ([`86e585a`](https://github.com/xosnrdev/cargonode/commit/86e585a7922a13ff885a43431e7a6a19b08ce139))
- Update README and assets for cargonode ([`1033869`](https://github.com/xosnrdev/cargonode/commit/1033869d0a97abe015f6f825a41e4a88af7c08ad))


### Changed

- Update changelog with patch information ([`735ef38`](https://github.com/xosnrdev/cargonode/commit/735ef3849da9dd0d0b0052d0ea8abf3d42070577))
- Remove unused build and native build inputs ([`838751d`](https://github.com/xosnrdev/cargonode/commit/838751dd6f349871801d6d0b0107bc8f7c480888))
- Use nixfmt ([`d9a91c2`](https://github.com/xosnrdev/cargonode/commit/d9a91c22d8ca3745db858ca7979ff8f301ffeb57))
- Format with nixfmt-rfc-style ([`25f7014`](https://github.com/xosnrdev/cargonode/commit/25f7014b948e74fba1be6a05f09e6975a4d78bd1))
- Streamline Nix configuration and add formatter ([`bbd2a5d`](https://github.com/xosnrdev/cargonode/commit/bbd2a5dc9b2dc98a93f5a8b4c15d59f8dc4fecdb))
- Remove unnecessary documentation comments from source files ([`17261b1`](https://github.com/xosnrdev/cargonode/commit/17261b1bf98cdafb9d32b422e158dc53b5d0a227))
- Drop once_cell's crate for std sync LazyLock ([`38e1c23`](https://github.com/xosnrdev/cargonode/commit/38e1c236942fc927b76ce3fc4c84c0f8befed91d))
- Replace installation_tests.yml with test_install.yml for improved testing structure ([`9e59272`](https://github.com/xosnrdev/cargonode/commit/9e592721880a8b4ce8aa6e5dce1e98f04c81359c))
- Enhance platform detection and mapping in install_cargonode.sh ([`3c58e0e`](https://github.com/xosnrdev/cargonode/commit/3c58e0efad7162f5c7ee1584aed0ad9823a87029))


### Documentation

- Add NixOS installation instructions for cargonode ([`24d3190`](https://github.com/xosnrdev/cargonode/commit/24d319087d81fbf86254096f858927154052b5e5))


### Fixed

- Update script path in installation tests to use relative path ([`3cfde37`](https://github.com/xosnrdev/cargonode/commit/3cfde37c2b139e4c6bed3d9aec4a18cc5426d00a))
- Change shebang to bash and update BASE_URL construction in install_cargonode.sh ([`3cbc706`](https://github.com/xosnrdev/cargonode/commit/3cbc706361fece45f7f757391d24b8106f853adc))


### Miscellaneous Tasks

- Prepare for publish on nixpkgs ([`4cf2794`](https://github.com/xosnrdev/cargonode/commit/4cf279495175c762c1e68528301d239904d359d5))
- Add package.nix ([`ce3b44a`](https://github.com/xosnrdev/cargonode/commit/ce3b44a2ad9e2c30b892efd8ff9eab246cadb0c7))
- Remove unused build configuration from Cargo.toml ([`dceb58d`](https://github.com/xosnrdev/cargonode/commit/dceb58ded6ffaf0ea26ee0c47cd4e42f221b98ae))
- Drop the shell script for nixfmt-rfc-style ([`9683192`](https://github.com/xosnrdev/cargonode/commit/968319250b69b0044e35e5b04c3d62538ff389a1))
- Add FUNDING.yml to specify GitHub sponsorship details ([`a2b6919`](https://github.com/xosnrdev/cargonode/commit/a2b69192a3b5cef899ed3bdce6426c3c3b59f7be))
- Add Dependabot configuration and security audit workflow ([`109d93e`](https://github.com/xosnrdev/cargonode/commit/109d93e32591ea347094b25db89aa4213cf1374a))
- Add CODEOWNERS file to define repository ownership ([`e48f101`](https://github.com/xosnrdev/cargonode/commit/e48f101adf19f2d3fa3f8222411a3032beb2368a))
- Add pull request template for improved contribution guidelines ([`ba8ba16`](https://github.com/xosnrdev/cargonode/commit/ba8ba162403315e3b38146c6e2bc5bfc18c33737))
- Update CI and release workflow names with emojis for better visibility ([`c462643`](https://github.com/xosnrdev/cargonode/commit/c46264314af41bbdf5f0bc25f870858065bb60ec))
- Update installation tests workflow triggers and add workflow dispatch ([`07a9e0b`](https://github.com/xosnrdev/cargonode/commit/07a9e0b0a5db47621d7b9daab409ea5d13048914))
- Bump the deps group with 3 updates ([`eabc573`](https://github.com/xosnrdev/cargonode/commit/eabc5734ed793feee56c724b1804c49c3c9f5bf1))
- Bump the dev-deps group ([`378fed3`](https://github.com/xosnrdev/cargonode/commit/378fed3adddf7b24dd909d02e8929b67c4078b73))
- Bump serde from 1.0.216 to 1.0.217 in the deps group ([#15](https://github.com/xosnrdev/cargonode/issues/15)) ([`6bfeaac`](https://github.com/xosnrdev/cargonode/commit/6bfeaac924e794086077c57a577e5b356fcabeac))
- Bump tempfile from 3.14.0 to 3.15.0 in the deps group ([#16](https://github.com/xosnrdev/cargonode/issues/16)) ([`6852a5e`](https://github.com/xosnrdev/cargonode/commit/6852a5e1517336b255470d37d04e0594d1acd9fb))
- Bump release-it in /templates/node_typescript ([#17](https://github.com/xosnrdev/cargonode/issues/17)) ([`ea8eb47`](https://github.com/xosnrdev/cargonode/commit/ea8eb4787f630ee78e160e6eebbba8d290252e89))
- Bump clap from 4.5.23 to 4.5.24 in the deps group ([#18](https://github.com/xosnrdev/cargonode/issues/18)) ([`bb76074`](https://github.com/xosnrdev/cargonode/commit/bb76074438c98542a6bf96462f0bb7e4d18a3e1e))
- Bump version to 0.1.3 ([`0122f75`](https://github.com/xosnrdev/cargonode/commit/0122f752f65e369fd85dabf83f59ad58859679eb))

## [0.1.2] - 2024-12-02


### Changed

- Update Rust version in CI workflow to stable ([`2dee9c3`](https://github.com/xosnrdev/cargonode/commit/2dee9c3d2fa495002da684484c17197615f05e43))
- Update changelog format for version 0.1.1 ([`56b5c41`](https://github.com/xosnrdev/cargonode/commit/56b5c41761abae195f651124369ffab1757edc47))
- Update how to install brew ([`eade76d`](https://github.com/xosnrdev/cargonode/commit/eade76db8a5e4839a485160fed85e489d199f213))


### Fixed

- Delegate help to external tools ([#2](https://github.com/xosnrdev/cargonode/issues/2)) ([`ddbd35a`](https://github.com/xosnrdev/cargonode/commit/ddbd35a402c3b906aea95c060ae37fcfffceb777))


### Miscellaneous Tasks

- Release cargonode version 0.1.2 ([`5392474`](https://github.com/xosnrdev/cargonode/commit/5392474ed3bbf323c1d2918c206b585cb812e3e0))

## [0.1.1] - 2024-12-01


### Changed

- Update formula ([`ae1281b`](https://github.com/xosnrdev/cargonode/commit/ae1281bc217d9d266056d6fee8669887864fd031))
- Update Cargonode formula ([`56f4eb3`](https://github.com/xosnrdev/cargonode/commit/56f4eb3e49f1e8eee883bdc005979df2231fb901))


### Miscellaneous Tasks

- Release cargonode version 0.1.1 ([`51f48f9`](https://github.com/xosnrdev/cargonode/commit/51f48f9330ed3cfec4c8f974b41934f66cda5d0e))

## [0.1.0] - 2024-12-01


### Added

- Add create new package command ([`18a2980`](https://github.com/xosnrdev/cargonode/commit/18a29801c417c13c6b965d70515090d5e9e63984))
- Implement display trait for error ([`4ce71e6`](https://github.com/xosnrdev/cargonode/commit/4ce71e6db983f7c26eb790b6efdcfab9cd098ad9))
- Add new command init ([`9fd5a3d`](https://github.com/xosnrdev/cargonode/commit/9fd5a3de2d35a53d7b434eba08a2b0d4aab5d904))
- Integrating biome ([`80480f2`](https://github.com/xosnrdev/cargonode/commit/80480f2c0eec1cee3cd1ef3682266b8333be6e09))
- Should execute npm install on package creation ([`4994b7c`](https://github.com/xosnrdev/cargonode/commit/4994b7cc53b1b247dcdb291086e85eedd504c093))
- Add tsup integration for building and bundling the current package ([`03cbfbd`](https://github.com/xosnrdev/cargonode/commit/03cbfbde2c83fb26dc438f876afd4cc4dff4d133))
- Add vitest integration for running tests ([`00db060`](https://github.com/xosnrdev/cargonode/commit/00db060fcaea1e7231aebf0075ac91ca20679744))
- Add release-it integration for automating release ([`3b949b0`](https://github.com/xosnrdev/cargonode/commit/3b949b020e4cbe2c1f3953b9dbacc3be1b6dca6f))
- Add .cargo/config.toml and .clippy.toml files ([`bb526a8`](https://github.com/xosnrdev/cargonode/commit/bb526a850c98c45a12b67d662e477d6de2a82375))
- Add script to build and publish cargonode release for aarch64-apple-darwin target ([`0351f02`](https://github.com/xosnrdev/cargonode/commit/0351f02a73e9a5e22031a9a92d06370903279912))
- Add build.rs to set Windows executable options and embed manifest ([`aaf27f1`](https://github.com/xosnrdev/cargonode/commit/aaf27f1b794702c23f4017839198ab5334c6c95f))
- Add Windows application manifest file ([`63f09db`](https://github.com/xosnrdev/cargonode/commit/63f09db10fc53d5df30b0db4bd747fa445d655cc))
- Add jemalloc global allocator for musl target on 64-bit systems ([`1605ef0`](https://github.com/xosnrdev/cargonode/commit/1605ef06f105398f23b5d326ebcd3d287e2227fa))
- Add release workflow for creating cross-binary releases ([`96a0486`](https://github.com/xosnrdev/cargonode/commit/96a04860c2b9dc6dceb56f55b53e158344b39d83))
- Add Homebrew formula for Cargo Node binary ([`8499dac`](https://github.com/xosnrdev/cargonode/commit/8499dac0e6b150b604fc89167832ffb638f7ad3e))
- Add project changelog ([`12430fc`](https://github.com/xosnrdev/cargonode/commit/12430fcf3303d241b8066c2246a6de4d87b735ea))
- Add script to generate SHA256 hashes for releases ([`a235d3a`](https://github.com/xosnrdev/cargonode/commit/a235d3a49cee386c5e950b033571c5314e83fe07))


### Changed

- Rename to package ([`8e190f3`](https://github.com/xosnrdev/cargonode/commit/8e190f3131a6cd290b590a4e39c2cbd845b3abfb))
- Rename template placeholder ([`a657799`](https://github.com/xosnrdev/cargonode/commit/a657799e3d7dbbeb21aac6ced7636576f1fb987b))
- Improve package creation logging ([`c8a8137`](https://github.com/xosnrdev/cargonode/commit/c8a81373060e670544224712b313b18c06964a1f))
- Std alread imported ([`99318b0`](https://github.com/xosnrdev/cargonode/commit/99318b0ffffc22e44fd38092f9f5a97f3205e085))
- Remove sample ([`a04a77b`](https://github.com/xosnrdev/cargonode/commit/a04a77b89924fe27ae7730b013e47b491f92a21d))
- Update cargo ([`e44a8f4`](https://github.com/xosnrdev/cargonode/commit/e44a8f4a5173358abf6a76c4b4d805c117fa6f0c))
- Remove redundant crate ([`e256d25`](https://github.com/xosnrdev/cargonode/commit/e256d2590d8dcf742632dd9b73fa74d80a9cd92d))
- Make clippy happy ([`ad0fdc4`](https://github.com/xosnrdev/cargonode/commit/ad0fdc4cbcf60d8687d0265e840ecaa10d5ea80a))
- Updated package.rs ([`869ce3b`](https://github.com/xosnrdev/cargonode/commit/869ce3be3aeb404ff7e55a7c0cdc8a0301f9be9d))
- Revamp main ([`c7b4f15`](https://github.com/xosnrdev/cargonode/commit/c7b4f154c85e49bd7e70ca2c81da27271cdc66d0))
- Update README.md with improved installation and usage instructions ([`1520046`](https://github.com/xosnrdev/cargonode/commit/1520046ff125c2bd6a93e18f55bfaaaadcd1ab1e))
- Update CargoNode module description ([`af38008`](https://github.com/xosnrdev/cargonode/commit/af3800806c84ad95d78e837277b703977af394d0))
- Add Apache License to the repository ([`2116b45`](https://github.com/xosnrdev/cargonode/commit/2116b451d16b41bac1eb510bdb13318b09fc1afc))
- Update .cargo/config.toml for Windows and MUSL targets ([`25822aa`](https://github.com/xosnrdev/cargonode/commit/25822aa02aab1531a4a16286fc22a69612cbaa4d))
- Add jemallocator crate for memory allocation optimization, update metadata ([`3014930`](https://github.com/xosnrdev/cargonode/commit/3014930a380d2a37e0b916812bdd30a2040570d4))
- Update assets path in Cargo.toml ([`3a56ad9`](https://github.com/xosnrdev/cargonode/commit/3a56ad99ef5cc58bb1992a6acbfd0e9db8926932))
- Update Ubuntu package installation script path ([`35ad111`](https://github.com/xosnrdev/cargonode/commit/35ad111c3a1c723c51ee74f18108f688b0694399))
- Update Ubuntu package installation script path ([`066cc40`](https://github.com/xosnrdev/cargonode/commit/066cc40f1ef94165e5cc1cb18502d83b27302c3d))
- Remove prec2 ([`312ca77`](https://github.com/xosnrdev/cargonode/commit/312ca77fd2f6974a59f421319d65bc7f3ab5b8e0))
- Add executable permission to Ubuntu package installation script ([`ceacec7`](https://github.com/xosnrdev/cargonode/commit/ceacec70e62979d5d892b2ecb91b059236dbcfa4))
- Update Cargo.toml to include build.rs ([`a9ee760`](https://github.com/xosnrdev/cargonode/commit/a9ee760924e14f4b029dcfd94c9e64c9540fef08))
- Update keywords in Cargo.toml ([`b3cf7a8`](https://github.com/xosnrdev/cargonode/commit/b3cf7a8ef1a06593edee14f523dbaa42e924e9f7))
- Update release.yml to match version tag pattern ([`f5ce22d`](https://github.com/xosnrdev/cargonode/commit/f5ce22d02c1849beee54781fde4a96526cddd746))
- Removed async runtime related ([`03b707e`](https://github.com/xosnrdev/cargonode/commit/03b707ee3f0ddfe50df65f253b9100e3f2e3f2e1))
- Handled edge and test cases better ([`c6c43b0`](https://github.com/xosnrdev/cargonode/commit/c6c43b047e846d263432929d20768a9caa72235e))
- Use macro to generate command handling ([`cbe3303`](https://github.com/xosnrdev/cargonode/commit/cbe33034a88a65d30da05d556db44ac2d19f6347))
- Added cargonode.toml boilerplate ([`08e02b6`](https://github.com/xosnrdev/cargonode/commit/08e02b639008c42c39bf4aab6ea41827316056a9))
- Added rust doc and move test related to the test directory ([`08de3f3`](https://github.com/xosnrdev/cargonode/commit/08de3f31736c4bddbff81509d1d25acbfaae56e1))
- Moved test related to the test dir, use macro for redundant processes and unified docs ([`5c4a0d1`](https://github.com/xosnrdev/cargonode/commit/5c4a0d110b4bcec0a5d3fe2c513fa361b0d27d21))
- Use default path ([`042e952`](https://github.com/xosnrdev/cargonode/commit/042e952b7495938b44e47d7f72e4a87292587f45))
- Reorganize imports and document process ([`7c42d05`](https://github.com/xosnrdev/cargonode/commit/7c42d0525a3d22e0dbf15bd502a46969148565d7))
- Ignore test_init_package ([`6d80636`](https://github.com/xosnrdev/cargonode/commit/6d80636bd15141ff8a9dbe50040c470698650137))
- Fix changelog path ([`e442b24`](https://github.com/xosnrdev/cargonode/commit/e442b2467b031dab6a4f0321f0bb8f8eb948778a))
- Update release.yml to correctly extract version from tag ([`cbe4350`](https://github.com/xosnrdev/cargonode/commit/cbe4350b7ea7c0719532ca6f16bc3569a5dd0449))
- Follow up fix to cbe4350 ([`5cd8b41`](https://github.com/xosnrdev/cargonode/commit/5cd8b418b4d3f309eef9898b6be72fb21b208272))
- Update version in CHANGELOG.md to 0.1.0 - 2024-11-30 ([`d60eac9`](https://github.com/xosnrdev/cargonode/commit/d60eac9987a3c6707e4242aeec4e4a9746c31b42))
- Removed v prefix in version ([`e8500de`](https://github.com/xosnrdev/cargonode/commit/e8500de1b7008184c2a57271f98f4e2050d46704))
- Pre-release preparation ([`3575d9f`](https://github.com/xosnrdev/cargonode/commit/3575d9fce26aca0b374dca151df619965ac80fb2))
- Remove unnecessary heading in README.md ([`f9155c6`](https://github.com/xosnrdev/cargonode/commit/f9155c6bd7c8fcddc2a3ddad9a23e75f0b249c39))
- Remove complete directory ([`7c57497`](https://github.com/xosnrdev/cargonode/commit/7c57497aed66f9d60df5a808e20a6015cb981b1c))
- Update sha256_releases script ([`eac8769`](https://github.com/xosnrdev/cargonode/commit/eac8769bf764ac756a62e1f99e8a9d61ad8a639a))
- Update description ([`13de669`](https://github.com/xosnrdev/cargonode/commit/13de6694707f7fc7357b5f166df042419ac6613e))
- Update CLI tool description and features ([`a339598`](https://github.com/xosnrdev/cargonode/commit/a339598a42bbd44ab160ead6076e8a78952c3998))
- Update CLI tool description and features ([`62c413b`](https://github.com/xosnrdev/cargonode/commit/62c413b54cba2d89225a9a2c557bebeace2ff4e7))
- Update keywords and description ([`ea2a624`](https://github.com/xosnrdev/cargonode/commit/ea2a624d7f5faeed259a6c0539a17967811b7825))


### Documentation

- Add cargonode logo ([`23636c8`](https://github.com/xosnrdev/cargonode/commit/23636c8ff955e11c5592a1b076d6a59a6bdb6113))
- Update README.md ([`af45de7`](https://github.com/xosnrdev/cargonode/commit/af45de722cfdbebac1d3fd1887adecbf765506d6))
- Added rust doc ([`76c79e8`](https://github.com/xosnrdev/cargonode/commit/76c79e8796cd5ce48152b2a58db2e8023deeb101))


### Fixed

- Set copy options to content only ([`d0cf6c2`](https://github.com/xosnrdev/cargonode/commit/d0cf6c2b3115865fbce0bafe0886f281ddfc5b2d))
- Param typo ([`77168cf`](https://github.com/xosnrdev/cargonode/commit/77168cf9e668c4dcd2ef61872e3b8968088a306e))
- Make ci happy ([`91731bc`](https://github.com/xosnrdev/cargonode/commit/91731bcbcd831629d100f4d05ec81a10b0ed7a3b))
- Add async_recursion to execute function resolve recursive `async fn` error ([`fc19d86`](https://github.com/xosnrdev/cargonode/commit/fc19d86690adec70597689bcde0e314c735dd30d))
- Remove pcre2 ([`ea31399`](https://github.com/xosnrdev/cargonode/commit/ea313996d65cc476522dbe2f341d0658599b3c47))
- Replace cargo-node with cargonode ([`cb6b72f`](https://github.com/xosnrdev/cargonode/commit/cb6b72fa7b1435ea1cb1beee01d6be72f1804c5f))
- Skip tests if required commands are missing in cross-docker environment ([`a224462`](https://github.com/xosnrdev/cargonode/commit/a224462cdbfec3aa3c0291883b74228d70f8c18d))
- Resolve "No such file or directory" error in GitHub Actions ([`9fc838a`](https://github.com/xosnrdev/cargonode/commit/9fc838a4273d4999bf7288fcb137f4bf4dce7a4f))
- Resolve "No such file or directory" error in GitHub Actions ([`7ddc7c6`](https://github.com/xosnrdev/cargonode/commit/7ddc7c65c889b30a995a194125d7898c736a7c88))
- Moves the leading v from $VERSION if it exists ([`8c0b3dd`](https://github.com/xosnrdev/cargonode/commit/8c0b3ddfe6188ecbd894661b5bc2ccc35a27a4d5))
- Bright as the sun ([`1633cce`](https://github.com/xosnrdev/cargonode/commit/1633cce25e75d8a72ffcba1e360a82ed826c05b6))


### Miscellaneous Tasks

- Add cn template for bootstrapping new typescript project ([`55a6a23`](https://github.com/xosnrdev/cargonode/commit/55a6a23f62aa7b90d68bc7752cf33e9dea42fc7e))
- Command exec module for handling command execution with child process in an isolated fashion ([`88a3e8f`](https://github.com/xosnrdev/cargonode/commit/88a3e8fd80363a58b552583f8df0f67bbb475c44))
- Add file_util module for handling reading and writing files ([`31bdf31`](https://github.com/xosnrdev/cargonode/commit/31bdf31f5308d15bafa96229559c50ee37c9cc3b))
- Add bootstrap module for handling and managing package creation ([`de290c8`](https://github.com/xosnrdev/cargonode/commit/de290c8f2d16531f2e13621cb888238d24c1d00b))
- Expose cargo_node modules ([`fd18be8`](https://github.com/xosnrdev/cargonode/commit/fd18be8677c85e956e8b42a167e657721c980dd1))
- Resolve cargo ([`4c1c658`](https://github.com/xosnrdev/cargonode/commit/4c1c6582937248c773ee864117378ad9e5dc85c9))
- Add "sample" to .gitignore ([`eedc5b5`](https://github.com/xosnrdev/cargonode/commit/eedc5b516ebb8e5ca252804eca9831fffe433168))
- Later things ([`a19ad63`](https://github.com/xosnrdev/cargonode/commit/a19ad6399495afd5a082982269c4b566fdd2b512))
- Add script to install required packages on Ubuntu ([`802ae79`](https://github.com/xosnrdev/cargonode/commit/802ae7930c34440804810ee2d843bb105bea5c82))
- Update yanked "url" crate ([`915ee75`](https://github.com/xosnrdev/cargonode/commit/915ee7513b7fceba9e5a2b7c6f2a4244867d807c))
- Update Rust version requirement to 1.80+ ([`3faa2ce`](https://github.com/xosnrdev/cargonode/commit/3faa2ce5a2aa34c69be8b0a7d002c5737336919d))
- Update changelog ([`37ede02`](https://github.com/xosnrdev/cargonode/commit/37ede0226efb3234a19f91347128f79d0ecc077f))
- Command typo ([`3bbc452`](https://github.com/xosnrdev/cargonode/commit/3bbc452d68d9a7d87a687fb8059789e30688d83d))
- Add git-cliff configuration for refined changelog generation ([`43582c8`](https://github.com/xosnrdev/cargonode/commit/43582c8edc24f29320f9bb2252efd6f01bfa9bbd))
- Rename to snakecase ([`cbc889f`](https://github.com/xosnrdev/cargonode/commit/cbc889f86303993e20ed2a39943541214c801643))
- Prepare homebrew tap ([`bcb2077`](https://github.com/xosnrdev/cargonode/commit/bcb2077fd4677e120ca595710b5ecf43ee7d92f0))
- Symbolic link ([`e99ec3a`](https://github.com/xosnrdev/cargonode/commit/e99ec3ac1200717c83095f6c117a0f6bcc92c929))
- Fix symbolic linking issue ([`cb02854`](https://github.com/xosnrdev/cargonode/commit/cb02854058c7b7c7a7937df44ba85215bcb7579c))


### Refactor

- Not necessary ([`01ca666`](https://github.com/xosnrdev/cargonode/commit/01ca6669c4a2892fc746bdb29665ecd652962b0d))
- Move npm install expression to a function call ([`f6490f1`](https://github.com/xosnrdev/cargonode/commit/f6490f1f835a3591220bd487ec13f41832095a32))
- "fmt" script not needed ([`46916c7`](https://github.com/xosnrdev/cargonode/commit/46916c7bc7c86cb2fd628efb6b41f977795196c2))
- Reduce boilerplate ([`99d3c23`](https://github.com/xosnrdev/cargonode/commit/99d3c2373d960318b6a79afc77b3f177df49b5ac))
- Update command descriptions and arguments in main.rs ([`2b2d746`](https://github.com/xosnrdev/cargonode/commit/2b2d746b0707962ac7332dd80d968f5d5ff3cad9))
- Std already imported ([`0b1d1a3`](https://github.com/xosnrdev/cargonode/commit/0b1d1a337d4bda345a05b7101fed9dafa8da6e55))
- Work_dir now generic type AsRef of Path ([`cb06b9d`](https://github.com/xosnrdev/cargonode/commit/cb06b9db72a7aca185e7c4b4cbedb2220fec8165))
- Update params ([`2c5e325`](https://github.com/xosnrdev/cargonode/commit/2c5e3255d2efa0fa13719da5b89eafb76e35f783))
- Use tokio::process::Command for async support ([`352ab14`](https://github.com/xosnrdev/cargonode/commit/352ab14192d0bc2e2dce3bf0f33e5c2159ce7f55))
- Revamping modules ([`2f9f18f`](https://github.com/xosnrdev/cargonode/commit/2f9f18ff1b1f4cdc0e577cf9f9f3af1ce00bc733))
- Should execute synchronously no async deadlocks ([`a0e2152`](https://github.com/xosnrdev/cargonode/commit/a0e2152fe65f3d83bfe32ea98f6bc6c62e533150))


### Ci

- Add GitHub Actions CI workflow ([`3257dd6`](https://github.com/xosnrdev/cargonode/commit/3257dd6c23a8efb38ef47c672bb24379d22e6ecb))

