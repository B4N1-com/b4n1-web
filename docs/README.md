# B4n1Web - Documentación

Ultra-lightweight Agentic Browser Engine para AI Agents

**Versión:** 0.7.0 | **Fecha:** 2026-05-20

---

## Documentos

| Archivo | Descripción |
|---------|-------------|
| [AGENTS.md](AGENTS.md) | Referencia para AI agents |
| [PLAN.md](PLAN.md) | Roadmap y plan de desarrollo |
| [DEPLOY.md](DEPLOY.md) | Guía de despliegue |
| [ESTADO.md](ESTADO.md) | Estado actual del proyecto |
| [PODMAN_E2E_STRATEGY.md](PODMAN_E2E_STRATEGY.md) | Tests E2E con Podman |

---

## Arquitectura Actual

### Repositorios

| Repo | URL | Visibilidad | Contenido |
|------|-----|-------------|-----------|
| **b4n1web** | `github.com/B4N1-com/b4n1-web` | Público | Engine + SDKs + Tests |

### Estructura del Workspace

```
b4n1-web/
├── engine/cli-core/       # Motor Rust (binario ~5.5MB)
├── sdks/
│   ├── python/           # SDK Python (PyPI: b4n1-web)
│   ├── javascript/       # SDK JS (npm: b4n1-web)
│   ├── go/               # SDK Go
│   ├── java/             # SDK Java (Maven)
│   └── csharp/           # SDK C# (NuGet)
├── tests/                # Tests E2E
│   ├── python/           # Tests unitarios Python
│   └── e2e/              # Tests E2E completos
├── internal/tests/       # Tests unitarios por SDK
│   ├── javascript/
│   ├── go/
│   ├── java/
│   └── csharp/
├── scripts/              # Scripts build/install
└── docs/                 # Esta documentación
```

### SDK con Binario Bundlado

Cada SDK incluye el binario Rust directamente:

```bash
pip install b4n1-web     # Python SDK + binario
npm install b4n1-web     # JS SDK + binario
go get .../b4n1web       # Go SDK + binario
```

**Flujo de búsqueda:** SDK busca binario bundlado → PATH → error

### Modos de Navegación

| Modo | Descripción | Uso |
|------|-------------|-----|
| **Light** | HTTP + HTML parsing | Scraping estático |
| **JS** | Light + scripts extraction | SPA analysis |
| **Render** | Chromium real + screenshots | E2E testing |

### MCP Server

```bash
b4n1web mcp  # stdio mode (default for local agent integration)
# For TCP mode (optional): b4n1web mcp  # stdio mode (default for local agent integration)
# For TCP mode (optional): b4n1web mcp -p 8080
```

---

## Distribución

| Lenguaje | Registry | Install |
|----------|----------|---------|
| Python | PyPI | `pip install b4n1-web` |
| JavaScript | npm | `npm install b4n1-web` |
| Go | GitHub | `go get github.com/B4N1-com/b4n1-web` |
| Java | Maven Central | `com.b4n1:b4n1-web:0.8.0` |
| C# | NuGet | `dotnet add package B4n1Web` |

### Binario Standalone

```bash
curl -sL https://web.b4n1.com/install | bash
b4n1web --version
b4n1web mcp  # stdio mode (default for local agent integration)
# For TCP mode (optional): b4n1web mcp  # stdio mode (default for local agent integration)
# For TCP mode (optional): b4n1web mcp -p 8080
```

---

## Tests

| SDK | Tests | Cobertura |
|-----|-------|-----------|
| Rust | 54 | Engine completo |
| Python | 250+ | SDK + Security + MCP |
| JavaScript | 180+ | SDK + Security + Errors |
| E2E Podman | 6 fases | Todos los SDKs |

---

## URLs

| Recurso | URL |
|---------|-----|
| Web | https://web.b4n1.com |
| PyPI | https://pypi.org/project/b4n1-web |
| GitHub (público) | https://github.com/B4N1-com/b4n1-web |
| MCP Registry | https://mcp.so/server/b4n1web/B4N1-com |
