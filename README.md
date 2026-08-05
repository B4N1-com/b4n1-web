<div align="center">

# 🌐 B4n1Web — Agentic Browser Engine

**Ultra-lightweight headless browser for AI agents.**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)

Single Rust binary · 4 language SDKs · 33 MCP tools.
Navigate URLs, extract structured content (markdown, links, screenshots), and build autonomous agent workflows.

**[📊 Project Stats → STATS.md](STATS.md)**

---

## 🌍 Languages / Idiomas / 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](README.es.md) | 🇫🇷 [Français](README.fr.md) | 🇩🇪 [Deutsch](README.de.md) | 🇵🇹 [Português](README.pt-BR.md) | 🇮🇹 [Italiano](README.it.md) |
| 🇨🇳 [简体中文](README.zh-CN.md) | 🇯🇵 [日本語](README.ja.md) | 🇰🇷 [한국어](README.ko.md) | 🇷🇺 [Русский](README.ru.md) | 🇸🇦 [العربية](README.ar.md) | 🇮🇳 [हिन्दी](README.hi.md) |

---

## Features

- **33 MCP tools** for AI agent integration
- **Single self-contained Rust binary** ~11MB, no runtime dependencies
- **4 language SDKs** (Python, JS, Java, C#) with bundled binary
- **Static linking (musl)** — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## Browser Modes

| Mode | Description | RAM | Startup |
|------|-------------|-----|---------|
| Light | HTTP fetch + HTML parsing | ~15MB | Instant |
| JS | Light + JavaScript extraction | ~15MB | Instant |
| Render | Full Chromium + screenshots | ~100MB | ~2s |

## Quick Start

Install the binary or use your preferred package manager:

```bash
# Binary (any Linux, no dependencies)
curl -sL https://b4n1.com/install | bash

# Or via package managers
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: add dependency from Maven Central
```

Basic usage:

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()
```

### MCP Server

```bash
# stdio mode (default)
b4n1web mcp
npx b4n1-web mcp
uvx b4n1-web mcp
```

## SDK Matrix

| Language | Package | Version | Binary |
|----------|---------|---------|--------|
| Python | `b4n1-web` | 0.12.3 | Bundled (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.3 | Bundled (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.3 | Bundled (musl) |
| C# (.NET) | `B4n1Web` | 0.12.3 | Bundled (musl) |

## Documentation

- [📖 Full Documentation](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [MCP Tools](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry
- [📊 Project Stats](STATS.md) — downloads, versions, releases

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License

Apache License 2.0