# Workflow Commands

Test the workflow commands of cargonode.

## Run command help

```console
$ cargonode run --help
Run a custom script or command

Usage: cargonode run [OPTIONS] [ARGS]...

Arguments:
  [ARGS]...  Arguments for the runner

Options:
  -c, --config-file <CONFIG FILE>
          Path to a JSON config file
  -x, --executable <EXECUTABLE>
          Override the configured executable
  -s, --subcommand <SUBCOMMAND>
          Single argument to pass to the executable
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