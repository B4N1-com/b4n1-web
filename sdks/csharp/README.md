# B4n1Web C# SDK

[![NuGet](https://img.shields.io/nuget/v/B4n1Web.svg)](https://www.nuget.org/packages/B4n1Web)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> Ultra-lightweight agentic browser engine with bundled binary.

## Install

```bash
dotnet add package B4n1Web
```

Binary is bundled — no separate installation needed.

## Quick Start

```csharp
using B4N1Web;

var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light });
var page = browser.Goto("https://example.com");
Console.WriteLine(page.Markdown);
Console.WriteLine($"{page.Links.Count} links found");
Console.WriteLine(page.GetMainContent());
Console.WriteLine(string.Join(", ", page.FindLinksByText("github")));
browser.Close();
```

## Async

```csharp
using B4N1Web;

var browser = new AgentBrowser();
var page = await browser.GotoAsync("https://example.com");
Console.WriteLine(page.Markdown);
browser.Close();
```

## Browser Modes

```csharp
new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light });  // default
new AgentBrowser(new BrowserOptions { Mode = BrowserMode.JS });
new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Render });
```

## Security

```csharp
var shield = new SecurityShield();
shield.MarkDomain("evil.com", isSafe: false);
var result = shield.IsUrlSafe("https://evil.com");
// result.IsSafe, result.NeedsApiCheck
```

## Version

SDK: **0.4.0** | Binary: **0.4.0** (bundled)

## Links

- [NuGet](https://www.nuget.org/packages/B4n1Web)
- [GitHub](https://github.com/B4N1-com/b4n1-web)
- [Website](https://web.b4n1.com)

---
*Built by B4N1 with ❤️ · All rights reserved.*
