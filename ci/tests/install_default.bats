#!/usr/bin/env bats

# ci/install_default.bats
# Test installing cargonode to the default directory (~/.local/bin)

setup() {
    # Create a temporary home directory
    export HOME="$(mktemp -d)"
    mkdir -p "$HOME/.local/bin"
}

teardown() {
    # Remove the temporary home directory after the test
    rm -rf "$HOME"
}

@test "Install cargonode to default directory" {
    # Run the installation script
    run ../../install_cargonode.sh --version=0.1.2

    # Assert the script exited successfully
    [ "$status" -eq 0 ]

    # Assert that cargonode is installed in ~/.local/bin
    [ -f "$HOME/.local/bin/cargonode" ]

    # Assert that cargonode is executable
    [ -x "$HOME/.local/bin/cargonode" ]
}
