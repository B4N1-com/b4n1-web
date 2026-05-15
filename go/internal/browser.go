package internal

import (
	"context"
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"time"
)

// BrowserMode represents the browser execution mode
type BrowserMode string

const (
	ModeLight  BrowserMode = "light"
	ModeJS     BrowserMode = "js"
	ModeRender BrowserMode = "render"
)

// AgentBrowser represents a B4n1Web browser instance
type AgentBrowser struct {
	mode           BrowserMode
	timeout        int
	userAgent      string
	binaryPath     string
	lastURL        string
	sessionID      string
	sessionName    string
	viewportWidth  uint32
	viewportHeight uint32
}

// Page represents structured page data
type Page struct {
	URL        string   `json:"url"`
	Markdown   string   `json:"markdown"`
	Links      []string `json:"links"`
	Screenshot string   `json:"screenshot,omitempty"`
	JsOutput   string   `json:"js_output,omitempty"`
}

// NewAgentBrowser creates a new browser instance
func NewAgentBrowser(opts ...BrowserOption) (*AgentBrowser, error) {
	b := &AgentBrowser{
		mode:      ModeLight,
		timeout:   30,
		userAgent: "B4N1Web-Agent/1.0",
		sessionID: fmt.Sprintf("%x", rand.Int63()),
	}

	for _, opt := range opts {
		opt(b)
	}

	path, err := findBinary()
	if err != nil {
		return nil, fmt.Errorf("%w: b4n1web binary not found", err)
	}
	b.binaryPath = path

	// Check version compatibility (non-fatal warning)
	CheckVersionCompatibility()

	return b, nil
}

// BrowserOption configures the browser
type BrowserOption func(*AgentBrowser)

// WithMode sets the browser mode
func WithMode(mode BrowserMode) BrowserOption {
	return func(b *AgentBrowser) {
		b.mode = mode
	}
}

// WithTimeout sets the request timeout
func WithTimeout(timeout int) BrowserOption {
	return func(b *AgentBrowser) {
		b.timeout = timeout
	}
}

// WithUserAgent sets custom user agent
func WithUserAgent(ua string) BrowserOption {
	return func(b *AgentBrowser) {
		b.userAgent = ua
	}
}

// Goto navigates to a URL and returns structured page data.
// waitFor is an optional CSS selector to wait for before extracting (render mode only).
func (b *AgentBrowser) Goto(url string, waitFor ...string) (*Page, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(b.timeout+5)*time.Second)
	defer cancel()

	args := []string{"goto", url, "--mode", string(b.mode)}
	if len(waitFor) > 0 && waitFor[0] != "" {
		args = append(args, "--wait-for", waitFor[0])
	}

	cmd := exec.CommandContext(ctx, b.binaryPath, args...)

	output, err := cmd.Output()
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			return nil, fmt.Errorf("binary error: %s", string(exitErr.Stderr))
		}
		return nil, fmt.Errorf("failed to execute: %w", err)
	}

	b.lastURL = url
	return b.parseOutput(url, string(output)), nil
}

// SessionStart starts a persistent browser session with the given name.
// All subsequent session operations (Click, TypeText, etc.) use this session name.
func (b *AgentBrowser) SessionStart(name string) error {
	b.sessionName = name
	_, err := b.runSessionCommand("start")
	return err
}

// SessionStop stops the current session's navigation.
// The session remains open for further operations.
func (b *AgentBrowser) SessionStop() error {
	if b.sessionName == "" {
		return fmt.Errorf("no active session, call SessionStart first")
	}
	_, err := b.Goto("about:blank")
	return err
}

// runSessionCommand runs a b4n1web session subcommand with the session name and extra args.
func (b *AgentBrowser) runSessionCommand(subcommand string, extraArgs ...string) (string, error) {
	if b.sessionName == "" {
		return "", fmt.Errorf("no active session, call SessionStart first")
	}
	args := append([]string{"session", subcommand, b.sessionName}, extraArgs...)
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(b.timeout+5)*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, b.binaryPath, args...)
	output, err := cmd.Output()
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			return "", fmt.Errorf("session %s failed: %s", subcommand, string(exitErr.Stderr))
		}
		return "", fmt.Errorf("session %s failed: %w", subcommand, err)
	}
	return string(output), nil
}

// parseOutput parses text output from the binary
func (b *AgentBrowser) parseOutput(url, output string) *Page {
	page := &Page{URL: url}

	var markdownLines []string
	for _, line := range splitLines(output) {
		switch {
		case hasPrefix(line, "URL:"):
			continue
		case hasPrefix(line, "Markdown:"):
			continue
		case hasPrefix(line, "Links:"):
			page.Links = parseLinks(line[6:])
		case hasPrefix(line, "Screenshot:"):
			page.Screenshot = string(trimSpace([]byte(line[11:])))
		case hasPrefix(line, "JsOutput:"):
			page.JsOutput = string(trimSpace([]byte(line[9:])))
		default:
			markdownLines = append(markdownLines, line)
		}
	}

	page.Markdown = joinLines(markdownLines)
	return page
}

// Close closes the browser session if one is active.
func (b *AgentBrowser) Close() {
	if b.sessionName != "" {
		b.runSessionCommand("close")
		b.sessionName = ""
	}
}

// Screenshot captures a screenshot of the last visited page.
// If a session is active, uses the session screenshot command.
// Otherwise falls back to render mode via Goto.
func (b *AgentBrowser) Screenshot(width, height uint32) (string, error) {
	if b.sessionName != "" && b.lastURL != "" {
		return b.runSessionCommand("screenshot", b.lastURL)
	}
	if b.lastURL == "" {
		return "", fmt.Errorf("no page loaded, call Goto first")
	}
	originalMode := b.mode
	b.mode = ModeRender
	defer func() { b.mode = originalMode }()

	page, err := b.Goto(b.lastURL)
	if err != nil {
		return "", fmt.Errorf("screenshot failed: %w", err)
	}
	if page.Screenshot == "" {
		return "", fmt.Errorf("no screenshot returned from render mode")
	}
	return page.Screenshot, nil
}

// WaitForSelector waits for a CSS selector to appear on the page.
// Returns true if the element was found within the timeout.
func (b *AgentBrowser) WaitForSelector(selector string, timeoutMs uint64) bool {
	args := []string{selector}
	if timeoutMs > 0 {
		args = append(args, "--timeout-ms", fmt.Sprintf("%d", timeoutMs))
	}
	_, err := b.runSessionCommand("wait", args...)
	return err == nil
}

// Click clicks on an element matching the CSS selector.
func (b *AgentBrowser) Click(selector string) error {
	_, err := b.runSessionCommand("click", selector)
	return err
}

// TypeText types text into an element matching the CSS selector.
// If clearFirst is true, the element's current value is cleared before typing.
func (b *AgentBrowser) TypeText(selector, text string, clearFirst bool) error {
	args := []string{selector, text}
	if clearFirst {
		args = append(args, "--clear-first")
	}
	_, err := b.runSessionCommand("type", args...)
	return err
}

// Frames lists all iframes on the current page.
func (b *AgentBrowser) Frames() ([]string, error) {
	output, err := b.runSessionCommand("frames")
	if err != nil {
		return nil, err
	}
	return splitLines(string(trimSpace([]byte(output)))), nil
}

// IframeText returns the text content of an iframe at the given index.
func (b *AgentBrowser) IframeText(index int) (string, error) {
	return b.runSessionCommand("iframe-text", fmt.Sprintf("%d", index))
}

// SetViewport sets the viewport dimensions for subsequent navigations.
func (b *AgentBrowser) SetViewport(width, height uint32) error {
	b.viewportWidth = width
	b.viewportHeight = height
	return nil
}

// EmulateDevice sets viewport and user agent to emulate a specific device.
// Supported devices: iphone 12/13/14, iphone se, pixel 5/6/7, ipad, ipad pro
func (b *AgentBrowser) EmulateDevice(device string) error {
	switch toLower(device) {
	case "iphone 12", "iphone 13", "iphone 14":
		b.userAgent = "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1"
		b.viewportWidth = 390
		b.viewportHeight = 844
	case "iphone se":
		b.userAgent = "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1"
		b.viewportWidth = 375
		b.viewportHeight = 667
	case "pixel 5", "pixel 6", "pixel 7":
		b.userAgent = "Mozilla/5.0 (Linux; Android 12; Pixel 5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Mobile Safari/537.36"
		b.viewportWidth = 393
		b.viewportHeight = 851
	case "ipad", "ipad air":
		b.userAgent = "Mozilla/5.0 (iPad; CPU OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1"
		b.viewportWidth = 820
		b.viewportHeight = 1180
	case "ipad pro":
		b.userAgent = "Mozilla/5.0 (iPad; CPU OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1"
		b.viewportWidth = 1024
		b.viewportHeight = 1366
	default:
		b.userAgent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36"
		b.viewportWidth = 1920
		b.viewportHeight = 1080
	}
	return nil
}

// GetLinksFromPage is a static helper that navigates to a URL and returns all links.
func GetLinksFromPage(url string) ([]string, error) {
	b, err := NewAgentBrowser()
	if err != nil {
		return nil, err
	}
	defer b.Close()
	page, err := b.Goto(url)
	if err != nil {
		return nil, err
	}
	return page.Links, nil
}

// GetLinksFromPage navigates to a URL and returns all links found on the page.
func (b *AgentBrowser) GetLinksFromPage(url string) ([]string, error) {
	page, err := b.Goto(url)
	if err != nil {
		return nil, err
	}
	return page.Links, nil
}

// GetLinks returns all links found on the page.
func (p *Page) GetLinks() []string {
	return p.Links
}

// GetMainContent extracts main content from markdown
func (p *Page) GetMainContent() string {
	lines := splitLines(p.Markdown)
	if len(lines) > 2 {
		return joinLines(lines[2:])
	}
	return p.Markdown
}

// FindLinksByText finds links containing specific text
func (p *Page) FindLinksByText(text string) []string {
	lowerText := toLower(text)
	var results []string
	for _, link := range p.Links {
		if contains(toLower(link), lowerText) {
			results = append(results, link)
		}
	}
	return results
}

// SDK_VERSION is the current SDK version string
const SDK_VERSION = "0.7.0"

// CheckVersionCompatibility checks if binary version matches SDK version
func CheckVersionCompatibility() (string, error) {
	binaryVersion := GetVersion()
	if binaryVersion == "unknown" {
		return "", fmt.Errorf("could not determine binary version")
	}

	if binaryVersion != SDK_VERSION {
		fmt.Fprintf(os.Stderr, "⚠️  Version mismatch: SDK v%s requires binary v%s, but found v%s. Some features may not work correctly.\n",
			SDK_VERSION, SDK_VERSION, binaryVersion)
	}
	return binaryVersion, nil
}

// GetVersion returns the binary version
func GetVersion() string {
	path, err := findBinary()
	if err != nil {
		return "unknown"
	}

	cmd := exec.Command(path, "--version")
	out, err := cmd.Output()
	if err != nil {
		return "unknown"
	}
	return string(trimSpace(out))
}

// findBinary locates the b4n1web binary
func findBinary() (string, error) {
	// 1. Check bundled binary (bundled with Go module)
	_, filename, _, _ := runtime.Caller(0)
	moduleDir := filepath.Dir(filepath.Dir(filename))
	bundledBinary := filepath.Join(moduleDir, "bin", "b4n1web-linux")
	if info, err := os.Stat(bundledBinary); err == nil {
		if info.Mode()&0111 != 0 {
			return bundledBinary, nil
		}
	}

	// 2. Check system install locations
	possiblePaths := []string{
		"/usr/local/bin/b4n1web",
		"/usr/bin/b4n1web",
		filepath.Join(os.Getenv("HOME"), ".local/bin/b4n1web"),
		filepath.Join(os.Getenv("HOME"), ".b4n1web/bin/b4n1web"),
	}

	for _, path := range possiblePaths {
		if info, err := os.Stat(path); err == nil {
			if info.Mode()&0111 != 0 {
				return path, nil
			}
		}
	}

	return "", os.ErrNotExist
}

// Helper functions to avoid importing strings package
func hasPrefix(s, prefix string) bool {
	return len(s) >= len(prefix) && s[:len(prefix)] == prefix
}

func splitLines(s string) []string {
	var lines []string
	start := 0
	for i, c := range s {
		if c == '\n' {
			lines = append(lines, s[start:i])
			start = i + 1
		}
	}
	if start < len(s) {
		lines = append(lines, s[start:])
	}
	return lines
}

func joinLines(lines []string) string {
	result := ""
	for i, line := range lines {
		if i > 0 {
			result += "\n"
		}
		result += line
	}
	return result
}

func toLower(s string) string {
	result := make([]byte, len(s))
	for i := 0; i < len(s); i++ {
		c := s[i]
		if c >= 'A' && c <= 'Z' {
			c += 'a' - 'A'
		}
		result[i] = c
	}
	return string(result)
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || findSubstring(s, substr))
}

func findSubstring(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}

func trimSpace(b []byte) []byte {
	start := 0
	end := len(b) - 1
	for start < len(b) && (b[start] == ' ' || b[start] == '\n' || b[start] == '\r') {
		start++
	}
	for end >= start && (b[end] == ' ' || b[end] == '\n' || b[end] == '\r') {
		end--
	}
	return b[start : end+1]
}

// parseLinks parses a JSON-like links string into a slice of strings
func parseLinks(s string) []string {
	s = string(trimSpace([]byte(s)))
	if len(s) < 2 || s[0] != '[' || s[len(s)-1] != ']' {
		return []string{}
	}
	s = s[1 : len(s)-1]
	if len(s) == 0 {
		return []string{}
	}

	var links []string
	for _, item := range splitLinks(s) {
		item = string(trimSpace([]byte(item)))
		if len(item) >= 2 {
			// Remove quotes (single or double)
			if (item[0] == '"' && item[len(item)-1] == '"') ||
				(item[0] == '\'' && item[len(item)-1] == '\'') {
				links = append(links, item[1:len(item)-1])
			}
		}
	}
	return links
}

// splitLinks splits a comma-separated list of quoted strings
func splitLinks(s string) []string {
	var result []string
	start := 0
	inQuote := false
	quoteChar := byte(0)
	for i := 0; i < len(s); i++ {
		c := s[i]
		if inQuote {
			if c == quoteChar {
				inQuote = false
			}
		} else {
			if c == '"' || c == '\'' {
				inQuote = true
				quoteChar = c
			} else if c == ',' {
				result = append(result, s[start:i])
				start = i + 1
			}
		}
	}
	if start < len(s) {
		result = append(result, s[start:])
	}
	return result
}
