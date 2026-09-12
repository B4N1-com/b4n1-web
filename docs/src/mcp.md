# MCP Integration

B4n1Web includes a **Model Context Protocol (MCP) server** with 33 tools for AI agent workflows.

## What is MCP?

MCP (Model Context Protocol) lets AI agents (Claude, Cursor, etc.) use tools through a standardized interface. B4n1Web's MCP server exposes browser automation as callable tools.

## Quick Start

```bash
# Start MCP server
b4n1web mcp
```

This runs on stdio. Configure your AI client to use it:

### Claude Desktop Config

```json
{
  "mcpServers": {
    "b4n1web": {
      "command": "b4n1web",
      "args": ["mcp"]
    }
  }
}
```

### Cursor Config

```json
{
  "mcp": {
    "servers": {
      "b4n1web": {
        "command": "b4n1web",
        "args": ["mcp"]
      }
    }
  }
}
```

## Available Tools (33)

### Navigation
| Tool | Description |
|------|-------------|
| `goto` | Navigate to URL and extract content |
| `click` | Click element by CSS selector |
| `type_text` | Type text into element |
| `wait_for_selector` | Wait for element to appear |

### Content Extraction
| Tool | Description |
|------|-------------|
| `evaluate` | Execute JavaScript |
| `screenshot` | Take screenshot |
| `pdf` | Generate PDF |
| `get_cookies` | Get all cookies |
| `clear_cookies` | Clear all cookies |

### Page Interaction
| Tool | Description |
|------|-------------|
| `hover` | Hover over element |
| `scroll` | Scroll page/element |
| `select_option` | Select dropdown option |
| `key_press` | Press keyboard key |
| `upload_file` | Upload file to input |
| `download_file` | Download file from URL |

### Network
| Tool | Description |
|------|-------------|
| `route` | Add network route handler |
| `unroute` | Remove route handler |
| `block_resources` | Block resource types |
| `wait_for_request` | Wait for request |
| `wait_for_response` | Wait for response |

### Browser State
| Tool | Description |
|------|-------------|
| `set_viewport` | Set viewport size |
| `set_user_agent` | Override user agent |
| `set_geolocation` | Set geolocation |
| `emulate_device` | Emulate device |
| `frames` | List iframes |
| `iframe_text` | Get iframe text |
| `iframe_click` | Click in iframe |
| `iframe_type_text` | Type in iframe |

### Utility
| Tool | Description |
|------|-------------|
| `performance_metrics` | Get Core Web Vitals |
| `save_state` | Save cookies/localStorage |
| `load_state` | Restore cookies/localStorage |
| `start_context` | Start incognito context |
| `quit_context` | Quit incognito context |

---

## Usage Example (AI Agent)

```
User: "Go to github.com and get the markdown of the trending page"

AI Agent calls:
1. goto(url="https://github.com/trending", mode="light")
2. Returns: {url, markdown, links, ...}
```

---

## Daemon Mode

For persistent connections:

```bash
# Start daemon
b4n1web daemon start

# Check status
b4n1web daemon status

# Stop daemon
b4n1web daemon stop
```

The daemon runs the MCP server in background. Useful for long-running agent sessions.

---

## Configuration

### Environment Variables

```bash
export B4N1WEB_BINARY=/custom/path/b4n1web
export B4N1WEB_CHROMIUM=/usr/bin/chromium
```

### Custom Chromium

The MCP server auto-downloads Chromium for Render mode tools. To use system Chromium:

```bash
export B4N1WEB_CHROMIUM=/usr/bin/chromium
b4n1web mcp
```

---

## All 33 Tools Reference

Each tool returns structured JSON. Example:

```json
{
  "goto": {
    "url": "https://example.com",
    "mode": "light",
    "wait_for": ".content"
  }
}
```

See [CLI Reference](cli.md) for individual tool parameters.

---

## Security

The MCP server runs with your user permissions. It can:
- Access any URL
- Take screenshots
- Download files
- Execute JavaScript

Only run with trusted AI agents.