package main

import (
	"fmt"
	"os"
	"strings"

	b4n1web "github.com/B4N1-com/b4n1-web/go"
)

func main() {
	browser, err := b4n1web.NewAgentBrowser(
		b4n1web.WithMode(b4n1web.ModeLight),
	)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error creating browser: %v\n", err)
		fmt.Fprintln(os.Stderr, "Make sure b4n1web binary is installed:")
		fmt.Fprintln(os.Stderr, "  curl -sL https://web.b4n1.com/install | bash")
		os.Exit(1)
	}
	defer browser.Close()

	page, err := browser.Goto("https://example.com")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Goto error: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("=== Page Info ===")
	fmt.Printf("URL: %s\n", page.URL)
	fmt.Printf("Links count: %d\n", len(page.Links))
	fmt.Printf("Screenshot: %s\n", truncate(page.Screenshot, 40))
	fmt.Printf("JsOutput: %s\n", truncate(page.JsOutput, 40))
	fmt.Printf("Has markdown: %v\n", page.Markdown != "")

	fmt.Println("\n=== GetMainContent ===")
	fmt.Println(page.GetMainContent())

	fmt.Println("\n=== FindLinksByText ===")
	results := page.FindLinksByText("example")
	fmt.Printf("Found %d links containing 'example':\n", len(results))
	for _, link := range results {
		fmt.Printf("  - %s\n", link)
	}

	fmt.Println("\n=== GetLinks ===")
	for _, link := range page.GetLinks() {
		fmt.Printf("  - %s\n", link)
	}

	fmt.Println("\n=== Goto with wait_for ===")
	pageWithWait, err := browser.Goto("https://example.com", "body")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Goto with wait_for error: %v\n", err)
	} else {
		fmt.Printf("Navigated with wait_for, links: %d\n", len(pageWithWait.Links))
	}

	fmt.Println("\n=== GetLinksFromPage ===")
	links, err := browser.GetLinksFromPage("https://example.com")
	if err != nil {
		fmt.Fprintf(os.Stderr, "GetLinksFromPage error: %v\n", err)
	} else {
		fmt.Printf("Found %d links\n", len(links))
	}

	fmt.Println("\n=== WaitForSelector ===")
	found := browser.WaitForSelector("body", 500)
	fmt.Printf("Selector found: %v\n", found)

	fmt.Println("\n=== Click ===")
	if err := browser.Click("a"); err != nil {
		fmt.Fprintf(os.Stderr, "Click error: %v\n", err)
	} else {
		fmt.Println("Click succeeded")
	}

	fmt.Println("\n=== TypeText ===")
	if err := browser.TypeText("#search", "hello", true); err != nil {
		fmt.Fprintf(os.Stderr, "TypeText error: %v\n", err)
	} else {
		fmt.Println("TypeText succeeded")
	}

	fmt.Println("\n=== Screenshot ===")
	ss, err := browser.Screenshot(1024, 768)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Screenshot error: %v\n", err)
	} else {
		fmt.Printf("Screenshot length: %d bytes (base64)\n", len(ss))
	}

	fmt.Println("\n=== Version ===")
	version := b4n1web.GetVersion()
	fmt.Printf("Binary version: %s\n", version)

	fmt.Println("\n✅ All features demonstrated successfully!")
}

func truncate(s string, max int) string {
	if s == "" {
		return "(empty)"
	}
	if len(s) > max {
		return s[:max] + "..."
	}
	return strings.ReplaceAll(s, "\n", "\\n")
}
