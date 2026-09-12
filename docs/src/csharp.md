# C# .NET SDK

Complete API reference for the C# .NET SDK.

## Installation

```bash
dotnet add package B4n1Web
```

Requires: .NET 6.0+

## Basic Usage

```csharp
using B4N1Web;

// Simple usage
var browser = new AgentBrowser();
var page = browser.Goto("https://example.com");
Console.WriteLine(page.Markdown);
browser.Close();

// Using statement (auto-dispose)
using var browser = new AgentBrowser();
var page = browser.Goto("https://example.com");
Console.WriteLine(page.Markdown);
```

## Browser Options

```csharp
using B4N1Web;

var options = new BrowserOptions
{
    Mode = BrowserMode.Light,      // Light, JS, or Render
    Timeout = 30,                  // seconds
    UserAgent = "MyBot/1.0"        // custom user agent
};

var browser = new AgentBrowser(options);
```

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `Mode` | `BrowserMode` | `Light` | Light, JS, or Render |
| `Timeout` | `int` | `30` | Request timeout in seconds |
| `UserAgent` | `string` | `"B4N1Web-Agent/1.0"` | Custom User-Agent string |

```csharp
var browser = new AgentBrowser(new BrowserOptions
{
    Mode = BrowserMode.Render,
    Timeout = 60,
    UserAgent = "CustomBot/1.0"
});
```

## BrowserMode Enum

```csharp
public enum BrowserMode
{
    Light = 0,
    JS = 1,
    Render = 2
}
```

## Page Object

```csharp
var page = browser.Goto("https://example.com");

// Properties
page.Url           // string - Final URL after redirects
page.Markdown      // string - Clean markdown content
page.Links         // List<string> - All extracted links
page.Screenshot    // string? - Base64 PNG (Render mode only)
page.JsOutput      // string? - JavaScript output (JS/Render mode)
```

### Page Methods

```csharp
// Get main content (skip headers)
var content = page.GetMainContent();

// Find links containing specific text (case-insensitive)
var contactLinks = page.FindLinksByText("contact");
// Returns List<string>
```

## Advanced Features (Render Mode Only)

```csharp
var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Render });

// Navigate with wait
var page = browser.Goto("https://example.com", waitFor: ".content");

// Screenshot
var screenshotB64 = browser.Screenshot(1920, 1080);
// Returns base64 PNG string

// Wait for selector
var found = browser.WaitForSelector(".my-element", 5000);
// Returns true/false

// Click element
browser.Click(".submit-button");

// Type text
browser.TypeText("#search", "query", true);  // clearFirst = true

// Get links from last page
var links = browser.GetLinks();
// Returns string[]

// Execute JavaScript
var result = browser.Evaluate("document.title");
// Returns JS output as string
```

## Async API

```csharp
var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light });

// Navigate async
var page = await browser.GotoAsync("https://example.com");
Console.WriteLine(page.Markdown);

// Screenshot async
var screenshot = await browser.ScreenshotAsync(1920, 1080);

// Wait for selector async
var found = await browser.WaitForSelectorAsync(".my-element", 5000);

// Click async
await browser.ClickAsync(".submit-button");

// Type text async
await browser.TypeTextAsync("#search", "query", true);

// Evaluate JavaScript async
var result = await browser.EvaluateAsync("document.title");
```

## Static Methods

```csharp
using B4N1Web;

// Get version without creating browser
var version = AgentBrowser.GetVersion();

// Get links from URL without creating browser instance
var links = AgentBrowser.GetLinksFromPage("https://example.com");
// Returns string[]
```

## Environment Variables

```bash
# Custom binary path
export B4N1WEB_BINARY=/custom/path/b4n1web

# Custom Chromium path
export B4N1WEB_CHROMIUM=/usr/bin/chromium
```

## Error Handling

```csharp
using B4N1Web;

try
{
    var browser = new AgentBrowser();
    var page = browser.Goto("https://example.com");
    Console.WriteLine(page.Markdown);
}
catch (BinaryNotFoundException)
{
    Console.WriteLine("Binary not found. Install with: dotnet add package B4n1Web");
}
catch (Exception ex)
{
    Console.WriteLine($"Error: {ex.Message}");
}
finally
{
    browser?.Dispose();
}
```

## BinaryNotFoundException

Thrown when the b4n1web binary cannot be found:

```csharp
try
{
    var browser = new AgentBrowser();
}
catch (BinaryNotFoundException ex)
{
    Console.WriteLine(ex.Message);
    // "B4n1Web binary not found. Please install it first..."
}
```

## Version Compatibility

The SDK checks binary version on startup. If mismatched, a warning is printed to stderr:

```
⚠️  Version mismatch: SDK v0.13.0 requires binary v0.13.0, but found v0.9.4
```

## Dependency Injection

```csharp
// Register in DI container
services.AddSingleton<IAgentBrowser>(provider => 
    new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light })
);

// Use in controller/service
public class MyService
{
    private readonly IAgentBrowser _browser;
    
    public MyService(IAgentBrowser browser) => _browser = browser;
    
    public async Task<string> ScrapeAsync(string url)
    {
        var page = await _browser.GotoAsync(url);
        return page.Markdown;
    }
}
```

## Platform Compatibility

The NuGet package bundles binaries for all 6 platforms:
- Linux x86_64
- Linux ARM64
- macOS x86_64
- macOS ARM64
- Windows x86_64
- Windows ARM64

The correct binary is selected automatically at runtime.

## Async Disposable

```csharp
// Using declaration (C# 8+)
await using var browser = new AgentBrowser();
var page = await browser.GotoAsync("https://example.com");
// Auto-disposes on scope exit
```

## Testing with FakeProcessRunner

```csharp
using B4N1Web.Tests;

// For unit testing
var fake = new FakeProcessRunner();
fake.QueueResult(new ProcessResult { ExitCode = 0, StdOut = "test" });

var browser = new AgentBrowser(new BrowserOptions(), fake);
var page = browser.Goto("https://example.com");
```