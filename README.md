# B4n1Web SDK

<p align="center">
  <img src="https://web.b4n1.com/logo.png" alt="B4n1Web Logo" width="200">
</p>

<p align="center">
  <a href="https://pypi.org/project/b4n1-web/"><img src="https://img.shields.io/pypi/v/b4n1-web.svg?color=blue" alt="PyPI version"></a>
  <a href="https://www.npmjs.com/package/b4n1-web"><img src="https://img.shields.io/npm/v/b4n1-web.svg?color=cb3837" alt="NPM version"></a>
  <a href="https://pkg.go.dev/github.com/B4N1-com/b4n1-web-go"><img src="https://img.shields.io/github/v/tag/B4N1-com/b4n1-web?label=go&color=00ADD8" alt="Go version"></a>
  <a href="https://www.nuget.org/packages/B4n1Web"><img src="https://img.shields.io/nuget/v/B4n1Web.svg?color=512BD4" alt="NuGet version"></a>
  <a href="https://central.sonatype.com/artifact/com.b4n1/b4n1-web"><img src="https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg?color=4C714E" alt="Maven Central"></a>
  <a href="https://pypi.org/project/b4n1-web/"><img src="https://img.shields.io/pypi/dm/b4n1-web.svg" alt="PyPI downloads"></a>
  <a href="https://github.com/B4N1-com/b4n1-web/blob/master/LICENSE"><img src="https://img.shields.io/pypi/l/b4n1-web.svg" alt="License"></a>
  <a href="https://web.b4n1.com"><img src="https://img.shields.io/badge/web-b4n1.com-00d4ff" alt="Website"></a>
</p>

Language-agnostic SDK for **B4n1Web: The Agentic Browser Engine**. Navigate URLs, extract structured content (markdown, links, screenshots), and build AI agent workflows.

## Installation

### 1. Install the B4n1Web Binary

```bash
curl -sL https://web.b4n1.com/install | bash
```

### 2. Install Your Preferred SDK

| Language | Package Manager | Install Command | Version |
|----------|-----------------|------------------|---------|
| **Python** | pip | `pip install b4n1-web` | [![PyPI](https://img.shields.io/pypi/v/b4n1-web.svg?color=blue)](https://pypi.org/project/b4n1-web/) |
| **JavaScript/TypeScript** | npm | `npm install b4n1-web` | [![npm](https://img.shields.io/npm/v/b4n1-web.svg?color=cb3837)](https://www.npmjs.com/package/b4n1-web) |
| **Go** | go | `go get github.com/B4N1-com/b4n1-web-go` | [![Go](https://img.shields.io/github/v/tag/B4N1-com/b4n1-web?label=go&color=00ADD8)](https://pkg.go.dev/github.com/B4N1-com/b4n1-web-go) |
| **C#/.NET** | NuGet | `dotnet add package B4n1Web` | [![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg?color=512BD4)](https://www.nuget.org/packages/B4n1Web) |
| **Java** | Maven | See below | [![Maven](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg?color=4C714E)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web) |

### Java (Maven)

```xml
<dependency>
    <groupId>com.b4n1</groupId>
    <artifactId>b4n1-web</artifactId>
    <version>0.4.0</version>
</dependency>
```

## Quick Start

### Python

```python
from b4n1web import AgentBrowser, BrowserMode

browser = AgentBrowser(mode=BrowserMode.LIGHT)
page = browser.goto("https://example.com")
print(page.markdown)
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
    b4n1web "github.com/B4N1-com/b4n1-web-go"
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

## Browser Modes

### Light Mode (Default)

- **Use case**: Reading articles, scraping static content, extracting links
- **Performance**: < 15MB RAM, instant startup
- **Capabilities**: HTML parsing, markdown conversion, link extraction
- **Limitations**: No JavaScript execution

### JS Mode

- **Use case**: Extracting JavaScript from pages, SPA analysis
- **Performance**: Same as Light, instant startup
- **Capabilities**: HTML parsing + JavaScript tag extraction

### Render Mode

- **Use case**: SPAs, form filling, visual verification, E2E testing
- **Performance**: Higher memory usage (~100MB), slower startup
- **Capabilities**: Full JavaScript execution, screenshots, interaction
- **Requirements**: Install Chromium via `b4n1web chromium install`

## Security

- All HTTP requests use TLS
- No arbitrary code execution
- Input validation and sanitization
- **Explicit Installation**: Binary must be installed separately via `curl -sL https://web.b4n1.com/install | bash` — no automatic downloads
- **Version Checking**: SDKs automatically warn if binary version doesn't match

### SecurityShield

All SDKs include a `SecurityShield` for domain-level safety validation with caching:

```python
# Python
from b4n1web.security import SecurityShield
shield = SecurityShield()
shield.mark_domain("evil.com", is_safe=False)
shield.is_url_safe("https://evil.com/page")  # (False, False)
```

Available in all 5 SDKs — Python (SQLite-backed), JavaScript, Go, Java, C#.

## API Reference

### Common Features (All SDKs)

| Feature | Description |
|---------|-------------|
| `AgentBrowser` | Main browser class |
| `BrowserMode` | `LIGHT`, `JS`, `RENDER` modes |
| `Page` | Structured page data (url, markdown, links, screenshot) |
| `goto/navigate` | Navigate to URL and return Page |
| `getMainContent()` | Extract main content, skipping headers |
| `findLinksByText()` | Find links containing specific text |
| `SecurityShield` | Domain safety validation with cache |

## MCP (Model Context Protocol)

Python SDK includes full MCP client support for AI agent integration:

```python
from b4n1web.mcp import AsyncMcpClient

async with AsyncMcpClient() as client:
    page = await client.goto("https://example.com")
    print(page.markdown)
```

Start the MCP server: `b4n1web mcp -p 8765`

## Architecture

```
┌─────────────────────────────────────┐
│         Your Application            │
│  (Python / JS / Go / Java / C#)    │
├─────────────────────────────────────┤
│        B4n1Web SDK (this repo)     │
│  Thin wrapper → spawns subprocess   │
├─────────────────────────────────────┤
│      b4n1web binary (Rust)         │
│  Installed separately via curl      │
├─────────────────────────────────────┤
│  Chromium (optional, render mode)  │
└─────────────────────────────────────┘
```

## Version Compatibility

SDK and binary versions should match for full compatibility. SDKs emit a warning when versions differ:

```
⚠️  Version mismatch: SDK v0.4.0 requires binary v0.4.0, but found v0.3.0.
```

Update the binary with: `curl -sL https://web.b4n1.com/install | bash`

## License

MIT OR Apache-2.0
