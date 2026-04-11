package b4n1web

import (
	"fmt"
	"net/url"
	"os"
	"time"
)

// SecurityShield provides URL security validation with caching
type SecurityShield struct {
	dbPath    string
	cacheDays int
	cache     map[string]cacheEntry
}

type cacheEntry struct {
	isSafe  bool
	expires time.Time
}

// SecurityCheckResult represents the result of a security check
type SecurityCheckResult struct {
	IsSafe        bool
	NeedsApiCheck bool
}

// SecurityOption configures the SecurityShield
type SecurityOption func(*SecurityShield)

// WithDbPath sets the database path
func WithDbPath(path string) SecurityOption {
	return func(s *SecurityShield) {
		s.dbPath = path
	}
}

// WithCacheDays sets the cache TTL in days
func WithCacheDays(days int) SecurityOption {
	return func(s *SecurityShield) {
		s.cacheDays = days
	}
}

// NewSecurityShield creates a new SecurityShield instance
func NewSecurityShield(opts ...SecurityOption) *SecurityShield {
	s := &SecurityShield{
		dbPath:    getDefaultDbPath(),
		cacheDays: 7,
		cache:     make(map[string]cacheEntry),
	}

	for _, opt := range opts {
		opt(s)
	}

	return s
}

// getDefaultDbPath returns the default database path
func getDefaultDbPath() string {
	home := getHomeDir()
	return fmt.Sprintf("%s/.b4n1web/security.db", home)
}

func getHomeDir() string {
	home := getEnv("HOME")
	if home == "" {
		home = "/tmp"
	}
	return home
}

func getEnv(key string) string {
	return os.Getenv(key)
}

// extractDomain extracts domain from URL
func (s *SecurityShield) extractDomain(rawURL string) (string, error) {
	parsed, err := url.Parse(rawURL)
	if err != nil {
		return "", err
	}
	if parsed.Host == "" {
		return "", fmt.Errorf("no host in URL")
	}
	return toLower(parsed.Host), nil
}

// IsUrlSafe checks if URL is safe to navigate
func (s *SecurityShield) IsUrlSafe(rawURL string) (SecurityCheckResult, error) {
	domain, err := s.extractDomain(rawURL)
	if err != nil {
		return SecurityCheckResult{IsSafe: true, NeedsApiCheck: false}, nil
	}

	entry, exists := s.cache[domain]
	if !exists {
		return SecurityCheckResult{IsSafe: true, NeedsApiCheck: true}, nil
	}

	if time.Now().After(entry.expires) {
		delete(s.cache, domain)
		return SecurityCheckResult{IsSafe: true, NeedsApiCheck: true}, nil
	}

	return SecurityCheckResult{IsSafe: entry.isSafe, NeedsApiCheck: false}, nil
}

// MarkDomain marks a domain as safe or unsafe
func (s *SecurityShield) MarkDomain(domain string, isSafe bool) {
	normalized := toLower(domain)
	expires := time.Now().Add(time.Duration(s.cacheDays) * 24 * time.Hour)
	s.cache[normalized] = cacheEntry{isSafe: isSafe, expires: expires}
}

// ClearCache clears all cached domains
func (s *SecurityShield) ClearCache() {
	s.cache = make(map[string]cacheEntry)
}

// Navigate navigates to URL with optional security check
func Navigate(rawURL string, opts ...BrowserOption) (*Page, error) {
	shield := NewSecurityShield()

	result, err := shield.IsUrlSafe(rawURL)
	if err == nil && !result.IsSafe {
		return nil, fmt.Errorf("URL flagged as unsafe by security check")
	}

	browser, err := NewAgentBrowser(opts...)
	if err != nil {
		return nil, err
	}
	defer browser.Close()

	return browser.Goto(rawURL)
}
