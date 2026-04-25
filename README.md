# B4n1Web SDK

<p align="center">
  <img src="https://github.com/B4N1-com/b4n1-web/raw/master/logo.png" alt="B4n1Web Logo" width="200">
</p>

<p align="center">
  <a href="https://github.com/B4N1-com/b4n1-web/stargazers"><img src="https://img.shields.io/github/stars/B4N1-com/b4n1-web?style=flat&color=yellow" alt="GitHub stars"></a>
  <a href="https://github.com/B4N1-com/b4n1-web/network"><img src="https://img.shields.io/github/forks/B4N1-com/b4n1-web?style=flat&color=blue" alt="GitHub forks"></a>
  <a href="https://github.com/B4N1-com/b4n1-web/releases/tag/v0.6.2"><img src="https://img.shields.io/github/v/release/B4N1-com/b4n1-web?style=flat&color=green" alt="GitHub release"></a>
  <a href="https://github.com/B4N1-com/b4n1-web/releases/latest"><img src="https://img.shields.io/github/downloads/B4N1-com/b4n1-web/total?style=flat&color=orange" alt="GitHub downloads"></a>
  <br>
  <a href="https://pypi.org/project/b4n1-web/"><img src="https://img.shields.io/pypi/v/b4n1-web.svg?style=flat&color=blue" alt="PyPI version"></a>
  <a href="https://pypi.org/project/b4n1-web/"><img src="https://img.shields.io/pypi/dm/b4n1-web?style=flat&color=blue" alt="PyPI downloads"></a>
  <a href="https://pypi.org/project/b4n1-web/"><img src="https://img.shields.io/pypi/l/b4n1-web.svg?style=flat&color=blue" alt="PyPI license"></a>
  <a href="https://www.npmjs.com/package/b4n1-web"><img src="https://img.shields.io/npm/v/b4n1-web.svg?style=flat&color=cb3837" alt="NPM version"></a>
  <a href="https://www.npmjs.com/package/b4n1-web"><img src="https://img.shields.io/npm/dm/b4n1-web.svg?style=flat&color=cb3837" alt="NPM downloads"></a>
  <a href="https://www.nuget.org/packages/B4n1Web"><img src="https://img.shields.io/nuget/v/B4n1Web.svg?style=flat&color=512BD4" alt="NuGet version"></a>
  <a href="https://central.sonatype.com/artifact/com.b4n1/b4n1-web"><img src="https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg?style=flat&color=4C714E" alt="Maven Central"></a>
  <br>
  <a href="https://github.com/B4N1-com/b4n1-web/blob/master/LICENSE"><img src="https://img.shields.io/github/license/B4N1-com/b4n1-web?style=flat&color=green" alt="License"></a>
  <a href="https://github.com/B4N1-com/b4n1-web"><img src="https://img.shields.io/badge/web-GitHub-00d4ff?style=flat" alt="Website"></a>
</p>

Ultra-lightweight agentic browser engine (5MB binary) for AI agents. Navigate URLs, extract structured content (markdown, links, screenshots), and build AI agent workflows across 5 languages.

## Installation

### 1. Install the B4n1Web Binary

```bash
curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz && ./b4n1web --version
```

### 2. Install Your Preferred SDK

| Language | Package Manager | Install Command |
|----------|-----------------|------------------|
| **Python** | pip | `pip install b4n1-web` |
| **JavaScript/TypeScript** | npm | `npm install b4n1-web` |
| **C#/.NET** | NuGet | `dotnet add package B4n1Web` |
| **Java** | Maven | See below |
| **Go** | go | `go get github.com/B4N1-com/b4n1-web/go` |

### Java (Maven)

```xml
<dependency>
    <groupId>com.b4n1</groupId>
    <artifactId>b4n1-web</artifactId>
    <version>0.6.0</version>
</dependency>
```

## Quick Start

### Python

```python
from b4n1web import AgentBrowser, BrowserMode

browser = AgentBrowser(mode=BrowserMode.LIGHT)
page = browser.goto("https://example.com")
print(page.markdown)
print(page.links)
browser.close()
```

### JavaScript/TypeScript

```typescript
import { AgentBrowser, BrowserMode } from 'b4n1-web';

const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
const page = await browser.goto('https://example.com');
console.log(page.markdown);
browser.close();
```

### Go

```go
package main

import (
    "fmt"
    b4n1web "github.com/B4N1-com/b4n1-web/go"
)

func main() {
    browser, err := b4n1web.NewAgentBrowser(
        b4n1web.WithMode(b4n1web.ModeLight),
    )
    if err != nil {
        panic(err)
    }
    defer browser.Close()

    page, err := browser.Goto("https://example.com")
    if err != nil {
        panic(err)
    }

    fmt.Println(page.Markdown)
    fmt.Println(page.Links)
}
```

### C#

```csharp
using B4N1Web;

var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light });
var page = browser.Goto("https://example.com");
Console.WriteLine(page.Markdown);
browser.Close();
```

### Java

```java
import com.b4n1.web.*;

AgentBrowser browser = new AgentBrowser(new BrowserOptions());
Page page = browser.goto_("https://example.com");
System.out.println(page.getMarkdown());
browser.close();
```

### CLI

```bash
b4n1web goto https://example.com --mode light
```

## Browser Modes

| Mode | Use Case | Performance | Capabilities |
|------|----------|-------------|-------------|
| **Light** | Reading articles, scraping static content, extracting links | < 15MB RAM, instant | HTML parsing, markdown conversion, link extraction |
| **JS** | Extracting JavaScript from pages, SPA analysis | Same as Light | HTML parsing + JS tag extraction |
| **Render** | SPAs, form filling, visual verification, E2E testing | ~100MB, slower startup | Full JS execution, screenshots, DOM interaction |

## Features

### `--wait-for` Selector (Render Mode)

Wait for a CSS selector to appear before extracting content — essential for dynamically-loaded pages:

```bash
# CLI
b4n1web goto "https://example.com" --mode render --wait-for "#search-results"

# Python
browser.goto("https://example.com", wait_for="#search-results")

# JavaScript
await browser.goto("https://example.com", { waitFor: "#search-results" })
```

Supports `#id`, `.class`, and `tag` selectors with 10s timeout.

### SecurityShield

Domain-level safety validation with SQLite-backed caching:

```python
from b4n1web.security import SecurityShield
shield = SecurityShield()
shield.mark_domain("evil.com", is_safe=False)
shield.is_url_safe("https://evil.com/page")  # (False, False)
```

Available in all 5 SDKs.

### MCP (Model Context Protocol)

Full MCP server + Python client for AI agent integration:

```bash
# Start MCP server
b4n1web mcp -p 8765
```

```python
from b4n1web.mcp import AsyncMcpClient

async with AsyncMcpClient() as client:
    page = await client.goto("https://example.com")
    print(page.markdown)
```

**Available MCP tools:**
- `goto` — Navigate to URL and extract content (params: `url`, `mode`, `wait_for`)
- `get_links` — Get all links from current page

**Auto-configure for AI agents:**
```bash
b4n1web install opencode       # ~/.opencode/config.json
b4n1web install cursor         # ~/.cursor/mcp.json
b4n1web install windsurf       # ~/.windsurf/mcp.json
b4n1web install claude-code    # ~/Library/Application Support/Claude/mcp.json
b4n1web install antigravity    # ~/.config/antigravity/mcp.json
```

## API Reference

### Common Features (All SDKs)

| Feature | Description |
|---------|-------------|
| `AgentBrowser` | Main browser class |
| `BrowserMode` | `LIGHT`, `JS`, `RENDER` modes |
| `Page` | Structured data: url, markdown, links, screenshot |
| `goto` / `navigate` | Navigate to URL and return Page |
| `wait_for` | CSS selector to wait for (render mode) |
| `getMainContent()` | Extract main content, skipping headers |
| `findLinksByText()` | Find links containing specific text |
| `SecurityShield` | Domain safety validation with cache |

## Architecture

```
┌─────────────────────────────────────┐
│         Your Application            │
│  (Python / JS / Go / Java / C#)    │
├─────────────────────────────────────┤
│        B4n1Web SDK (this repo)     │
│  Thin wrapper → spawns subprocess   │
├─────────────────────────────────────┤
│        b4n1web binary (~5MB)       │
│  Installed separately via curl      │
├─────────────────────────────────────┤
│  Chromium (optional, render mode)  │
└─────────────────────────────────────┘
```

## Chromium Management

```bash
b4n1web chromium install      # Download Chromium (~150MB)
b4n1web chromium update       # Update to latest
b4n1web chromium version      # Show current version
b4n1web chromium remove       # Remove Chromium
```

## License

MIT OR Apache-2.0
