#!/bin/bash
# B4N1-WEB Version Manager
# Centralizes version bumping across all SDKs and Core.

set -e

# Base directory (root of b4n1-web repo)
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Define targets
TARGETS=(
    "engine/cli-core/Cargo.toml:version = \"VERSION\""
    "sdks/python/pyproject.toml:version = \"VERSION\""
    "sdks/javascript/package.json:\"version\": \"VERSION\""
    "sdks/java/pom.xml:<version>VERSION</version>"
    "sdks/csharp/src/B4n1Web.csproj:<Version>VERSION</Version>"
    "MANIFEST.md:B4N1-WEB vVERSION"
)

usage() {
    echo "Usage: $0 [get | set <version>]"
    echo "  get: Show current versions across all files"
    echo "  set: Update all files to the new version"
    exit 1
}

get_versions() {
    echo "Current versions in B4N1-WEB:"
    for entry in "${TARGETS[@]}"; do
        file="${entry%%:*}"
        pattern="${entry#*:}"
        
        # Construct grep pattern (ERE)
        grep_pattern=$(echo "$pattern" | sed 's/VERSION/[0-9.]+/')
        
        if [ -f "$BASE_DIR/$file" ]; then
            version_line=$(grep -E "$grep_pattern" "$BASE_DIR/$file" | head -n 1)
            # Extract just the version number (e.g. 0.9.2)
            version=$(echo "$version_line" | grep -oE "[0-9]+\.[0-9]+\.[0-9]+")
            echo "  $file: $version ($version_line)"
        else
            echo "  $file: NOT FOUND"
        fi
    done
}

set_version() {
    NEW_VERSION=$1
    if [[ ! $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "❌ Invalid version format. Use x.y.z"
        exit 1
    fi

    echo "🚀 Bumping all versions to $NEW_VERSION..."

    for entry in "${TARGETS[@]}"; do
        file="${entry%%:*}"
        pattern="${entry#*:}"
        
        if [ ! -f "$BASE_DIR/$file" ]; then
            echo "⚠️ Skipping $file (not found)"
            continue
        fi

        # Find existing version line
        grep_pattern=$(echo "$pattern" | sed 's/VERSION/[0-9.]+/')
        current_line=$(grep -oE "$grep_pattern" "$BASE_DIR/$file" | head -n 1)
        
        if [ -n "$current_line" ]; then
            new_line=$(echo "$pattern" | sed "s/VERSION/$NEW_VERSION/")
            # Use @ as delimiter for sed to handle slashes/quotes
            sed -i "s@$current_line@$new_line@g" "$BASE_DIR/$file"
            echo "  ✅ Updated $file"
        else
            echo "  ❌ Could not find version pattern in $file"
        fi
    done

    echo "✨ All versions updated to $NEW_VERSION"
    echo "Don't forget to update CHANGELOG.md manually!"
}

case "$1" in
    get) get_versions ;;
    set) set_version "$2" ;;
    *) usage ;;
esac
