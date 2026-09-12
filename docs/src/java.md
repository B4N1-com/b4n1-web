# Java SDK

Complete API reference for the Java SDK.

## Installation

### Maven

```xml
<dependency>
    <groupId>com.b4n1</groupId>
    <artifactId>b4n1-web</artifactId>
    <version>0.13.0</version>
</dependency>
```

### Gradle

```groovy
implementation 'com.b4n1:b4n1-web:0.13.0'
```

Requires: Java 11+

## Basic Usage

```java
import com.b4n1.AgentBrowser;
import com.b4n1.Page;
import com.b4n1.BrowserOptions;
import com.b4n1.BrowserMode;

// Simple usage
AgentBrowser browser = new AgentBrowser();
Page page = browser.goto("https://example.com");
System.out.println(page.getMarkdown());
browser.close();

// Try-with-resources (auto-close)
try (AgentBrowser browser = new AgentBrowser()) {
    Page page = browser.goto("https://example.com");
    System.out.println(page.getMarkdown());
}
```

## Browser Options

```java
import com.b4n1.AgentBrowser;
import com.b4n1.BrowserOptions;
import com.b4n1.BrowserMode;

BrowserOptions options = new BrowserOptions()
    .setMode(BrowserMode.LIGHT)      // LIGHT, JS, or RENDER
    .setTimeout(30)                  // seconds
    .setUserAgent("MyBot/1.0");      // custom user agent

AgentBrowser browser = new AgentBrowser(options);
```

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `mode` | `BrowserMode` | `LIGHT` | LIGHT, JS, or RENDER |
| `timeout` | `int` | `30` | Request timeout in seconds |
| `userAgent` | `String` | `"B4N1Web-Agent/1.0"` | Custom User-Agent string |

```java
AgentBrowser browser = new AgentBrowser(
    new BrowserOptions()
        .setMode(BrowserMode.RENDER)
        .setTimeout(60)
        .setUserAgent("CustomBot/1.0")
);
```

## BrowserMode Enum

```java
public enum BrowserMode {
    LIGHT,
    JS,
    RENDER
}
```

## Page Object

```java
Page page = browser.goto("https://example.com");

// Properties
page.getUrl()           // String - Final URL after redirects
page.getMarkdown()      // String - Clean markdown content
page.getLinks()         // List<String> - All extracted links
page.getScreenshot()    // String - Base64 PNG (Render mode only, or null)
page.getJsOutput()      // String - JavaScript output (JS/Render mode, or null)
```

### Page Methods

```java
// Get main content (skip headers)
String content = page.getMainContent();

// Find links containing specific text (case-insensitive)
List<String> contactLinks = page.findLinksByText("contact");
// Returns List<String>
```

## Advanced Features (Render Mode Only)

```java
AgentBrowser browser = new AgentBrowser(
    new BrowserOptions().setMode(BrowserMode.RENDER)
);

// Navigate with wait
Page page = browser.goto("https://example.com", ".content");

// Screenshot
String screenshotB64 = browser.screenshot(1920, 1080);
// Returns base64 PNG string

// Wait for selector
boolean found = browser.waitForSelector(".my-element", 5000);
// Returns true/false

// Click element
browser.click(".submit-button");

// Type text
browser.typeText("#search", "query", true);  // clearFirst = true

// Get links from last page
String[] links = browser.getLinks();
// Returns String[]

// Execute JavaScript
String result = browser.evaluate("document.title");
// Returns JS output as String
```

## Static Methods

```java
import com.b4n1.AgentBrowser;

// Get version without creating browser
String version = AgentBrowser.getVersion();

// Get links from URL without creating browser instance
String[] links = AgentBrowser.getLinksFromPage("https://example.com");
// Returns String[]
```

## Environment Variables

```bash
# Custom binary path
export B4N1WEB_BINARY=/custom/path/b4n1web

# Custom Chromium path
export B4N1WEB_CHROMIUM=/usr/bin/chromium
```

## Error Handling

```java
import com.b4n1.AgentBrowser;
import com.b4n1.BinaryNotFoundException;

try {
    AgentBrowser browser = new AgentBrowser();
    Page page = browser.goto("https://example.com");
    System.out.println(page.getMarkdown());
} catch (BinaryNotFoundException e) {
    System.err.println("Binary not found: " + e.getMessage());
    // "B4n1Web binary not found. Please install it first..."
} catch (Exception e) {
    System.err.println("Error: " + e.getMessage());
}
```

## BinaryNotFoundException

Thrown when the b4n1web binary cannot be found:

```java
try {
    AgentBrowser browser = new AgentBrowser();
} catch (BinaryNotFoundException e) {
    System.err.println(e.getMessage());
    // "B4n1Web binary not found. Please install it first..."
}
```

## Version Compatibility

The SDK checks binary version on startup. If mismatched, a warning is printed to stderr:

```
⚠️  Version mismatch: SDK v0.13.0 requires binary v0.13.0, but found v0.9.4
```

## Dependency Injection (Spring)

```java
@Configuration
public class BrowserConfig {
    
    @Bean
    public AgentBrowser agentBrowser() {
        return new AgentBrowser(
            new BrowserOptions().setMode(BrowserMode.LIGHT)
        );
    }
}

@Service
public class ScraperService {
    
    private final AgentBrowser browser;
    
    public ScraperService(AgentBrowser browser) {
        this.browser = browser;
    }
    
    public String scrape(String url) {
        Page page = browser.goto(url);
        return page.getMarkdown();
    }
}
```

## Platform Compatibility

The JAR bundles native binaries for all 6 platforms:
- Linux x86_64
- Linux ARM64
- macOS x86_64
- macOS ARM64
- Windows x86_64
- Windows ARM64

The correct binary is selected automatically at runtime via the `native/` directory structure.

## AutoCloseable

```java
// Try-with-resources (Java 7+)
try (AgentBrowser browser = new AgentBrowser()) {
    Page page = browser.goto("https://example.com");
    System.out.println(page.getMarkdown());
} // browser.close() called automatically
```

## Maven Packaging

The SDK is published as a single JAR with all native binaries included. No additional dependencies needed.

```xml
<dependency>
    <groupId>com.b4n1</groupId>
    <artifactId>b4n1-web</artifactId>
    <version>0.13.0</version>
</dependency>
```

## Testing

```java
// The SDK uses ProcessBuilder internally
// For testing, you can mock the binary behavior
// or use the real binary in integration tests
```

## Version Information

```java
import com.b4n1.AgentBrowser;

String sdkVersion = AgentBrowser.getVersion();
// Returns binary version or "unknown" if not found
```