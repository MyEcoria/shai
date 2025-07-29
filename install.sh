#!/bin/bash

set -e

# Configuration
REPO="ovh/shai"
BINARY_NAME="shai"
INSTALL_DIR="$HOME/.local/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    local os arch platform

    # Detect OS
    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        CYGWIN*|MINGW*|MSYS*) os="windows" ;;
        *)          
            log_error "Unsupported OS: $(uname -s)"
            exit 1
            ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        arm64|aarch64)  arch="aarch64" ;;
        *)              
            log_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac

    # Special handling for macOS (only x86_64 and aarch64 available)
    if [ "$os" = "macos" ]; then
        if [ "$arch" = "x86_64" ]; then
            platform="${BINARY_NAME}-macos-x86_64"
        elif [ "$arch" = "aarch64" ]; then
            platform="${BINARY_NAME}-macos-aarch64"
        fi
    elif [ "$os" = "linux" ]; then
        platform="${BINARY_NAME}-linux-x86_64"
    elif [ "$os" = "windows" ]; then
        platform="${BINARY_NAME}-windows-x86_64.exe"
        BINARY_NAME="${BINARY_NAME}.exe"
    fi

    echo "$platform"
}

# Get latest release info from GitHub API
get_latest_release() {
    local api_url="https://api.github.com/repos/$REPO/releases/latest"
    
    log_info "Fetching latest release information..."
    
    # Try with curl first, then wget
    if command -v curl >/dev/null 2>&1; then
        curl -s "$api_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$api_url"
    else
        log_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

# Download binary
download_binary() {
    local download_url="$1"
    local output_file="$2"
    
    log_info "Downloading $BINARY_NAME from $download_url"
    
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$output_file" "$download_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$output_file" "$download_url"
    else
        log_error "Neither curl nor wget is available"
        exit 1
    fi
}

# Create install directory
create_install_dir() {
    if [ ! -d "$INSTALL_DIR" ]; then
        log_info "Creating install directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR"
    fi
}

# Add to PATH if not already there
update_path() {
    local shell_profile=""
    
    # Detect shell profile
    if [ -n "$ZSH_VERSION" ]; then
        shell_profile="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        if [ -f "$HOME/.bash_profile" ]; then
            shell_profile="$HOME/.bash_profile"
        else
            shell_profile="$HOME/.bashrc"
        fi
    fi
    
    # Check if directory is already in PATH
    if [ "${PATH#*:$INSTALL_DIR:}" = "$PATH" ] && [ "${PATH#$INSTALL_DIR:}" = "$PATH" ] && [ "${PATH%:$INSTALL_DIR}" = "$PATH" ] && [ "$PATH" != "$INSTALL_DIR" ]; then
        if [ -n "$shell_profile" ] && [ -f "$shell_profile" ]; then
            echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$shell_profile"
            log_success "Added $INSTALL_DIR to PATH in $shell_profile"
            log_warn "Please run 'source $shell_profile' or restart your terminal"
        else
            log_warn "Could not automatically add to PATH. Please add $INSTALL_DIR to your PATH manually"
        fi
    fi
}

# Main installation function
main() {
    log_info "Installing $BINARY_NAME..."
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    log_info "Detected platform: $platform"
    
    # Get latest release
    local release_json
    release_json=$(get_latest_release)
    
    if [ -z "$release_json" ]; then
        log_error "Failed to fetch release information"
        exit 1
    fi
    
    # Extract download URL
    local download_url
    download_url=$(echo "$release_json" | grep -o "\"browser_download_url\":[[:space:]]*\"[^\"]*$platform[^\"]*\"" | cut -d'"' -f4)
    
    if [ -z "$download_url" ]; then
        log_error "Could not find download URL for platform: $platform"
        log_error "Available assets:"
        echo "$release_json" | grep -o "\"name\":[[:space:]]*\"[^\"]*\"" | cut -d'"' -f4
        exit 1
    fi
    
    # Create install directory
    create_install_dir
    
    # Download binary
    local temp_file="/tmp/$platform"
    download_binary "$download_url" "$temp_file"
    
    # Install binary
    local install_path="$INSTALL_DIR/$BINARY_NAME"
    mv "$temp_file" "$install_path"
    chmod +x "$install_path"
    
    log_success "$BINARY_NAME installed to $install_path"
    
    # Update PATH
    update_path
    
    # Verify installation
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        log_success "Installation completed! You can now run '$BINARY_NAME'"
    else
        log_success "Installation completed! You can run '$install_path' or add $INSTALL_DIR to your PATH"
    fi
}

# Parse command line arguments
while [ $# -gt 0 ]; do
    case $1 in
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [--install-dir DIR] [--help]"
            echo ""
            echo "Options:"
            echo "  --install-dir DIR    Install to DIR (default: $HOME/.local/bin)"
            echo "  --help              Show this help message"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Run main function
main