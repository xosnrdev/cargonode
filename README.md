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

```
-c, --config-file <FILE>         Point to your config
-x, --executable <EXECUTABLE>     Use a different tool
-a, --args <ARGS>                Pass extra arguments
-e, --env-vars [<KEY=VALUE>...]  Set environment variables
-w, --working-dir <DIR>          Change working directory
-p, --pre-checks [<CHECKS>...]   Run checks first
-t, --timeout <SECONDS>          Set a time limit (default: 60)
-v, --verbose                    Get more details
-h, --help                       See all options
-V, --version                    Check version
```

Want to tweak how things work? Add your settings to `package.json`:

```json
{
  "cargonode": {
    "global": {
      "executable": "npx",
      "timeout": 300
    },
    "subcommands": {
      "build": {
        "args": ["tsup", "src/main.js"],
        "pre-checks": ["check"]
      }
    }
  }
}
```

Here's how we use it day-to-day:

```bash
# Quick development cycle
cargonode run app.js           # Test specific file
cargonode fmt                 # Tidy up code
cargonode check              # Spot issues
cargonode test -v           # Run tests with details
cargonode build            # Package it up

# Need something special?
cargonode build --timeout 300          # Take your time
cargonode test -e NODE_ENV=test       # Test environment
cargonode fmt -w src/                # Format specific files
```

Want to learn more? Check out:

- [How we designed it](https://hackmd.io/@xosnrdev/ryUXVLXPye)
- [Under the hood](https://docs.rs/cargonode)
- [Report issues](https://github.com/xosnrdev/cargonode/issues)

Use it under [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) - whichever works for you.
