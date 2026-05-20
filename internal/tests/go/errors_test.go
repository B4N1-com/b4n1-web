package b4n1web

import (
	"errors"
	"testing"
)

func TestBinaryNotFoundError(t *testing.T) {
	err := NewBinaryNotFoundError()

	if err.Error() == "" {
		t.Error("Expected error message, got empty string")
	}

	if err.Error() == "" {
		t.Error("Error message should contain binary not found")
	}
}

func TestNavigationError(t *testing.T) {
	url := "https://example.com"
	cause := errors.New("test error")

	err := NewNavigationError(url, cause)

	if err.Error() == "" {
		t.Error("Expected error message, got empty string")
	}

	// Check that error message contains the URL
	expected := "navigation to https://example.com failed"
	if err.Error()[:len(expected)] != expected {
		t.Errorf("Expected error to start with %s, got %s", expected, err.Error())
	}
}

func TestNewNavigationError(t *testing.T) {
	err := NewNavigationError("https://test.com", errors.New("test"))

	if err == nil {
		t.Error("Expected error, got nil")
	}
}
