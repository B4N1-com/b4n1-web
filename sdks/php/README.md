# b4n1-web PHP SDK

Ultra-lightweight agentic browser for AI agents — PHP SDK wrapping the
`b4n1web` binary. Requires PHP >= 8.1.

## Install

```bash
composer require b4n1/b4n1-web
```

The SDK resolves the browser binary in this order:

1. `B4N1WEB_BIN_PATH` (explicit override)
2. `B4N1WEB_BINARY` + well-known user/system locations (`~/.local/bin`,
   `~/.b4n1web/bin`, `/usr/local/bin`, `/usr/bin`, …)
3. `PATH` scan

If nothing resolves, `Binary::resolveOrThrow()` throws
`B4n1Web\Exception\BinaryNotFoundException` with an actionable install hint
(no silent fallback).

## Usage

```php
<?php

require 'vendor/autoload.php';

use B4n1Web\AgentBrowser;
use B4n1Web\BrowserMode;

$browser = new AgentBrowser(mode: BrowserMode::Light, timeout: 30);
$page = $browser->goto('https://example.com');

echo $page->markdown;            // extracted markdown
print_r($page->links);           // array of links (JSON-parsed)
echo $page->getMainContent();    // main content heuristic
print_r($page->findLinksByText('docs'));

// Execute JavaScript in the page context
$result = $browser->evaluate('https://example.com', 'document.title');

$browser->close();               // no-op (binary is stateless)
```

### Modes

| Mode | Purpose |
|------|---------|
| `BrowserMode::Light` | Fast HTTP fetch, no JS (default) |
| `BrowserMode::Js` | Headless Chromium with JavaScript |
| `BrowserMode::Render` | Full render + wait-for selector |

### CLI wrapper

Composer installs a `b4n1web` executable that passes arguments straight to the
resolved binary:

```bash
vendor/bin/b4n1web goto https://example.com --mode light
```

### Constructor options

- `mode:` — browser mode (default `light`)
- `timeout:` — seconds before killing the binary (default `30`)
- `binary:` — explicit binary path (skips auto-resolution)
- `runner:` — injectable process runner (tests only)

## Tests

```bash
composer test          # or: php tests/run_tests.php
```

Offline unit tests (no network, no phpunit dependency) plus a real-binary
smoke test when a binary resolves locally.

## License

BUSL-1.1 (see `LICENSE`). Binary use is subject to `EULA.txt`.
