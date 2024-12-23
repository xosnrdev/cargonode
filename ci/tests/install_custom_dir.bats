#!/usr/bin/env bats

# ci/install_custom_dir.bats
# Test installing cargonode to a custom directory via INSTALL_DIR

setup() {
    # Create a temporary directory for custom installation
    CUSTOM_DIR="$(mktemp -d)"
}

teardown() {
    # Remove the temporary custom directory after the test
    rm -rf "$CUSTOM_DIR"
}

@test "Install cargonode to custom directory via INSTALL_DIR" {
    # Run the installation script with INSTALL_DIR set
    run INSTALL_DIR="$CUSTOM_DIR" ./install_cargonode.sh --version=0.1.2

    # Assert the script exited successfully
    [ "$status" -eq 0 ]

    # Assert that cargonode is installed in the custom directory
    [ -f "$CUSTOM_DIR/cargonode" ]

    # Assert that cargonode is executable
    [ -x "$CUSTOM_DIR/cargonode" ]
}
