# B4n1Web SDK — Ultra-lightweight Agentic Browser Engine

B4n1Web is a 5MB binary browser engine for AI agents. Navigate URLs, extract structured content (markdown, links, screenshots), and build AI agent workflows.

## Installation

```bash
# Install the binary
curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz && ./b4n1web --version

# Install the Python SDK
pip install b4n1-web
```

## Quick Start

```python
from b4n1web import AgentBrowser, BrowserMode

browser = AgentBrowser(mode=BrowserMode.LIGHT)
page = browser.goto("https://example.com")
print(page.markdown)
print(page.links)
browser.close()
```

## Features

- **Ultra-lightweight** — 5MB binary, instant startup
- **3 modes** — Light (HTTP+HTML), JS (tag extraction), Render (full Chromium)
- **MCP server** — Integrates with Claude, Cursor, OpenCode, Windsurf, Antigravity
- **SecurityShield** — SQLite-based domain safety validation
- **`--wait-for` selector** — Wait for dynamic content in render mode

## Links

- [GitHub](https://github.com/B4N1-com/b4n1-web)

## License

MIT OR Apache-2.0
