# b4n1-web Go SDK

Ultra-lightweight agentic browser for AI agents — Go SDK wrapping the
`b4n1web` binary.

## Install

```bash
go get github.com/B4N1-com/b4n1-web/sdks/go
```

The SDK resolves the browser binary in this order:

1. explicit `Options.Binary` (taken as-is; a missing file surfaces as a loud
   spawn error at run time)
2. `B4N1WEB_BIN_PATH` (must exist + be executable)
3. `B4N1WEB_BINARY` + well-known user/system locations (`~/.local/bin`,
   `~/.b4n1web/bin`, `/usr/local/bin`, `/usr/bin`)
4. `PATH` scan

If nothing resolves, `New` returns `ErrBinaryNotFound` with an actionable
install hint (no silent fallback).

## Usage

```go
package main

import (
	"context"
	"fmt"
	"log"

	b4n1web "github.com/B4N1-com/b4n1-web/sdks/go"
)

func main() {
	browser, err := b4n1web.New(b4n1web.Options{Mode: b4n1web.ModeLight})
	if err != nil {
		log.Fatal(err)
	}
	defer browser.Close() // no-op (binary is stateless)

	page, err := browser.Goto(context.Background(), "https://example.com", "")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(page.Markdown)          // extracted markdown
	fmt.Println(page.Links)             // links (JSON-parsed)
	fmt.Println(page.GetMainContent())  // main content heuristic
	fmt.Println(page.FindLinksByText("docs"))

	// Execute JavaScript in the page context
	result, err := browser.Evaluate(context.Background(), "https://example.com", "document.title")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(result)
}
```

### Modes

| Mode | Purpose |
|------|---------|
| `ModeLight` | Fast HTTP fetch, no JS (default) |
| `ModeJs` | Headless Chromium with JavaScript |
| `ModeRender` | Full render + wait-for selector |

### Options

- `Mode` — browser mode (default `ModeLight`)
- `Timeout` — hard timeout before killing the binary **and its process
  group** (default `30s`; on expiry `Goto`/`Evaluate` return `ErrTimeout`)
- `Binary` — explicit binary path (skips auto-resolution)
- `Runner` — injectable process runner (tests only)

### Errors

All failures are typed and loud (fail-loud contract, no fallbacks):

- `ErrBinaryNotFound` — nothing resolved
- `ErrTimeout` — binary (process group) killed after `Options.Timeout`
- otherwise a `b4n1web exited <code>: <stderr>` error

## Tests

```bash
go test ./... -cover
```

Offline unit tests (fake runner, no network) + real-runner process tests +
a real-binary smoke test that runs when a binary resolves locally.
Measured statement coverage: **100%**.

## License

BUSL-1.1 (see `LICENSE`). Binary use is subject to `EULA.txt`.
