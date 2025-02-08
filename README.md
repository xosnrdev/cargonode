<div align="center">
  <a href="https://github.com/xosnrdev/cargonode">
    <img src="https://raw.githubusercontent.com/xosnrdev/cargonode/master/assets/logo.svg" alt="cargonode logo" width="100">
  </a>

  <h1>cargonode</h1>
  
  <p>A unified CLI tool that brings Cargo's developer experience to Node.js</p>

[![CI](https://img.shields.io/github/actions/workflow/status/xosnrdev/cargonode/ci.yml?style=flat-square&logo=github)](https://github.com/xosnrdev/cargonode/actions?query=workflow%3ACI)
[![Version](https://img.shields.io/crates/v/cargonode?style=flat-square&logo=rust)](https://crates.io/crates/cargonode)
[![Downloads](https://img.shields.io/crates/d/cargonode?style=flat-square&logo=rust)](https://crates.io/crates/cargonode)
[![Docs](https://img.shields.io/docsrs/cargonode?style=flat-square&logo=rust)](https://docs.rs/cargonode)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square)](LICENSE-MIT)

</div>

## ⚠️ Development Status

**This project is currently in active development and not ready for production use.**

- The installation methods mentioned in previous releases are temporarily unavailable
- We're working on a new release with improved stability and features
- Star and watch the repository for updates on the first stable release
- Feel free to contribute or report issues

## Why cargonode?

If you've ever worked with Rust's Cargo, you know how pleasant it is to have a single, consistent interface for all your development tasks. cargonode brings that same experience to Node.js development by:

- Eliminating the need to remember different commands for different tools
- Providing sensible defaults while remaining fully configurable
- Working seamlessly with your existing Node.js toolchain
- Offering a consistent experience across all your projects

## Building from Source

While we prepare for the first stable release, you can try cargonode by building from source:

```bash
git clone https://github.com/xosnrdev/cargonode.git
cd cargonode
cargo build --release
```

## License

Licensed under either of [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at your option.
