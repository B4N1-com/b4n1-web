# CLI Reference

Complete command-line interface for the `b4n1web` binary.

## Installation

```bash
# One-liner
curl -sL https://b4n1.com/install | bash

# Or download from GitHub Releases
```

## Commands

### `b4n1web goto <URL>`

Navigate to a URL and extract content.

```bash
# Basic usage
b4n1web goto https://example.com

# With mode
b4n1web goto https://example.com --mode light
b4n1web goto https://example.com --mode js
b4n1web goto https://example.com --mode render

# Wait for selector
b4n1web goto https://example.com --wait-for ".content"

# Output to file
b4n1web goto https://example.com --output page.md
```

**Options:**

| Option | Short | Description |
|--------|-------|-------------|
| `--mode` | `-m` | `light`, `js`, or `render` (default: light) |
| `--wait-for` | `-w` | CSS selector to wait for |
| `--output` | `-o` | Output file path |
| `--timeout` | `-t` | Timeout in seconds (default: 30) |
| `--user-agent` | `-u` | Custom User-Agent |

---

### `b4n1web screenshot`

Take a screenshot (requires Render mode).

```bash
b4n1web screenshot --url https://example.com --width 1920 --height 1080
```

**Options:**

| Option | Short | Description |
|--------|-------|-------------|
| `--url` | `-u` | URL to screenshot (required) |
| `--width` | `-W` | Width in pixels (default: 1920) |
| `--height` | `-H` | Height in pixels (default: 1080) |
| `--full-page` | `-f` | Full page screenshot |

---

### `b4n1web click`

Click an element (requires Render mode).

```bash
b4n1web click ".submit-button" --url https://example.com
```

---

### `b4n1web type-text`

Type text into an element (requires Render mode).

```bash
b4n1web type-text "#search" "query" --url https://example.com --clear-first
```

---

### `b4n1web wait-for-selector`

Wait for an element to appear (requires Render mode).

```bash
b4n1web wait-for-selector ".results" --url https://example.com --timeout 5000
```

---

### `b4n1web evaluate`

Execute JavaScript (requires Render mode).

```bash
b4n1web evaluate "document.title" --url https://example.com
```

---

### `b4n1web scroll`

Scroll the page (requires Render mode).

```bash
b4n1web scroll --x 0 --y 500 --url https://example.com
```

---

### `b4n1web pdf`

Generate PDF (requires Render mode).

```bash
b4n1web pdf --url https://example.com --output page.pdf
```

---

### `b4n1web --version`

Show version.

```bash
b4n1web --version
# b4n1web 0.13.0
```

---

### `b4n1web --help`

Show help.

```bash
b4n1web --help
```

---

## MCP Server

### `b4n1web mcp`

Start MCP server for AI agents.

```bash
b4n1web mcp
```

This starts a Model Context Protocol server on stdio with 33 tools.

---

### `b4n1web daemon`

Start background daemon.

```bash
b4n1web daemon start
b4n1web daemon stop
b4n1web daemon status
```

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `B4N1WEB_BINARY` | Path to custom binary |
| `B4N1WEB_CHROMIUM` | Path to Chromium executable |
| `B4N1WEB_CHROMIUM_DIR` | Chromium download directory |

```bash
export B4N1WEB_BINARY=/custom/path/b4n1web
export B4N1WEB_CHROMIUM=/usr/bin/chromium
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Invalid arguments |
| `3` | Binary not found |
| `4` | Chromium not found |
| `5` | Timeout |
| `6` | Connection error |

---

## Examples

### Scrape a blog post

```bash
b4n1web goto https://blog.example.com/post --output post.md
```

### Screenshot a dashboard

```bash
b4n1web screenshot --url https://dashboard.example.com --width 1920 --height 1080 --output dashboard.png
```

### Extract links for crawling

```bash
b4n1web goto https://example.com --mode light | grep -A 100 "Links:" | jq -r '.[]'
```

### Interactive session

```bash
# Start daemon
b4n1web daemon start

# Use MCP tools
# (via MCP client like Claude, Cursor, etc.)

# Stop daemon
b4n1web daemon stop
```

---

## Configuration File

Create `~/.config/b4n1web/config.toml`:

```toml
[default]
mode = "light"
timeout = 30
user_agent = "MyBot/1.0"

[chromium]
path = "/usr/bin/chromium"
download_dir = "~/.b4n1web/chromium"
```

Then run without flags:

```bash
b4n1web goto https://example.com
```