# B4n1Web — AGENTS.md

## Overview

**B4n1Web** — Navegador agéntico ultra-ligero para IA
**Versión actual:** 0.8.0
**Fecha:** 2026-05-20

## Quick Reference

| Componente | Versión | Estado |
|------------|---------|--------|
| engine (Rust) | 0.8.0 | ✅ Compilado |
| SDK Python | 0.8.0 | ✅ PyPI |
| SDK JavaScript | 0.8.0 | ✅ npm |
| SDK Go | --- | ❌ Removed (per request) |
| SDK Java | 0.8.0 | ✅ Maven Central |
| SDK C# | 0.8.0 | ✅ NuGet |
| MCP Server | 0.8.0 | ✅ Funcional |

---

## Acciones Comunes

### 🧪 Test

```bash
# Rust engine
cd engine/cli-core && cargo test --lib          # 203 unit tests
# (integration tests folded into unit test suite)

# Python SDK
PYTHONPATH=sdks/python:$PYTHONPATH python3 -m pytest tests/python/test_mcp.py -v  # 38 MCP tests

# JavaScript SDK
cd sdks/javascript && npm install && npx vitest run  # JS SDK tests

# Go SDK (requires Go 1.22+)
export PATH=$HOME/go1.22/go/bin:$PATH
cd sdks/go && go test ./internal/ -v            # 37 tests

# E2E con Podman
bash scripts/run_e2e_simple.sh                  # All SDKs in containers
```

### 🏗️ Build

```bash
# Compilar engine
cd engine/cli-core && cargo build --release

# El binario queda en:
# engine/cli-core/target/release/b4n1web (~5.5MB)

# Copiar binario a SDKs
cp engine/cli-core/target/release/b4n1web sdks/python/b4n1web/bin/
cp engine/cli-core/target/release/b4n1web sdks/javascript/bin/
```

### 🚀 Release

```bash
# 1. Actualizar versión en engine/cli-core/Cargo.toml
# 2. Actualizar versión en todos los SDKs (sdks/*/...)
# 3. Compilar: cargo build --release
# 4. Copiar binario a todos los SDKs
# 5. Commit & push: git add -A && git commit -m "Release vX.Y.Z" && git push
# 6. Publicar SDKs a sus registries (PyPI, npm, etc)
# 7. Crear GitHub release en repo público
```

### 📸 Render Mode Testing

```bash
# Test screenshot capture
b4n1web goto https://example.com --mode render 2>/dev/null \
  | grep "Screenshot:" | sed 's/Screenshot: //' \
  | base64 -d > test_screenshot.png
file test_screenshot.png  # PNG image data, 1280 x 720
```

### 🔧 MCP Server

```bash
# Puerto por defecto
b4n1web mcp -p 8080

# Puerto custom
b4n1web mcp -p 8765
```

### 📦 PyPI

```bash
cd sdks/python
rm -rf dist build *.egg-info
python -m build
twine upload dist/b4n1_web-*.whl -u __token__ -p $PYPI_TOKEN
```

### ⚡ Dev Loop

```bash
cd engine/cli-core
cargo test
cargo build --release
git add -A && git commit -m "Fix: descripción" && git push
```

---

## Estructura del Proyecto

```
b4n1-web/
├── engine/cli-core/       # Motor Rust (binario ~5.5MB)
├── sdks/
│   ├── python/           # SDK Python (PyPI)
│   ├── javascript/       # SDK JS (npm)
│   ├── go/               # SDK Go
│   ├── java/             # SDK Java
│   └── csharp/           # SDK C#
├── tests/                # Tests E2E + Python
│   ├── python/           # Tests unitarios Python
│   └── e2e/              # Tests E2E completos
├── internal/tests/       # Tests unitarios por SDK
│   ├── javascript/
│   ├── go/
│   ├── java/
│   └── csharp/
├── scripts/              # Scripts build/install
└── docs/                 # Documentación
```

---

## Modos de Navegación

| Modo | Flag | Descripción | RAM | Speed |
|------|------|-------------|-----|-------|
| **Light** | `--mode light` | HTTP fetch + HTML parsing | ~15MB | Instant |
| **JS** | `--mode js` | Light + extracción de scripts | ~15MB | Instant |
| **Render** | `--mode render` | Chromium real + screenshots | ~100MB | ~2s |

---

## URLs Importantes

| Recurso | URL |
|---------|-----|
| Web | https://web.b4n1.com |
| PyPI | https://pypi.org/project/b4n1-web |
| npm | https://www.npmjs.com/package/b4n1-web |
| GitHub (público) | https://github.com/B4N1-com/b4n1-web |
| MCP Registry | https://mcp.so/server/b4n1web/B4N1-com |

---

## Roadmap

| Versión | Estado | Notas |
|---------|--------|-------|
| **0.8.0** | ✅ | MCP stdio fix, visual tests, full test coverage |
| **0.5.0** | ✅ | Binarios bundlados, Go imports, MCP port |
| **1.0.0** | ⏳ | Stable release |

---

## Notas Importantes

- **Binario ~9.2MB** (LTO + stripped) (ultra-ligero, compilado con LTO)
- **SDKs bundlan el binario** — `pip install` = todo listo
- **MCP Server** funcional en puerto 8080
- **E2E con Podman** — tests aislados y reproducibles
- **Screenshots** en formato PNG base64 (Render mode)
- **Go SDK** requiere Go 1.22+ (1.21 tiene bug con kernels nuevos)
- **v0.8.0**: MCP stdio fix, visual tests fixed, CHANGELOG + MANIFEST updated

---

## MCP Server (Configuración)

### Iniciar servidor

```bash
b4n1web mcp -p 8080
```

### OpenCode - Configuración

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "b4n1web": {
      "type": "local",
      "command": ["b4n1web", "mcp", "-p", "8765"],
      "enabled": true
    }
  }
}
```

### Herramientas disponibles

- `goto(url, mode)` — Navegar a URL
  - `mode`: "light", "js", "render"
- `get_links()` — Obtener links de la página

---

## Gestión de Sesión

Si pierdes la sesión:
1. Leer `docs/README.md` para contexto general
2. Leer este `AGENTS.md` para acciones disponibles
3. Revisar último commit: `git log --oneline -5`
