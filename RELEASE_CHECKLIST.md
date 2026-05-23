# Release Checklist v0.9.0

## Pre-Release
- [x] Versiones centralizadas en `VERSION` (0.9.0)
- [x] Rust engine compila sin errores (9.3MB release binary, `cargo build --release` OK)
- [ ] Rust unit tests corriendo (`cargo test --lib` — chromiumoxide tarda ~20 min en compilar)
- [x] Tests Python — 160/160 passing ✅ (35.89s)
- [x] MANIFEST.md actualizado a v0.9.0
- [x] CHANGELOG.md completo desde v0.1.0 hasta v0.9.0
- [x] docs/README.md y docs/AGENTS.md alineados a v0.9.0
- [x] `scripts/bump-version.sh` creado y probado
- [x] main.rs: 4 errores de compilación corregidos
- [x] mcp.rs: `CARGO_PKG_VERSION` en `serverInfo.version` (dinámico)
- [x] B4n1Web.csproj 0.9.0, pom.xml 0.9.0

## Binario
- [x] Compilar: `cargo build --release` → b4n1web v0.9.0 (9.3MB)
- [x] Tarball creado: `releases/b4n1web-v0.9.0-linux-x86_64.tar.gz`
- [ ] Subir a GitHub Releases
- [ ] Generar SHA256 y actualizar `scripts/install.sh`

## Package Managers
- [ ] **PyPI**: `cd sdks/python && python3 -m build && twine upload dist/*`
- [ ] **npm**: `cd sdks/javascript && npm publish`
- [ ] **Maven**: `cd sdks/java && mvn deploy`
- [ ] **NuGet**: `cd sdks/csharp && dotnet pack && dotnet nuget push`

## GitHub Release
- [ ] Crear tag `v0.9.0` + push
- [ ] Publicar release notes desde CHANGELOG.md
- [ ] Adjuntar tarball con SHA256
- [ ] Notariado en `.b4n1/`

## Post-Release
- [ ] Verificar instalación desde cada registry
- [ ] Verificar `b4n1web mcp` con Kilo/OpenCode
- [ ] Verificar ejemplo de cada SDK
- [ ] Sync private-repos/b4n1-web si corresponde
- [ ] Actualizar `install.sh` apuntando a nuevo release
