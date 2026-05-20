# B4n1Web Go SDK

<p align="center">
  <a href="https://pkg.go.dev/github.com/B4N1-com/b4n1-web-go"><img src="https://img.shields.io/go/v/b4n1-web-go.svg" alt="Go version"></a>
  <a href="https://pkg.go.dev/github.com/B4N1-com/b4n1-web-go"><img src="https://img.shields.io/badge/pkg.go.dev-reference-blue" alt="Go Reference"></a>
  <a href="https://github.com/B4N1-com/b4n1-web"><img src="https://img.shields.io/github/stars/B4N1-com/b4n1-web?style=flat" alt="GitHub stars"></a>
</p>

Go bindings for B4n1Web: The Agentic Browser Engine.

## Installation

### 1. Install the B4n1Web Binary

```bash
curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.7.0-flat.tar.gz | tar -xz && ./b4n1web --version
```

### 2. Install the Go SDK

```bash
go get github.com/B4N1-com/b4n1-web-go
```

## Quick Start

```go
package main

import (
    "fmt"
    b4n1web "github.com/B4N1-com/b4n1-web-go"
)

func main() {
    browser, err := b4n1web.NewAgentBrowser(
        b4n1web.WithMode(b4n1web.ModeLight),
    )
    if err != nil {
        panic(err)
    }
    defer browser.Close()

    page, err := browser.Goto("https://example.com")
    if err != nil {
        panic(err)
    }

    fmt.Println("Page content:", page.Markdown)
    fmt.Println("Links:", page.Links)

    // Find specific links
    githubLinks := page.FindLinksByText("github")
    fmt.Println("GitHub links:", githubLinks)

    // Get main content
    mainContent := page.GetMainContent()
    fmt.Println("Main content:", mainContent[:200])
}
```

## Browser Modes

### Light Mode (Default)

- **Use case**: Reading articles, scraping static content
- **Performance**: < 15MB RAM, instant startup

```go
browser, _ := b4n1web.NewAgentBrowser(b4n1web.WithMode(b4n1web.ModeLight))
```

### JS Mode

- **Use case**: Extracting JavaScript from pages, SPA analysis

```go
browser, _ := b4n1web.NewAgentBrowser(b4n1web.WithMode(b4n1web.ModeJS))
```

### Render Mode (Coming Soon)

- **Use case**: Full JavaScript execution, screenshots
- **Status**: Coming in v0.3.0

## Options

```go
// Custom timeout (seconds)
b4n1web.WithTimeout(60)

// Custom user agent
b4n1web.WithUserAgent("MyCustomAgent/1.0")
```

## SecurityShield

Optional security validation:

```go
shield := b4n1web.NewSecurityShield(
    b4n1web.WithCacheDays(30),
)

result, _ := shield.IsUrlSafe("https://example.com")
if !result.IsSafe {
    fmt.Println("URL is unsafe!")
}
```

## License

MIT
