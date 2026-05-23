## v0.9.1 — 2026-05-23 [WIP]

### Added
- (pending)

---


## v0.9.0 — 2026-05-23 [WIP]

### Added
- (pending)

---


# CHANGELOG — b4n1-web

## v0.8.0 — 2026-05-23 (pre-release)

### Added
- All SDKs updated to version 0.8.0
- SecurityShield improvements for IPv6, data:, and javascript: URI handling
- Session persistence (save_state/load_state) in Rust engine
- MCP stdio mode fixes (protocolVersion dynamic)
- 100% test coverage across Python and JavaScript SDKs
- Go SDK removed (not included in binary distribution per request)
- Visual regression testing framework (`visual.rs` — compare/encode/decode)
- `evaluate` MCP tool for arbitrary JavaScript execution (render mode)
- Full-page screenshot support in Chromium render mode
- Chromium compatibility hardening for SDL/libwayland environments
- `screenshot`, `click`, `type_text`, `wait_for_selector`, `set_viewport` MCP tools
- RGB color utility module (`css/rgb.rs`)
- DOM tree with full CSS Selector and XPath support
- Tox + conda Python 3.12 test environments
- Auto-seal notary proof per commit

### Fixed
- `eval()` reemplazado por `JSON.parse` / `json.loads` en JS y Python
- SecurityShield ahora refleja verdaderamente si una URL es `javascript:` o `data:`
- MCP `skip_serializing_if = Option::is_none` para omitir `null` en responses
- `b4n1web goto` ya no emite `DOMReady` ni `goto()` warnings
- `run_stdio_sync` usando `BufReader` y `stdin`/`stdout` nativo (sin tokio overhead)
- MCP protocol version negociado dinámicamente desde el cliente

### Changed
- Rust engine ahora es edición 2021
- Binary target consolidado: `engine/cli-core/` → `b4n1web` crate única
- Tests: Python, Rust, JS, Go consolidados bajo estructura uniforme
- CHANGELOG unificado bajo formato Keep-a-Changelog

---

## v0.7.0 — 2026-05-20

### Added
- Full-page screenshot support in Chromium render mode (`session.rs` improvements)
- `evaluate` MCP tool for arbitrary JavaScript execution (render mode)
- Chromium compatibility hardening for SDL/libwayland environments
- Visual regression testing framework (`visual.rs` — compare/encode/decode)
- `security` and `security_schema` deferred to V3

---

## v0.6.2 — 2026-05-20

### Added
- External `b4n1-web` GitHub repo as primary install source
- Hash-verified curl install: `curl -sL https://web.b4n1.com/install | bash`
- Docker/Podman E2E test support with Ubuntu 22.04 image

---

## v0.6.1 — 2026-05-19

### Changed
- Binaries are auto-isolated by OS + arch (ventanas separadas por plataforma)
- Default isolate: `~/.b4n1web` (sobrescribible con `isolate_init("path")`)
- README documentation compañera bilingüe ES/EN (próximamente)

---

## v0.6.0 — 2026-05-18

### Added
- Auto-isolate tokens for each domain
- `isolate_init(path)` para forzar rutas de session
- Pure Rust HTTP engine (sin reqwest overhead)

### Fixed
- Binary paths now absolute para permitir ejecución desde cualquier directorio

---

## v0.5.0 — 2026-05-17

### Added
- `b4n1web` binary empaquetado standalone (~2MB)
- Bundled binaries en cada SDK
- Go imports deshabilitados por defecto
- MCP port configurable

---

## v0.4.0 — 2026-05-16

### Added
- SDK APIs lanzadas: Python, JS, Java, C#, Go
- `AgentBrowser` API con goto/get_html/get_text
- SDK inheritance chain: `browser.py` → `browser.js` → `AgentBrowser.java`

---

## v0.3.0 — 2026-05-15

### Added
- API Go SDK publicada (standalone)
- `binary_not_found` sera tratado como `BinaryNotFoundError` excepción
- Restricciones de URL en SecurityShield extendidas

---

## v0.2.3 — 2026-05-14

### Added
- SDK Spring Boot con `bootstrap.java`

---

## v0.1.4 — 2026-05-13

### Changed
- Console output cleanup (sin traces verbose en STDERR)

---

## v0.1.3 — 2026-05-12

### Changed
- Console output movido a STDERR para pipelines

---

## v0.1.2 — 2026-05-11

### Changed
- Binarios empaquetados con version en nombre de archivo

---

## v0.1.1 — 2026-05-10

### Added
- Rust binary ahora descargado desde GitHub releases
- Install script: `curl -sL https://web.b4n1.com/install | bash`

---

## v0.1.0 — 2026-05-09

### Added
- First public b4n1web release (Python-only)
- Engine Rust inicial + Python SDK
