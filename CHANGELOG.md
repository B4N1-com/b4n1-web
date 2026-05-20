# CHANGELOG — b4n1-web

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
