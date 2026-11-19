# Browser Modes

B4n1Web offers three browser modes, each with different capabilities and resource usage.

## Mode Comparison

| Feature | Light | JS | Render |
|---------|-------|-----|--------|
| **HTML Parsing** | ✅ | ✅ | ✅ |
| **JavaScript Extraction** | ❌ | ✅ | ✅ |
| **Screenshots** | ❌ | ❌ | ✅ |
| **Click/Type/Wait** | ❌ | ❌ | ✅ |
| **Full Chromium** | ❌ | ❌ | ✅ |
| **RAM Usage** | ~15MB | ~15MB | ~100MB |
| **Startup Time** | Instant | Instant | ~2s |
| **Binary Size** | ~15MB | ~15MB | ~15MB |

---

## Light Mode

**Best for:** Fast scraping, SEO content, static sites, APIs returning HTML.

```python
browser = AgentBrowser()  # Default is Light
page = browser.goto("https://example.com")
```

**What it does:**
- Fetches HTML via HTTP
- Parses HTML to extract text, links, meta tags
- Converts to clean markdown
- No JavaScript execution

**Use cases:**
- Blog posts, articles, documentation
- Product listings, directories
- Any content rendered server-side

---

## JS Mode

**Best for:** Sites that load content via JavaScript but don't need screenshots.

```python
from b4n1web import AgentBrowser, BrowserMode

browser = AgentBrowser(mode=BrowserMode.JS)
page = browser.goto("https://example.com")
```

**What it does:**
- Same as Light mode
- PLUS: Extracts data from JavaScript variables
- Can evaluate JavaScript expressions
- Returns `page.js_output` with JS results

**Use cases:**
- Single Page Applications (SPAs)
- Sites with JSON data embedded in scripts
- Dynamic content loaded via fetch/XHR

---

## Render Mode

**Best for:** Screenshots, clicking, typing, waiting, full browser automation.

```python
from b4n1web import AgentBrowser, BrowserMode

browser = AgentBrowser(mode=BrowserMode.RENDER)
page = browser.goto("https://example.com", wait_for=".content")

# Screenshot
screenshot_b64 = browser.screenshot(1920, 1080)

# Interact
browser.click(".button")
browser.type_text("#search", "query")
browser.wait_for_selector(".results", 5000)
```

**What it does:**
- Full Chromium browser (headless)
- Screenshots (full page or viewport)
- Click, type, hover, scroll
- Wait for selectors
- Full JavaScript execution
- All MCP tools available

**Use cases:**
- Visual regression testing
- Form automation
- Complex SPAs
- Screenshots for reports
- Any interaction requiring real browser

---

## Choosing the Right Mode

```
START: Do you need screenshots or interaction (click/type/wait)?
  YES → RENDER MODE
  NO  → Does the site load content via JavaScript?
    YES → JS MODE
    NO  → LIGHT MODE
```

---

## Mode Switching

```python
# Change mode per request
browser = AgentBrowser(mode=BrowserMode.LIGHT)

# Light request
page = browser.goto("https://example.com")

# Switch to Render for screenshot
browser2 = AgentBrowser(mode=BrowserMode.RENDER)
page2 = browser2.goto("https://example.com")
screenshot = browser2.screenshot(1920, 1080)
```

**Note:** Each `AgentBrowser` instance has one mode. Create separate instances for different modes.

---

## Performance Tips

| Mode | Optimize For |
|------|--------------|
| Light | Use for 90% of scraping tasks |
| JS | Use when Light misses content |
| Render | Use only when screenshots/interaction needed |

---

## Memory Comparison

```
Light:     ████░░░░░░░░░░░░░░░░░░░░  ~15 MB
JS:        ████░░░░░░░░░░░░░░░░░░░░  ~15 MB
Render:    ████████████░░░░░░░░░░░░  ~100 MB
```

---

## Quick Reference

| Need | Mode |
|------|------|
| "Give me the article text" | Light |
| "Extract JSON from page script" | JS |
| "Screenshot this dashboard" | Render |
| "Click login, type password" | Render |
| "Wait for infinite scroll" | Render |
| "Get all links from blog" | Light |
| "SPA with React/Vue" | JS or Render |