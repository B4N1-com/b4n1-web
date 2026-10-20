# b4n1-web Ruby SDK

Ultra-lightweight agentic browser for AI agents — Ruby gem wrapping the
`b4n1web` binary.

## Install

```bash
gem install b4n1-web
```

The gem resolves the browser binary in this order:

1. `B4N1WEB_BIN_PATH` (explicit override)
2. bundled binary inside the gem (when shipped)
3. well-known user/system locations (`~/.local/bin`, `~/.b4n1web/bin`, …)
4. `PATH`

If nothing resolves, it fails loudly (no silent fallback) with an actionable
install hint.

## Usage

```ruby
require "b4n1web"

browser = B4n1Web::AgentBrowser.new(mode: B4n1Web::BrowserMode::LIGHT)
page = browser.goto("https://example.com")

puts page.markdown          # extracted markdown
puts page.links             # array of links (JSON-parsed)
puts page.get_main_content  # main content heuristic
puts page.find_links_by_text("docs")
browser.close               # no-op (binary is stateless)

# Execute JavaScript in the page context
result = browser.evaluate("https://example.com", "document.title")
```

### Modes

| Mode | Purpose |
|------|---------|
| `B4n1Web::BrowserMode::LIGHT` | Fast HTTP fetch, no JS (default) |
| `B4n1Web::BrowserMode::JS` | Headless Chromium with JavaScript |
| `B4n1Web::BrowserMode::RENDER` | Full render + wait-for selector |

### CLI wrapper

The gem installs a `b4n1web` executable that passes arguments straight to the
resolved binary:

```bash
b4n1web goto https://example.com --mode light
```

### Options

- `mode:` — browser mode (default `light`)
- `timeout:` — seconds before killing the binary (default `30`)
- `binary:` — explicit binary path (skips auto-resolution)
- `runner:` — injectable process runner (tests only)

## License

BUSL-1.1 (see `LICENSE`). Binary use is subject to `EULA.txt`.
