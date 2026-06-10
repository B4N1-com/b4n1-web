# B4n1Web JavaScript/TypeScript SDK

[![npm version](https://badge.fury.io/js/b4n1-web.svg)](https://www.npmjs.com/package/b4n1-web)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Node.js 18+](https://img.shields.io/badge/node-%3E%3D18-blue.svg)](https://nodejs.org/)

> Ultra-lightweight agentic browser engine with bundled binary.

## Install

```bash
npm install b4n1-web
```

Binary is bundled — no separate installation needed.

## Quick Start

```typescript
import { AgentBrowser, BrowserMode } from 'b4n1-web';

const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
const page = await browser.goto('https://example.com');
console.log(page.markdown);
console.log(`${page.links.length} links found`);
browser.close();
```

## Page API

```typescript
page.url                    // string
page.markdown               // string
page.links                  // string[]
page.screenshot             // string | undefined

page.getMainContent()       // string: content without headers
page.findLinksByText("more") // string[]: matching links
```

## Browser Modes

| Mode | Description |
|------|-------------|
| `BrowserMode.LIGHT` | HTTP + HTML parsing |
| `BrowserMode.JS` | Light + script extraction |
| `BrowserMode.RENDER` | Full Chromium + screenshots |

## Security

```typescript
import { SecurityShield } from 'b4n1-web';

const shield = new SecurityShield();
shield.markDomain('evil.com', false);
const { isSafe, needsApiCheck } = shield.isUrlSafe('https://evil.com');
```

## Error Handling

```typescript
import { BinaryNotFoundError } from 'b4n1-web';

try {
  const browser = new AgentBrowser();
} catch (e) {
  if (e instanceof BinaryNotFoundError) {
    console.error('Install binary: curl -sL https://web.b4n1.com/install | bash');
  }
}
```

## Context Manager

```typescript
import { AgentBrowser } from 'b4n1-web';

async function main() {
  const browser = new AgentBrowser();
  try {
    const page = await browser.goto('https://example.com');
    console.log(page.markdown);
  } finally {
    browser.close();
  }
}

// Or with async dispose:
const browser = new AgentBrowser();
try {
  await browser.goto('https://example.com');
} finally {
  await browser[Symbol.asyncDispose]();
}
```

## Version

SDK: **0.4.0** | Binary: **0.4.0** (bundled)

## Links

- [npm](https://www.npmjs.com/package/b4n1-web)
- [GitHub](https://github.com/B4N1-com/b4n1-web)
- [Website](https://web.b4n1.com)

---
*Built by B4N1 with ❤️ · All rights reserved.*
