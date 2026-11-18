# B4n1Web — JavaScript / TypeScript SDK

[![npm version](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![License](https://img.shields.io/badge/License-BUSL--1.1-blue.svg)](LICENSE)

> Ultra-lightweight agentic browser engine with the native binary bundled.
> Navigate URLs, extract structured content (markdown, links, screenshots) and
> build agent workflows from Node.js 18+.

📖 **Full documentation**: https://B4N1-com.github.io/b4n1-web/book/

## Install

```bash
npm install b4n1-web
```

The package carries the `b4n1web` binary for `linux-x64`, `linux-arm64`,
`linux-ia32`, `macos-x64`, `macos-arm64` and `windows-x64`; the right one is
selected at runtime. No compiler, no separate binary install.

Binary resolution order:

1. `B4N1WEB_BIN_PATH` (explicit override)
2. bundled binary inside the package
3. `B4N1WEB_BINARY` + well-known locations (`~/.local/bin`, `~/.b4n1web/bin`,
   `/usr/local/bin`, `/usr/bin`, …)
4. `PATH`

If nothing resolves, the SDK throws `BinaryNotFoundError` with an actionable
install hint (no silent fallback).

## Quick Start

```ts
import { AgentBrowser, BrowserMode } from 'b4n1-web';

const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
const page = await browser.goto('https://example.com');

console.log(page.url);
console.log(page.markdown);       // page content as markdown
console.log(page.links);          // string[] of every link on the page
browser.close();
```

CommonJS works too:

```js
const { AgentBrowser, BrowserMode } = require('b4n1-web');
```

## Page Object

| Member | Type | Description |
|---|---|---|
| `page.url` | `string` | final URL after redirects |
| `page.markdown` | `string` | page content as markdown |
| `page.links` | `string[]` | all links on the page |
| `page.screenshot` | `string \| undefined` | base64 screenshot (render mode) |
| `page.jsOutput` | `string \| undefined` | extracted script output (js mode) |
| `page.getMainContent()` | `string` | content without headers/footers |
| `page.findLinksByText(text)` | `string[]` | links containing `text` |

## Links

```ts
// Links from the last visited page
const page = await browser.goto('https://example.com');
console.log(browser.getLinks());              // string[]: all links
console.log(page.findLinksByText('more'));    // string[]: matching links

// One-shot fetch from a URL, without a browser instance
import { getLinksFromPage } from 'b4n1-web';
const links = await getLinksFromPage('https://example.com');
```

## Interactions

```ts
const browser = new AgentBrowser({ mode: BrowserMode.RENDER });
await browser.goto('https://example.com');

await browser.waitForSelector('h1', 5000);
await browser.click('a.more');
await browser.typeText('input[name=q]', 'b4n1', true);
const shot = await browser.screenshot('https://example.com', true);
await browser.setViewport(1280, 800);
await browser.emulateDevice('iPhone 13');
```

## Browser Modes

| Mode | Description | RAM | Startup |
|------|-------------|-----|---------|
| `BrowserMode.LIGHT` | HTTP fetch + HTML parsing | ~15 MB | instant |
| `BrowserMode.JS` | Light + JavaScript extraction | ~15 MB | instant |
| `BrowserMode.RENDER` | Full Chromium + screenshots | ~100 MB | ~2 s |

## Security

```ts
import { SecurityShield } from 'b4n1-web';

const shield = new SecurityShield();
shield.markDomain('evil.com', false);
const { isSafe, needsApiCheck } = shield.isUrlSafe('https://evil.com/page');
```

## Error Handling

```ts
import {
  BinaryNotFoundError,
  NavigationError,
  SelectorTimeoutError,
} from 'b4n1-web';

try {
  const page = await browser.goto('https://example.com', 'main');
} catch (err) {
  if (err instanceof SelectorTimeoutError) console.error('selector never appeared');
}
```

## MCP Server

The package ships no CLI shim: run the bundled binary directly (this is the
path `getB4n1webBinary()` returns).

```ts
import { getB4n1webBinary } from 'b4n1-web';
import { spawn } from 'node:child_process';

const bin = getB4n1webBinary();
if (!bin) throw new Error('b4n1web binary not found');
spawn(bin, ['mcp'], { stdio: 'inherit' });
```

## Version

SDK version: **0.16.3**
Bundled binary version: **0.16.3**

## Links

- [npm](https://www.npmjs.com/package/b4n1-web)
- [Documentation](https://B4N1-com.github.io/b4n1-web/book/)
- [GitHub](https://github.com/B4N1-com/b4n1-web)
- [Website](https://b4n1.com)

---
*Built by B4N1 with ❤️ · All rights reserved.*
