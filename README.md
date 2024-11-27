<div align="center">

<img src="./docs/logo.svg" width="250px" />

A CLI tool built with Rust to revolutionize Node.js development workflows. By integrating powerful tools like [BiomeJS](https://biomejs.dev/guides/getting-started/), [Tsup](https://tsup.egoist.dev/), [Vitest](https://vitest.dev/guide/), and [Release-It](https://github.com/release-it/release-it?tab=readme-ov-file#readme), it transforms complex development processes into simple, efficient commands.

</div>

## Why cargonode?

Developers face numerous challenges in modern Node.js projects:

- Repetitive configuration
- Scattered tooling
- Inconsistent development workflows
- Performance bottlenecks

> **cargonode solves these problems** by providing a unified, lightning-fast CLI that simplifies your entire development lifecycle.

## Key Features

- 🚀 **High-Performance**: Built with Rust for maximum speed
- 🔧 **Modular Design**: Seamless integration with popular tools
- 🤖 **Automated Workflows**: Streamline build, test, lint, and release processes
- 🛠️ **Flexible Configuration**: Customize every aspect of your development environment

## Requirements

### System Compatibility

- **Node.js**: >=20.11.0 (Latest LTS recommended)
- **Platforms**:
  - macOS (x64, ARM)
  - Linux (x64, ARM)
  - Windows (x64)

### Development Prerequisites

- Rust 1.80+ (for building from source)

## Quick Start

### 1. Installation

```bash
# Homebrew
brew install cargonode

# npm
npm install -g cargonode

# Binary (manual download)
# Download from releases page and verify checksum
shasum -a 256 cargonode-*.tar.gz
```

### 2. Project Initialization

```bash
# Create new project
cargonode new my-awesome-project

# Or initialize in existing project
cd my-project
cargonode init
```

### 3. Development Workflow

```bash
# Build project
cargonode build

# Run tests
cargonode test

# Lint and format
cargonode fmt --fix
cargonode check
```

## Configuration Mastery

### Configuration Precedence

Configuration resolution follows a clear hierarchy:

1. CLI Flags (Highest Priority)
2. Project `cargonode.toml`
3. Global cargonode Settings
4. Default Values

### Example `cargonode.toml`

```toml
# Global timeout
default_timeout_secs = 180

[commands.build]
pre_checks = ["check", "format"]
env_vars = { NODE_ENV = "production" }
timeout_secs = 300
```

## Advanced Usage

### Debugging & Troubleshooting

```bash
# Enable verbose logging
cargonode <command> --verbose
```

#### Common Troubleshooting

- Verify Node.js version (>=20.11.0)
- Check configuration precedence
- Validate tool dependencies

## Contributing

### How to Contribute

1. Fork the repository
2. Create a feature branch
3. Implement changes
4. Write comprehensive tests
5. Update documentation
6. Submit a pull request

#### Development Setup

```bash
# Clone repository
git clone https://github.com/xosnrdev/cargonode.git

# Setup development environment
cargo build
cargo test
```

## Roadmap & Vision

- [ ] Enhanced plugin ecosystem
- [ ] Advanced CI/CD integrations
- [ ] Performance benchmarking
- [ ] Expanded community tooling

## Support

- **Issues**: [GitHub Issues](https://github.com/xosnrdev/cargonode/issues)

## License

cargonode is open-source, released under the MIT License.
[View LICENSE](LICENSE)
