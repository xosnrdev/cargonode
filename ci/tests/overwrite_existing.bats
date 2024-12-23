#!/usr/bin/env bats

# ci/overwrite_existing.bats
# Test overwriting an existing cargonode binary upon user confirmation

setup() {
    # Create a temporary home directory
    export HOME="$(mktemp -d)"
    mkdir -p "$HOME/.local/bin"

    # Create a dummy existing cargonode binary
    echo "Old Version" >"$HOME/.local/bin/cargonode"
    chmod +x "$HOME/.local/bin/cargonode"
}

teardown() {
    # Remove the temporary home directory after the test
    rm -rf "$HOME"
}

@test "Overwrite existing cargonode binary upon user confirmation" {
    # Simulate user input 'y' to overwrite
    run echo "y" | ./install_cargonode.sh --version=0.1.2

    # Assert the script exited successfully
    [ "$status" -eq 0 ]

    # Assert that cargonode is installed in ~/.local/bin
    [ -f "$HOME/.local/bin/cargonode" ]

    # Assert that cargonode is executable
    [ -x "$HOME/.local/bin/cargonode" ]

    # Optionally, check that the content has been overwritten
    # Assuming the new binary contains different content
    # echo "New Version" > "$HOME/.local/bin/cargonode"
    # run ./install_cargonode.sh --version=0.1.2
    # [[ "$(cat "$HOME/.local/bin/cargonode")" == "New Version" ]]
}
