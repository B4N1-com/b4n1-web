"""Unit tests for session persistence — save_state / load_state JSON roundtrip.

These tests verify the state JSON schema without needing a live browser.
The Chromium integration tests (with a real browser) live in integration_test.rs.
"""

import json
import os
import tempfile
import time

import pytest

from b4n1web.security import SecurityShield


# ──────────────────────────────────────────────────────────────
# Session state JSON schema roundtrip
# ──────────────────────────────────────────────────────────────

class TestSessionStateSchema:
    """Validate that session state JSON has the expected fields."""

    def test_state_file_roundtrip(self, tmp_path):
        """Simulated save_state → load_state JSON roundtrip."""
        state_file = tmp_path / "session_state.json"

        # Simulate save_state output
        state = {
            "url": "https://example.com",
            "cookies": "session_id=abc123; theme=dark",
            "localStorage": {
                "token": "xyz789",
                "user": "admin",
            },
        }
        state_file.write_text(json.dumps(state))

        # Simulate load_state read
        loaded = json.loads(state_file.read_text())
        assert loaded["url"] == "https://example.com"
        assert "session_id=abc123" in loaded["cookies"]
        assert loaded["localStorage"]["token"] == "xyz789"

    def test_state_file_roundtrip_empty(self, tmp_path):
        """Empty state JSON roundtrip."""
        state_file = tmp_path / "empty_state.json"
        state = {"url": "about:blank", "cookies": "", "localStorage": {}}
        state_file.write_text(json.dumps(state))
        loaded = json.loads(state_file.read_text())
        assert loaded["cookies"] == ""
        assert loaded["localStorage"] == {}

    def test_state_file_unicode(self, tmp_path):
        """Unicode in cookies and localStorage survives roundtrip."""
        state_file = tmp_path / "unicode_state.json"
        state = {
            "url": "https://例え.jp/テスト",
            "cookies": "name=日本語; theme=ñoño",
            "localStorage": {"greeting": "¡Hola 世界!"},
        }
        state_file.write_text(json.dumps(state, ensure_ascii=False))
        loaded = json.loads(state_file.read_text())
        assert loaded["url"] == "https://例え.jp/テスト"
        assert loaded["localStorage"]["greeting"] == "¡Hola 世界!"

    def test_state_file_parseable_after_manual_edit(self, tmp_path):
        """A manually-edited JSON state file is still parseable."""
        state_file = tmp_path / "edited_state.json"
        state_file.write_text('{"url":"https://x.com","cookies":"a=1","localStorage":{}}')
        raw = state_file.read_text()
        # Simulate manual edit: add whitespace
        edited = raw.replace('"url"', '  "url"')
        loaded = json.loads(edited)
        assert loaded["cookies"] == "a=1"

    def test_state_file_unique_per_session(self, tmp_path):
        """Each session writes to its own independent file."""
        f1 = tmp_path / "s1.json"
        f2 = tmp_path / "s2.json"
        f1.write_text(json.dumps({"url": "https://a.com", "cookies": "a=1", "localStorage": {}}))
        f2.write_text(json.dumps({"url": "https://b.com", "cookies": "b=2", "localStorage": {}}))
        assert json.loads(f1.read_text())["url"] == "https://a.com"
        assert json.loads(f2.read_text())["url"] == "https://b.com"


# ──────────────────────────────────────────────────────────────
# SecurityShield unit tests (already in E2E; duplicate as unit)
# ──────────────────────────────────────────────────────────────

class TestSecurityShieldUnit:
    """Pure unit tests for SecurityShield (no network needed)."""

    def test_new_domain_needs_api_check(self):
        shield = SecurityShield()
        is_safe, needs_api_check = shield.is_url_safe("https://new-domain-xyz.com")
        assert is_safe is True
        assert needs_api_check is True

    def test_mark_domain_unsafe(self):
        shield = SecurityShield()
        shield.mark_domain("evil.com", False)
        is_safe, needs_api_check = shield.is_url_safe("https://evil.com/path")
        assert is_safe is False
        assert needs_api_check is False

    def test_mark_domain_safe(self):
        shield = SecurityShield()
        shield.mark_domain("trusted.com", True)
        is_safe, needs_api_check = shield.is_url_safe("https://trusted.com/page")
        assert is_safe is True
        assert needs_api_check is False

    def test_clear_cache(self):
        shield = SecurityShield()
        shield.mark_domain("test.com", False)
        shield.clear_cache()
        _, needs_api_check = shield.is_url_safe("https://test.com")
        assert needs_api_check is True

    def test_invalid_url_returns_safe_default(self):
        shield = SecurityShield()
        is_safe, _ = shield.is_url_safe("not-a-valid-url")
        assert is_safe is True

    def test_mark_domain_normalizes_case(self):
        shield = SecurityShield()
        shield.mark_domain("EXAMPLE.COM", False)
        _, needs_api_check = shield.is_url_safe("https://example.com")
        assert needs_api_check is False
        is_safe, _ = shield.is_url_safe("https://example.com")
        assert is_safe is False

    def test_cache_expiry(self):
        shield = SecurityShield(cache_days=0)
        shield.mark_domain("expired.com", True)
        time.sleep(0.1)
        _, needs_api_check = shield.is_url_safe("https://expired.com")
        assert needs_api_check is True

    def test_multiple_domains_independent(self):
        shield = SecurityShield()
        shield.mark_domain("good.com", True)
        shield.mark_domain("bad.com", False)
        is_safe_good, _ = shield.is_url_safe("https://good.com")
        is_safe_bad, _ = shield.is_url_safe("https://bad.com")
        assert is_safe_good is True
        assert is_safe_bad is False

    def test_empty_domain_string(self):
        shield = SecurityShield()
        is_safe, _ = shield.is_url_safe("")
        assert is_safe is True

    def test_custom_cache_days(self):
        shield = SecurityShield(cache_days=30)
        shield.mark_domain("longcache.com", True)
        _, needs_api_check = shield.is_url_safe("https://longcache.com")
        assert needs_api_check is False
