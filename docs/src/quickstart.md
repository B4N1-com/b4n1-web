# B4n1Web — Agentic Browser Engine

> **Ultra-lightweight headless browser for AI agents.** Single Rust binary, 4 language SDKs. Navigate URLs, extract structured content (markdown, links, screenshots), and build autonomous agent workflows.

[![License](https://img.shields.io/badge/license-BSL%201.1-blue)](LICENSE)
[![PyPI](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![npm](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![Maven Central](https://img.shields.io/maven-central/v/com.b4n1/b4n1-web.svg)](https://central.sonatype.com/artifact/com.b4n1/b4n1-web)

## Quick Start

### 1. Install the binary

```bash
# One-liner (Linux/macOS/Windows via WSL)
curl -sL https://b4n1.com/install | bash
```

Or use your preferred package manager:

```bash
# Python
pip install b4n1-web

# JavaScript/TypeScript
npm install b4n1-web

# Java (Maven)
# Add to pom.xml:
# <dependency>
#   <groupId>com.b4n1</groupId>
#   <artifactId>b4n1-web</artifactId>
#   <version>0.13.0</version>
# </dependency>

# C# .NET
dotnet add package B4n1Web
```

### 2. Extract content from any URL

```python
from b4n1web import AgentBrowser

browser = AgentBrowser()
page = browser.goto("https://example.com")

print(page.url)       # URL
print(page.markdown)  # Clean markdown content
print(page.links)     # All extracted links
print(page.screenshot) # Base64 PNG (if available)
print(page.js_output)  # JavaScript output (if evaluated)

browser.close()
```

```javascript
import { AgentBrowser } from 'b4n1-web';

const browser = new AgentBrowser();
const page = await browser.gotoAsync('https://example.com');

console.log(page.url);
console.log(page.markdown);
console.log(page.links);

await browser.close();
```

```csharp
using B4N1Web;

var browser = new AgentBrowser();
var page = browser.Goto("https://example.com");

Console.WriteLine(page.Url);
Console.WriteLine(page.Markdown);
Console.WriteLine(page.Links);

browser.Close();
```

```java
import com.b4n1.AgentBrowser;
import com.b4n1.Page;

AgentBrowser browser = new AgentBrowser();
Page page = browser.goto("https://example.com");

System.out.println(page.getUrl());
System.out.println(page.getMarkdown());
System.out.println(page.getLinks());

browser.close();
```

## What You Get

| Feature | Description |
|---------|-------------|
| **Markdown** | Clean, structured content from any page |
| **Links** | All internal/external links extracted |
| **Screenshots** | Full-page or viewport PNG (base64) |
| **JavaScript** | Execute custom JS and get results |
| **Wait for selector** | Wait until element appears |
| **Click/Type** | Interact with elements |
| **MCP Server** | 33 tools for AI agent workflows |

## Browser Modes

| Mode | Description | RAM | Startup |
|------|-------------|-----|---------|
| **Light** | HTTP fetch + HTML parsing only | ~15MB | Instant |
| **JS** | Light + JavaScript extraction | ~15MB | Instant |
| **Render** | Full Chromium + screenshots | ~100MB | ~2s |

## SDK Matrix

| Language | Package | Version | Binary Bundled |
|----------|---------|---------|----------------|
| Python | `b4n1-web` | 0.13.0 | ✅ All platforms |
| JavaScript/TypeScript | `b4n1-web` | 0.13.0 | ✅ All platforms |
| Java (Maven) | `com.b4n1:b4n1-web` | 0.13.0 | ✅ All platforms |
| C# .NET (NuGet) | `B4n1Web` | 0.13.0 | ✅ All platforms |

**All platforms = linux-amd64, linux-arm64, macos-x64, macos-arm64, windows-amd64, windows-arm64**

## Next Steps

- [Installation](installation.md) — Detailed install instructions for each platform
- [Browser Modes](modes.md) — Choose the right mode for your use case
- [Python SDK](python.md) — Complete Python API reference
- [JavaScript SDK](javascript.md) — Complete JS/TS API reference
- [C# SDK](csharp.md) — Complete C# API reference
- [Java SDK](java.md) — Complete Java API reference
- [CLI Reference](cli.md) — Command-line interface
- [MCP Integration](mcp.md) — Use with AI agents via MCP