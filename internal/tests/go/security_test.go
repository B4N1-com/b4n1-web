package b4n1web

import (
	"testing"
)

func TestSecurityShieldNew(t *testing.T) {
	shield := NewSecurityShield()
	if shield == nil {
		t.Error("Expected SecurityShield, got nil")
	}
}

func TestSecurityShieldOptions(t *testing.T) {
	shield := NewSecurityShield(WithCacheDays(30))
	if shield == nil {
		t.Error("Expected SecurityShield, got nil")
	}
}

func TestSecurityShieldIsUrlSafe(t *testing.T) {
	shield := NewSecurityShield()

	// New domain should be safe but need check
	result, _ := shield.IsUrlSafe("https://newdomain.com")
	if !result.IsSafe {
		t.Error("Expected new domain to be safe")
	}
	if !result.NeedsApiCheck {
		t.Error("Expected new domain to need API check")
	}
}

func TestSecurityShieldIsUrlSafeInvalid(t *testing.T) {
	shield := NewSecurityShield()

	// Invalid URL should be safe with no check
	result, _ := shield.IsUrlSafe("not-a-valid-url")
	if !result.IsSafe {
		t.Error("Expected invalid URL to be safe")
	}
	if result.NeedsApiCheck {
		t.Error("Expected invalid URL to not need API check")
	}
}

func TestSecurityShieldMarkDomain(t *testing.T) {
	shield := NewSecurityShield()

	// Mark domain as safe
	shield.MarkDomain("example.com", true)
	result, _ := shield.IsUrlSafe("https://example.com")

	if !result.IsSafe {
		t.Error("Expected domain to be safe")
	}
	if result.NeedsApiCheck {
		t.Error("Expected cached domain to not need API check")
	}
}

func TestSecurityShieldMarkDomainUnsafe(t *testing.T) {
	shield := NewSecurityShield()

	// Mark domain as unsafe
	shield.MarkDomain("malware.com", false)
	result, _ := shield.IsUrlSafe("https://malware.com")

	if result.IsSafe {
		t.Error("Expected domain to be unsafe")
	}
}

func TestSecurityShieldClearCache(t *testing.T) {
	shield := NewSecurityShield()

	// Mark domains and clear
	shield.MarkDomain("a.com", true)
	shield.MarkDomain("b.com", true)
	shield.ClearCache()

	// After clear, should need API check again
	result, _ := shield.IsUrlSafe("https://a.com")
	if result.NeedsApiCheck {
		t.Error("Expected cleared domain to need API check")
	}
}

func TestNavigate(t *testing.T) {
	// Navigate should return error for non-existent binary
	// This will fail since there's no binary, but should not panic
	_, err := Navigate("https://example.com", WithTimeout(1))
	if err == nil {
		t.Error("Expected error for missing binary")
	}
}
