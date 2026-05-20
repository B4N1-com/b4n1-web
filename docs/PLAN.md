# B4n1Web - Plan de Desarrollo

**Última actualización:** 2026-04-11

---

## Estado Actual

### v0.4.0 - Completado ✅

| Feature | Estado |
|---------|--------|
| Light mode | ✅ HTTP fetch + HTML parsing |
| JS mode | ✅ Extrae script tags |
| Render mode | ✅ Chromium real + screenshots |
| SecurityShield | ✅ SQLite cache (7 días) |
| SDKs con binario bundlado | ✅ Python, JS, Go, Java, C# |
| MCP Server | ✅ Puerto 8080 |
| E2E Podman | ✅ 6 fases por SDK |
| Tests exhaustivos | ✅ 500+ tests |

---

## Arquitectura Actual

### SDKs con Binario Bundlado

Cada SDK incluye el binario Rust (~5.5MB):

```
pip install b4n1-web     → SDK Python + binario
npm install b4n1-web     → SDK JS + binario
go get .../b4n1web       → SDK Go + binario
```

**Búsqueda:** SDK busca binario bundlado → PATH → BinaryNotFoundError

### Modos de Navegación

```
b4n1web (5.5MB)
    │
    ├── Light: HTTP + HTML parsing (~15MB RAM)
    ├── JS: Light + script extraction
    └── Render: Chromium real + screenshots (~100MB)
```

---

## Roadmap

### v0.5.0 - Próximo

1. ⬜ API key licensing system
2. ⬜ Modo PRO con features avanzados
3. ⬜ Rate limiting + quotas
4. ⬜ Analytics/dashboard de uso
5. ⬜ Compilar binarios macOS (cross-compile)
6. ⬜ Tests E2E multi-distro (Alpine, Fedora)

### v1.0.0 - Stable Release

1. ⬜ API estable y documentada
2. ⬜ Tests de cobertura >90%
3. ⬜ CI/CD automatizado
4. ⬜ Documentación completa por SDK
5. ⬜ Video demos / tutoriales

---

## Estructura del Proyecto

```
b4n1-web/
├── engine/cli-core/       # Motor Rust (Cargo.toml + src/)
├── sdks/                  # SDKs con binarios bundlados
│   ├── python/           # PyPI: b4n1-web
│   ├── javascript/       # npm: b4n1-web
│   ├── go/               # GitHub: B4N1-com/b4n1-web
│   ├── java/             # Maven: com.b4n1:b4n1-web
│   └── csharp/           # NuGet: B4n1Web
├── tests/                # Tests E2E
├── internal/tests/       # Tests unitarios por SDK
├── scripts/              # Build/install scripts
└── docs/                 # Documentación
```

### Repos

| Repo | URL | Visibilidad |
|------|-----|-------------|
| **b4n1web** | `github.com/B4N1-com/b4n1-web` | Público |

---

## Tests

### Ejecutar Tests

```bash
# Rust engine
cd engine/cli-core && cargo test        # 54 tests

# Python SDK
cd sdks/python && python -m pytest tests/ -v   # 250+ tests

# JavaScript SDK
cd sdks/javascript && npx vitest run    # 180+ tests

# E2E con Podman
bash scripts/run_e2e.sh                 # 6 fases
bash scripts/run_e2e.sh python          # Solo Python
bash scripts/run_e2e.sh binary          # Solo binario
```

### Cobertura

| SDK | Tests | Cobertura |
|-----|-------|-----------|
| Rust | 54 | Engine completo |
| Python | 250+ | SDK + Security + MCP + E2E |
| JavaScript | 180+ | SDK + Security + Errors |
| E2E Podman | 6 fases | Todos los SDKs + binario |

---

## Distribución

| Lenguaje | Registry | Paquete | Install |
|----------|----------|---------|---------|
| Python | PyPI | `b4n1-web` | `pip install b4n1-web` |
| JavaScript | npm | `b4n1-web` | `npm install b4n1-web` |
| Go | GitHub | `b4n1web` | `go get github.com/B4N1-com/b4n1-web` |
| Java | Maven Central | `com.b4n1:b4n1-web` | `0.4.0` |
| C# | NuGet | `B4n1Web` | `dotnet add package B4n1Web` |

### Binario Standalone

```bash
curl -sL https://web.b4n1.com/install | bash
b4n1web --version
b4n1web mcp -p 8080
```

---

## URLs

| Recurso | URL |
|---------|-----|
| Web | https://web.b4n1.com |
| PyPI | https://pypi.org/project/b4n1-web |
| npm | https://www.npmjs.com/package/b4n1-web |
| GitHub (público) | https://github.com/B4N1-com/b4n1-web |
| MCP Registry | https://mcp.so/server/b4n1web/B4N1-com |
