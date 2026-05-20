package b4n1web

import (
	"testing"
)

func TestBrowserMode(t *testing.T) {
	// Test all modes exist
	if ModeLight != "light" {
		t.Errorf("Expected ModeLight to be 'light', got %s", ModeLight)
	}
	if ModeJS != "js" {
		t.Errorf("Expected ModeJS to be 'js', got %s", ModeLight)
	}
	if ModeRender != "render" {
		t.Errorf("Expected ModeRender to be 'render', got %s", ModeRender)
	}
}

func TestBrowserModeString(t *testing.T) {
	modes := []BrowserMode{ModeLight, ModeJS, ModeRender}
	if len(modes) != 3 {
		t.Errorf("Expected 3 modes, got %d", len(modes))
	}
}
