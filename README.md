# Cargonode

A simple build tool for Node.js projects.

## Install

Get it from [releases page](https://github.com/xosnrdev/cargonode/releases):
- Mac: `brew install xosnrdev/cargonode/cargonode`
- Unix: Run our install script
- Windows: Use the installer

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

## Setup

In your `package.json`, add:

```json
{
  "cargonode": {
    "tools": {
      "build": {
        "command": "tsc",
        "inputs": ["src/**/*.ts"],
        "outputs": ["dist/**/*.js"]
      }
    }
  }
}
```

```bash
cargonode run build
```

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
cargonode run dev
cargonode test
```

## License

[MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE)
