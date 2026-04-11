package b4n1web

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
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
	mode       BrowserMode
	timeout    int
	userAgent  string
	binaryPath string
}

// Page represents structured page data
type Page struct {
	URL        string   `json:"url"`
	Markdown   string   `json:"markdown"`
	Links      []string `json:"links"`
	Screenshot string   `json:"screenshot,omitempty"`
}

// NewAgentBrowser creates a new browser instance
func NewAgentBrowser(opts ...BrowserOption) (*AgentBrowser, error) {
	b := &AgentBrowser{
		mode:      ModeLight,
		timeout:   30,
		userAgent: "B4N1Web-Agent/1.0",
	}

	for _, opt := range opts {
		opt(b)
	}

	path, err := findBinary()
	if err != nil {
		return nil, fmt.Errorf("%w: please install b4n1web binary", err)
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

// Goto navigates to a URL and returns structured page data
func (b *AgentBrowser) Goto(url string) (*Page, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(b.timeout+5)*time.Second)
	defer cancel()

	cmd := exec.CommandContext(ctx, b.binaryPath, "goto", url, "--mode", string(b.mode))

	output, err := cmd.Output()
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			return nil, fmt.Errorf("binary error: %s", string(exitErr.Stderr))
		}
		return nil, fmt.Errorf("failed to execute: %w", err)
	}

	return b.parseOutput(url, string(output)), nil
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
		default:
			markdownLines = append(markdownLines, line)
		}
	}

	page.Markdown = joinLines(markdownLines)
	return page
}

// Close closes the browser session
func (b *AgentBrowser) Close() {
	// No persistent session in current implementation
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
const SDK_VERSION = "0.4.0"

// CheckVersionCompatibility checks if binary version matches SDK version
func CheckVersionCompatibility() (string, error) {
	binaryVersion := GetVersion()
	if binaryVersion == "unknown" {
		return "", fmt.Errorf("could not determine binary version")
	}

	if binaryVersion != SDK_VERSION {
		fmt.Fprintf(os.Stderr, "⚠️  Version mismatch: SDK v%s requires binary v%s, but found v%s. Some features may not work correctly. To update: curl -sL https://web.b4n1.com/install | bash\n",
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
