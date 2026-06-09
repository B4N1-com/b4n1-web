#!/bin/bash
# B4n1Web Build Script
# Compiles the Rust engine for multiple platforms
# Usage: ./scripts/build.sh [version] [--all]
#
# --all    Build for all platforms (requires cross for non-native targets)

set -e

VERSION="${1:-0.1.0}"
BUILD_ALL=false

for arg in "$@"; do
    if [ "$arg" = "--all" ]; then
        BUILD_ALL=true
    fi
done

PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
ENGINE_DIR="$PROJECT_DIR/engine/cli-core"
RELEASES_DIR="$PROJECT_DIR/releases"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${GREEN}B4n1Web Build Script${NC}"
echo "===================="
echo "Version: $VERSION"
echo "Engine dir: $ENGINE_DIR"
echo "Releases dir: $RELEASES_DIR"
echo ""

# Detect current platform
CURRENT_OS=$(uname -s | tr '[:upper:]' '[:lower:]')
CURRENT_ARCH=$(uname -m)
case "$CURRENT_ARCH" in
    x86_64) CURRENT_ARCH="x86_64" ;;
    aarch64|arm64) CURRENT_ARCH="aarch64" ;;
esac
case "$CURRENT_OS" in
    linux) CURRENT_OS="linux" ;;
    darwin) CURRENT_OS="macos" ;;
esac

# Platform targets
declare -A TARGETS=(
    ["linux-x86_64"]="x86_64-unknown-linux-gnu"
    ["linux-aarch64"]="aarch64-unknown-linux-gnu"
    ["macos-x86_64"]="x86_64-apple-darwin"
    ["macos-aarch64"]="aarch64-apple-darwin"
    ["windows-x86_64"]="x86_64-pc-windows-msvc"
)

# Binary name per platform
declare -A BINARY_NAMES=(
    ["linux-x86_64"]="b4n1web"
    ["linux-aarch64"]="b4n1web"
    ["macos-x86_64"]="b4n1web"
    ["macos-aarch64"]="b4n1web"
    ["windows-x86_64"]="b4n1web.exe"
)

build_target() {
    local platform="$1"
    local target="$2"
    local binary_name="$3"
    
    echo -e "${CYAN}Building for $platform...${NC}"
    
    cd "$ENGINE_DIR"
    
    # Use cross for non-native targets, cargo for native
    if [ "$platform" = "${CURRENT_OS}-${CURRENT_ARCH}" ]; then
        cargo build --release --target "$target" 2>&1 | tail -3
    else
        if command -v cross &> /dev/null; then
            cross build --release --target "$target" 2>&1 | tail -3
        else
            echo -e "${YELLOW}  Skipping $platform (cross not installed)${NC}"
            return 0
        fi
    fi
    
    # Create release package
    local bin_dir="$ENGINE_DIR/target/$target/release"
    local release_name="b4n1web-v${VERSION}-${platform}"
    local pkg_dir="$RELEASES_DIR/$release_name"
    
    mkdir -p "$pkg_dir"
    
    if [ "$platform" = "windows-x86_64" ]; then
        cp "$bin_dir/b4n1web.exe" "$pkg_dir/b4n1web.exe"
    else
        cp "$bin_dir/b4n1web" "$pkg_dir/b4n1web"
        chmod +x "$pkg_dir/b4n1web"
    fi
    
    # Create tarball
    cd "$RELEASES_DIR"
    if [ "$platform" = "windows-x86_64" ]; then
        tar -czf "${release_name}.tar.gz" "$release_name/"
    else
        tar -czf "${release_name}.tar.gz" "$release_name/"
    fi
    
    # Clean up
    rm -rf "$pkg_dir"
    
    local size=$(du -h "$RELEASES_DIR/${release_name}.tar.gz" | cut -f1)
    echo -e "${GREEN}  ✓ $platform: ${release_name}.tar.gz ($size)${NC}"
}

mkdir -p "$RELEASES_DIR"

if [ "$BUILD_ALL" = true ]; then
    echo -e "${YELLOW}Building for all platforms...${NC}"
    echo ""
    
    for platform in "${!TARGETS[@]}"; do
        build_target "$platform" "${TARGETS[$platform]}" "${BINARY_NAMES[$platform]}"
    done
else
    # Build only for current platform
    platform="${CURRENT_OS}-${CURRENT_ARCH}"
    target="${TARGETS[$platform]}"
    binary_name="${BINARY_NAMES[$platform]}"
    
    build_target "$platform" "$target" "$binary_name"
fi

echo ""
echo -e "${GREEN}✓ Build complete!${NC}"
echo ""
echo "Releases:"
ls -lh "$RELEASES_DIR"/*.tar.gz 2>/dev/null || echo "  No releases built"
