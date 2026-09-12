# Version Bump Guide — b4n1-web

> **Every time you bump the version, update ALL files below. No exceptions.**

The canonical version is in `VERSION` (single-line file). All other files must match.

---

## Automated (via `scripts/bump-version.sh`)

Run from the **private repo** (`private-repos/b4n1-web`):

```bash
bash scripts/bump-version.sh <new-version>
```

This updates 15 files:

| # | File | What changes |
|---|------|-------------|
| 1 | `engine/cli-core/Cargo.toml` | `version = "x.y.z"` |
| 2 | `sdks/python/pyproject.toml` | `version = "x.y.z"` |
| 3 | `sdks/python/b4n1web/__init__.py` | `__version__ = "x.y.z"` |
| 4 | `sdks/python/b4n1web/browser.py` | `SDK_VERSION = "x.y.z"` |
| 5 | `sdks/javascript/package.json` | `"version": "x.y.z"` |
| 6 | `sdks/javascript/src/binary.ts` | `SDK_VERSION = 'x.y.z'` |
| 7 | `sdks/java/pom.xml` | `<version>x.y.z</version>` (project only) |
| 8 | `sdks/java/src/main/java/com/b4n1/web/AgentBrowser.java` | `SDK_VERSION = "x.y.z"` |
| 9 | `sdks/csharp/src/B4n1Web.csproj` | `<Version>x.y.z</Version>` |
| 10 | `MANIFEST.md` | `B4N1-WEB vX.Y.Z` |
| 11 | `docs/README.md` | `**Versión:** x.y.z` |
| 12 | `docs/AGENTS.md` | `**Versión actual:** x.y.z` |
| 13 | `docs/ESTADO.md` | `**Versión actual:** x.y.z` |
| 14 | `docs/cms/web.json` | `vX.Y.Z · Ultra-lightweight...` |
| 15 | `docs/cms/docs.json` | `vX.Y.Z · Ultra-lightweight...` |

---

## Manual (NOT covered by bump-version.sh) — CRITICAL

These files **must be updated manually** after running the script. This is where version drift happens.

### Public repo files (`public-repos/b4n1-web/`)

| # | File | What changes |
|---|------|-------------|
| 16 | `VERSION` | Single-line version (e.g. `0.14.0`) |
| 17 | `CHANGELOG.md` | Add new version entry with date + changes |
| 18 | `README.md` | SDK Matrix table: all 4 rows → new version |
| 19 | `i18n/README.es.md` | SDK Matrix table: all 4 rows → new version |
| 20 | `i18n/README.pt-BR.md` | SDK Matrix table: all 4 rows → new version |
| 21 | `i18n/README.fr.md` | SDK Matrix table: all 4 rows → new version |
| 22 | `i18n/README.de.md` | SDK Matrix table: all 4 rows → new version |
| 23 | `i18n/README.zh-CN.md` | SDK Matrix table: all 4 rows → new version |
| 24 | `i18n/README.ja.md` | SDK Matrix table: all 4 rows → new version |
| 25 | `i18n/README.ko.md` | SDK Matrix table: all 4 rows → new version |
| 26 | `i18n/README.ar.md` | SDK Matrix table: all 4 rows → new version |
| 27 | `i18n/README.hi.md` | SDK Matrix table: all 4 rows → new version |
| 28 | `i18n/README.it.md` | SDK Matrix table: all 4 rows → new version |
| 29 | `i18n/README.ru.md` | SDK Matrix table: all 4 rows → new version |

### SDK README files (`sdks/*/README.md`)

| # | File | What changes |
|---|------|-------------|
| 30 | `sdks/python/README.md` | Version section: SDK + Binary version |
| 31 | `sdks/javascript/README.md` | Version section: SDK + Binary version |
| 32 | `sdks/java/README.md` | Version section: SDK + Binary version + Maven example `<version>` |
| 33 | `sdks/csharp/README.md` | Version section: SDK + Binary version |

### C# source code

| # | File | What changes |
|---|------|-------------|
| 34 | `sdks/csharp/src/AgentBrowser.cs` | `SdkVersion = "x.y.z"` (line 24) |

### mdBook documentation (`docs/src/`)

| # | File | What changes |
|---|------|-------------|
| 35 | `docs/src/quickstart.md` | SDK Matrix table + Maven example |
| 36 | `docs/src/installation.md` | CLI output + Maven/Gradle examples |
| 37 | `docs/src/java.md` | Maven/Gradle examples + error messages |
| 38 | `docs/src/cli.md` | CLI output version |
| 39 | `docs/src/troubleshooting.md` | Error message examples |
| 40 | `docs/src/faq.md` | Error message examples |
| 41 | `docs/src/python.md` | Error message examples |
| 42 | `docs/src/csharp.md` | Error message examples |
| 43 | `docs/src/javascript.md` | Error message examples |
| 44 | `docs/src/changelog.md` | `Current: **x.y.z**` line |
| 45 | `docs/src/contributing.md` | Example commands |

### Test mocks (if version is checked in tests)

| # | File | What changes |
|---|------|-------------|
| 46 | `sdks/javascript/tests/e2e.test.ts` | Mock version if present |
| 36 | Other test files | Check for hardcoded version strings |

---

## Quick Reference: Search for stale versions

After bumping, run these to catch any file still referencing the old version:

```bash
# From private-repos/b4n1-web
OLD="0.13.0"  # ← replace with previous version
grep -rn "$OLD" --include='*.md' --include='*.toml' --include='*.json' \
  --include='*.py' --include='*.ts' --include='*.java' --include='*.cs' \
  --include='*.csproj' --include='*.xml' .

# From public-repos/b4n1-web
grep -rn "$OLD" --include='*.md' --include='*.cs' .
```

---

## Post-Bump Checklist

- [ ] Run `bash scripts/bump-version.sh <version>` (private repo)
- [ ] Update `public-repos/b4n1-web/VERSION`
- [ ] Update `CHANGELOG.md` with new entry
- [ ] Update all 12 README files (1 English + 11 i18n) — SDK Matrix versions
- [ ] Update all 4 SDK README files — version sections
- [ ] Update `sdks/csharp/src/AgentBrowser.cs` — `SdkVersion` constant
- [ ] Search for old version string: `grep -rn "OLD_VERSION" .`
- [ ] Run tests: `cargo test --lib` + SDK tests
- [ ] Build release: `cargo build --profile release`
- [ ] Sync public repo: copy changed files to `public-repos/b4n1-web/`
- [ ] Git commit + tag + push

---

## Why This File Exists

On 2026-09-12, the repo had:
- `VERSION` = 0.13.0
- English README = 0.9.x
- i18n READMEs = 0.12.3
- SDK READMEs = 0.9.8
- C# source = 0.12.0

**5 different versions across the same repo.** This file ensures it never happens again.
