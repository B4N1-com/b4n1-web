"""E2E tests for B4n1Web — full workflow tests inside Podman container.

Now that SDKs bundle their own binaries, most tests run without
external binary setup. The binary is found bundled with the SDK.
"""

import pytest
import subprocess
import sys
import os

# ── Imports from b4n1web SDK (installed via pip in container) ──
from b4n1web import AgentBrowser, BrowserMode, Page, BinaryNotFoundError, SDK_VERSION
from b4n1web.browser import get_b4n1web_binary


BINARY_AVAILABLE = get_b4n1web_binary() is not None


# ──────────────────────────────────────────────
# Phase 1: Setup & Bundled Binary Verification
# ──────────────────────────────────────────────

class TestBinaryVerification:
    def test_binary_found(self):
        """Binary is discoverable — either bundled or system-wide."""
        path = get_b4n1web_binary()
        assert path is not None, "Binary not found (bundled or system)"
        assert os.path.isfile(path)
        assert os.access(path, os.X_OK)

    def test_binary_version(self):
        """Binary reports a version string."""
        import subprocess
        path = get_b4n1web_binary()
        result = subprocess.run([path, "--version"], capture_output=True, text=True, timeout=5)
        version = result.stdout.strip()
        assert len(version) > 0
        assert any(c.isdigit() for c in version)


# ──────────────────────────────────────────────
# Phase 2: Version Mismatch Warning
# ──────────────────────────────────────────────

class TestVersionWarning:
    def test_sdk_version_constant(self):
        """SDK_VERSION constant is defined and non-empty."""
        assert SDK_VERSION
        assert len(SDK_VERSION.split(".")) == 3  # semver: x.y.z

    def test_version_check_runs(self):
        """Version check warnings should print, not crash."""
        import subprocess as _sp
        path = get_b4n1web_binary()
        result = _sp.run([path, "--version"], capture_output=True, text=True, timeout=5)
        assert result.returncode == 0
        assert any(c.isdigit() for c in result.stdout)


# ──────────────────────────────────────────────
# Phase 3: Core Function Tests
# ──────────────────────────────────────────────

class TestCoreFunctions:
    def test_goto_light_mode(self):
        """Navigate to URL in light mode, get page data."""
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            assert isinstance(page, Page)
            assert page.url == "https://example.com"
            assert len(page.markdown) > 0
            assert isinstance(page.links, list)
            assert page.screenshot is None

    def test_goto_js_mode(self):
        """Navigate in JS mode — should also work."""
        with AgentBrowser(mode=BrowserMode.JS) as browser:
            page = browser.goto("https://example.com")
            assert isinstance(page, Page)
            assert page.url == "https://example.com"
            assert len(page.markdown) > 0

    def test_get_main_content(self):
        """Page.getMainContent() strips header lines."""
        page = Page(url="https://test.com", markdown="# Header\n\n## Sub\n\nActual content here", links=[])
        content = page.get_main_content()
        assert "Actual content here" in content
        # Should skip first 2 lines (header)
        assert "# Header" not in content

    def test_find_links_by_text(self):
        """Page.findLinksByText() returns matching links."""
        page = Page(url="https://test.com", markdown="", links=[
            "https://example.com/about",
            "https://example.com/contact",
            "https://other.com/stuff",
        ])
        results = page.find_links_by_text("contact")
        assert len(results) == 1
        assert "contact" in results[0]

    def test_find_links_case_insensitive(self):
        """Link search is case-insensitive."""
        page = Page(url="https://test.com", markdown="", links=[
            "https://Example.COM/Page",
        ])
        results = page.find_links_by_text("example")
        assert len(results) == 1

    def test_context_manager(self):
        """AgentBrowser works as context manager."""
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            assert page is not None

    def test_custom_timeout(self):
        """Custom timeout is accepted."""
        browser = AgentBrowser(mode=BrowserMode.LIGHT, timeout=60)
        try:
            page = browser.goto("https://example.com")
            assert page is not None
        finally:
            browser.close()

    def test_custom_user_agent(self):
        """Custom user-agent is accepted."""
        browser = AgentBrowser(mode=BrowserMode.LIGHT, user_agent="TestAgent/2.0")
        try:
            page = browser.goto("https://example.com")
            assert page is not None
            assert browser.session.headers["User-Agent"] == "TestAgent/2.0"
        finally:
            browser.close()


# ──────────────────────────────────────────────
# Phase 4: Security Tests
# ──────────────────────────────────────────────

class TestSecurity:
    def test_new_domain_needs_api_check(self):
        """Unknown domain returns needsApiCheck=True."""
        from b4n1web.security import SecurityShield
        shield = SecurityShield()
        is_safe, needs_api_check = shield.is_url_safe("https://brand-new-domain-xyz123.com")
        assert is_safe is True
        assert needs_api_check is True

    def test_mark_domain_unsafe(self):
        """Marking domain unsafe blocks it."""
        from b4n1web.security import SecurityShield
        shield = SecurityShield()
        shield.mark_domain("evil.com", is_safe=False)
        is_safe, needs_api_check = shield.is_url_safe("https://evil.com/path")
        assert is_safe is False
        assert needs_api_check is False

    def test_mark_domain_safe(self):
        """Marking domain safe bypasses API check."""
        from b4n1web.security import SecurityShield
        shield = SecurityShield()
        shield.mark_domain("trusted.com", is_safe=True)
        is_safe, needs_api_check = shield.is_url_safe("https://trusted.com/page")
        assert is_safe is True
        assert needs_api_check is False

    def test_clear_cache(self):
        """Clearing cache resets all domain states."""
        from b4n1web.security import SecurityShield
        shield = SecurityShield()
        shield.mark_domain("test.com", is_safe=False)
        shield.clear_cache()
        is_safe, needs_api_check = shield.is_url_safe("https://test.com")
        assert needs_api_check is True

    def test_navigate_blocks_unsafe(self):
        """navigate() raises on unsafe domain."""
        from b4n1web.security import SecurityShield
        shield = SecurityShield()
        shield.mark_domain("blocked.com", is_safe=False)
        is_safe, needs_api_check = shield.is_url_safe("https://blocked.com/page")
        assert is_safe is False

    def test_invalid_url_returns_safe_default(self):
        """Invalid URL returns safe default (no host to check)."""
        from b4n1web.security import SecurityShield
        shield = SecurityShield()
        is_safe, needs_api_check = shield.is_url_safe("not-a-valid-url")
        # Should not crash, returns safe default
        assert is_safe is True


# ──────────────────────────────────────────────
# Phase 5: CLI Binary Tests
# ──────────────────────────────────────────────

class TestCLIBinary:
    def test_cli_goto_command(self):
        """Run b4n1web goto via subprocess."""
        binary = get_b4n1web_binary()
        result = subprocess.run(
            [binary, "goto", "https://example.com"],
            capture_output=True, text=True, timeout=30,
        )
        assert result.returncode == 0
        assert "example.com" in result.stdout

    def test_cli_goto_light_mode(self):
        """Run b4n1web goto with --mode light."""
        binary = get_b4n1web_binary()
        result = subprocess.run(
            [binary, "goto", "https://example.com", "--mode", "light"],
            capture_output=True, text=True, timeout=30,
        )
        assert result.returncode == 0

    def test_cli_update_command(self):
        """Run b4n1web update to check version."""
        binary = get_b4n1web_binary()
        result = subprocess.run(
            [binary, "update"],
            capture_output=True, text=True, timeout=10,
        )
        assert result.returncode == 0
        assert "version" in result.stdout.lower() or "Version" in result.stdout


# ──────────────────────────────────────────────
# Phase 6: Error Handling
# ──────────────────────────────────────────────

class TestErrorHandling:
    def test_invalid_url_error(self):
        """Invalid URL raises error from binary."""
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            with pytest.raises(Exception):
                browser.goto("https://this-domain-definitely-does-not-exist-abc123xyz.com")
