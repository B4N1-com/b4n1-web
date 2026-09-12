# Contributing

Thanks for contributing to B4n1Web!

## Development Setup

```bash
# Clone
git clone https://github.com/B4N1-com/b4n1-web.git
cd b4n1-web

# Rust binary
cd engine/cli-core
cargo build --release

# Python SDK
cd ../../sdks/python
pip install -e ".[dev]"

# JavaScript SDK
cd ../../sdks/javascript
npm install

# Java SDK
cd ../../sdks/java
mvn install -DskipTests

# C# SDK
cd ../../sdks/csharp
dotnet build
```

## Running Tests

```bash
# All tests
bash test_all.sh

# Rust
cd engine/cli-core && cargo test --lib

# JavaScript
cd sdks/javascript && npx vitest run

# Python
cd sdks/python && PYTHONPATH=. pytest tests/ -v

# C#
cd sdks/csharp && dotnet test
```

## Code Style

- **Rust**: `cargo fmt` + `cargo clippy`
- **TypeScript**: `npm run lint` (ESLint + Prettier)
- **Python**: `black` + `ruff`
- **C#**: `dotnet format`
- **Java**: Google Java Format

## Pull Request Process

1. Fork the repo
2. Create feature branch: `git checkout -b feat/my-feature`
3. Make changes with tests
4. Run all tests: `bash test_all.sh`
5. Commit: `git commit -m "feat: my feature"`
6. Push and open PR

## Commit Convention

```
feat: new feature
fix: bug fix
docs: documentation changes
refactor: code restructuring
test: adding tests
chore: maintenance
```

Example: `feat(python): add async context manager support`

## Adding a New SDK

1. Create `sdks/<language>/` directory
2. Implement `AgentBrowser` class with same API
2. Add tests in `sdks/<language>/tests/`
3. Add to `test_all.sh`
4. Update documentation

## Releasing

```bash
# Bump version
bash scripts/bump-version.sh 0.13.0

# Commit and tag
git add -A && git commit -m "release: v0.13.0"
git tag v0.13.0
git push && git push --tags

# GitHub Actions builds and publishes automatically
```

## Reporting Issues

Use [GitHub Issues](https://github.com/B4N1-com/b4n1-web/issues) with:
- OS and version
- b4n1web version
- Full error message
- Minimal reproduction

## Security

Report security issues privately via email to security@b4n1.com

## Code of Conduct

Be respectful. No harassment, discrimination, or spam.