#!/bin/bash
# B4n1Web Install Script
# Downloads and installs the B4n1Web binary
# Usage: curl -sL https://web.b4n1.com/install | bash

set -e

VERSION=$(curl -s https://api.github.com/repos/B4N1-com/b4n1-web/releases/latest | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4 | sed 's/v//')
GITHUB_REPO="B4N1-com/b4n1-web"
BINARY_NAME="b4n1web"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS and architecture
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    
    case "$ARCH" in
        x86_64) ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        *) echo -e "${RED}Error: Unsupported architecture: $ARCH${NC}"; exit 1 ;;
    esac
    
    case "$OS" in
        linux) OS="linux" ;;
        darwin) OS="macos" ;;
        *) echo -e "${RED}Error: Unsupported OS: $OS${NC}"; exit 1 ;;
    esac
    
    echo "${OS}-${ARCH}"
}

# Determine install directory
get_install_dir() {
    if [ -n "$INSTALL_DIR" ]; then
        echo "$INSTALL_DIR"
    elif [ -w "/usr/local/bin" ]; then
        echo "/usr/local/bin"
    elif [ -d "$HOME/.local/bin" ] && [ -w "$HOME/.local/bin" ]; then
        echo "$HOME/.local/bin"
    else
        echo "$HOME/.local/bin"
        mkdir -p "$HOME/.local/bin"
    fi
}

# Download and install
install_b4n1web() {
    local platform
    platform=$(detect_platform)
    local install_dir
    install_dir=$(get_install_dir)
    local filename="b4n1web-v${VERSION}-${platform}"
    local download_url="https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${filename}.tar.gz"
    
    echo -e "${GREEN}B4n1Web Installer${NC}"
    echo "=================="
    echo "Platform: $platform"
    echo "Version: $VERSION"
    echo "Install dir: $install_dir"
    echo ""
    
    # Create temp directory
    local temp_dir
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    # Download
    echo -e "${YELLOW}Downloading B4n1Web v${VERSION}...${NC}"
    if ! curl -sL --fail "$download_url" -o "$temp_dir/b4n1web.tar.gz" 2>/dev/null; then
        echo -e "${RED}Error: Failed to download from $download_url${NC}"
        echo ""
        echo "This might mean:"
        echo "  1. The release hasn't been published yet"
        echo "  2. Your platform ($platform) isn't supported yet"
        echo ""
        echo "You can also install from source:"
        echo "  1. Clone the repository"
        echo "  2. Run: cd engine/cli-core && cargo build --release"
        echo "  3. Copy target/release/b4n1web to your PATH"
        exit 1
    fi
    
    # Extract
    echo -e "${YELLOW}Extracting...${NC}"
    tar -xzf "$temp_dir/b4n1web.tar.gz" -C "$temp_dir"
    
    # Find the binary (it might be in a subdirectory or directly in the archive)
    local binary_path=""
    if [ -f "$temp_dir/b4n1web" ]; then
        binary_path="$temp_dir/b4n1web"
    elif [ -f "$temp_dir/$filename/b4n1web" ]; then
        binary_path="$temp_dir/$filename/b4n1web"
    else
        # Find any executable file
        binary_path=$(find "$temp_dir" -type f -name "b4n1web" -perm /111 | head -1)
        if [ -z "$binary_path" ]; then
            binary_path=$(find "$temp_dir" -type f -name "b4n1web" | head -1)
        fi
    fi
    
    if [ -z "$binary_path" ]; then
        echo -e "${RED}Error: Could not find b4n1web binary in archive${NC}"
        exit 1
    fi
    
    # Install
    echo -e "${YELLOW}Installing to $install_dir...${NC}"
    cp "$binary_path" "$install_dir/b4n1web"
    chmod +x "$install_dir/b4n1web"
    
    # Verify installation
    if "$install_dir/b4n1web" --version >/dev/null 2>&1; then
        local installed_version
        installed_version=$("$install_dir/b4n1web" --version 2>&1 || echo "installed")
        echo -e "${GREEN}✓ B4n1Web installed successfully!${NC}"
        echo "  Location: $install_dir/b4n1web"
        echo "  Version: $installed_version"
    else
        echo -e "${GREEN}✓ B4n1Web binary installed to $install_dir/b4n1web${NC}"
    fi
    
    # PATH instructions
    if ! echo "$PATH" | grep -q "$install_dir"; then
        echo ""
        echo -e "${YELLOW}Note: $install_dir is not in your PATH${NC}"
        echo "Add this to your ~/.bashrc or ~/.zshrc:"
        echo "  export PATH=\"$install_dir:\$PATH\""
    fi
}

# Main
install_b4n1web
