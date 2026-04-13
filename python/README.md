# B4n1Web SDK — Ultra-lightweight Agentic Browser Engine

B4n1Web is a 5MB binary browser engine for AI agents. Navigate URLs, extract structured content (markdown, links, screenshots), and build AI agent workflows.

## Installation

```bash
# Install the binary
curl -sL https://web.b4n1.com/install | bash

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
- [Website](https://web.b4n1.com)
- [Documentation](https://web.b4n1.com/documentacion)

## License

MIT OR Apache-2.0
