# FAQ

## General

### What is b4n1web?

An ultra-lightweight headless browser engine with a single Rust binary and 4 language SDKs (Python, JavaScript/TypeScript, Java, C#). Designed for AI agents and web scraping.

### How is it different from Selenium/Playwright/Puppeteer?

| Feature | Selenium | Playwright | b4n1web |
|---------|----------|------------|---------|
| Binary size | ~200MB+ | ~200MB+ | ~9MB |
| Install | Multiple deps | Multiple deps | `pip install` |
| Startup | ~5s | ~3s | Instant |
| Languages | Many | Many | 4 SDKs |
| AI Agent ready | ❌ | ❌ | ✅ MCP |

### Which mode should I use?

```
START: Do you need screenshots or interaction (click/type/wait)?
  YES → RENDER MODE
  NO  → Does the site load content via JavaScript?
    YES → JS MODE
    NO  → LIGHT MODE
```

---

## Installation

### "Binary not found" error

The SDK bundles binaries for all 6 platforms. If you see this:
1. Reinstall: `pip install --upgrade b4n1-web`
2. Or manually: `curl -sL https://b4n1.com/install | bash`

### Python: "No module named 'b4n1web'"

```bash
pip install b4n1-web
# Note: package name has hyphen, import uses underscore
from b4n1web import AgentBrowser
```

### npm: "Cannot find module 'b4n1-web'"

```bash
npm install b4n1-web
# or
pnpm add b4n1-web
```

---

## Runtime Issues

### "Chromium not found" (Render mode)

Render mode needs Chromium. It auto-downloads (~160MB) on first use.

**Solutions:**
1. Wait for auto-download (requires internet)
2. Use system Chromium:
   ```bash
   export B4N1WEB_CHROMIUM=/usr/bin/chromium
   ```
3. Linux ARM64 (Raspberry Pi):
   ```bash
   sudo apt install chromium-browser
   export B4N1WEB_CHROMIUM=/usr/bin/chromium
   ```

### "Permission denied" on binary

```bash
chmod +x ~/.local/bin/b4n1web
```

### "Version mismatch" warning

```
⚠️ Version mismatch: SDK v0.13.0 requires binary v0.13.0, but found v0.9.4
```

Update both:
```bash
pip install --upgrade b4n1-web
curl -sL https://b4n1.com/install | bash
```

---

## SDK-Specific

### Python: Async vs Sync

```python
# Sync (blocking)
browser = AgentBrowser()
page = browser.goto("https://example.com")

# Async (non-blocking)
browser = AgentBrowser()
page = await browser.goto_async("https://example.com")
```

### JavaScript: CommonJS vs ESM

```javascript
// ESM (recommended)
import { AgentBrowser } from 'b4n1-web';

// CommonJS
const { AgentBrowser } = require('b4n1-web');
```

### C#: Dependency Injection

```csharp
services.AddSingleton<IAgentBrowser>(provider => 
    new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light }));
```

### Java: Spring Boot

```java
@Bean
public AgentBrowser agentBrowser() {
    return new AgentBrowser(new BrowserOptions().setMode(BrowserMode.LIGHT));
}
```

---

## Platform Support

| Platform | Binary | Chromium |
|----------|--------|----------|
| Linux x86_64 | ✅ | ✅ |
| Linux ARM64 | ✅ | ✅ |
| macOS Intel | ✅ | ✅ |
| macOS Apple Silicon | ✅ | ✅ |
| Windows x86_64 | ✅ | ✅ |
| Windows ARM64 | ✅ | ✅ |

---

## MCP Server

### "Connection refused"

```bash
# Start daemon first
b4n1web daemon start
b4n1web mcp
```

### "Tool not found"

Update to latest version:
```bash
curl -sL https://b4n1.com/install | bash
```

---

## Performance

| Mode | RAM | Startup | Use Case |
|------|-----|---------|----------|
| Light | ~15MB | Instant | 90% of scraping |
| JS | ~15MB | Instant | SPA content |
| Render | ~100MB | ~2s | Screenshots, interaction |

---

## Still Stuck?

1. Check [GitHub Issues](https://github.com/B4N1-com/b4n1-web/issues)
2. Search existing issues
3. Create new issue with:
   - OS + version
   - `b4n1web --version`
   - Full error message
   - Minimal reproduction code