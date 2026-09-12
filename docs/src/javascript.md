# JavaScript/TypeScript SDK

Complete API reference for the JavaScript/TypeScript SDK.

## Installation

```bash
npm install b4n1-web
# or
pnpm add b4n1-web
# or
yarn add b4n1-web
```

## Basic Usage

```javascript
import { AgentBrowser, BrowserMode } from 'b4n1-web';

// Simple usage
const browser = new AgentBrowser();
const page = await browser.gotoAsync('https://example.com');
console.log(page.markdown);
await browser.close();

// Async context manager
await using browser = new AgentBrowser();
const page = await browser.gotoAsync('https://example.com');
console.log(page.markdown);
```

## Browser Options

```javascript
import { AgentBrowser, BrowserMode, BrowserOptions } from 'b4n1-web';

const options: BrowserOptions = {
    mode: BrowserMode.LIGHT,      // LIGHT, JS, or RENDER
    timeout: 30,                  // seconds
    userAgent: 'MyBot/1.0'        // custom user agent
};

const browser = new AgentBrowser(options);
```

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `mode` | `BrowserMode` | `LIGHT` | LIGHT, JS, or RENDER |
| `timeout` | `number` | `30` | Request timeout in seconds |
| `userAgent` | `string` | `"B4N1Web-Agent/1.0"` | Custom User-Agent string |

```javascript
import { BrowserMode } from 'b4n1-web';

const browser = new AgentBrowser({
    mode: BrowserMode.RENDER,
    timeout: 60,
    userAgent: 'CustomBot/1.0'
});
```

## Page Object

```javascript
const page = await browser.gotoAsync('https://example.com');

// Properties
page.url           // string - Final URL after redirects
page.markdown      // string - Clean markdown content
page.links         // string[] - All extracted links
page.screenshot    // string | null - Base64 PNG (Render mode only)
page.jsOutput      // string | null - JavaScript output (JS/Render mode)
```

### Page Methods

```javascript
// Get main content (skip headers)
const content = page.getMainContent();

// Find links containing specific text (case-insensitive)
const contactLinks = page.findLinksByText('contact');
```

## Advanced Features (Render Mode Only)

```javascript
const browser = new AgentBrowser({ mode: BrowserMode.RENDER });

// Navigate with wait
const page = await browser.gotoAsync('https://example.com', { waitFor: '.content' });

// Screenshot
const screenshotB64 = await browser.screenshot(1920, 1080);
// Returns base64 PNG string

// Wait for selector
const found = await browser.waitForSelector('.my-element', 5000);
// Returns true/false

// Click element
await browser.click('.submit-button');

// Type text
await browser.typeText('#search', 'query', true); // clearFirst = true

// Get links from last page
const links = browser.getLinks();

// Execute JavaScript
const result = await browser.evaluate('document.title');
// Returns JS output as string
```

## Synchronous API

```javascript
import { AgentBrowser, BrowserMode } from 'b4n1-web';

const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
const page = browser.goto('https://example.com');  // Blocking
console.log(page.markdown);
browser.close();
```

## Environment Variables

```bash
# Custom binary path
export B4N1WEB_BINARY=/custom/path/b4n1web

# Custom Chromium path
export B4N1WEB_CHROMIUM=/usr/bin/chromium
```

## TypeScript Support

Full TypeScript definitions included. Works out of the box with VS Code, tsc, etc.

```typescript
import { AgentBrowser, BrowserOptions, BrowserMode, Page } from 'b4n1-web';

interface MyPageData {
    url: string;
    markdown: string;
    links: string[];
}

async function extractData(url: string): Promise<MyPageData> {
    const options: BrowserOptions = {
        mode: BrowserMode.LIGHT,
        timeout: 30
    };
    
    const browser = new AgentBrowser(options);
    const page: Page = await browser.gotoAsync(url);
    
    return {
        url: page.url,
        markdown: page.markdown,
        links: page.links
    };
}
```

## Version Compatibility

The SDK checks binary version on startup. If mismatched, a warning is printed to stderr:

```
⚠️  Version mismatch: SDK v0.13.0 requires binary v0.13.0, but found v0.9.4
```

## Error Handling

```javascript
import { AgentBrowser, BrowserMode } from 'b4n1-web';

try {
    const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
    const page = await browser.gotoAsync('https://example.com');
    console.log(page.markdown);
} catch (error) {
    if (error.message.includes('not found')) {
        console.error('Binary not found. Install with: npm install b4n1-web');
    } else {
        console.error('Error:', error);
    }
} finally {
    await browser.close();
}
```

## Static Methods

```javascript
import { AgentBrowser } from 'b4n1-web';

// Get version without creating browser
const version = AgentBrowser.getVersion();

// Get links from URL without creating browser instance
const links = await AgentBrowser.getLinksFromPage('https://example.com');
```

## Vite/Next.js/Framework Integration

Works with all modern frameworks. Just import and use:

```javascript
// Next.js API route
export async function GET() {
    const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
    const page = await browser.gotoAsync('https://example.com');
    return Response.json({ markdown: page.markdown });
}
```

## Async Disposable (using declaration)

```javascript
// Automatic cleanup with using (Node 20+ / TypeScript 5.2+)
await using browser = new AgentBrowser({ mode: BrowserMode.RENDER });
const page = await browser.gotoAsync('https://example.com');
const screenshot = await browser.screenshot(1920, 1080);
// browser.close() called automatically
```