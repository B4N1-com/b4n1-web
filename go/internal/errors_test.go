package b4n1web

import (
	"errors"
	"strings"
	"testing"
)

func TestBinaryNotFoundErrorDefaultMessage(t *testing.T) {
	err := NewBinaryNotFoundError()
	if err == nil {
		t.Fatal("Error is nil")
	}
	if !strings.Contains(err.Error(), "binary not found") {
		t.Errorf("Error doesn't mention binary: %q", err.Error())
	}
}

func TestBinaryNotFoundErrorImplementsError(t *testing.T) {
	var e error = NewBinaryNotFoundError()
	if e.Error() == "" {
		t.Error("Error() is empty")
	}
}

func TestNavigationError(t *testing.T) {
	inner := errors.New("connection failed")
	err := NewNavigationError("https://example.com", inner)
	if err == nil {
		t.Fatal("Error is nil")
	}
	if !strings.Contains(err.Error(), "https://example.com") {
		t.Errorf("Error missing URL: %q", err.Error())
	}
	if !strings.Contains(err.Error(), "connection failed") {
		t.Errorf("Error missing inner error: %q", err.Error())
	}
}

func TestNavigationErrorUnwrap(t *testing.T) {
	inner := errors.New("test")
	err := NewNavigationError("https://x.com", inner)
	unwrapped := err.Unwrap()
	if unwrapped != inner {
		t.Error("Unwrap doesn't return inner error")
	}
}

func TestErrorsDistinguishable(t *testing.T) {
	bnf := NewBinaryNotFoundError()
	nav := NewNavigationError("u", errors.New("e"))
	if bnf.Error() == nav.Error() {
		t.Error("Errors should be different")
	}
}
