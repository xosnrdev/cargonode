#!/usr/bin/env sh

set -euo pipefail
IFS=$'\n\t'

# install_cargonode.sh
# ---------------------
# This script downloads and installs the cargonode binary for the user's platform.
#
# Usage:
#   ./install_cargonode.sh [options]
#
# Options:
#   --version=VERSION    Specify version to install (default: 0.1.2)
#   --verbose            Enable verbose output for detailed logging
#   --help               Display this help message
#
# The script performs the following tasks:
#   1. Detects OS and architecture
#   2. Determines the installation directory based on OS or environment variable
#   3. Maps to the appropriate release
#   4. Downloads the release archive and checksum from GitHub
#   5. Verifies checksum
#   6. Installs the cargonode binary to the appropriate directory
#   7. Configures shell environment

# --------------------
# Color Variables
# --------------------
RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
RESET="\033[0m"

# --------------------
# Logging Functions
# --------------------
# Initialize VERBOSE to false
VERBOSE=false

info() {
    if [ "$VERBOSE" = true ]; then
        printf "${GREEN}:: %s${RESET}\n" "$1"
    fi
}

warn() {
    printf "${YELLOW}:: %s${RESET}\n" "$1"
}

error() {
    printf "${RED}:: %s${RESET}\n" "$1"
    exit 1
}

# --------------------
# Usage Function
# --------------------
usage() {
    printf "Usage: %s [options]\n\n" "$(basename "$0")"
    printf "Options:\n"
    printf "  --version=VERSION    Specify version to install (default: 0.1.2)\n"
    printf "  --verbose            Enable verbose output for detailed logging\n"
    printf "  --help               Display this help message\n\n"
    printf "Examples:\n"
    printf "  Install the default version (0.1.2):\n"
    printf "    ./install_cargonode.sh\n\n"
    printf "  Install a specific version with verbose output:\n"
    printf "    ./install_cargonode.sh --version=0.2.0 --verbose\n\n"
    exit 0
}

# --------------------
# Default Values
# --------------------
VERSION="0.1.2"
GITHUB_REPO="xosnrdev/cargonode"

# --------------------
# Global Variables (to be set later)
# --------------------
BIN_DIR=""
TEMP_DIR=""

# --------------------
# Cleanup Function
# --------------------
cleanup() {
    if [ -d "$TEMP_DIR" ]; then
        info "Cleaning up temporary files..."
        rm -rf "$TEMP_DIR"
    fi
}

# Trap EXIT to ensure cleanup is called
trap cleanup EXIT INT TERM

# --------------------
# Argument Parsing Function
# --------------------
parse_arguments() {
    for arg in "$@"; do
        case $arg in
        --version=*)
            VERSION="${arg#*=}"
            ;;
        --verbose)
            VERBOSE=true
            ;;
        --help)
            usage
            ;;
        *)
            error "Unknown option: $arg"
            ;;
        esac
    done
}

# --------------------
# Command Checks Function
# --------------------
check_commands() {
    missing_commands=""
    for cmd in curl tar shasum unzip; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            missing_commands="${missing_commands} $cmd"
        fi
    done

    if [ -n "$missing_commands" ]; then
        error "Missing required commands:${missing_commands}"
    fi

    # Additional checks for Windows
    if [ "$PLATFORM_OS" = "pc-windows-msvc" ] || [ "$PLATFORM_OS" = "pc-windows-gnu" ]; then
        if ! command -v powershell >/dev/null 2>&1; then
            warn "PowerShell not found. PATH configuration on Windows may not be automated."
        fi
    fi
}

# --------------------
# Platform Detection Function
# --------------------
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
    Linux*)
        PLATFORM_OS="unknown-linux-gnu"
        ;;
    Darwin*)
        PLATFORM_OS="apple-darwin"
        ;;
    CYGWIN* | MINGW* | MSYS*)
        PLATFORM_OS="pc-windows-msvc"
        ;;
    *)
        error "Unsupported operating system: $OS"
        ;;
    esac

    case "$ARCH" in
    x86_64 | amd64)
        PLATFORM_ARCH="x86_64"
        ;;
    aarch64 | arm64)
        PLATFORM_ARCH="aarch64"
        ;;
    armv7* | armv7*)
        PLATFORM_ARCH="armv7"
        ;;
    i686 | i386)
        PLATFORM_ARCH="i686"
        ;;
    powerpc64*)
        PLATFORM_ARCH="powerpc64"
        ;;
    s390x*)
        PLATFORM_ARCH="s390x"
        ;;
    *)
        error "Unsupported architecture: $ARCH"
        ;;
    esac

    # Special handling for certain OS and ARCH combinations
    if [ "$PLATFORM_OS" = "apple-darwin" ] && [ "$PLATFORM_ARCH" = "aarch64" ]; then
        PLATFORM_ARCH="aarch64"
    fi

    PLATFORM="${PLATFORM_ARCH}-${PLATFORM_OS}"
}

# --------------------
# Determine Installation Directory Function
# --------------------
determine_install_dir() {
    # Allow overriding BIN_DIR via INSTALL_DIR environment variable
    # If INSTALL_DIR is set, use it; otherwise, determine based on OS
    BIN_DIR="${INSTALL_DIR:-}"

    if [ -z "$BIN_DIR" ]; then
        case "$PLATFORM_OS" in
        "unknown-linux-gnu" | "unknown-linux-musl" | "apple-darwin")
            BIN_DIR="$HOME/.local/bin"
            ;;
        "pc-windows-msvc" | "pc-windows-gnu")
            # For Windows, use %APPDATA%\Programs\cargonode\bin
            BIN_DIR="$APPDATA/Programs/cargonode/bin"
            ;;
        *)
            error "Unsupported operating system for installation directory determination."
            ;;
        esac
    else
        info "Using installation directory from INSTALL_DIR: $BIN_DIR"
    fi

    # Create the installation directory if it doesn't exist
    mkdir -p "$BIN_DIR"
}

# --------------------
# Release Mapping Function
# --------------------
map_release() {
    case "$PLATFORM_OS" in
    "unknown-linux-gnu")
        # Determine if it's musl or glibc
        if [ "$PLATFORM_ARCH" = "x86_64" ] && ldd --version 2>&1 | grep -q musl; then
            PLATFORM="${PLATFORM_ARCH}-unknown-linux-musl"
        fi
        ;;
    "pc-windows-msvc")
        # Default to msvc; adjust if necessary
        PLATFORM="${PLATFORM_ARCH}-pc-windows-msvc"
        ;;
    esac
}

# --------------------
# Archive Type Determination Function
# --------------------
determine_archive_type() {
    case "$PLATFORM_OS" in
    "pc-windows-msvc" | "pc-windows-gnu")
        ARCHIVE_EXT="zip"
        ;;
    *)
        if [ "$PLATFORM_OS" = "unknown-linux-gnu" ] && [ "$PLATFORM_ARCH" = "x86_64" ]; then
            # For Debian-based on x86_64, prefer .deb
            DEB_PACKAGE=true
            ARCHIVE_EXT="deb"
        else
            ARCHIVE_EXT="tar.gz"
        fi
        ;;
    esac
}

# --------------------
# URL Construction Function
# --------------------
construct_urls() {
    BASE_URL="https://github.com/${GITHUB_REPO}/releases/download/cargonode-v${VERSION}"
    if [ "$ARCHIVE_EXT" = "deb" ]; then
        ARCHIVE_NAME="cargonode_${VERSION}-1_amd64.deb"
        CHECKSUM_FILE="${ARCHIVE_NAME}.sha256"
    else
        ARCHIVE_NAME="cargonode-${VERSION}-${PLATFORM}.${ARCHIVE_EXT}"
        CHECKSUM_FILE="${ARCHIVE_NAME}.sha256"
    fi
    DOWNLOAD_URL="${BASE_URL}/${ARCHIVE_NAME}"
    CHECKSUM_URL="${BASE_URL}/${CHECKSUM_FILE}"
}

# --------------------
# Download Files Function
# --------------------
download_files() {
    info "Downloading cargonode ${VERSION} for ${PLATFORM}..."

    cd "$TEMP_DIR"

    # Check if the release exists
    if ! curl --head --silent --fail "$DOWNLOAD_URL" >/dev/null; then
        error "Release archive not found: $DOWNLOAD_URL"
    fi

    if ! curl --head --silent --fail "$CHECKSUM_URL" >/dev/null; then
        error "Checksum file not found: $CHECKSUM_URL"
    fi

    # Download archive with progress
    info "Downloading archive..."
    curl --retry 3 --retry-delay 5 --fail --location --progress-bar -o "${ARCHIVE_NAME}" "${DOWNLOAD_URL}" || {
        error "Failed to download archive"
    }

    # Download checksum
    info "Downloading checksum..."
    curl --retry 3 --retry-delay 5 --fail --location --silent -o "${CHECKSUM_FILE}" "${CHECKSUM_URL}" || {
        error "Failed to download checksum"
    }
}

# --------------------
# Checksum Verification Function
# --------------------
verify_checksum() {
    info "Verifying checksum..."

    cd "$TEMP_DIR"

    if [ "$ARCHIVE_EXT" = "deb" ]; then
        # For .deb, calculate checksum manually
        CALCULATED_CHECKSUM=$(shasum -a 256 "$ARCHIVE_NAME" | awk '{print $1}')
        EXPECTED_CHECKSUM=$(awk '{print $1}' "$CHECKSUM_FILE")

        if [ "$CALCULATED_CHECKSUM" != "$EXPECTED_CHECKSUM" ]; then
            error "Checksum verification failed for $ARCHIVE_NAME"
        fi
    else
        # For tar.gz and zip, use shasum to verify
        if ! shasum -a 256 -c "$CHECKSUM_FILE" >/dev/null 2>&1; then
            error "Checksum verification failed for $ARCHIVE_NAME"
        fi
    fi

    info "Checksum verified successfully."
}

# --------------------
# Install Files Function
# --------------------
install_files() {
    info "Installing cargonode..."

    if [ "$ARCHIVE_EXT" = "deb" ]; then
        # Install .deb package
        if command -v dpkg >/dev/null 2>&1; then
            info "Installing .deb package (requires sudo privileges)..."
            sudo dpkg -i "$TEMP_DIR/$ARCHIVE_NAME" || {
                error "Failed to install .deb package. Ensure you have sudo privileges."
            }
            info ".deb package installed successfully."
        else
            error "dpkg not found. Cannot install .deb package."
        fi
    else
        # Extract tar.gz or unzip zip
        case "$ARCHIVE_EXT" in
        "tar.gz")
            tar -xzf "$ARCHIVE_NAME" || {
                error "Failed to extract archive"
            }
            ;;
        "zip")
            unzip "$ARCHIVE_NAME" || {
                error "Failed to extract zip archive"
            }
            ;;
        *)
            error "Unsupported archive format: $ARCHIVE_EXT"
            ;;
        esac

        # Find the extracted directory (assuming it contains the cargonode binary)
        EXTRACT_DIR=$(find "$TEMP_DIR" -mindepth 1 -maxdepth 1 -type d | head -n 1)
        if [ -z "$EXTRACT_DIR" ] || [ ! -d "$EXTRACT_DIR" ]; then
            error "Extracted directory not found."
        fi

        cd "$EXTRACT_DIR"

        # Check if cargonode is already installed
        if [ -f "$BIN_DIR/cargonode" ]; then
            warn "cargonode is already installed in $BIN_DIR."
            printf "Do you want to overwrite it? [y/N]: "
            read -r response
            case "$response" in
            [yY][eE][sS] | [yY])
                info "Overwriting existing cargonode binary..."
                ;;
            *)
                info "Skipping installation."
                return
                ;;
            esac
        fi

        # Install binaries
        info "Installing cargonode binary..."
        if [ -f "cargonode" ]; then
            install -Dm755 cargonode "$BIN_DIR/cargonode" || {
                error "Failed to install cargonode binary"
            }
            info "cargonode installed successfully to $BIN_DIR."

            # Verify that cargonode is executable
            if [ -x "$BIN_DIR/cargonode" ]; then
                info "cargonode is executable."
            else
                warn "cargonode binary is not executable. Please check permissions."
            fi
        else
            error "cargonode binary not found in the extracted archive."
        fi
    fi
}

# --------------------
# Shell Environment Configuration Function
# --------------------
configure_shell_env() {
    info "Configuring shell environment..."

    if [ "$PLATFORM_OS" = "pc-windows-msvc" ] || [ "$PLATFORM_OS" = "pc-windows-gnu" ]; then
        configure_windows_path
    else
        configure_unix_shells
    fi
}

# Function to configure UNIX-like shells
configure_unix_shells() {
    # Function to check if a line exists in a file
    line_exists() {
        grep -Fxq "$1" "$2" 2>/dev/null
    }

    # Function to add PATH configuration for sh/bash/zsh
    setup_posix_shell() {
        local shell_rc="$1"
        local shell_name="$2"

        info "Setting up $shell_name configuration..."

        # Backup shell config if not already backed up
        if [ ! -f "${shell_rc}.bak" ]; then
            cp "$shell_rc" "${shell_rc}.bak" 2>/dev/null || true
        fi

        # PATH configuration
        if ! line_exists 'export PATH="$HOME/.local/bin:$PATH"' "$shell_rc"; then
            echo >>"$shell_rc"
            echo '# Added by cargonode installer' >>"$shell_rc"
            echo 'export PATH="$HOME/.local/bin:$PATH"' >>"$shell_rc"
            info "Updated $shell_rc to include ~/.local/bin in PATH."
        else
            info "$shell_rc already contains ~/.local/bin in PATH."
        fi
    }

    # Function to set up fish shell
    setup_fish() {
        local fish_config="$HOME/.config/fish/config.fish"

        info "Setting up fish configuration..."

        # Create config directory if it doesn't exist
        mkdir -p "$(dirname "$fish_config")"
        touch "$fish_config"

        # Backup fish config if not already backed up
        if [ ! -f "${fish_config}.bak" ]; then
            cp "$fish_config" "${fish_config}.bak" 2>/dev/null || true
        fi

        # PATH configuration
        if ! grep -Fxq "set -gx PATH $HOME/.local/bin \$PATH" "$fish_config"; then
            echo >>"$fish_config"
            echo '# Added by cargonode installer' >>"$fish_config"
            echo 'set -gx PATH $HOME/.local/bin $PATH' >>"$fish_config"
            info "Updated fish config to include ~/.local/bin in PATH."
        else
            info "Fish config already contains ~/.local/bin in PATH."
        fi
    }

    # Function to check if a shell is available
    check_shell() {
        command -v "$1" >/dev/null 2>&1
    }

    # Configure bash if installed
    if check_shell bash; then
        setup_posix_shell "$HOME/.bashrc" "bash"
    fi

    # Configure zsh if installed
    if check_shell zsh; then
        setup_posix_shell "$HOME/.zshrc" "zsh"
    fi

    # Configure fish if installed
    if check_shell fish; then
        setup_fish
    fi

    # Always configure .profile for POSIX shell compatibility
    setup_posix_shell "$HOME/.profile" "sh"

    info "Shell environment configured. Please restart your shell or source your shell's config file."
}

# Function to configure Windows PATH
configure_windows_path() {
    local bin_dir="$BIN_DIR"

    info "Configuring PATH for Windows..."

    # Check if the bin_dir is already in PATH
    if ! echo "$PATH" | tr ';' '\n' | grep -Fxq "$bin_dir"; then
        # Add bin_dir to PATH for the current user using PowerShell
        if command -v powershell >/dev/null 2>&1; then
            powershell -Command "if (-not ([Environment]::GetEnvironmentVariable('Path', 'User') -split ';') -contains '$bin_dir') { [Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';$bin_dir', 'User') }"
            info "Added $bin_dir to user PATH."
        else
            warn "PowerShell not found. Cannot automatically configure PATH."
            warn "Please add $bin_dir to your PATH manually."
        fi
    else
        info "$bin_dir is already in the user PATH."
    fi

    info "Please restart your terminal or log out and log back in for changes to take effect."
}

# --------------------
# Main Function
# --------------------
main() {
    parse_arguments "$@"
    detect_platform
    determine_install_dir
    check_commands
    map_release
    determine_archive_type
    construct_urls
    download_files
    verify_checksum
    install_files
    configure_shell_env

    info "Installation completed successfully!"
    info "cargonode installed to: $BIN_DIR"

    if [ "$ARCHIVE_EXT" = "deb" ]; then
        info "cargonode has been installed via .deb package."
    else
        if [ "$PLATFORM_OS" = "pc-windows-msvc" ] || [ "$PLATFORM_OS" = "pc-windows-gnu" ]; then
            info "Please ensure that $BIN_DIR is in your PATH."
        else
            info "Please ensure that $BIN_DIR is in your PATH."
        fi
    fi
}

# --------------------
# Execute Main Function
# --------------------
main "$@"
