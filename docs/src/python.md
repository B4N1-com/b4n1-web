# Python SDK

Complete API reference for the Python SDK.

## Installation

```bash
pip install b4n1-web
```

## Basic Usage

```python
from b4n1web import AgentBrowser, BrowserOptions, BrowserMode

# Simple usage
browser = AgentBrowser()
page = browser.goto("https://example.com")
print(page.markdown)
browser.close()

# Context manager (auto-close)
with AgentBrowser() as browser:
    page = browser.goto("https://example.com")
    print(page.markdown)
```

## Browser Options

```python
from b4n1web import BrowserOptions, BrowserMode

options = BrowserOptions(
    mode=BrowserMode.LIGHT,      # LIGHT, JS, or RENDER
    timeout=30,                  # seconds
    user_agent="MyBot/1.0"       # custom user agent
)

browser = AgentBrowser(options=options)
```

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `mode` | `BrowserMode` | `LIGHT` | LIGHT, JS, or RENDER |
| `timeout` | `int` | `30` | Request timeout in seconds |
| `user_agent` | `str` | `"B4N1Web-Agent/1.0"` | Custom User-Agent string |

## Page Object

```python
page = browser.goto("https://example.com")

# Properties
page.url          # str - Final URL after redirects
page.markdown     # str - Clean markdown content
page.links        # List[str] - All extracted links
page.screenshot   # str | None - Base64 PNG (Render mode only)
page.js_output    # str | None - JavaScript output (JS/Render mode)

# Methods
page.get_main_content()     # str - Content skipping first 2 lines
page.find_links_by_text("click")  # List[str] - Links containing text
```

### Page Methods

```python
# Get main content (skip headers)
content = page.get_main_content()

# Find links containing specific text (case-insensitive)
contact_links = page.find_links_by_text("contact")
```

## Advanced Features (Render Mode Only)

```python
browser = AgentBrowser(options=BrowserOptions(mode=BrowserMode.RENDER))

# Navigate with wait
page = browser.goto("https://example.com", wait_for=".content")

# Screenshot
screenshot_b64 = browser.screenshot(width=1920, height=1080)
# Returns base64 PNG string

# Wait for selector
found = browser.wait_for_selector(".my-element", timeout_ms=5000)
# Returns True/False

# Click element
browser.click(".submit-button")

# Type text
browser.type_text("#search", "query", clear_first=True)

# Get links from last page
links = browser.get_links()

# Execute JavaScript
result = browser.evaluate("document.title")
# Returns JS output as string
```

## Async Usage

```python
import asyncio
from b4n1web import AgentBrowser

async def main():
    browser = AgentBrowser()
    page = await browser.goto_async("https://example.com")
    print(page.markdown)
    await browser.close_async()

asyncio.run(main())
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `B4N1WEB_BINARY` | Path to custom binary |
| `B4N1WEB_CHROMIUM` | Path to Chromium executable |

```bash
export B4N1WEB_BINARY=/custom/path/b4n1web
export B4N1WEB_CHROMIUM=/usr/bin/chromium
```

## Error Handling

```python
from b4n1web import AgentBrowser, BrowserOptions

try:
    browser = AgentBrowser()
    page = browser.goto("https://example.com")
except FileNotFoundError:
    print("Binary not found. Install with: pip install b4n1-web")
except Exception as e:
    print(f"Error: {e}")
```

## Version Compatibility

The SDK checks binary version on startup. If mismatched, a warning is printed:

```
⚠️  Version mismatch: SDK v0.13.0 requires binary v0.13.0, but found v0.9.4
```

## Type Hints

Full type annotations included. Works with mypy, pyright, VS Code IntelliSense.

```python
from b4n1web import AgentBrowser, Page
from typing import List

def extract_links(url: str) -> List[str]:
    browser: AgentBrowser = AgentBrowser()
    page: Page = browser.goto(url)
    return page.links
```

## Async Context Manager

```python
async with AgentBrowser() as browser:
    page = await browser.goto_async("https://example.com")
    # Auto-closes on exit
```