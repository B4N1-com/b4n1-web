/*
Package b4n1web provides Go bindings for B4n1Web: The Agentic Browser Engine.

Installation:

    1. Install the B4n1Web Binary
       curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz

    2. Install the Go SDK
       go get github.com/B4N1-com/b4n1-web/go

Usage:

    package main

    import (
        "fmt"
        b4n1web "github.com/B4N1-com/b4n1-web/go"
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
    }
*/
package b4n1web

import (
	"github.com/B4N1-com/b4n1-web/go/internal"
)

// Re-export types and functions
type AgentBrowser = internal.AgentBrowser
type BrowserMode = internal.BrowserMode
type Page = internal.Page
type BrowserOption = internal.BrowserOption
type SecurityShield = internal.SecurityShield
type SecurityCheckResult = internal.SecurityCheckResult
type BinaryNotFoundError = internal.BinaryNotFoundError

const (
	ModeLight  = internal.ModeLight
	ModeJS     = internal.ModeJS
	ModeRender = internal.ModeRender
)

// NewAgentBrowser creates a new browser instance
var NewAgentBrowser = internal.NewAgentBrowser

// NewSecurityShield creates a new security shield
var NewSecurityShield = internal.NewSecurityShield

// GetVersion returns the binary version
var GetVersion = internal.GetVersion

// Navigate navigates with security check
var Navigate = internal.Navigate
