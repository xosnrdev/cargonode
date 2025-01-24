<div align="center">
  <a href="https://github.com/xosnrdev/cargonode" target="_blank">
    <img src="https://raw.githubusercontent.com/xosnrdev/cargonode/master/assets/logo.svg" alt="cargonode logo" width="100"></img>
  </a>

  <h1 align="center">cargonode</h1>

  <p>Unified tooling for Node.js development</p>

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

> [!WARNING]
> This project is still a work in progress and may undergo breaking changes.

After years of juggling different tools for Node.js projects, we wanted something simpler - a way to handle all our development tasks through one consistent interface. That's why we built cargonode, bringing the excellent developer experience of Rust's cargo to the Node.js world.

Want to jump right in? Here's how to get started:

```bash
# On macOS or Linux
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.sh | sh

# Using Windows PowerShell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/xosnrdev/cargonode/releases/download/0.1.3/cargonode-installer.ps1 | iex"

# If you're on macOS and use Homebrew
brew install xosnrdev/cargonode/cargonode

# For NixOS users
nix-env -iA nixpkgs.cargonode

# If you're familiar with Rust
cargo install cargonode
```

Once installed, you've got a powerful set of commands at your fingertips:

```bash
cargonode new my-project     # Start fresh
cargonode init              # Set up existing project
cargonode run              # Launch your app (or r for short)
cargonode fmt             # Clean up code formatting
cargonode check          # Catch problems (c works too)
cargonode build         # Bundle it up (b for short)
cargonode test         # Run your tests (t if you're busy)
cargonode release     # Ship it
```

Need more control? Every command takes these options:

```bash
  -c, --config-file <CONFIG FILE>        Path to a JSON config file
  -x, --executable <EXECUTABLE>          Override the configured executable
  -a, --args <ARGS>                      Additional arguments passed to the executable
  -e, --envs [<ENVS>...]         Environment variables (KEY=VALUE)
  -w, --working-dir <WORKING DIRECTORY>  Working directory
      --workflow-step [<STEPS>...]       Extra steps to run before the main executable
  -t, --timeout <SECONDS>                Time limit in seconds
  -v, --verbose...                       Increase logging verbosity (use -vv for more)
  -h, --help                             Print help
  -V, --version                          Print version
```

Want to tweak how things work? Add your settings to `package.json`:

```json
{
  "cargonode": {
    "build": {
      "executable": "npx",
      "subcommand": "tsup",
      "args": ["src/main.js"],
      "steps": ["check"],
      "envs": {
        "NODE_ENV": "production"
      },
      "working-dir": "src",
      "verbosity": 2
    }
  }
}
```

Here's how we use it day-to-day:

```bash
# Quick development cycle
cargonode run app.js           # Run a script
cargonode fmt                 # Tidy up code
cargonode check              # Spot issues
cargonode test -v           # Run tests with details
cargonode build            # Package it up

# Need something special?
cargonode test -e NODE_ENV=test       # Test environment
cargonode fmt -w src/                # Format specific files
```

Want to learn more? Check out:

- [How we designed it](https://hackmd.io/@xosnrdev/ryUXVLXPye)
- [Under the hood](https://docs.rs/cargonode)
- [Report issues](https://github.com/xosnrdev/cargonode/issues)

## License

This project is licensed under either of [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at your option.
