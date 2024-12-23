#!/usr/bin/env bats

# ci/verbose_mode.bats
# Test verbose mode outputs detailed logs

setup() {
    # Create a temporary home directory
    export HOME="$(mktemp -d)"
    mkdir -p "$HOME/.local/bin"
}

teardown() {
    # Remove the temporary home directory after the test
    rm -rf "$HOME"
}

@test "Verbose mode outputs detailed logs" {
    # Run the installation script with verbose flag
    run ../../install_cargonode.sh --version=0.1.2 --verbose

    # Assert the script exited successfully
    [ "$status" -eq 0 ]

    # Assert that cargonode is installed in ~/.local/bin
    [ -f "$HOME/.local/bin/cargonode" ]

    # Assert that cargonode is executable
    [ -x "$HOME/.local/bin/cargonode" ]

    # Assert that verbose messages are present
    [[ "${output}" == *"Downloading cargonode"* ]]
    [[ "${output}" == *"Verifying checksum"* ]]
    [[ "${output}" == *"Installing cargonode"* ]]
}
