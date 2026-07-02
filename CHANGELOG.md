# CHANGELOG — b4n1-web

## v0.11.0 — 2026-07-01

### Added
- **Cross-platform builds**: 8 binary targets (linux amd64/arm64/ia32, macOS x64/arm64, Windows amd64/arm64/ia32).
- **32-bit support**: i686 Linux and Windows builds added to release matrix.

### Changed
- All SDK native binaries updated to v0.11.0 builds.
- Version bumped across all 15 files (engine, SDKs, docs).

## v0.10.0 — 2026-07-01

### Fixed
- **PDF generation**: `goto` MCP handler now shares session ChromiumBrowser with `pdf`/`screenshot` instead of creating a separate ephemeral browser per call. PDFs are no longer blank (848 bytes → real content).
- **Hardcoded version**: MCP server now uses `env!("CARGO_PKG_VERSION")` at compile time instead of hardcoded `"0.8.0"`.

### Changed
- **Version bump automation**: `bump-version.sh` now updates 15 files (was 5). Covers all SDK runtime constants (`__init__.py`, `browser.py`, `binary.ts`, `AgentBrowser.java`), `MANIFEST.md`, and docs.
- **BREAKING**: All components now at **v0.10.0** — unified version across Rust engine, Python SDK, JS SDK, C# SDK, Java SDK, MCP server, and docs.
- **RELEASE_CHECKLIST.md**: Rewritten with complete end-to-end pipeline.

## v0.9.11 — 2026-07-01

### Fixed
- **npm SDK**: Removed broken `postinstall` script (not included in package `files` → `npm install` crash). Handled `chmod` in `binary.ts` at runtime instead.
- **npm SDK**: Added `import` entry to `package.json` exports for ESM compatibility.
- **verify-publish.sh**: Fixed ESM `import` → CJS `require()` for npm test. Fixed Java `List.size()` (was `.length`).

### Changed
- Published npm v0.9.9, v0.9.10, v0.9.11 with fixes (postinstall removal + ESM exports).
- All 4 SDKs (Python, npm, C#, Java) smoke-tested E2E.
- Removed stale `~/.npmrc` global auth token to prevent accidental publishes.

## v0.9.10 — 2026-07-01

### Fixed
- Same as v0.9.9. Published to fix ESM exports in package.json.

## v0.9.9 — 2026-07-01

### Fixed
- Same as v0.9.11 (npm postinstall fix only). Superseded by v0.9.10.

## v0.9.8 — 2026-07-01

### Added
- **Maven Central**: `b4n1-web` Java SDK published to Maven Central (v0.9.8).
- **verify-publish.sh**: Smoke test script for all 4 SDK registries (pip, npm, NuGet, Maven).
- **PIPELINE_GUIDE.md**: Complete documentation of the multi-platform build + publish pipeline.

### Fixed
- **URL web.b4n1.com → b4n1.com**: Updated across 29 files (SDK metadata, READMEs, docs, examples, tests, nginx).
- **Maven Central credentials**: Stale `~/.m2/settings.xml` updated with correct Sonatype token.
- **C# build artifacts**: `**/bin/` and `**/obj/` removed from git tracking.

## v0.9.7 — 2026-07-01

### Fixed
- **SDK version bump**: Published PyPI `b4n1-web==0.9.7`, npm `b4n1-web@0.9.7`, NuGet `B4n1Web 0.9.7`.
- **Maven Central**: First publish attempt — stalled in PUBLISHING state, auto-published after delay.

## v0.9.6 — 2026-06-30

### Fixed
- **Python SDK (`security.py`)**: Fragment handling in `_extract_domain` — strip `#section` before parsing.
- **Java SDK (`AgentBrowser.java`)**: `getVersion()` returned `"b4n1web 0.9.4"` instead of `"0.9.4"` — parse version string correctly.
- **C# SDK (`AgentBrowser.cs`)**: Same version parsing bug — strip `"b4n1web "` prefix from version output.

### Added
- **Rust engine**: Full MCP server test suite (322 tests total) — 33 tools with schema validation.
- **Python SDK**: Comprehensive test suite (130 tests) covering browser, MCP, security, errors, MCP types.
- **JavaScript SDK**: Comprehensive test suite (50 tests) — vitest with page, types, errors, security, browser, binary.
- **Java SDK**: Comprehensive test suite (27 tests) — Page, BrowserMode, AgentBrowser, errors.
- **C# SDK**: Comprehensive test suite (16 tests) — Page, Models, AgentBrowser.
- **E2E integration tests**: Binary version, help flags, MCP tool listing verification.

### Changed
- Updated all SDKs and engine to v0.9.5.

## v0.9.3 — 2026-06-08

### Refactored
- **Core (Rust)**: Massive modularization of `browser.rs`, `mcp.rs`, and `session.rs`.
- **SDKs (Python/JS)**: Extracted MCP types and separated `Page`/`AgentBrowser` classes.
- **Portability**: Removed all hardcoded absolute paths to `/home/b4n1`.

### Fixed
- **MCP Client (Python)**: Robust non-blocking stdio communication (fixes `TypeError` and timeout issues).
- **Output Parser (JS)**: Improved line-by-line parsing for structured data and screenshots.

### Added
- Unified Test Runner (`test_all.sh`) covering Core, Python, and JS.
- Support for `Symbol.asyncDispose` in JS SDK.

## v0.7.0 — 2026-05-20

### Added
- Full-page screenshot support in Chromium render mode (`session.rs` improvements)
- `evaluate` MCP tool for arbitrary JavaScript execution (render mode)
- Chromium compatibility hardening for SDL/libwayland environments
- Visual regression testing framework (`visual.rs` — compare/encode/decode)
- `security` and `security_schema` deferred to V3

### Changed
- **MCP stdio mode is now the default and primary mode** (TCP route was dead-code, now fully removed from main.rs)
- MCP server strip-block bug eliminated
- MCP client cleaned (static list, no eval-in-Python vector)
- `b4n1web` → `b4n1web` everywhere in docs and code
- Docs: audience, accomplishment and index names updated (repo renamed to public `b4n1-web`)
- MCP tests (Python) tripled in focus; 38 tests now cover handshake + goto + links (all edge cases)

### Fixed
- MCP stdio mode fix: `run_mcp_server_stdio()` replaces dead `await` in main entrypoint
- `get_links` handler in MCP server now returns live page links
- Visual diff test data schema: `prod_data` block with `md5 / sha256 / size / mime`
- Tempfile collision in Rust visual tests (`write_test_png` now uses atomic counter, not PID)
- `MANIFEST.md`: b4n1-mcp interface marked ✅activa (was stale ❌no)
- AGENTS.md: `b4n1-web-private` → `b4n1-web`, private GitHub URL removed

### Security
- **Critical**: `.env` removed from git history (contained live API keys: PyPI, NuGet, Sonatype)
- `.gitignore` hardened: now 101 patterns (Rust target/, Python `__pycache__`, node_modules, C# obj/bin, Go pkg, IDE files, OS files, etc.)
- All C# internal test build artifacts removed from index (40 files with absolute /home/b4n1/ paths)

### Release highlights
- b4n1web binary: 203 tests passing (Rust engine)
- Python MCP test suite: 38/38 passing
- All 5 SDKs (Rust/Python/JS/Java/C#) at v0.7.0
