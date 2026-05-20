# Release Checklist v0.8.0

## Pre-Release
- [x] Versiones actualizadas a 0.8.0 en todos los SDKs
- [x] Rust engine: 203 tests pasando
- [x] Visual regression test framework (compare/encode/decode)
- [x] Security: .env removed from git + .gitignore hardened (101 patterns)
- [x] MANIFEST.md actualizado a v0.7.0
- [x] docs/README.md y docs/AGENTS.md actualizados a v0.7.0
- [x] docs/ESTADO.md actualizado a v0.7.0
- [x] docs/cms/web.json y docs/cms/docs.json actualizadas
- [x] SDKs completos con todas las features (screenshot, click, typeText, etc.)
- [x] Ejemplos ejecutables en cada SDK
- [x] MCP fix: skip_serializing_if, protocolVersion dinámico
- [x] eval() reemplazado por JSON.parse/json.loads en JS/Python
- [x] get_links handler añadido al MCP server
- [x] Changelog actualizado
- [x] README actualizado

## Binario
- [x] Compilar release binary: `cargo build --release` → b4n1web v0.7.0 (~9.2MB)
- [ ] Subir a GitHub Releases como `b4n1web-v0.7.0-x86_64-linux.tar.gz`
- [ ] Generar checksum SHA256
- [ ] Actualizar URL en install scripts

## Package Managers
- [ ] **PyPI**: `cd sdks/python && python3 -m build && twine upload dist/*`
- [ ] **npm**: `cd sdks/javascript && npm publish`
- [ ] **Maven**: `cd sdks/java && mvn deploy`
- [ ] **NuGet**: `cd sdks/csharp && dotnet pack && dotnet nuget push`

## GitHub Release
- [ ] Crear tag `v0.7.0`
- [ ] Publicar release notes
- [ ] Adjuntar binario comprimido
- [ ] Sincronizar public-repos/b4n1-web

## Post-Release
- [ ] Verificar instalación desde cada package manager
- [ ] Verificar `b4n1web mcp` funciona con OpenCode/Kilo
- [ ] Verificar ejemplo básico de cada SDK
