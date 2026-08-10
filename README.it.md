<div align="center">

<img src="b4n1web_presentation.png" alt="B4n1Web" width="100%">

# 🌐 B4n1Web — Motore di Navigazione Agéntico

**Browser headless ultraleggero per agenti IA.**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

[![PyPI Downloads/month](https://img.shields.io/pypi/dm/b4n1-web)](https://pypi.org/project/b4n1-web/)
[![npm Downloads/month](https://img.shields.io/npm/dm/b4n1-web)](https://www.npmjs.com/package/b4n1-web)
[![NuGet Downloads](https://img.shields.io/nuget/dt/B4n1Web)](https://www.nuget.org/packages/B4n1Web)
[![GitHub Release](https://img.shields.io/github/v/release/B4N1-com/b4n1-web)](https://github.com/B4N1-com/b4n1-web/releases)
[![GitHub Downloads](https://img.shields.io/github/downloads/B4N1-com/b4n1-web/total)](https://github.com/B4N1-com/b4n1-web/releases)

Un singolo binario Rust · 4 SDK di linguaggio · 33 strumenti MCP.
Naviga URL, estrai contenuti strutturati (markdown, link, screenshot) e costruisci flussi di lavoro autonomi per agenti.

---

## 🌍 Languages / Idiomas / 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## 🖥 🖥 Supporto piattaforme

8 binari pre-compilati — funziona ovunque:

| Platform | Architectures | Binary |
|----------|---------------|--------|
| **Linux** | x86_64, aarch64, i686 | `musl` (static, no glibc) |
| **macOS** | x86_64, arm64 | `universal` |
| **Windows** | x86_64, arm64, i686 | `MSVC` |

## Caratteristiche

- **33 MCP tools** — integrazione agenti IA
- **Single self-contained Rust binary** ~11MB, no runtime dependencies
- **4 language SDKs** (Python, JS, Java, C#) with bundled binary
- **Static linking (musl)** — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## Modalità del Browser

| Modalità | Descrizione | RAM | Avvio |
|------|-------------|-----|---------|
| Light | Fetch HTTP + parsing HTML | ~15MB | Instant |
| JS | Light + estrazione JavaScript | ~15MB | Instant |
| Render | Chromium completo + screenshot | ~100MB | ~2s |

## Avvio Rapido

Installa il binario o usa il tuo gestore di pacchetti preferito:

```bash
# Binario (Linux, macOS, Windows — senza dipendenze)
curl -sL https://b4n1.com/install | bash

# Oppure tramite gestori di pacchetti
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: aggiungi la dipendenza da Maven Central
```

Utilizzo di base:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### Server MCP

```bash
# modalità stdio (predefinita)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## Matrice SDK

| Linguaggio | Package | Version | Binario |
|----------|---------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | Integrato (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Integrato (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Integrato (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Integrato (musl) |

## Documentazione

- [📖 Documentazione completa](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [Strumenti MCP](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License

Apache License 2.0
