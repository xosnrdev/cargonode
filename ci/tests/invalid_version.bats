#!/usr/bin/env bats

# ci/invalid_version.bats
# Test script behavior when specifying a non-existent version

setup() {
    # Create a temporary home directory
    export HOME="$(mktemp -d)"
    mkdir -p "$HOME/.local/bin"
}

teardown() {
    # Remove the temporary home directory after the test
    rm -rf "$HOME"
}

@test "Exit with error when specifying a non-existent version" {
    # Run the installation script with an invalid version
    run ../../install_cargonode.sh --version=999.999.999

    # Assert that the script exited with an error
    [ "$status" -ne 0 ]

    # Assert that the error message mentions the release was not found
    [[ "${output}" == *"Release archive not found"* ]]
}
