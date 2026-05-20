# B4n1Web - Guía de Despliegue

**Versión:** 0.4.0 | **Fecha:** 2026-04-11

---

## Arquitectura de Repos

| Repo | URL | Visibilidad | Contenido |
|------|-----|-------------|-----------|
| **b4n1web** | `github.com/B4N1-com/b4n1-web` | Público | Engine + SDKs + Tests |

---

## Convenciones de Nombres

| Componente | Nombre | Notas |
|------------|--------|-------|
| **PyPI** | `b4n1-web` | Con guion (requerido) |
| **npm** | `b4n1-web` | Con guion |
| **Módulo Python** | `b4n1web` | Sin guion |
| **Binario** | `b4n1web` | Sin guion |
| **Go module** | `github.com/B4N1-com/b4n1-web` | Sin guion |
| **Java** | `com.b4n1:b4n1-web` | Con guion |
| **C#** | `B4n1Web` | PascalCase |

---

## Gestión de Versiones

### Regla: SDK y binario MISMA versión

Todos deben tener la misma versión (ej: 0.4.0):
- `engine/cli-core/Cargo.toml`
- `sdks/python/pyproject.toml`
- `sdks/javascript/package.json`
- `sdks/go/go.mod`
- `sdks/java/pom.xml`
- `sdks/csharp/src/B4n1Web.csproj`

### Orden de Publicación

1. **Engine Rust** → `cargo build --release` → tests → commit
2. **Python SDK** → actualizar versión → `python -m build` → PyPI
3. **JavaScript SDK** → actualizar versión → `npm publish`
4. **Go SDK** → actualizar versión → push
5. **Java SDK** → actualizar versión → Maven Central
6. **C# SDK** → actualizar versión → NuGet
7. **GitHub Release** → binario + notas
8. **Commit final** → `git add -A && git commit -m "Release vX.Y.Z"`

---

## Compilar Engine

```bash
cd engine/cli-core
cargo build --release
# Binario: target/release/b4n1web (~5.5MB)
```

### Cross-compilation (futuro)

```bash
cargo install cross
# macOS:
cross build --target x86_64-app-darwin --release
cross build --target aarch64-apple-darwin --release
```

---

## Publicar SDKs

### Python (PyPI)

```bash
cd sdks/python
rm -rf dist build *.egg-info
python -m build
twine upload dist/b4n1_web-*.whl -u __token__ -p $PYPI_TOKEN
```

### JavaScript (npm)

```bash
cd sdks/javascript
npm run build
npm publish
```

### Go (GitHub)

```bash
cd sdks/go
git tag v0.4.0
git push --tags
```

### Java (Maven Central)

```bash
cd sdks/java
mvn clean deploy
```

### C# (NuGet)

```bash
cd sdks/csharp/src
dotnet pack -c Release
dotnet nuget push *.nupkg
```

---

## GitHub Release

```bash
# Crear tarball del binario
tar -czf b4n1web-v0.4.0-x86_64.tar.gz -C engine/cli-core/target/release b4n1web

# Crear release (repo público)
gh release create v0.4.0 b4n1web-v0.4.0-x86_64.tar.gz \
  --title "B4n1Web v0.4.0" \
  --notes "Release notes..." \
  --repo B4N1-com/b4n1-web
```

---

## Landing Server (web.b4n1.com)

```bash
# Reiniciar después de cambios
systemctl --user restart b4n1web-landing.service

# Ver logs
journalctl --user -u b4n1web-landing.service -f
```

---

## Tests

### Rust

```bash
cd engine/cli-core && cargo test
```

### Python

```bash
cd sdks/python && python -m pytest tests/ -v
```

### JavaScript

```bash
cd sdks/javascript && npx vitest run
```

### E2E Podman

```bash
bash scripts/run_e2e.sh          # Todos
bash scripts/run_e2e.sh python   # Solo Python
bash scripts/run_e2e.sh binary   # Solo binario
```

---

## Estado Actual

| Componente | Versión | Estado |
|------------|---------|--------|
| Engine Rust | 0.4.0 | ✅ Compilado |
| Python SDK | 0.4.0 | ✅ PyPI |
| JavaScript SDK | 0.4.0 | ✅ npm |
| Go SDK | 0.4.0 | ✅ GitHub |
| Java SDK | 0.4.0 | ✅ Maven |
| C# SDK | 0.4.0 | ✅ NuGet |
| Web | web.b4n1.com | ✅ Online |
| MCP Registry | mcp.so | ✅ Indexado |
