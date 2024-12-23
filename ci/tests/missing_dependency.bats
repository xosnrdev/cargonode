#!/usr/bin/env bats

# ci/missing_dependency.bats
# Test script behavior when a required dependency is missing

setup() {
    # Create a temporary home directory
    export HOME="$(mktemp -d)"
    mkdir -p "$HOME/.local/bin"

    # Mock missing 'curl' by renaming it, if possible
    if command -v curl >/dev/null 2>&1; then
        CURL_PATH="$(command -v curl)"
        sudo mv "$CURL_PATH" "${CURL_PATH}.bak" || skip "Cannot rename curl"
        trap 'sudo mv "${CURL_PATH}.bak" "$CURL_PATH"' EXIT
    else
        skip "curl is not installed; cannot test missing dependency"
    fi
}

teardown() {
    # Restore the original 'curl' command
    if [ -n "$CURL_PATH" ] && [ -f "${CURL_PATH}.bak" ]; then
        sudo mv "${CURL_PATH}.bak" "$CURL_PATH"
    fi

    # Remove the temporary home directory
    rm -rf "$HOME"
}

@test "Exit with error when a required command is missing" {
    # Run the installation script
    run ../../install_cargonode.sh --version=0.1.2

    # Assert that the script exited with an error
    [ "$status" -ne 0 ]

    # Assert that the error message mentions missing commands
    [[ "${output}" == *"Missing required commands"* ]]
}
