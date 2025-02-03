# Basic CLI Commands

Test the basic CLI commands of cargonode.

## Version command

```console
$ cargonode --version
cargonode [..]
```

## Help command

```console
$ cargonode --help
A unified CLI tool that brings Cargo's developer experience to Node.js

Usage: cargonode [OPTIONS] <COMMAND>

Commands:
  new      Create a new project at the specified path
  init     Initialize a project in the current directory
  run      Run a custom script or command [aliases: r]
  fmt      Format code
  check    Check code [aliases: c]
  build    Build or bundle [aliases: b]
  test     Run tests [aliases: t]
  release  Release project
  help     Print this message or the help of the given subcommand(s)

Options:
  -c, --config-file <CONFIG FILE>
          Path to a JSON config file
  -x, --executable <EXECUTABLE>
          Override the configured executable
  -s, --subcommand <SUBCOMMAND>
          Single argument to pass to the executable
  -a, --args <ARGS>
          Additional arguments passed to the executable
  -e, --envs [<ENVS>...]
          Environment variables (KEY=VALUE)
  -w, --working-dir <WORKING DIRECTORY>
          Working directory
      --steps [<STEPS>...]
          Extra jobs to run before the main job [possible values: build, check, fmt, release, run,
          test]
  -v, --verbosity...
          Increase logging verbosity (-v, -vv, -vvv)
  -h, --help
          Print help
  -V, --version
          Print version

```

## New command help

```console
$ cargonode new --help
Create a new project at the specified path

Usage: cargonode new [OPTIONS] <NAME>

Arguments:
  <NAME>  Name or path for the new project

Options:
  -p, --package-manager <PACKAGE MANAGER>
          Package manager to use [possible values: npm, yarn, pnpm, bun]
  -c, --config-file <CONFIG FILE>
          Path to a JSON config file
  -x, --executable <EXECUTABLE>
          Override the configured executable
  -s, --subcommand <SUBCOMMAND>
          Single argument to pass to the executable
  -a, --args <ARGS>
          Additional arguments passed to the executable
  -e, --envs [<ENVS>...]
          Environment variables (KEY=VALUE)
  -w, --working-dir <WORKING DIRECTORY>
          Working directory
      --steps [<STEPS>...]
          Extra jobs to run before the main job [possible values: build, check, fmt, release, run,
          test]
  -v, --verbosity...
          Increase logging verbosity (-v, -vv, -vvv)
  -h, --help
          Print help
  -V, --version
          Print version

```

## Init command help

```console
$ cargonode init --help
Initialize a project in the current directory

Usage: cargonode init [OPTIONS]

Options:
  -p, --package-manager <PACKAGE MANAGER>
          Package manager to use [possible values: npm, yarn, pnpm, bun]
  -c, --config-file <CONFIG FILE>
          Path to a JSON config file
  -x, --executable <EXECUTABLE>
          Override the configured executable
  -s, --subcommand <SUBCOMMAND>
          Single argument to pass to the executable
  -a, --args <ARGS>
          Additional arguments passed to the executable
  -e, --envs [<ENVS>...]
          Environment variables (KEY=VALUE)
  -w, --working-dir <WORKING DIRECTORY>
          Working directory
      --steps [<STEPS>...]
          Extra jobs to run before the main job [possible values: build, check, fmt, release, run,
          test]
  -v, --verbosity...
          Increase logging verbosity (-v, -vv, -vvv)
  -h, --help
          Print help
  -V, --version
          Print version

```
