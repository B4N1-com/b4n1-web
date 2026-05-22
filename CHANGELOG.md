# CHANGELOG — b4n1-web

## v0.8.0 — 2026-05-20

### Added
- All SDKs updated to version 0.8.0
- SecurityShield improvements for IPv6, data:, and javascript: URI handling
- Session persistence (save_state/load_state) in Rust engine
- MCP stdio mode fixes
- 100% test coverage across Python and JavaScript SDKs
- Go SDK removed (not included in binary distribution per request)

## v0.7.0 — 2026-05-20

### Added
- Full-page screenshot support in Chromium render mode (`session.rs` improvements)
- `evaluate` MCP tool for arbitrary JavaScript execution (render mode)
- Chromium compatibility hardening for SDL/libwayland environments
- Visual regression testing framework (`visual.rs` — compare/encode/decode)
- `security` and `security_schema` deferred to V3
