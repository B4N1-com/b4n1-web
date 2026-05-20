"""Integration tests for b4n1web SDK - tests binary interaction."""

import pytest
from b4n1web.browser import (
    get_b4n1web_binary,
    AgentBrowser,
    BrowserMode,
)
from b4n1web.errors import BinaryNotFoundError


class TestBinaryDetection:
    def test_get_b4n1web_binary_returns_path_or_none(self):
        result = get_b4n1web_binary()
        assert result is None or isinstance(result, str)


class TestAgentBrowserIntegration:
    def test_browser_creation_light_mode(self):
        binary = get_b4n1web_binary()
        if binary is None:
            pytest.skip("FastWeb binary not installed")
        browser = AgentBrowser(mode=BrowserMode.LIGHT)
        assert browser.mode == BrowserMode.LIGHT
        browser.close()

    def test_browser_creation_render_mode(self):
        binary = get_b4n1web_binary()
        if binary is None:
            pytest.skip("FastWeb binary not installed")
        browser = AgentBrowser(mode=BrowserMode.RENDER)
        assert browser.mode == BrowserMode.RENDER
        browser.close()

    def test_browser_custom_timeout(self):
        binary = get_b4n1web_binary()
        if binary is None:
            pytest.skip("FastWeb binary not installed")
        browser = AgentBrowser(timeout=60)
        assert browser.timeout == 60
        browser.close()

    def test_browser_custom_user_agent(self):
        binary = get_b4n1web_binary()
        if binary is None:
            pytest.skip("FastWeb binary not installed")
        browser = AgentBrowser(user_agent="Custom-Agent/2.0")
        assert "Custom-Agent" in browser.session.headers.get("User-Agent", "")
        browser.close()

    def test_browser_goto_simple_page(self):
        binary = get_b4n1web_binary()
        if binary is None:
            pytest.skip("FastWeb binary not installed")
        browser = AgentBrowser(mode=BrowserMode.LIGHT)
        try:
            page = browser.goto("https://example.com")
            assert page.url == "https://example.com"
            assert len(page.markdown) > 0
            assert isinstance(page.links, list)
        finally:
            browser.close()

    def test_browser_context_manager(self):
        binary = get_b4n1web_binary()
        if binary is None:
            pytest.skip("FastWeb binary not installed")
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            assert page.url == "https://example.com"
