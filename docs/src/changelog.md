# Changelog

All notable changes to B4n1Web.

## [Unreleased]

### Added
- Full mdBook documentation site
- Windows ARM64 support
- Linux ARM64 support (Raspberry Pi, ARM servers)
- macOS Intel (x64) support
- Chromium auto-download for all 6 platforms
- MCP server with 33 tools

### Changed
- Release pipeline builds 6 platforms
- Bundled binaries in all 4 SDKs

---

## [0.9.5] - 2024-06-27

### Added
- Java SDK (Maven Central)
- C# SDK (NuGet)
- Python SDK (PyPI)
- JavaScript/TypeScript SDK (npm)
- Binary bundling in all SDKs
- Browser modes: Light, JS, Render
- MCP server with 33 tools
- CLI with goto, screenshot, click, type, wait, evaluate
- Daemon mode for persistent MCP

### Fixed
- Chromium auto-download for Linux ARM64
- Version compatibility checks
- Binary path resolution

---

## [0.9.0] - 2024-01-15

### Added
- Initial Rust binary
- Light mode (HTTP + HTML parsing)
- JS mode (JavaScript extraction)
- Python SDK
- JavaScript SDK

### Changed
- Single binary architecture
- Cross-platform compilation

---

## Version Format

B4n1Web uses [Semantic Versioning](https://semver.org/):

- **Major** (X.0.0): Breaking changes
- **Minor** (0.X.0): New features, backward compatible
- **Patch** (0.0.X): Bug fixes, backward compatible

Current: **0.13.0** (pre-1.0, API may change)

---

## Release Schedule

- **Patch releases**: As needed for bug fixes
- **Minor releases**: Monthly (new features)
- **Major releases**: When stable API reached (1.0)

---

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.9.x   | ✅ Yes    |
| < 0.9   | ❌ No     |

---

## Upgrade Guide

### 0.9.x → 0.9.x (Patch)

Safe upgrade. Just reinstall:

```bash
pip install --upgrade b4n1-web
npm install b4n1-web@latest
```

### Future: 0.x → 1.0 (Major)

Will include breaking changes. Migration guide will be provided.

---

## Deprecation Policy

- Features marked deprecated in minor release
- Removed in next major release
- Minimum 6 months notice