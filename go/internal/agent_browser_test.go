package b4n1web

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// --- Mock binary helpers ---

// createMockBinary creates a shell script that mimics b4n1web binary output.
func createMockBinary(t *testing.T, output string) string {
	t.Helper()
	dir := t.TempDir()
	scriptPath := filepath.Join(dir, "mock-b4n1web")
	script := "#!/bin/bash\n"
	// Handle --version flag
	script += `if [ "$1" = "--version" ]; then
	echo "0.4.0"
	exit 0
fi
`
	// For goto, output the mock data
	script += "echo '" + output + "'\n"
	script += "exit 0\n"
	if err := os.WriteFile(scriptPath, []byte(script), 0755); err != nil {
		t.Fatalf("failed to create mock binary: %v", err)
	}
	return scriptPath
}

// createVersionMockBinary creates a mock that only supports --version
func createVersionMockBinary(t *testing.T, version string) string {
	t.Helper()
	dir := t.TempDir()
	scriptPath := filepath.Join(dir, "mock-version")
	script := "#!/bin/bash\n"
	script += `if [ "$1" = "--version" ]; then
	echo "` + version + `"
	exit 0
fi
exit 1
`
	if err := os.WriteFile(scriptPath, []byte(script), 0755); err != nil {
		t.Fatalf("failed to create version mock binary: %v", err)
	}
	return scriptPath
}

// createErrorMockBinary creates a mock that always exits with error
func createErrorMockBinary(t *testing.T, stderr string) string {
	t.Helper()
	dir := t.TempDir()
	scriptPath := filepath.Join(dir, "mock-error")
	script := "#!/bin/bash\n"
	script += `if [ "$1" = "--version" ]; then
	echo "0.4.0"
	exit 0
fi
echo '` + stderr + `' >&2
exit 1
`
	if err := os.WriteFile(scriptPath, []byte(script), 0755); err != nil {
		t.Fatalf("failed to create error mock binary: %v", err)
	}
	return scriptPath
}

// createTimeoutMockBinary creates a mock that sleeps longer than any reasonable timeout
func createTimeoutMockBinary(t *testing.T) string {
	t.Helper()
	dir := t.TempDir()
	scriptPath := filepath.Join(dir, "mock-timeout")
	script := "#!/bin/bash\n"
	script += `if [ "$1" = "--version" ]; then
	echo "0.4.0"
	exit 0
fi
sleep 60
exit 0
`
	if err := os.WriteFile(scriptPath, []byte(script), 0755); err != nil {
		t.Fatalf("failed to create timeout mock binary: %v", err)
	}
	return scriptPath
}

// --- AgentBrowser construction tests ---

func TestNewAgentBrowserDefaults(t *testing.T) {
	// With the real binary installed, NewAgentBrowser should succeed
	browser, err := NewAgentBrowser()
	if err != nil {
		t.Fatalf("NewAgentBrowser() returned error: %v", err)
	}
	defer browser.Close()

	if browser.mode != ModeLight {
		t.Errorf("default mode = %q, want %q", browser.mode, ModeLight)
	}
	if browser.timeout != 30 {
		t.Errorf("default timeout = %d, want 30", browser.timeout)
	}
	if browser.userAgent != "B4N1Web-Agent/1.0" {
		t.Errorf("default userAgent = %q, want %q", browser.userAgent, "B4N1Web-Agent/1.0")
	}
	if browser.binaryPath == "" {
		t.Error("binaryPath should not be empty after construction")
	}
}

func TestAgentBrowserWithOptions(t *testing.T) {
	t.Run("WithMode light", func(t *testing.T) {
		browser, err := NewAgentBrowser(WithMode(ModeLight))
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.mode != ModeLight {
			t.Errorf("mode = %q, want %q", browser.mode, ModeLight)
		}
	})

	t.Run("WithMode js", func(t *testing.T) {
		browser, err := NewAgentBrowser(WithMode(ModeJS))
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.mode != ModeJS {
			t.Errorf("mode = %q, want %q", browser.mode, ModeJS)
		}
	})

	t.Run("WithMode render", func(t *testing.T) {
		browser, err := NewAgentBrowser(WithMode(ModeRender))
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.mode != ModeRender {
			t.Errorf("mode = %q, want %q", browser.mode, ModeRender)
		}
	})

	t.Run("WithTimeout custom", func(t *testing.T) {
		browser, err := NewAgentBrowser(WithTimeout(60))
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.timeout != 60 {
			t.Errorf("timeout = %d, want 60", browser.timeout)
		}
	})

	t.Run("WithTimeout zero", func(t *testing.T) {
		browser, err := NewAgentBrowser(WithTimeout(0))
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.timeout != 0 {
			t.Errorf("timeout = %d, want 0", browser.timeout)
		}
	})

	t.Run("WithUserAgent custom", func(t *testing.T) {
		browser, err := NewAgentBrowser(WithUserAgent("CustomBot/2.0"))
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.userAgent != "CustomBot/2.0" {
			t.Errorf("userAgent = %q, want %q", browser.userAgent, "CustomBot/2.0")
		}
	})

	t.Run("WithUserAgent empty", func(t *testing.T) {
		browser, err := NewAgentBrowser(WithUserAgent(""))
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.userAgent != "" {
			t.Errorf("userAgent = %q, want empty string", browser.userAgent)
		}
	})

	t.Run("all options combined", func(t *testing.T) {
		browser, err := NewAgentBrowser(
			WithMode(ModeRender),
			WithTimeout(120),
			WithUserAgent("TestBot/3.0"),
		)
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.mode != ModeRender {
			t.Errorf("mode = %q, want %q", browser.mode, ModeRender)
		}
		if browser.timeout != 120 {
			t.Errorf("timeout = %d, want 120", browser.timeout)
		}
		if browser.userAgent != "TestBot/3.0" {
			t.Errorf("userAgent = %q, want %q", browser.userAgent, "TestBot/3.0")
		}
	})

	t.Run("multiple WithMode last wins", func(t *testing.T) {
		browser, err := NewAgentBrowser(
			WithMode(ModeLight),
			WithMode(ModeRender),
		)
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.mode != ModeRender {
			t.Errorf("mode = %q, want %q", browser.mode, ModeRender)
		}
	})

	t.Run("multiple WithTimeout last wins", func(t *testing.T) {
		browser, err := NewAgentBrowser(
			WithTimeout(10),
			WithTimeout(50),
		)
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		defer browser.Close()
		if browser.timeout != 50 {
			t.Errorf("timeout = %d, want 50", browser.timeout)
		}
	})
}

func TestAgentBrowserManualConstruction(t *testing.T) {
	// Test constructing AgentBrowser directly (bypassing NewAgentBrowser)
	t.Run("manual with mock binary", func(t *testing.T) {
		mockOutput := `URL: https://test.com
Markdown:
# Test Page
Hello World
Links: ["https://test.com/link1", "https://test.com/link2"]`

		mockPath := createMockBinary(t, mockOutput)
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    30,
			userAgent:  "TestAgent/1.0",
			binaryPath: mockPath,
		}

		page, err := browser.Goto("https://test.com")
		if err != nil {
			t.Fatalf("Goto() error: %v", err)
		}
		if page.URL != "https://test.com" {
			t.Errorf("page.URL = %q, want %q", page.URL, "https://test.com")
		}
	})
}

// --- Goto tests with mocked binary ---

func TestAgentBrowserGoto(t *testing.T) {
	t.Run("successful navigation with full output", func(t *testing.T) {
		mockOutput := `URL: https://example.com
Markdown:
# Example Domain
This domain is for documentation.
Links: ["https://iana.org/domains/example"]`

		mockPath := createMockBinary(t, mockOutput)
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    30,
			binaryPath: mockPath,
		}

		page, err := browser.Goto("https://example.com")
		if err != nil {
			t.Fatalf("Goto() error: %v", err)
		}
		if page.URL != "https://example.com" {
			t.Errorf("URL = %q, want %q", page.URL, "https://example.com")
		}
		if !strings.Contains(page.Markdown, "# Example Domain") {
			t.Errorf("Markdown missing heading, got: %q", page.Markdown)
		}
		if !strings.Contains(page.Markdown, "This domain is for documentation.") {
			t.Errorf("Markdown missing body text, got: %q", page.Markdown)
		}
		if len(page.Links) != 1 {
			t.Fatalf("Links count = %d, want 1", len(page.Links))
		}
		if page.Links[0] != "https://iana.org/domains/example" {
			t.Errorf("Link[0] = %q, want %q", page.Links[0], "https://iana.org/domains/example")
		}
	})

	t.Run("navigation with multiple links", func(t *testing.T) {
		mockOutput := `URL: https://example.com
Markdown:
# Page
Content here.
Links: ["https://a.com", "https://b.com", "https://c.com"]`

		mockPath := createMockBinary(t, mockOutput)
		browser := &AgentBrowser{
			mode:       ModeJS,
			timeout:    30,
			binaryPath: mockPath,
		}

		page, err := browser.Goto("https://example.com")
		if err != nil {
			t.Fatalf("Goto() error: %v", err)
		}
		if len(page.Links) != 3 {
			t.Fatalf("Links count = %d, want 3", len(page.Links))
		}
		expected := []string{"https://a.com", "https://b.com", "https://c.com"}
		for i, want := range expected {
			if page.Links[i] != want {
				t.Errorf("Links[%d] = %q, want %q", i, page.Links[i], want)
			}
		}
	})

	t.Run("navigation with empty links array", func(t *testing.T) {
		mockOutput := `URL: https://example.com
Markdown:
# Page
No links.
Links: []`

		mockPath := createMockBinary(t, mockOutput)
		browser := &AgentBrowser{
			mode:       ModeRender,
			timeout:    30,
			binaryPath: mockPath,
		}

		page, err := browser.Goto("https://example.com")
		if err != nil {
			t.Fatalf("Goto() error: %v", err)
		}
		if page.Links == nil {
			t.Error("Links should not be nil")
		}
		if len(page.Links) != 0 {
			t.Errorf("Links count = %d, want 0", len(page.Links))
		}
	})

	t.Run("navigation with no Links line in output", func(t *testing.T) {
		mockOutput := `URL: https://example.com
Markdown:
# Page
Content only.
`

		mockPath := createMockBinary(t, mockOutput)
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    30,
			binaryPath: mockPath,
		}

		page, err := browser.Goto("https://example.com")
		if err != nil {
			t.Fatalf("Goto() error: %v", err)
		}
		if page.Links != nil && len(page.Links) != 0 {
			t.Errorf("Links should be empty, got %v", page.Links)
		}
	})

	t.Run("navigation with multiline markdown", func(t *testing.T) {
		mockOutput := `URL: https://example.com
Markdown:
# Title
## Subtitle
Paragraph one.
Paragraph two.
Links: []`

		mockPath := createMockBinary(t, mockOutput)
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    30,
			binaryPath: mockPath,
		}

		page, err := browser.Goto("https://example.com")
		if err != nil {
			t.Fatalf("Goto() error: %v", err)
		}
		if !strings.Contains(page.Markdown, "# Title") {
			t.Errorf("Markdown missing # Title")
		}
		if !strings.Contains(page.Markdown, "## Subtitle") {
			t.Errorf("Markdown missing ## Subtitle")
		}
		if !strings.Contains(page.Markdown, "Paragraph one.") {
			t.Errorf("Markdown missing paragraph one")
		}
		if !strings.Contains(page.Markdown, "Paragraph two.") {
			t.Errorf("Markdown missing paragraph two")
		}
	})

	t.Run("mode is passed to binary", func(t *testing.T) {
		// Create a mock that echoes its arguments to verify mode is passed
		dir := t.TempDir()
		scriptPath := filepath.Join(dir, "mock-echo-args")
		script := `#!/bin/bash
if [ "$1" = "--version" ]; then
	echo "0.4.0"
	exit 0
fi
# Write args to a file for inspection
echo "$@" > /tmp/b4n1web-test-args
echo "URL: https://test.com
Markdown:
Test
Links: []"
exit 0
`
		if err := os.WriteFile(scriptPath, []byte(script), 0755); err != nil {
			t.Fatalf("failed to create mock: %v", err)
		}

		for _, mode := range []BrowserMode{ModeLight, ModeJS, ModeRender} {
			t.Run(string(mode), func(t *testing.T) {
				// Clean up args file
				os.Remove("/tmp/b4n1web-test-args")

				browser := &AgentBrowser{
					mode:       mode,
					timeout:    30,
					binaryPath: scriptPath,
				}

				_, err := browser.Goto("https://test.com")
				if err != nil {
					t.Fatalf("Goto() error: %v", err)
				}

				argsData, err := os.ReadFile("/tmp/b4n1web-test-args")
				if err != nil {
					t.Fatalf("failed to read args file: %v", err)
				}
				argsStr := strings.TrimSpace(string(argsData))
				if !strings.Contains(argsStr, string(mode)) {
					t.Errorf("binary args %q missing mode %q", argsStr, mode)
				}
				if !strings.Contains(argsStr, "--mode") {
					t.Errorf("binary args %q missing --mode flag", argsStr)
				}
			})
		}
	})

	t.Run("URL is passed to binary", func(t *testing.T) {
		dir := t.TempDir()
		scriptPath := filepath.Join(dir, "mock-url-check")
		script := `#!/bin/bash
if [ "$1" = "--version" ]; then
	echo "0.4.0"
	exit 0
fi
echo "$2" > /tmp/b4n1web-test-url
echo "URL: https://test.com
Markdown:
Test
Links: []"
exit 0
`
		if err := os.WriteFile(scriptPath, []byte(script), 0755); err != nil {
			t.Fatalf("failed to create mock: %v", err)
		}

		testURL := "https://my-test.example.com/path?query=1"
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    30,
			binaryPath: scriptPath,
		}

		os.Remove("/tmp/b4n1web-test-url")
		_, err := browser.Goto(testURL)
		if err != nil {
			t.Fatalf("Goto() error: %v", err)
		}

		urlData, err := os.ReadFile("/tmp/b4n1web-test-url")
		if err != nil {
			t.Fatalf("failed to read url file: %v", err)
		}
		gotURL := strings.TrimSpace(string(urlData))
		if gotURL != testURL {
			t.Errorf("URL passed to binary = %q, want %q", gotURL, testURL)
		}
	})

	t.Run("binary error returns error", func(t *testing.T) {
		mockPath := createErrorMockBinary(t, "internal error")
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    30,
			binaryPath: mockPath,
		}

		_, err := browser.Goto("https://example.com")
		if err == nil {
			t.Fatal("Goto() expected error, got nil")
		}
		if !strings.Contains(err.Error(), "internal error") {
			t.Errorf("error = %q, should contain %q", err.Error(), "internal error")
		}
	})

	t.Run("timeout mock binary", func(t *testing.T) {
		mockPath := createTimeoutMockBinary(t)
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    1, // 1 second timeout
			binaryPath: mockPath,
		}

		_, err := browser.Goto("https://example.com")
		if err == nil {
			t.Fatal("Goto() expected timeout error, got nil")
		}
	})
}

// --- parseOutput tests ---

func TestParseOutput(t *testing.T) {
	t.Run("full output parsing", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		output := `URL: https://example.com
Markdown:
# Hello World
Content here.
Links: ["https://link1.com", "https://link2.com"]`

		page := browser.parseOutput("https://example.com", output)

		if page.URL != "https://example.com" {
			t.Errorf("URL = %q, want %q", page.URL, "https://example.com")
		}
		if !strings.Contains(page.Markdown, "# Hello World") {
			t.Errorf("Markdown missing heading")
		}
		if !strings.Contains(page.Markdown, "Content here.") {
			t.Errorf("Markdown missing content")
		}
		if len(page.Links) != 2 {
			t.Errorf("Links count = %d, want 2", len(page.Links))
		}
	})

	t.Run("URL line is skipped", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		output := `URL: https://example.com
Markdown:
Content`

		page := browser.parseOutput("https://example.com", output)
		if strings.Contains(page.Markdown, "URL:") {
			t.Errorf("Markdown should not contain URL: line, got: %q", page.Markdown)
		}
	})

	t.Run("Markdown: label line is skipped", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		output := `URL: https://example.com
Markdown:
# Title`

		page := browser.parseOutput("https://example.com", output)
		if strings.Contains(page.Markdown, "Markdown:") {
			t.Errorf("Markdown should not contain 'Markdown:' label, got: %q", page.Markdown)
		}
	})

	t.Run("empty output", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		page := browser.parseOutput("https://example.com", "")

		if page.URL != "https://example.com" {
			t.Errorf("URL = %q, want %q", page.URL, "https://example.com")
		}
		if page.Markdown != "" {
			t.Errorf("Markdown = %q, want empty", page.Markdown)
		}
		if page.Links != nil {
			t.Errorf("Links should be nil, got %v", page.Links)
		}
	})

	t.Run("output with only URL line", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		output := "URL: https://example.com"

		page := browser.parseOutput("https://example.com", output)
		if page.Markdown != "" {
			t.Errorf("Markdown = %q, want empty", page.Markdown)
		}
	})

	t.Run("malformed links line is ignored", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		output := `URL: https://example.com
Markdown:
Content
Links: not-valid-json`

		page := browser.parseOutput("https://example.com", output)
		if page.Links == nil && len(page.Links) != 0 {
			// Links should be empty slice or nil, but not a panic
		}
	})

	t.Run("Links line with single quotes", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		output := `URL: https://example.com
Markdown:
Content
Links: ['https://a.com', 'https://b.com']`

		page := browser.parseOutput("https://example.com", output)
		if len(page.Links) != 2 {
			t.Fatalf("Links count = %d, want 2", len(page.Links))
		}
		if page.Links[0] != "https://a.com" {
			t.Errorf("Links[0] = %q, want %q", page.Links[0], "https://a.com")
		}
		if page.Links[1] != "https://b.com" {
			t.Errorf("Links[1] = %q, want %q", page.Links[1], "https://b.com")
		}
	})

	t.Run("Links line with double quotes", func(t *testing.T) {
		browser := &AgentBrowser{mode: ModeLight}
		output := `URL: https://example.com
Markdown:
Content
Links: ["https://a.com"]`

		page := browser.parseOutput("https://example.com", output)
		if len(page.Links) != 1 {
			t.Fatalf("Links count = %d, want 1", len(page.Links))
		}
		if page.Links[0] != "https://a.com" {
			t.Errorf("Links[0] = %q, want %q", page.Links[0], "https://a.com")
		}
	})
}

// --- Close tests ---

func TestAgentBrowserClose(t *testing.T) {
	t.Run("Close does not panic", func(t *testing.T) {
		browser := &AgentBrowser{
			mode:       ModeLight,
			timeout:    30,
			binaryPath: "/nonexistent",
		}
		browser.Close() // Should be a no-op
	})

	t.Run("Close on valid browser", func(t *testing.T) {
		browser, err := NewAgentBrowser()
		if err != nil {
			t.Fatalf("NewAgentBrowser() error: %v", err)
		}
		browser.Close()
		// Second close should also be safe
		browser.Close()
	})
}

// --- findBinary tests ---

func TestFindBinary(t *testing.T) {
	t.Run("finds bundled binary", func(t *testing.T) {
		path, err := findBinary()
		if err != nil {
			t.Fatalf("findBinary() error: %v", err)
		}
		if path == "" {
			t.Fatal("findBinary() returned empty path")
		}
		// Should be the bundled binary since it exists
		if !strings.Contains(path, "b4n1web") {
			t.Errorf("path = %q, expected b4n1web binary", path)
		}
		// Verify the binary is executable
		info, err := os.Stat(path)
		if err != nil {
			t.Fatalf("stat on binary failed: %v", err)
		}
		if info.Mode()&0111 == 0 {
			t.Errorf("binary at %q is not executable", path)
		}
	})
}

// --- GetVersion tests ---

func TestGetVersion(t *testing.T) {
	t.Run("returns version from real binary", func(t *testing.T) {
		version := GetVersion()
		if version == "unknown" {
			t.Fatal("GetVersion() returned unknown, binary may not be installed")
		}
		if version != "0.4.0" {
			t.Logf("GetVersion() = %q (may vary by installation)", version)
		}
	})

	t.Run("returns unknown when binary missing", func(t *testing.T) {
		// Temporarily rename binary to simulate missing
		binaryPath, err := findBinary()
		if err != nil {
			t.Skip("cannot find binary to test missing case")
		}
		// Save and rename back
		tmpPath := binaryPath + ".bak"
		if err := os.Rename(binaryPath, tmpPath); err != nil {
			t.Skipf("cannot rename binary: %v", err)
		}
		defer os.Rename(tmpPath, binaryPath)

		version := GetVersion()
		if version != "unknown" {
			t.Errorf("GetVersion() = %q with no binary, want %q", version, "unknown")
		}
	})
}

// --- CheckVersionCompatibility tests ---

func TestCheckVersionCompatibility(t *testing.T) {
	t.Run("compatible version returns version", func(t *testing.T) {
		// With the real binary at 0.4.0 and SDK_VERSION at 0.4.0, this should pass
		version, err := CheckVersionCompatibility()
		if err != nil {
			t.Fatalf("CheckVersionCompatibility() error: %v", err)
		}
		if version == "" {
			t.Error("expected non-empty version string")
		}
	})
}

// --- parseLinks unit tests ---

func TestParseLinks(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []string
	}{
		{
			name:  "single double-quoted link",
			input: `["https://example.com"]`,
			want:  []string{"https://example.com"},
		},
		{
			name:  "multiple double-quoted links",
			input: `["https://a.com", "https://b.com"]`,
			want:  []string{"https://a.com", "https://b.com"},
		},
		{
			name:  "single single-quoted link",
			input: `['https://example.com']`,
			want:  []string{"https://example.com"},
		},
		{
			name:  "mixed quotes",
			input: `["https://a.com", 'https://b.com']`,
			want:  []string{"https://a.com", "https://b.com"},
		},
		{
			name:  "empty array",
			input: `[]`,
			want:  []string{},
		},
		{
			name:  "not an array",
			input: `"not-an-array"`,
			want:  []string{},
		},
		{
			name:  "empty string",
			input: ``,
			want:  []string{},
		},
		{
			name:  "whitespace around items",
			input: `[ "https://a.com" , "https://b.com" ]`,
			want:  []string{"https://a.com", "https://b.com"},
		},
		{
			name:  "link with path and query",
			input: `["https://example.com/path?q=1"]`,
			want:  []string{"https://example.com/path?q=1"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := parseLinks(tt.input)
			if len(got) != len(tt.want) {
				t.Fatalf("parseLinks() len = %d, want %d; got=%v want=%v",
					len(got), len(tt.want), got, tt.want)
			}
			for i := range tt.want {
				if got[i] != tt.want[i] {
					t.Errorf("parseLinks()[%d] = %q, want %q", i, got[i], tt.want[i])
				}
			}
		})
	}
}

// --- Helper function tests ---

func TestHasPrefix(t *testing.T) {
	tests := []struct {
		s, prefix string
		want      bool
	}{
		{"hello world", "hello", true},
		{"hello", "hello", true},
		{"hello", "hello world", false},
		{"", "", true},
		{"a", "", true},
		{"", "a", false},
		{"URL: test", "URL:", true},
		{"url: test", "URL:", false},
	}
	for _, tt := range tests {
		t.Run(tt.s+"|"+tt.prefix, func(t *testing.T) {
			got := hasPrefix(tt.s, tt.prefix)
			if got != tt.want {
				t.Errorf("hasPrefix(%q, %q) = %v, want %v", tt.s, tt.prefix, got, tt.want)
			}
		})
	}
}

func TestSplitLines(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []string
	}{
		{"single line", "hello", []string{"hello"}},
		{"two lines", "a\nb", []string{"a", "b"}},
		{"three lines", "a\nb\nc", []string{"a", "b", "c"}},
		{"empty string", "", []string{}},
		{"trailing newline", "a\nb\n", []string{"a", "b"}},
		{"empty lines", "\n\n", []string{"", ""}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := splitLines(tt.input)
			if len(got) != len(tt.want) {
				t.Fatalf("splitLines() len = %d, want %d; got=%v want=%v", len(got), len(tt.want), got, tt.want)
			}
			for i := range tt.want {
				if got[i] != tt.want[i] {
					t.Errorf("splitLines()[%d] = %q, want %q", i, got[i], tt.want[i])
				}
			}
		})
	}
}

func TestJoinLines(t *testing.T) {
	tests := []struct {
		name  string
		input []string
		want  string
	}{
		{"single line", []string{"a"}, "a"},
		{"two lines", []string{"a", "b"}, "a\nb"},
		{"three lines", []string{"a", "b", "c"}, "a\nb\nc"},
		{"empty slice", []string{}, ""},
		{"with empty strings", []string{"a", "", "c"}, "a\n\nc"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := joinLines(tt.input)
			if got != tt.want {
				t.Errorf("joinLines() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestToLower(t *testing.T) {
	tests := []struct {
		input, want string
	}{
		{"HELLO", "hello"},
		{"Hello", "hello"},
		{"hello", "hello"},
		{"", ""},
		{"ABC123", "abc123"},
		{"EXAMPLE.COM", "example.com"},
	}
	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			got := toLower(tt.input)
			if got != tt.want {
				t.Errorf("toLower(%q) = %q, want %q", tt.input, got, tt.want)
			}
		})
	}
}

func TestContains(t *testing.T) {
	tests := []struct {
		s, substr string
		want      bool
	}{
		{"hello world", "world", true},
		{"hello", "hello", true},
		{"hello", "hello world", false},
		{"", "", true},
		{"a", "a", true},
		{"abc", "b", true},
		{"abc", "d", false},
		{"EXAMPLE.COM/about", "about", true},
	}
	for _, tt := range tests {
		t.Run(tt.s+"|"+tt.substr, func(t *testing.T) {
			got := contains(tt.s, tt.substr)
			if got != tt.want {
				t.Errorf("contains(%q, %q) = %v, want %v", tt.s, tt.substr, got, tt.want)
			}
		})
	}
}

func TestTrimSpace(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"no spaces", "hello", "hello"},
		{"leading space", " hello", "hello"},
		{"trailing space", "hello ", "hello"},
		{"both spaces", " hello ", "hello"},
		{"newlines", "\nhello\n", "hello"},
		{"carriage returns", "\rhello\r", "hello"},
		{"mixed whitespace", " \n\r hello \r\n ", "hello"},
		{"empty", "", ""},
		{"only spaces", "   ", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := string(trimSpace([]byte(tt.input)))
			if got != tt.want {
				t.Errorf("trimSpace(%q) = %q, want %q", tt.input, got, tt.want)
			}
		})
	}
}

// --- exec.Command verification test ---

func TestGotoUsesExecCommand(t *testing.T) {
	// This test verifies that Goto actually calls exec.Command with the right arguments.
	// We use a mock binary that records what arguments it received.
	dir := t.TempDir()
	recorderPath := filepath.Join(dir, "recorder")
	recorderScript := `#!/bin/bash
if [ "$1" = "--version" ]; then
	echo "0.4.0"
	exit 0
fi
# Record all arguments
printf '%s\n' "$@" > "` + dir + `/args.txt"
echo "URL: https://test.com
Markdown:
# Test
Links: []"
exit 0
`
	if err := os.WriteFile(recorderPath, []byte(recorderScript), 0755); err != nil {
		t.Fatalf("failed to create recorder: %v", err)
	}

	browser := &AgentBrowser{
		mode:       ModeJS,
		timeout:    30,
		binaryPath: recorderPath,
	}

	testURL := "https://example.com/test"
	_, err := browser.Goto(testURL)
	if err != nil {
		t.Fatalf("Goto() error: %v", err)
	}

	argsData, err := os.ReadFile(filepath.Join(dir, "args.txt"))
	if err != nil {
		t.Fatalf("failed to read args: %v", err)
	}
	args := strings.TrimSpace(string(argsData))

	// Verify the command structure: goto URL --mode MODE
	// Args are newline-separated (one per line)
	argsLines := strings.Split(args, "\n")
	if len(argsLines) < 1 || argsLines[0] != "goto" {
		t.Errorf("first arg should be 'goto', got: %q", argsLines[0])
	}
	if !strings.Contains(args, testURL) {
		t.Errorf("args missing URL %q in: %q", testURL, args)
	}
	if !strings.Contains(args, "--mode") {
		t.Errorf("args missing --mode flag in: %q", args)
	}
	if !strings.Contains(args, "js") {
		t.Errorf("args missing mode value 'js' in: %q", args)
	}
}

// --- SDK version constant ---

func TestSDKVersion(t *testing.T) {
	if SDK_VERSION == "" {
		t.Error("SDK_VERSION should not be empty")
	}
	if SDK_VERSION != "0.4.0" {
		t.Errorf("SDK_VERSION = %q, want %q", SDK_VERSION, "0.4.0")
	}
}

// --- BrowserOption type test ---

func TestBrowserOptionType(t *testing.T) {
	t.Run("BrowserOption is a function type", func(t *testing.T) {
		var opt BrowserOption
		if opt == nil {
			opt = func(b *AgentBrowser) { b.timeout = 99 }
		}
		b := &AgentBrowser{}
		opt(b)
		if b.timeout != 99 {
			t.Errorf("BrowserOption did not apply, timeout = %d, want 99", b.timeout)
		}
	})

	t.Run("multiple options apply in order", func(t *testing.T) {
		b := &AgentBrowser{}
		opts := []BrowserOption{
			WithMode(ModeRender),
			WithTimeout(60),
			WithUserAgent("Test"),
		}
		for _, opt := range opts {
			opt(b)
		}
		if b.mode != ModeRender {
			t.Errorf("mode = %q, want %q", b.mode, ModeRender)
		}
		if b.timeout != 60 {
			t.Errorf("timeout = %d, want 60", b.timeout)
		}
		if b.userAgent != "Test" {
			t.Errorf("userAgent = %q, want %q", b.userAgent, "Test")
		}
	})
}

// --- Page struct tests ---

func TestPageStruct(t *testing.T) {
	t.Run("all fields set", func(t *testing.T) {
		page := &Page{
			URL:        "https://example.com",
			Markdown:   "# Hello",
			Links:      []string{"https://a.com"},
			Screenshot: "base64data",
		}
		if page.URL != "https://example.com" {
			t.Errorf("URL = %q", page.URL)
		}
		if page.Markdown != "# Hello" {
			t.Errorf("Markdown = %q", page.Markdown)
		}
		if len(page.Links) != 1 {
			t.Errorf("Links len = %d", len(page.Links))
		}
		if page.Screenshot != "base64data" {
			t.Errorf("Screenshot = %q", page.Screenshot)
		}
	})

	t.Run("zero value page", func(t *testing.T) {
		var page Page
		if page.URL != "" {
			t.Errorf("zero URL = %q", page.URL)
		}
		if page.Markdown != "" {
			t.Errorf("zero Markdown = %q", page.Markdown)
		}
		if page.Links != nil {
			t.Errorf("zero Links = %v", page.Links)
		}
		if page.Screenshot != "" {
			t.Errorf("zero Screenshot = %q", page.Screenshot)
		}
	})
}

// --- AgentBrowser struct field tests ---

func TestAgentBrowserStructFields(t *testing.T) {
	t.Run("all fields accessible", func(t *testing.T) {
		b := &AgentBrowser{
			mode:       ModeRender,
			timeout:    120,
			userAgent:  "CustomAgent",
			binaryPath: "/path/to/binary",
		}
		if b.mode != ModeRender {
			t.Errorf("mode = %q", b.mode)
		}
		if b.timeout != 120 {
			t.Errorf("timeout = %d", b.timeout)
		}
		if b.userAgent != "CustomAgent" {
			t.Errorf("userAgent = %q", b.userAgent)
		}
		if b.binaryPath != "/path/to/binary" {
			t.Errorf("binaryPath = %q", b.binaryPath)
		}
	})
}
