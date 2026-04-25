# B4n1Web .NET SDK

<p align="center">
  <a href="https://www.nuget.org/packages/B4n1Web"><img src="https://img.shields.io/nuget/v/B4n1Web.svg" alt="NuGet version"></a>
  <a href="https://www.nuget.org/packages/B4n1Web"><img src="https://img.shields.io/nuget/dt/B4n1Web.svg" alt="NuGet downloads"></a>
  <a href="https://github.com/B4N1-com/b4n1-web"><img src="https://img.shields.io/github/stars/B4N1-com/b4n1-web?style=flat" alt="GitHub stars"></a>
</p>

.NET/C# bindings for B4n1Web: The Agentic Browser Engine.

## Installation

### 1. Install the B4n1Web Binary

```bash
curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz && ./b4n1web --version
```

### 2. Install the .NET SDK

```bash
dotnet add package B4n1Web
```

## Quick Start

```csharp
using B4N1Web;

// Create a browser instance
var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light });

// Navigate to a page
var page = await browser.GotoAsync("https://example.com");

// Access structured data
Console.WriteLine("Page content: " + page.Markdown);
Console.WriteLine("Found " + page.Links.Count + " links");

// Find specific links
var githubLinks = page.FindLinksByText("github");
Console.WriteLine("GitHub links: " + string.Join(", ", githubLinks));

// Get main content
var mainContent = page.GetMainContent();
Console.WriteLine("Main content: " + mainContent.Substring(0, Math.Min(200, mainContent.Length)) + "...");

// Close browser
browser.Close();
```

## Browser Modes

### Light Mode (Default)

```csharp
var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light });
```

### JS Mode

```csharp
var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.JS });
```

### Render Mode (Coming Soon)

```csharp
var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Render });
```

## Options

```csharp
// Custom timeout
var browser = new AgentBrowser(new BrowserOptions { Timeout = 60 });

// Custom user agent
var browser = new AgentBrowser(new BrowserOptions { UserAgent = "MyAgent/1.0" });
```

## SecurityShield

Optional security validation:

```csharp
var shield = new SecurityShield(cacheDays: 30);
var (isSafe, needsCheck) = shield.IsUrlSafe("https://example.com");

if (!isSafe)
{
    Console.WriteLine("URL is unsafe!");
}
```

## License

MIT
