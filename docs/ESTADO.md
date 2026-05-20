# B4n1Web — Estado del Proyecto

**Fecha:** 2026-05-20
**Versión actual:** 0.8.0

---

## Resumen

**v0.8.0** - MCP stdio, visual regression tests, CHANGELOG, CMS docs actualizados, publicados en todos los registries.

---

## Lo que se hizo en esta sesión

### 1. Release v0.6.2 publicado
- ✅ CLI actualizado a 0.6.2
- ✅ Todos los SDKs sincronizados
- ✅ CMS docs actualizados
- ✅ Publicados en todos los registries

### 2. Release v0.6.1
- ✅ SDKs sincronizados
- ✅ Docs actualizados
- ✅ Publicados en todos los registries

### 3. Release v0.5.0
- ✅ Binarios bundlados actualizados
- ✅ Go imports corregidos
- ✅ MCP port фихed

### 4. Repo limpio
- ✅ **b4n1web** (público) → Engine + SDKs + Tests
- ✅ Sin @qwencoder como contributor

### 5. Documentación actualizada
- ✅ README.md, AGENTS.md actualizados
- ✅ docs.json corregido (campos completos)
- ✅ Ejemplos de código compilables

---

## Estructura actual

```
b4n1-web/
├── engine/cli-core/       # Motor Rust (v0.6.2)
├── sdks/
│   ├── python/           # SDK + binario
│   ├── javascript/       # SDK + binario
│   ├── go/               # SDK + binario
│   ├── java/             # SDK + binario
│   └── csharp/           # SDK + binario
├── tests/python/         # Tests
├── tests/e2e/            # E2E completos
├── internal/tests/       # Tests por SDK
├── scripts/run_e2e.sh   # Runner E2E
└── docs/                 # Documentación
```

---

## Pendiente

- [ ] Compilar binarios macOS (requiere cross o Mac)
- [ ] Tests E2E con Podman (ejecutar)

---

## Commits recientes

| Repo | Commit | Descripción |
|------|--------|-------------|
| b4n1web | latest | v0.8.0: MCP stdio, tests 38/38 |


---

## URLs

| Recurso | URL |
|---------|-----|
| Web | https://web.b4n1.com |
| PyPI | https://pypi.org/project/b4n1-web |
| GitHub (público) | https://github.com/B4N1-com/b4n1-web |
| MCP Registry | https://mcp.so/server/b4n1web/B4N1-com |
