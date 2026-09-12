<div align="center">

<img src="b4n1web_presentation.png" alt="B4n1Web" width="100%">

# 🌐 B4n1Web — Agentic Browser Engine

**Ultra-lightweight headless browser for AI agents.**

[![License](https://img.shields.io/badge/license-BSL%201.1-blue)](LICENSE)
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

Single Rust binary · 4 language SDKs · 33 MCP tools.
Navigate URLs, extract structured content (markdown, links, screenshots), and build autonomous agent workflows.

📖 **Full documentation**: https://B4N1-com.github.io/b4n1-web/

</div>

---

## 🌍 Languages / Idiomas / 语言

|  |  |  |  |  |  |
|--|--|--|--|--|--|
| 🇬🇧 [English](README.md) | 🇪🇸 [Español](i18n/README.es.md) | 🇫🇷 [Français](i18n/README.fr.md) | 🇩🇪 [Deutsch](i18n/README.de.md) | 🇵🇹 [Português](i18n/README.pt-BR.md) | 🇮🇹 [Italiano](i18n/README.it.md) |
| 🇨🇳 [简体中文](i18n/README.zh-CN.md) | 🇯🇵 [日本語](i18n/README.ja.md) | 🇰🇷 [한국어](i18n/README.ko.md) | 🇷🇺 [Русский](i18n/README.ru.md) | 🇸🇦 [العربية](i18n/README.ar.md) | 🇮🇳 [हिन्दी](i18n/README.hi.md) |

---

## 🖥 Platform Support

8 pre-compiled binaries — works everywhere:

| Platform | Architectures | Binary |
|----------|---------------|--------|
| **Linux** | x86_64, aarch64, i686 | `musl` (static, no glibc) |
| **macOS** | x86_64, arm64 | `universal` |
| **Windows** | x86_64, arm64, i686 | `MSVC` |

## Quick Start

```bash
# Install the binary
curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash

# Or use your preferred package manager
pip install b4n1-web
npm install b4n1-web
dotnet add package B4n1Web
# Java: add dependency from Maven Central
```

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
| Python | `b4n1-web` | 0.13.0 | Bundled (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.13.0 | Bundled (musl) |
| Java | `com.b4n1:b4n1-web` | 0.13.0 | Bundled (musl) |
| C# (.NET) | `B4n1Web` | 0.13.0 | Bundled (musl) |

## Browser Modes

| Mode | Description | RAM | Startup |
|------|-------------|-----|---------|
| Light | HTTP fetch + HTML parsing | ~15MB | Instant |
| JS | Light + JavaScript extraction | ~15MB | Instant |
| Render | Full Chromium + screenshots | ~100MB | ~2s |

## Features

- **33 MCP tools** for AI agent integration
- **4 language SDKs** (Python, JS, Java, C#) with bundled binary
- **Self-contained binary** ~12MB, no dependencies
- **Static linking (musl)** — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## Documentation

- [📖 Full documentation](https://B4N1-com.github.io/b4n1-web/) — mdBook
- [Agent Reference](docs/AGENTS.md) — AI agent integration
- [MCP Tools](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License & Commercial Terms

- **Original Source Code**: 100% Proprietary and Closed Source (**All Rights Reserved** by Bani Montoya).
- **Binaries & SDK Tools**: Provided under the **Business Source License 1.1 (BSL 1.1)**.
  - **Free Tier**: 100% free for development, evaluation, testing, personal projects, and startups with gross annual revenue **< $100,000 USD**.
  - **Mandatory Enterprise B2B License**: Required for government agencies, public bidding projects, and enterprises with gross annual revenue **>= $100,000 USD**.

For Enterprise B2B licensing inquiries, visit https://b4n1.com/licensing or email `b4n1@b4n1.com`.
