#!/bin/bash
# bump-version.sh — Bumps all version references across b4n1-web to the version in VERSION
# Usage: bash scripts/bump-version.sh [version]
set -e

BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
NEW_VERSION="${1:-$(cat "${BASE_DIR}/VERSION" 2>/dev/null || echo "0.9.0")}"

OLD_VERSION="0.8.0"

echo "🔁 Bumping version ${OLD_VERSION} → ${NEW_VERSION}"
echo ""

# File list: pattern → sed replacement
# Cargo.toml (Rust engine)
sed -i "s/^version = \"${OLD_VERSION}\"/version = \"${NEW_VERSION}\"/" "${BASE_DIR}/engine/cli-core/Cargo.toml"
echo "✓ engine/cli-core/Cargo.toml"

# Python SDK
sed -i "s/^version = \"${OLD_VERSION}\"/version = \"${NEW_VERSION}\"/" "${BASE_DIR}/sdks/python/pyproject.toml"
sed -i "s/__version__ = \"${OLD_VERSION}\"/__version__ = \"${NEW_VERSION}\"/" "${BASE_DIR}/sdks/python/b4n1web/__init__.py" 2>/dev/null || true
sed -i "s/\"${OLD_VERSION}\"/\"${NEW_VERSION}\"/g" "${BASE_DIR}/sdks/python/b4n1web/browser.py" 2>/dev/null || true
echo "✓ sdks/python (pyproject.toml + __init__.py)"

# JavaScript SDK
sed -i "s/\"version\": \"${OLD_VERSION}\"/\"version\": \"${NEW_VERSION}\"/" "${BASE_DIR}/sdks/javascript/package.json"
echo "✓ sdks/javascript/package.json"

# Java SDK
sed -i "s/<version>${OLD_VERSION}</<version>${NEW_VERSION}</" "${BASE_DIR}/sdks/java/pom.xml"
echo "✓ sdks/java/pom.xml"

# C# SDK
sed -i "s/<Version>${OLD_VERSION}</<Version>${NEW_VERSION}</" "${BASE_DIR}/sdks/csharp/src/B4n1Web.csproj"
echo "✓ sdks/csharp/B4n1Web.csproj"

# SDK READMEs — version badges and mentions
for sdk in sdks/python sdks/javascript sdks/java sdks/csharp; do
  readme="${BASE_DIR}/${sdk}/README.md"
  if [ -f "$readme" ]; then
    sed -i "s/${OLD_VERSION}/${NEW_VERSION}/g" "$readme"
    echo "✓ ${sdk}/README.md"
  fi
done

# MANIFEST.md
sed -i "s/v${OLD_VERSION}/v${NEW_VERSION}/g" "${BASE_DIR}/MANIFEST.md"
echo "✓ MANIFEST.md"

# CHANGELOG.md — add new version heading if not present
if ! grep -q "^## v${NEW_VERSION}" "${BASE_DIR}/CHANGELOG.md" 2>/dev/null; then
  sed -i "1i\\## v${NEW_VERSION} — $(date +%Y-%m-%d) [WIP]\n\n### Added\n- (pending)\n\n---\n\n" "${BASE_DIR}/CHANGELOG.md"
  echo "✓ CHANGELOG.md — new version heading added"
else
  echo "✓ CHANGELOG.md — version heading exists"
fi

# RELEASE_CHECKLIST.md — update version references
sed -i "s/release(b4n1-web v${OLD_VERSION})/release(b4n1-web v${NEW_VERSION})/g" "${BASE_DIR}/RELEASE_CHECKLIST.md"
sed -i "s/# Release Checklist v${OLD_VERSION}/# Release Checklist v${NEW_VERSION}/g" "${BASE_DIR}/RELEASE_CHECKLIST.md"
sed -i "s/b4n1web v${OLD_VERSION}/b4n1web v${NEW_VERSION}/g" "${BASE_DIR}/RELEASE_CHECKLIST.md"
sed -i "s/v${OLD_VERSION}-x86_64/v${NEW_VERSION}-x86_64/g" "${BASE_DIR}/RELEASE_CHECKLIST.md"
sed -i "s/Crear tag 'v${OLD_VERSION}'/Crear tag 'v${NEW_VERSION}'/g" "${BASE_DIR}/RELEASE_CHECKLIST.md"
echo "✓ RELEASE_CHECKLIST.md"

# scripts/build.sh default version
sed -i "s/VERSION=\"\${1:-.*}\"/VERSION=\"\${1:-${NEW_VERSION}}\"/" "${BASE_DIR}/scripts/build.sh"
echo "✓ scripts/build.sh"

echo ""
echo "✅ Version bump complete: ${NEW_VERSION}"
echo ""
echo "Files updated:"
echo "  - engine/cli-core/Cargo.toml"
echo "  - sdks/python/pyproject.toml"
echo "  - sdks/javascript/package.json"
echo "  - sdks/java/pom.xml"
echo "  - sdks/csharp/src/B4n1Web.csproj"
echo "  - sdks/*/README.md (all 4)"
echo "  - MANIFEST.md"
echo "  - CHANGELOG.md"
echo "  - RELEASE_CHECKLIST.md"
echo "  - scripts/build.sh"
echo ""
echo "Config files (internal):"
echo "  - sdks/python/b4n1web/__init__.py (__version__)"
echo "  - sdks/python/b4n1web/browser.py (SDK_VERSION)"
echo ""
echo "Run 'git diff' to review before committing."
