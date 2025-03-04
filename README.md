# Cargonode

A simple build tool for Node.js projects.

## Install

```bash
# macOS
brew install xosnrdev/cargonode/cargonode

# Linux
curl -LsSf https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.sh | sh

# Windows
iwr https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.ps1 | iex
```

Checkout the [releases page](https://github.com/xosnrdev/cargonode/releases)

## Use

```bash
Usage: cargonode <COMMAND>

Commands:
  new    Create a new Node.js project at PATH
  init   Create a new Node.js project in an existing directory
  run    Run a specific tool
  check  Check files for errors
  build  Build the project
  test   Run tests
  help   Print this message

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuration Protocol

Cargonode uses a simple protocol in your `package.json` to define build tools:

```json
{
  "cargonode": {
    "tools": {
      "build": {
        "command": "tsc",                // Command to execute
        "args": ["--outDir", "dist"],    // Command arguments (optional)
        "env": {                         // Environment variables (optional)
          "NODE_ENV": "production"
        },
        "working_dir": "packages/core",  // Working directory (optional)
        "inputs": ["src/**/*.ts"],       // Input file patterns (required)
        "outputs": ["dist/**/*.js"]      // Output file patterns (optional)
      }
    }
  }
}
```

### Protocol Fields

- `command`: The executable to run (required)
- `args`: List of command-line arguments (optional)
- `env`: Environment variables to set (optional)
- `working_dir`: Directory to run the command in (optional)
- `inputs`: Glob patterns for input files (required)
- `outputs`: Glob patterns for output files (optional)
  - Only specify for commands that generate files
  - Directories will be created automatically

## Examples

```json
{
  "cargonode": {
    "tools": {
      "dev": {
        "command": "ts-node-dev",
        "args": ["src/index.ts"],
        "inputs": ["src/**/*.ts"]
      },
      "test": {
        "command": "jest",
        "args": ["--coverage"],
        "inputs": ["src/**/*.ts", "test/**/*.ts"],
        "outputs": ["coverage/**/*"]
      }
    }
  }
}
```

```bash
cargonode run dev # Make calls to the dev protocol
cargonode test  # Make calls to the test protocol
```

## Error Handling

Cargonode provides clear error messages and handles common scenarios:

- Missing output directories are created automatically
- Command failures include helpful suggestions
- Use verbose mode (`-v`) to see detailed command output and progress

## License

[MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE)
