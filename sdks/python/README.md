# B4n1Web Python SDK

[![PyPI version](https://badge.fury.io/py/b4n1-web.svg)](https://pypi.org/project/b4n1-web/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)

> Ultra-lightweight agentic browser engine with bundled binary. Navigate URLs, extract structured content (markdown, links, screenshots), and build AI agent workflows.

## Install

```bash
pip install b4n1-web
```

No additional binary installation needed — the SDK bundles the b4n1web binary.

## Quick Start

```python
from b4n1web import AgentBrowser, BrowserMode

# Light mode - fastest
browser = AgentBrowser(mode=BrowserMode.LIGHT)
page = browser.goto("https://example.com")
print(page.markdown)
print(f"Found {len(page.links)} links")
browser.close()

# JS mode - extract scripts
browser = AgentBrowser(mode=BrowserMode.JS)
page = browser.goto("https://example.com")
browser.close()

# Render mode - full Chromium (requires chromium install)
# browser = AgentBrowser(mode=BrowserMode.RENDER)
# page = browser.goto("https://example.com")
# print(page.screenshot)  # base64 screenshot
```

## Context Manager

```python
with AgentBrowser() as browser:
    page = browser.goto("https://example.com")
    print(page.markdown)
```

## Page Object

```python
page.url          # str: final URL
page.markdown     # str: page content as markdown
page.links        # list[str]: all links on page
page.screenshot   # str|None: base64 screenshot (render mode)

page.get_main_content()           # str: content without headers
page.find_links_by_text("more")   # list[str]: links containing "more"
```

## Browser Modes

| Mode | Description | RAM | Startup |
|------|-------------|-----|---------|
| `LIGHT` | HTTP fetch + HTML parsing | ~15MB | Instant |
| `JS` | Light + JavaScript extraction | ~15MB | Instant |
| `RENDER` | Full Chromium + screenshots | ~100MB | ~2s |

## Security

```python
from b4n1web.security import SecurityShield

shield = SecurityShield()
shield.mark_domain("evil.com", is_safe=False)
is_safe, needs_api = shield.is_url_safe("https://evil.com/page")
# (False, False)
```

## MCP Integration

```python
from b4n1web.mcp import AsyncMcpClient

async with AsyncMcpClient() as client:
    page = await client.goto("https://example.com")
    print(page.markdown)
```

Start MCP server:
```bash
b4n1web mcp -p 8080
```

## Error Handling

```python
from b4n1web import BinaryNotFoundError

try:
    browser = AgentBrowser()
except BinaryNotFoundError:
    print("b4n1web binary not found. Install with:")
    print("  curl -sL https://web.b4n1.com/install | bash")
```

## Version

SDK version: **0.4.0**
Bundled binary version: **0.4.0**

## Links

- [PyPI](https://pypi.org/project/b4n1-web/)
- [GitHub](https://github.com/B4N1-com/b4n1-web)
- [Website](https://web.b4n1.com)

---
*Built by B4N1 with ❤️ · All rights reserved.*
