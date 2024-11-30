<div align="center">

<img src="./docs/logo.svg" width="250px" />

A CLI tool built with Rust to streamline Node.js development workflows. By integrating essential tools like [BiomeJS](https://biomejs.dev/guides/getting-started/), [Tsup](https://tsup.egoist.dev/), [Vitest](https://vitest.dev/guide/), and [Release-It](https://github.com/release-it/release-it?tab=readme-ov-file#readme), cargonode simplifies complex development tasks.

</div>

## Overview

cargonode addresses key challenges in Node.js projects, including:

- Repetitive configuration
- Fragmented tooling
- Inconsistent workflows
- Performance issues

With cargonode, you gain a unified, high-performance CLI that enhances your development process.

## Key Features

- **High Performance**: Built with Rust for optimal speed.
- **Modular Design**: Seamless integration with popular tools.
- **Automated Workflows**: Streamlined build, test, lint, and release processes.
- **Flexible Configuration**: Easily customizable environment.

## Requirements

### System Compatibility

- **Node.js**: Version 20.11.0 or higher (Latest LTS recommended).
- **Supported Platforms**:
  - **macOS**: x64, ARM
  - **Linux**: x64, ARM (Ubuntu)
  - **Windows**: x64, ARM

### Development Prerequisites

- Rust 1.80 or higher (for building from source).

## Installation

To install `cargonode`, use Homebrew on macOS:

```bash
brew install cargonode
```

## Getting Started

### Project Initialization

Create a new project or initialize an existing one:

```bash
# Create a new project
cargonode new my-awesome-project

# Initialize in an existing project
cd my-project
cargonode init
```

### Development Workflow

Manage your project with the following commands:

```bash
# Build the project
cargonode build

# Run tests
cargonode test

# Lint and format code
cargonode fmt --fix
cargonode check
```

## Configuration Example

Here’s a sample `cargonode.toml` configuration file:

```toml
[commands.format]
# Command to format code
command = "biome"
# Arguments for the command
args = ["format"]
# Pre-checks to run before the command
pre_checks = []
# Environment variables to set
env_vars = {}

[commands.check]
command = "biome"
args = ["check"]
pre_checks = []
env_vars = {}

# ... Add more commands as needed
```

_Note: This configuration serves as a template. Customize the commands and parameters as needed._

## Configuration Precedence

Configuration resolution follows this order:

1. CLI Flags (Highest Priority)
2. Project `cargonode.toml`
3. Global cargonode Settings
4. Default Values

## Contributing

To contribute to cargonode:

1. Fork the repository.
2. Create a feature branch.
3. Implement your changes.
4. Write tests.
5. Update documentation.
6. Submit a pull request.

### Development Setup

Clone the repository and set up the development environment:

```bash
git clone https://github.com/xosnrdev/cargonode.git
cd cargonode
cargo build
cargo test
```

## Support

For issues or questions, please visit [GitHub Issues](https://github.com/xosnrdev/cargonode/issues).

## License

cargonode is open-source software released under the MIT License. [View LICENSE](LICENSE).
