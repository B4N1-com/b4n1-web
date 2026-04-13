# B4n1Web JavaScript/TypeScript SDK

<p align="center">
  <a href="https://www.npmjs.com/package/b4n1-web"><img src="https://img.shields.io/npm/v/b4n1-web.svg" alt="NPM version"></a>
  <a href="https://www.npmjs.com/package/b4n1-web"><img src="https://img.shields.io/npm/dt/b4n1-web.svg" alt="NPM downloads"></a>
  <a href="https://github.com/B4N1-com/b4n1-web"><img src="https://img.shields.io/github/stars/B4N1-com/b4n1-web?style=flat" alt="GitHub stars"></a>
</p>

TypeScript/JavaScript bindings for B4n1Web: The Agentic Browser Engine.

## Installation

### 1. Install the B4n1Web Binary

```bash
curl -sL https://web.b4n1.com/install | bash
```

### 2. Install the JavaScript SDK

```bash
npm install b4n1-web
```

Or with yarn:

```bash
yarn add b4n1-web
```

Or with pnpm:

```bash
pnpm add b4n1-web
```

## Quick Start

```typescript
import { AgentBrowser, BrowserMode, Page } from 'b4n1-web';

// Create a browser instance
const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });

// Navigate to a page
const page = await browser.goto('https://example.com');

// Access structured data
console.log('Page content:', page.markdown);
console.log('Found', page.links.length, 'links');

// Extract main content
const mainContent = page.getMainContent();
console.log('Main content:', mainContent.substring(0, 200) + '...');

// Find specific links
const githubLinks = page.findLinksByText('github');
console.log('GitHub links:', githubLinks);

// Close browser when done
browser.close();
```

## Browser Modes

### Light Mode (Default)

- **Use case**: Reading articles, scraping static content, extracting links
- **Performance**: < 15MB RAM, instant startup
- **Capabilities**: HTML parsing, markdown conversion, link extraction
- **Limitations**: No JavaScript execution

```typescript
const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
```

### JS Mode

- **Use case**: Extracting JavaScript from pages, SPA analysis
- **Performance**: Same as Light, instant startup
- **Capabilities**: HTML parsing + JavaScript tag extraction

```typescript
const browser = new AgentBrowser({ mode: BrowserMode.JS });
```

### Render Mode (Coming Soon)

- **Use case**: SPAs, form filling, visual verification, E2E testing
- **Status**: Coming in v0.3.0

## API Reference

### AgentBrowser

Main browser class for web automation.

#### Constructor Options

- `mode`: BrowserMode.LIGHT, BrowserMode.JS, or BrowserMode.RENDER
- `timeout`: Request timeout in seconds (default: 30)
- `userAgent`: Custom user agent string

#### Methods

- `goto(url: string)`: Navigate to URL and return structured page data
- `close()`: Close the browser session

### Page

Structured data from a web page.

#### Properties

- `url: string`: The page URL
- `markdown: string`: Clean markdown content
- `links: string[]`: Extracted links
- `screenshot?: string`: Base64-encoded screenshot (render mode only)

#### Methods

- `getMainContent()`: Extract main content, skipping headers/footers
- `findLinksByText(text: string)`: Find links containing specific text

## SecurityShield

Optional security validation with caching:

```typescript
import { SecurityShield, navigate } from 'b4n1-web';

// With security check
const result = await navigate('https://example.com');

// With custom shield
const shield = new SecurityShield({ cacheDays: 30 });
const { isSafe, needsApiCheck } = shield.isUrlSafe('https://example.com');
```

## TypeScript

This package includes TypeScript type definitions. No additional installation needed.

## License

MIT
