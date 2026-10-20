package b4n1web

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

const sampleOutput = "URL: https://example.com\n" +
	"Markdown:\n" +
	"# Example Domain\n" +
	"\n" +
	"This domain is for use in documentation examples.\n" +
	"Links: [\"https://example.com/a\", \"https://other.org/b\", \"/relative\"]\n" +
	"Screenshot: /tmp/shot.png\n"

// --- ParsePage ---------------------------------------------------------------

func TestParsePageFull(t *testing.T) {
	page, err := ParsePage("https://example.com", sampleOutput)
	if err != nil {
		t.Fatalf("ParsePage: %v", err)
	}
	if page.URL != "https://example.com" {
		t.Errorf("URL = %q", page.URL)
	}
	if !strings.Contains(page.Markdown, "# Example Domain") {
		t.Errorf("markdown missing title: %q", page.Markdown)
	}
	if len(page.Links) != 3 {
		t.Errorf("links = %d, want 3", len(page.Links))
	}
	if page.Screenshot != "/tmp/shot.png" {
		t.Errorf("screenshot = %q", page.Screenshot)
	}
}

func TestParsePageNoScreenshot(t *testing.T) {
	page, err := ParsePage("https://x.io", "URL: https://x.io\nMarkdown:\ncontent\nLinks: []\n")
	if err != nil {
		t.Fatalf("ParsePage: %v", err)
	}
	if page.Screenshot != "" {
		t.Errorf("screenshot = %q, want empty", page.Screenshot)
	}
	if len(page.Links) != 0 {
		t.Errorf("links = %d, want 0", len(page.Links))
	}
}

func TestParsePageMalformedLinks(t *testing.T) {
	_, err := ParsePage("u", "Markdown:\nx\nLinks: not-json\n")
	if err == nil {
		t.Fatal("malformed Links must fail loud (no silent fallback)")
	}
	if !strings.Contains(err.Error(), "Links") {
		t.Errorf("error should name the payload: %v", err)
	}
}

func TestParsePageCRLF(t *testing.T) {
	page, err := ParsePage("u", "URL: u\r\nMarkdown:\r\nline\r\nLinks: []\r\n")
	if err != nil {
		t.Fatalf("ParsePage CRLF: %v", err)
	}
	if page.Markdown != "line" {
		t.Errorf("markdown = %q, want %q", page.Markdown, "line")
	}
}

func TestGetMainContent(t *testing.T) {
	page, _ := ParsePage("u", sampleOutput)
	if strings.Contains(page.GetMainContent(), "# Example Domain") {
		t.Error("main content should drop the first two lines")
	}
	short := &Page{URL: "u", Markdown: "only"}
	if short.GetMainContent() != "only" {
		t.Errorf("short markdown = %q", short.GetMainContent())
	}
}

func TestFindLinksByText(t *testing.T) {
	page, _ := ParsePage("u", sampleOutput)
	got := page.FindLinksByText("EXAMPLE")
	if len(got) != 1 || got[0] != "https://example.com/a" {
		t.Errorf("FindLinksByText = %v", got)
	}
	if empty := page.FindLinksByText(""); len(empty) != 0 {
		t.Errorf("empty needle = %v, want []", empty)
	}
}

// --- resolveBinary -----------------------------------------------------------

func TestResolveBinaryEnvOverride(t *testing.T) {
	fake := filepath.Join(t.TempDir(), "fake-bin")
	if err := os.WriteFile(fake, []byte("#!/bin/sh\n"), 0o755); err != nil {
		t.Fatal(err)
	}
	t.Setenv("B4N1WEB_BIN_PATH", fake)
	got, err := resolveBinary("")
	if err != nil {
		t.Fatalf("resolve: %v", err)
	}
	if got != fake {
		t.Errorf("got %q, want %q", got, fake)
	}
}

func TestResolveBinaryEnvMissingFallsThrough(t *testing.T) {
	t.Setenv("B4N1WEB_BIN_PATH", "/nonexistent/b4n1web")
	t.Setenv("B4N1WEB_BINARY", "")
	// May find a system binary; must not error out.
	_, _ = resolveBinary("")
}

func TestResolveBinaryNotFound(t *testing.T) {
	empty := t.TempDir()
	t.Setenv("B4N1WEB_BIN_PATH", "")
	t.Setenv("B4N1WEB_BINARY", "")
	t.Setenv("PATH", empty)
	t.Setenv("HOME", empty)
	// USERHOME override for os.UserHomeDir on unix uses $HOME.
	_, err := resolveBinary("")
	if err == nil {
		t.Fatal("expected ErrBinaryNotFound in an empty environment")
	}
	if !errors.Is(err, ErrBinaryNotFound) {
		t.Errorf("err = %v, want ErrBinaryNotFound", err)
	}
	if !strings.Contains(err.Error(), "B4N1WEB_BIN_PATH") {
		t.Errorf("error should carry the install hint: %v", err)
	}
}

func TestResolveBinaryExplicitPassthrough(t *testing.T) {
	// Explicit override is taken as-is (parity with Ruby/PHP): no existence
	// check at resolve time; a missing binary fails loud at spawn instead.
	explicit := filepath.Join(t.TempDir(), "missing")
	got, err := resolveBinary(explicit)
	if err != nil {
		t.Fatalf("resolveBinary(explicit) must not error: %v", err)
	}
	if got != explicit {
		t.Errorf("got %q, want %q", got, explicit)
	}
}

// mustNew fails the test when New cannot resolve a browser.
func mustNew(t *testing.T, opts Options) *AgentBrowser {
	t.Helper()
	b, err := New(opts)
	if err != nil {
		t.Fatalf("New: %v", err)
	}
	return b
}

func TestNewWithoutBinaryFailsLoud(t *testing.T) {
	empty := t.TempDir()
	t.Setenv("B4N1WEB_BIN_PATH", "")
	t.Setenv("B4N1WEB_BINARY", "")
	t.Setenv("PATH", empty)
	t.Setenv("HOME", empty)
	_, err := New(Options{})
	if !errors.Is(err, ErrBinaryNotFound) {
		t.Errorf("New = %v, want ErrBinaryNotFound", err)
	}
}

// --- AgentBrowser with injected runner ---------------------------------------

func fakeRunner(stdout, stderr string, exitCode int, capture *[]string) func([]string, time.Duration) (string, string, int) {
	return func(argv []string, _ time.Duration) (string, string, int) {
		if capture != nil {
			*capture = append([]string{}, argv...)
		}
		return stdout, stderr, exitCode
	}
}

func TestGotoParsesOutput(t *testing.T) {
	var argv []string
	b := mustNew(t, Options{
		Binary: "/fake/b4n1web",
		Runner: fakeRunner(sampleOutput, "", 0, &argv),
	})
	page, err := b.Goto(context.Background(), "https://example.com", "")
	if err != nil {
		t.Fatalf("Goto: %v", err)
	}
	if page.URL != "https://example.com" {
		t.Errorf("URL = %q", page.URL)
	}
	want := []string{"/fake/b4n1web", "goto", "https://example.com", "--mode", "light"}
	if strings.Join(argv, " ") != strings.Join(want, " ") {
		t.Errorf("argv = %v, want %v", argv, want)
	}
}

func TestGotoWithWaitFor(t *testing.T) {
	var argv []string
	b := mustNew(t, Options{Binary: "/fake/b4n1web", Runner: fakeRunner(sampleOutput, "", 0, &argv)})
	if _, err := b.Goto(context.Background(), "https://x.io", "#ready"); err != nil {
		t.Fatal(err)
	}
	joined := strings.Join(argv, " ")
	if !strings.HasSuffix(joined, "--wait-for #ready") {
		t.Errorf("argv = %v, want --wait-for suffix", argv)
	}
}

func TestGotoNonZeroExit(t *testing.T) {
	b := mustNew(t, Options{Binary: "/fake/b4n1web", Runner: fakeRunner("", "boom", 3, nil)})
	_, err := b.Goto(context.Background(), "https://x.io", "")
	if err == nil {
		t.Fatal("non-zero exit must fail loud")
	}
	if !strings.Contains(err.Error(), "3") || !strings.Contains(err.Error(), "boom") {
		t.Errorf("error should carry exit code + stderr: %v", err)
	}
}

func TestGotoTimeout(t *testing.T) {
	b := mustNew(t, Options{Binary: "/fake/b4n1web", Runner: fakeRunner("", "", -1, nil)})
	_, err := b.Goto(context.Background(), "https://x.io", "")
	if !errors.Is(err, ErrTimeout) {
		t.Errorf("err = %v, want ErrTimeout", err)
	}
}

func TestEvaluate(t *testing.T) {
	var argv []string
	b := mustNew(t, Options{Binary: "/fake/b4n1web", Runner: fakeRunner("42\n", "", 0, &argv)})
	got, err := b.Evaluate(context.Background(), "https://x.io", "6*7")
	if err != nil {
		t.Fatalf("Evaluate: %v", err)
	}
	if got != "42" {
		t.Errorf("got %q, want 42", got)
	}
	want := "/fake/b4n1web evaluate https://x.io 6*7"
	if strings.Join(argv, " ") != want {
		t.Errorf("argv = %v, want %q", argv, want)
	}
}

func TestEvaluateNonZeroExit(t *testing.T) {
	b := mustNew(t, Options{Binary: "/fake/b4n1web", Runner: fakeRunner("", "js error", 1, nil)})
	if _, err := b.Evaluate(context.Background(), "u", "x"); err == nil {
		t.Fatal("non-zero exit must fail loud")
	}
}

func TestCloseNoop(t *testing.T) {
	b := mustNew(t, Options{Binary: "/fake/b4n1web", Runner: fakeRunner("", "", 0, nil)})
	if err := b.Close(); err != nil {
		t.Errorf("Close = %v, want nil", err)
	}
}

func TestDefaultsApplied(t *testing.T) {
	b, err := New(Options{Binary: "/fake/b4n1web", Runner: fakeRunner("", "", 0, nil)})
	if err != nil {
		t.Fatal(err)
	}
	if b.opts.Mode != ModeLight {
		t.Errorf("mode = %q, want light", b.opts.Mode)
	}
	if b.opts.Timeout != 30*time.Second {
		t.Errorf("timeout = %s, want 30s", b.opts.Timeout)
	}
}

func TestBinaryPath(t *testing.T) {
	b, err := New(Options{Binary: "/fake/b4n1web", Runner: fakeRunner("", "", 0, nil)})
	if err != nil {
		t.Fatal(err)
	}
	if b.BinaryPath() != "/fake/b4n1web" {
		t.Errorf("BinaryPath = %q", b.BinaryPath())
	}
}

func TestSDKVersionFormat(t *testing.T) {
	if SDKVersion == "" || !strings.Contains(SDKVersion, ".") {
		t.Errorf("SDKVersion = %q, want X.Y.Z", SDKVersion)
	}
}

// --- Real binary smoke (when resolvable) -------------------------------------

func TestRealBinarySmoke(t *testing.T) {
	b, err := New(Options{Timeout: 60 * time.Second})
	if err != nil {
		t.Skipf("no b4n1web binary resolvable here: %v", err)
	}
	page, err := b.Goto(context.Background(), "https://example.com", "")
	if err != nil {
		t.Fatalf("real Goto: %v", err)
	}
	if page.URL != "https://example.com" {
		t.Errorf("URL = %q", page.URL)
	}
	if len(page.Markdown) == 0 {
		t.Error("markdown is empty")
	}
}
