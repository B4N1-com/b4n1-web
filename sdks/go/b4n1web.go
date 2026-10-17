// Package b4n1web is the Go SDK for the B4N1 agentic browser engine.
//
// It wraps the b4n1web binary: resolve it, run it, parse its text protocol.
//
//	browser := b4n1web.New(b4n1web.Options{Mode: b4n1web.ModeLight})
//	page, err := browser.Goto(ctx, "https://example.com")
//	fmt.Println(page.Markdown)
package b4n1web

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

// SDKVersion is bumped by scripts/bump-version.sh alongside the other SDKs.
const SDKVersion = "0.16.0"

// BrowserMode selects how the binary fetches the page.
type BrowserMode string

const (
	ModeLight  BrowserMode = "light"  // fast HTTP fetch, no JS (default)
	ModeJs     BrowserMode = "js"     // headless Chromium with JavaScript
	ModeRender BrowserMode = "render" // full render + wait-for selector
)

// currentGOOS is runtime.GOOS behind a variable so tests can exercise the
// Windows-specific branches on any build platform (no production behavior
// change: it is never assigned outside of tests).
var currentGOOS = runtime.GOOS

// Options configures an AgentBrowser.
type Options struct {
	Mode    BrowserMode   // default ModeLight
	Timeout time.Duration // default 30s; kills the binary on expiry
	Binary  string        // explicit path; skips auto-resolution when set

	// Runner is injectable for tests: argv + timeout in, stdout/stderr/exit
	// (exit = -1 means timeout/killed). nil uses the real exec runner.
	Runner func(argv []string, timeout time.Duration) (stdout, stderr string, exitCode int)
}

// Errors (fail loud, no silent fallbacks).
var (
	// ErrBinaryNotFound: nothing resolved (env/bundled/system/PATH).
	ErrBinaryNotFound = fmt.Errorf("b4n1web binary not found")
	// ErrTimeout: the binary exceeded Options.Timeout and was killed.
	ErrTimeout = fmt.Errorf("b4n1web timed out")
)

// Page is structured page data returned by the binary.
type Page struct {
	URL        string
	Markdown   string
	Links      []string
	Screenshot string // "" when absent
}

// GetMainContent applies the main-content heuristic (drops the first two
// lines: title + empty line).
func (p *Page) GetMainContent() string {
	lines := strings.Split(p.Markdown, "\n")
	if len(lines) > 2 {
		lines = lines[2:]
	}
	return strings.TrimSpace(strings.Join(lines, "\n"))
}

// FindLinksByText returns links containing text (case-insensitive).
func (p *Page) FindLinksByText(text string) []string {
	if text == "" {
		return []string{}
	}
	needle := strings.ToLower(text)
	found := []string{}
	for _, link := range p.Links {
		if strings.Contains(strings.ToLower(link), needle) {
			found = append(found, link)
		}
	}
	return found
}

// ParsePage parses the binary's text protocol:
//
//	URL: <url>
//	Markdown:
//	<free-form markdown lines...>
//	Links: ["https://...", ...]
//	Screenshot: <path>            (optional)
func ParsePage(url, output string) (*Page, error) {
	var markdown strings.Builder
	var links []string
	screenshot := ""
	inMarkdown := false

	for _, line := range strings.Split(output, "\n") {
		line = strings.TrimRight(line, "\r")
		switch {
		case strings.HasPrefix(line, "URL:"):
			// informational; the caller's url is authoritative
		case strings.HasPrefix(line, "Markdown:"):
			inMarkdown = true
			if rest := strings.TrimSpace(strings.TrimPrefix(line, "Markdown:")); rest != "" {
				markdown.WriteString(rest)
				markdown.WriteString("\n")
			}
		case strings.HasPrefix(line, "Links:"):
			inMarkdown = false
			raw := strings.TrimSpace(strings.TrimPrefix(line, "Links:"))
			links = nil
			if raw != "" {
				if err := json.Unmarshal([]byte(raw), &links); err != nil {
					return nil, fmt.Errorf("b4n1web: malformed Links payload: %w", err)
				}
			}
			if links == nil {
				links = []string{}
			}
		case strings.HasPrefix(line, "Screenshot:"):
			inMarkdown = false
			screenshot = strings.TrimSpace(strings.TrimPrefix(line, "Screenshot:"))
		default:
			if inMarkdown {
				markdown.WriteString(line)
				markdown.WriteString("\n")
			}
		}
	}

	return &Page{
		URL:        url,
		Markdown:   strings.TrimSpace(markdown.String()),
		Links:      links,
		Screenshot: screenshot,
	}, nil
}

// resolveBinary mirrors the other SDKs' order:
//
//  1. explicit override (Options.Binary)
//  2. B4N1WEB_BIN_PATH env (must exist + be executable)
//  3. B4N1WEB_BINARY env + well-known user/system locations
//  4. PATH scan
func resolveBinary(explicit string) (string, error) {
	if explicit != "" {
		// Explicit override is taken as-is (parity with the Ruby/PHP SDKs):
		// a missing binary surfaces as a loud spawn error at run time.
		return explicit, nil
	}

	if env := os.Getenv("B4N1WEB_BIN_PATH"); env != "" && runnable(env) {
		return env, nil
	}

	candidates := []string{}
	if binaryEnv := os.Getenv("B4N1WEB_BINARY"); binaryEnv != "" {
		candidates = append(candidates, binaryEnv)
	}
	if home, err := os.UserHomeDir(); err == nil && home != "" {
		candidates = append(candidates,
			filepath.Join(home, ".local", "bin", "b4n1web"),
			filepath.Join(home, ".b4n1web", "bin", "b4n1web"),
		)
	}
	candidates = append(candidates, "/usr/local/bin/b4n1web", "/usr/bin/b4n1web")

	exe := "b4n1web"
	if currentGOOS == "windows" {
		exe += ".exe"
	}
	for _, dir := range strings.Split(os.Getenv("PATH"), string(os.PathListSeparator)) {
		if dir != "" {
			candidates = append(candidates, filepath.Join(dir, exe))
		}
	}

	for _, candidate := range candidates {
		if runnable(candidate) {
			return candidate, nil
		}
	}
	return "", fmt.Errorf("%w. Install it (curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash) or set B4N1WEB_BIN_PATH", ErrBinaryNotFound)
}

func runnable(path string) bool {
	info, err := os.Stat(path)
	if err != nil || info.IsDir() {
		return false
	}
	if currentGOOS == "windows" {
		return true
	}
	return info.Mode()&0o111 != 0
}

// runProcess is the real runner: ctx-aware exec with a hard timeout.
func runProcess(argv []string, timeout time.Duration) (string, string, int) {
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	cmd := exec.CommandContext(ctx, argv[0], argv[1:]...)
	prepareProcess(cmd)
	// Bound post-exit I/O wait: a grandchild inheriting stdout/stderr could
	// otherwise block Run() until ITS exit (observed: sleep holding the pipe
	// for the full 30s). After the delay the pipes are force-closed.
	cmd.WaitDelay = 250 * time.Millisecond
	var stdout, stderr strings.Builder
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	err := cmd.Run()

	if ctx.Err() == context.DeadlineExceeded {
		// Reap grandchildren still alive after the leader was killed.
		killProcessGroup(cmd)
		return stdout.String(), stderr.String(), -1 // timeout sentinel
	}
	exitCode := 0
	if err != nil {
		var exitErr *exec.ExitError
		if errors.As(err, &exitErr) {
			exitCode = exitErr.ExitCode()
		} else {
			// Spawn failure (missing binary, permissions...): surface the
			// exec error text so the caller's loud failure explains itself.
			return stdout.String(), err.Error(), -2
		}
	}
	return stdout.String(), stderr.String(), exitCode
}

// AgentBrowser is a thin wrapper over the b4n1web binary.
type AgentBrowser struct {
	opts   Options
	binary string
}

// New resolves the binary (or uses opts.Binary) and returns a browser.
// It returns ErrBinaryNotFound when nothing resolves.
func New(opts Options) (*AgentBrowser, error) {
	if opts.Mode == "" {
		opts.Mode = ModeLight
	}
	if opts.Timeout <= 0 {
		opts.Timeout = 30 * time.Second
	}
	if opts.Runner == nil {
		opts.Runner = runProcess
	}
	binary, err := resolveBinary(opts.Binary)
	if err != nil {
		return nil, err
	}
	return &AgentBrowser{opts: opts, binary: binary}, nil
}

// BinaryPath returns the resolved binary path.
func (b *AgentBrowser) BinaryPath() string { return b.binary }

// Goto navigates to url and extracts the page. waitFor is an optional CSS
// selector (render mode). Redirects are NOT followed by the caller beyond
// what the binary itself does: evidence comes from the binary (AGENTS §9).
func (b *AgentBrowser) Goto(ctx context.Context, url, waitFor string) (*Page, error) {
	argv := []string{b.binary, "goto", url, "--mode", string(b.opts.Mode)}
	if waitFor != "" {
		argv = append(argv, "--wait-for", waitFor)
	}
	stdout, stderr, exitCode := b.opts.Runner(argv, b.opts.Timeout)
	if exitCode == -1 {
		return nil, fmt.Errorf("%w after %s", ErrTimeout, b.opts.Timeout)
	}
	if exitCode != 0 {
		return nil, fmt.Errorf("b4n1web exited %d: %s", exitCode, strings.TrimSpace(stderr))
	}
	return ParsePage(url, stdout)
}

// Evaluate executes JavaScript in the page context and returns the raw result.
func (b *AgentBrowser) Evaluate(ctx context.Context, url, js string) (string, error) {
	argv := []string{b.binary, "evaluate", url, js}
	stdout, stderr, exitCode := b.opts.Runner(argv, b.opts.Timeout)
	if exitCode == -1 {
		return "", fmt.Errorf("%w after %s", ErrTimeout, b.opts.Timeout)
	}
	if exitCode != 0 {
		return "", fmt.Errorf("b4n1web exited %d: %s", exitCode, strings.TrimSpace(stderr))
	}
	return strings.TrimSpace(stdout), nil
}

// Close is a no-op (the binary is stateless); kept for API parity.
func (b *AgentBrowser) Close() error { return nil }
