<div align="center">
  <a href="https://github.com/xosnrdev/cargonode">
    <img src="https://raw.githubusercontent.com/xosnrdev/cargonode/master/assets/logo.svg" alt="cargonode logo" width="100">
  </a>

  <h1>cargonode</h1>
  
  <p>A unified CLI tool that brings Cargo's developer experience to Node.js</p>

[![Build Status](https://github.com/xosnrdev/cargonode/actions/workflows/ci.yml/badge.svg)](https://github.com/xosnrdev/cargonode/actions?query=)
[![Crate](https://img.shields.io/crates/v/cargonode?label=crates)](https://crates.io/crates/cargonode)
[![Documentation](https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue)](https://docs.rs/cargonode)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/License-MIT%20-blue.svg)](LICENSE-MIT)

</div>

## Why cargonode?

If you've ever worked with Rust's Cargo, you know how pleasant it is to have a single, consistent interface for all your development tasks. cargonode brings that same experience to Node.js development by:

- Eliminating the need to remember different commands for different tools
- Providing sensible defaults while remaining fully configurable
- Working seamlessly with your existing Node.js toolchain
- Offering a consistent experience across all your projects

> ⚠️ **Note:** This project is in active development. While we maintain stability, some features may change.

## Getting Started

### Prerequisites

- Node.js 16.x or later
- npm, yarn, pnpm, or bun
- Git (optional, but recommended)

### Installation

Choose your preferred installation method:

<details>
<summary><b>Shell Script (macOS/Linux)</b></summary>

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.sh | sh
```

</details>

<details>
<summary><b>PowerShell (Windows)</b></summary>

```powershell
irm https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.ps1 | iex
```

</details>

<details>
<summary><b>Package Managers</b></summary>

```bash
# Homebrew (macOS)
brew install xosnrdev/cargonode/cargonode

# NixOS
nix-env -iA nixpkgs.cargonode

# Cargo (Rust)
cargo install cargonode
```

</details>

Verify your installation:

```bash
cargonode --version
```

### Creating Your First Project

1. Create a new project:

   ```bash
   cargonode new my-app
   cd my-app
   ```

2. Start development:
   ```bash
   cargonode run
   ```

That's it! Your project is set up with modern Node.js best practices.

## Command Reference

### Core Commands

| Command   | Alias | Description                 | Common Usage        |
| --------- | ----- | --------------------------- | ------------------- |
| `run`     | `r`   | Execute your application    | `cargonode r`       |
| `fmt`     | -     | Format your code            | `cargonode fmt`     |
| `check`   | `c`   | Run linting and type checks | `cargonode c`       |
| `build`   | `b`   | Bundle your application     | `cargonode b`       |
| `test`    | `t`   | Run test suites             | `cargonode t`       |
| `release` | -     | Create a new release        | `cargonode release` |

### Command Options

Every command supports these options:

```bash
Options:
  -c, --config-file <PATH>     Use a custom config file
  -x, --executable <PATH>      Override the default executable
  -a, --args <ARGS>           Pass additional arguments
  -e, --envs <KEY=VALUE>      Set environment variables
      --working-dir <PATH>    Change working directory
  -h, --help                 Show help information
```

## Configuration Guide

### Basic Configuration

Add your settings to `package.json`:

```json
{
  "cargonode": {
    "build": {
      "executable": "npx",
      "subcommand": "tsup",
      "args": ["src/main.js"],
      "steps": ["check"]
    }
  }
}
```

### Advanced Configuration Examples

<details>
<summary><b>TypeScript Project</b></summary>

```json
{
  "cargonode": {
    "build": {
      "executable": "npx",
      "subcommand": "tsc",
      "args": ["--project", "tsconfig.json"],
      "steps": ["check"]
    },
    "check": {
      "executable": "npx",
      "subcommand": "eslint",
      "args": ["src/**/*.ts"]
    }
  }
}
```

</details>

<details>
<summary><b>Next.js Project</b></summary>

```json
{
  "cargonode": {
    "run": {
      "executable": "npx",
      "subcommand": "next",
      "args": ["dev"],
      "envs": {
        "NODE_ENV": "development"
      }
    },
    "build": {
      "executable": "npx",
      "subcommand": "next",
      "args": ["build"],
      "steps": ["check"]
    }
  }
}
```

</details>

## Common Workflows

### Development Workflow

```bash
# Start development server
cargonode run

# Format code and run checks in watch mode
cargonode fmt -a --watch
cargonode check -a --watch

# Run tests with coverage
cargonode test -a --coverage
```

### Production Workflow

```bash
# Build for production
cargonode build -e NODE_ENV=production

# Run type checking and tests
cargonode check
cargonode test -e NODE_ENV=test

# Create a new release
cargonode release
```

## Troubleshooting

### Common Issues

<details>
<summary><b>Command not found after installation</b></summary>

Add cargonode to your PATH:

```bash
# For bash/zsh
echo 'export PATH="$HOME/.cargonode/bin:$PATH"' >> ~/.bashrc

# For fish
echo 'set -x PATH $HOME/.cargonode/bin $PATH' >> ~/.config/fish/config.fish
```

</details>

<details>
<summary><b>Permission errors on Linux/macOS</b></summary>

Fix permissions:

```bash
chmod +x ~/.cargonode/bin/cargonode
```

</details>

## Learn More

- 📖 [Design Philosophy](https://hackmd.io/@xosnrdev/ryUXVLXPye)
- 📚 [API Documentation](https://docs.rs/cargonode)
- 🐛 [Issue Tracker](https://github.com/xosnrdev/cargonode/issues)

## Contributing

We welcome contributions from everyone! Feel free to open an issue or submit a pull request.

## License

Licensed under either of [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at your option.
