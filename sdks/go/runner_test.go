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

// --- runProcess (the real exec runner) ---------------------------------------

func TestRunProcessRealSuccess(t *testing.T) {
	stdout, stderr, code := runProcess([]string{"/bin/sh", "-c", "echo hi"}, 5*time.Second)
	if code != 0 {
		t.Fatalf("exit = %d (stderr %q), want 0", code, stderr)
	}
	if strings.TrimSpace(stdout) != "hi" {
		t.Errorf("stdout = %q, want hi", stdout)
	}
}

func TestRunProcessRealExitCode(t *testing.T) {
	stdout, stderr, code := runProcess([]string{"/bin/sh", "-c", "echo boom >&2; exit 3"}, 5*time.Second)
	if code != 3 {
		t.Errorf("exit = %d, want 3", code)
	}
	if !strings.Contains(stderr, "boom") {
		t.Errorf("stderr = %q, want boom", stderr)
	}
	_ = stdout
}

func TestRunProcessRealTimeout(t *testing.T) {
	start := time.Now()
	_, _, code := runProcess([]string{"/bin/sh", "-c", "sleep 30"}, 300*time.Millisecond)
	if code != -1 {
		t.Fatalf("exit = %d, want -1 (timeout sentinel)", code)
	}
	if elapsed := time.Since(start); elapsed > 5*time.Second {
		t.Errorf("kill took too long: %s", elapsed)
	}
}

func TestRunProcessSpawnFailure(t *testing.T) {
	_, stderr, code := runProcess([]string{filepath.Join(t.TempDir(), "missing-bin")}, time.Second)
	if code != -2 {
		t.Fatalf("exit = %d, want -2 (spawn failure)", code)
	}
	if stderr == "" {
		t.Error("spawn failure must surface the exec error text")
	}
}

// --- resolveBinary: remaining candidates -------------------------------------

func TestResolveBinaryB4n1WebBinaryEnv(t *testing.T) {
	fake := filepath.Join(t.TempDir(), "b4n1web")
	if err := os.WriteFile(fake, []byte("#!/bin/sh\n"), 0o755); err != nil {
		t.Fatal(err)
	}
	t.Setenv("B4N1WEB_BIN_PATH", "")
	t.Setenv("B4N1WEB_BINARY", fake)
	got, err := resolveBinary("")
	if err != nil {
		t.Fatalf("resolve: %v", err)
	}
	if got != fake {
		t.Errorf("got %q, want %q", got, fake)
	}
}

func TestResolveBinaryHomeCandidate(t *testing.T) {
	home := t.TempDir()
	binDir := filepath.Join(home, ".local", "bin")
	if err := os.MkdirAll(binDir, 0o755); err != nil {
		t.Fatal(err)
	}
	fake := filepath.Join(binDir, "b4n1web")
	if err := os.WriteFile(fake, []byte("#!/bin/sh\n"), 0o755); err != nil {
		t.Fatal(err)
	}
	t.Setenv("B4N1WEB_BIN_PATH", "")
	t.Setenv("B4N1WEB_BINARY", "")
	t.Setenv("HOME", home)
	got, err := resolveBinary("")
	if err != nil {
		t.Fatalf("resolve: %v", err)
	}
	if got != fake {
		t.Errorf("got %q, want home candidate %q", got, fake)
	}
}

func TestResolveBinaryPATHScan(t *testing.T) {
	binDir := t.TempDir()
	fake := filepath.Join(binDir, "b4n1web")
	if err := os.WriteFile(fake, []byte("#!/bin/sh\n"), 0o755); err != nil {
		t.Fatal(err)
	}
	t.Setenv("B4N1WEB_BIN_PATH", "")
	t.Setenv("B4N1WEB_BINARY", "")
	t.Setenv("HOME", t.TempDir())
	t.Setenv("PATH", binDir)
	got, err := resolveBinary("")
	if err != nil {
		t.Fatalf("resolve: %v", err)
	}
	if got != fake {
		t.Errorf("got %q, want PATH candidate %q", got, fake)
	}
}

func TestRunnableRejectsDirectory(t *testing.T) {
	if runnable(t.TempDir()) {
		t.Error("a directory must not count as runnable")
	}
}

func TestRunnableRejectsNonExecutable(t *testing.T) {
	plain := filepath.Join(t.TempDir(), "plain")
	if err := os.WriteFile(plain, []byte("data"), 0o644); err != nil {
		t.Fatal(err)
	}
	if runnable(plain) {
		t.Error("a non-executable file must not count as runnable")
	}
}

// --- Evaluate: timeout branch -------------------------------------------------

func TestEvaluateTimeout(t *testing.T) {
	b := mustNew(t, Options{Binary: "/fake/b4n1web", Runner: fakeRunner("", "", -1, nil)})
	_, err := b.Evaluate(context.Background(), "u", "x")
	if !errors.Is(err, ErrTimeout) {
		t.Errorf("err = %v, want ErrTimeout", err)
	}
}

// --- ParsePage: remaining branches -------------------------------------------

func TestParsePageLooseLinesBeforeMarkdown(t *testing.T) {
	// Lines outside the markdown block are dropped (no silent accumulation).
	page, err := ParsePage("u", "garbage line\nMarkdown:\nx\nLinks:\n")
	if err != nil {
		t.Fatalf("ParsePage: %v", err)
	}
	if page.Markdown != "x" {
		t.Errorf("markdown = %q, want x", page.Markdown)
	}
	if page.Links == nil || len(page.Links) != 0 {
		t.Errorf("links = %#v, want empty non-nil slice", page.Links)
	}
}

func TestParsePageMarkdownMarkerWithoutRest(t *testing.T) {
	page, err := ParsePage("u", "Markdown:\nLinks: []\n")
	if err != nil {
		t.Fatalf("ParsePage: %v", err)
	}
	if page.Markdown != "" {
		t.Errorf("markdown = %q, want empty", page.Markdown)
	}
}

// --- Windows branches (exercised on linux via currentGOOS override) ----------

func TestResolveBinaryWindowsEXESuffix(t *testing.T) {
	binDir := t.TempDir()
	fake := filepath.Join(binDir, "b4n1web.exe")
	if err := os.WriteFile(fake, []byte("MZ"), 0o755); err != nil {
		t.Fatal(err)
	}
	restore := currentGOOS
	currentGOOS = "windows"
	t.Cleanup(func() { currentGOOS = restore })

	t.Setenv("B4N1WEB_BIN_PATH", "")
	t.Setenv("B4N1WEB_BINARY", "")
	t.Setenv("HOME", t.TempDir())
	t.Setenv("PATH", binDir)
	got, err := resolveBinary("")
	if err != nil {
		t.Fatalf("resolve: %v", err)
	}
	if got != fake {
		t.Errorf("got %q, want %q", got, fake)
	}
}

func TestRunnableWindowsSkipsExecBit(t *testing.T) {
	restore := currentGOOS
	currentGOOS = "windows"
	t.Cleanup(func() { currentGOOS = restore })

	plain := filepath.Join(t.TempDir(), "plain.exe")
	if err := os.WriteFile(plain, []byte("MZ"), 0o644); err != nil {
		t.Fatal(err)
	}
	if !runnable(plain) {
		t.Error("windows has no exec bit: a plain file must be runnable")
	}
}

func TestParsePageInlineContentAfterMarker(t *testing.T) {
	// Content on the very same "Markdown:" line must be kept (the binary
	// emits it when the payload fits on one line).
	page, err := ParsePage("u", "Markdown: inline body\nLinks: []\n")
	if err != nil {
		t.Fatalf("ParsePage: %v", err)
	}
	if page.Markdown != "inline body" {
		t.Errorf("markdown = %q, want %q", page.Markdown, "inline body")
	}
}
