# B4n1Web — Agentic Browser Engine

[![GitHub](https://img.shields.io/github/license/B4N1-com/b4n1-web)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)
[![Docs](https://img.shields.io/badge/docs-mdBook-blue)](https://B4N1-com.github.io/b4n1-web/)



Ultra-lightweight headless browser for AI agents.

📖 **Full documentation**: https://B4N1-com.github.io/b4n1-web/ Single Rust binary, 4 language SDKs. Navigate URLs, extract structured content (markdown, links, screenshots), and build autonomous agent workflows.

## Quick Start

```bash
# Install the binary
curl -sL https://b4n1.com/install | bash

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

# Or with each SDK
npx b4n1-web mcp
uvx b4n1-web mcp
```

## SDK Matrix

| Language | Package | Version | Binary |
|----------|---------|---------|--------|
| Python | `b4n1-web` | 0.12.2 | Bundled (musl) |
| JavaScript/TypeScript | `b4n1-web` | 0.12.2 | Bundled (musl) |
| Java | `com.b4n1:b4n1-web` | 0.12.2 | Bundled (musl) |
| C# (.NET) | `B4n1Web` | 0.12.2 | Bundled (musl) |

## Browser Modes

| Mode | Description | RAM | Startup |
|------|-------------|-----|---------|
| Light | HTTP fetch + HTML parsing | ~15MB | Instant |
| JS | Light + JavaScript extraction | ~15MB | Instant |
| Render | Full Chromium + screenshots | ~100MB | ~2s |

## Features

- **33 MCP tools** for AI agent integration
- **4 language SDKs** (Python, JS, Java, C#)
- **Self-contained binary** ~11MB, no dependencies
- **Static linking** (musl) — works on any Linux, no glibc required
- **Three modes**: Light (instant), JS (scripts), Render (Chromium)
- **Security shield**: domain filtering, safe browsing
- **Network interception**: block resources, mock responses
- **MCP Server**: stdio transport, no port needed

## Documentation

- [SDK Docs](docs/README.md) — Full documentation
- [Agent Reference](docs/AGENTS.md) — AI agent integration
- [MCP Tools](https://mcp.so/server/b4n1web/B4N1-com) — MCP registry

## Links

- Website: https://b4n1.com
- GitHub: https://github.com/B4N1-com/b4n1-web
- PyPI: https://pypi.org/project/b4n1-web
- npm: https://www.npmjs.com/package/b4n1-web
- NuGet: https://www.nuget.org/packages/B4n1Web
- Maven Central: https://central.sonatype.com/artifact/com.b4n1/b4n1-web

## License

MIT
