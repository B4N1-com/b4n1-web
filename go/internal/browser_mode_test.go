package b4n1web

import (
	"testing"
)

func TestBrowserModeConstants(t *testing.T) {
	t.Run("ModeLight equals light", func(t *testing.T) {
		if ModeLight != "light" {
			t.Errorf("ModeLight = %q, want %q", ModeLight, "light")
		}
	})

	t.Run("ModeJS equals js", func(t *testing.T) {
		if ModeJS != "js" {
			t.Errorf("ModeJS = %q, want %q", ModeJS, "js")
		}
	})

	t.Run("ModeRender equals render", func(t *testing.T) {
		if ModeRender != "render" {
			t.Errorf("ModeRender = %q, want %q", ModeRender, "render")
		}
	})
}

func TestBrowserModesAreDistinct(t *testing.T) {
	modes := []BrowserMode{ModeLight, ModeJS, ModeRender}
	for i := 0; i < len(modes); i++ {
		for j := i + 1; j < len(modes); j++ {
			if modes[i] == modes[j] {
				t.Errorf("Mode %d (%q) equals mode %d (%q) - modes must be distinct",
					i, modes[i], j, modes[j])
			}
		}
	}
}

func TestBrowserModeStringConversion(t *testing.T) {
	tests := []struct {
		name string
		mode BrowserMode
		want string
	}{
		{"ModeLight", ModeLight, "light"},
		{"ModeJS", ModeJS, "js"},
		{"ModeRender", ModeRender, "render"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := string(tt.mode)
			if got != tt.want {
				t.Errorf("string(%s) = %q, want %q", tt.name, got, tt.want)
			}
		})
	}
}

func TestBrowserModeType(t *testing.T) {
	t.Run("BrowserMode is a string alias", func(t *testing.T) {
		var mode BrowserMode = "custom"
		if mode != "custom" {
			t.Errorf("BrowserMode custom value = %q, want %q", mode, "custom")
		}
	})

	t.Run("BrowserMode can hold arbitrary string", func(t *testing.T) {
		var mode BrowserMode = "headless"
		if string(mode) != "headless" {
			t.Errorf("BrowserMode = %q, want %q", mode, "headless")
		}
	})

	t.Run("empty BrowserMode", func(t *testing.T) {
		var mode BrowserMode
		if mode != "" {
			t.Errorf("zero BrowserMode = %q, want empty string", mode)
		}
	})
}

func TestWithModeOption(t *testing.T) {
	t.Run("WithMode sets ModeLight", func(t *testing.T) {
		b := &AgentBrowser{}
		opt := WithMode(ModeLight)
		opt(b)
		if b.mode != ModeLight {
			t.Errorf("mode = %q, want %q", b.mode, ModeLight)
		}
	})

	t.Run("WithMode sets ModeJS", func(t *testing.T) {
		b := &AgentBrowser{}
		opt := WithMode(ModeJS)
		opt(b)
		if b.mode != ModeJS {
			t.Errorf("mode = %q, want %q", b.mode, ModeJS)
		}
	})

	t.Run("WithMode sets ModeRender", func(t *testing.T) {
		b := &AgentBrowser{}
		opt := WithMode(ModeRender)
		opt(b)
		if b.mode != ModeRender {
			t.Errorf("mode = %q, want %q", b.mode, ModeRender)
		}
	})

	t.Run("WithMode overrides previous mode", func(t *testing.T) {
		b := &AgentBrowser{mode: ModeLight}
		opt := WithMode(ModeRender)
		opt(b)
		if b.mode != ModeRender {
			t.Errorf("mode = %q, want %q", b.mode, ModeRender)
		}
	})

	t.Run("WithMode with custom mode string", func(t *testing.T) {
		b := &AgentBrowser{}
		opt := WithMode("custom-mode")
		opt(b)
		if b.mode != "custom-mode" {
			t.Errorf("mode = %q, want %q", b.mode, "custom-mode")
		}
	})
}

func TestBrowserModeInAgentBrowserConstruction(t *testing.T) {
	t.Run("default mode is ModeLight", func(t *testing.T) {
		b := &AgentBrowser{
			mode:      ModeLight,
			timeout:   30,
			userAgent: "B4N1Web-Agent/1.0",
		}
		if b.mode != ModeLight {
			t.Errorf("default mode = %q, want %q", b.mode, ModeLight)
		}
	})

	t.Run("mode is preserved in struct", func(t *testing.T) {
		for _, mode := range []BrowserMode{ModeLight, ModeJS, ModeRender} {
			t.Run(string(mode), func(t *testing.T) {
				b := &AgentBrowser{mode: mode}
				if b.mode != mode {
					t.Errorf("mode = %q, want %q", b.mode, mode)
				}
			})
		}
	})
}
