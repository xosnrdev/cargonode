<div align="center">
  <a href="https://github.com/xosnrdev/cargonode" target="_blank">
    <img src="https://raw.githubusercontent.com/xosnrdev/cargonode/master/assets/logo.svg" alt="cargonode logo" width="100"></img>
  </a>

  <h1 align="center">cargonode</h1>

  <p>
    <a href="https://github.com/xosnrdev/cargonode/actions?query=">
      <img src="https://github.com/xosnrdev/cargonode/actions/workflows/ci.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://crates.io/crates/cargonode">
      <img src="https://img.shields.io/crates/v/cargonode?label=crates" alt="cargonode Crate">
    </a>
    <a href="https://docs.rs/cargonode">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="cargonode Docs">
    </a>
    <a href="https://github.com/xosnrdev/cargonode/blob/master/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
      <img src="https://img.shields.io/badge/License-MIT%20-blue.svg" alt="License">
    </a>
  </p>
</div>

## Overview

**cargonode** is a Rust-based CLI that simplifies Node.js development by consolidating common tooling under a single
executable. It serves as a wrapper around key utilities for building, testing, formatting, linting, and releasing your
projects.

### Why cargonode?

1. **High Performance**  
   Written in Rust, cargonode offers fast execution and low overhead.
2. **Centralized Commands**  
   Replaces multiple shell scripts or separate binary invocations with a single CLI.
3. **Flexible Customization**  
   Allows swapping out default commands or adding custom prechecks.
4. **Cross-Platform Compatibility**  
   Runs on macOS (Intel and ARM), Linux (x64, ARM), and Windows (x64, ARM).

---

## Requirements

- Node.js ≥ 20.11.0  
  Needed for the underlying tools (Biome, Tsup, Vitest, Release-It).
- Rust ≥ 1.80  
  Required for installation from source or cargo. Binary releases do not require a Rust compiler on the end-user
  machine.

### Supported Platforms

- **macOS** (x64, ARM)
- **Linux** (x64, ARM)
- **Windows** (x64, ARM)

---

## Installation

Choose the option that fits your environment:

1. **Install prebuilt binaries via shell script**

   ```bash

   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.sh | sh

   ```

2. **Install prebuilt binaries via powershell script**

    ```bash
        powershell -ExecutionPolicy ByPass -c "irm https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.ps1 | iex"
    ```

3. **Homebrew (macOS)**

   ```bash
   brew install xosnrdev/cargonode/cargonode
   ```

   Recommended if you prefer managing software through Homebrew on macOS.

4. **Nix (nixOS)**

   ```bash
   nix-env -iA nixpkgs.cargonode
   ```

   See [nixpkgs](https://search.nixos.org/packages?channel=unstable&query=cargonode) for additional details.

5. **Cargo (Rust)**
   ```bash
   cargo install cargonode
   ```
   Installs the executable from source via the Rust package manager.

---

## Usage

Below are common commands. Each command calls an underlying tool with sensible defaults:

```bash
# Create a new project
cargonode new my-app

# Convert an existing Node.js project
cargonode init

# Build using tsup by default
cargonode build

# Test using vitest
cargonode test

# Format code with biome
cargonode fmt

# Lint/check code with biome
cargonode check

# Release with release-it
cargonode release
```

### Passing Tool Arguments

Any extra flags provided after the subcommand go directly to the underlying tool:

```bash
# Calls 'vitest run'
cargonode test run
# Calls 'biome check --fix'
cargonode check --fix
```

To see available flags, run:

```bash
cargonode --help
cargonode build --help
```

---

## Configuration

By default, cargonode uses several best-practice settings, but it can be customized through a `cargonode.toml` in your
project root. For instance:

```toml
[commands.format]
command = "eslint"
args = ["--fix"]

[commands.release]
prechecks = ["test", "build"]
```

In this example, `eslint` replaces biome for the `format` command, and `prechecks` ensures tests and builds run before
any release process.

### Configuration Precedence

1. Command-line arguments
2. Project configuration in `cargonode.toml`
3. Built-in defaults

See the [Template Reference](./templates/node_typescript/cargonode.toml) for additional examples.

---

## Support

For issues, feature requests, or general feedback, visit
the [GitHub Issues](https://github.com/xosnrdev/cargonode/issues) page. Contributions are welcome, whether in the form
of bug reports, pull requests, or suggestions.

---

## License

This project is available under a dual license: [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE). Choose whichever
license works best for your project or organization.
