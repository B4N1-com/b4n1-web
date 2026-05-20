"""Shared fixtures for b4n1web tests."""

import pytest
from b4n1web.browser import get_b4n1web_binary, AgentBrowser, BrowserMode


@pytest.fixture
def binary_available():
    """Skip test if b4n1web binary is not installed."""
    binary = get_b4n1web_binary()
    if binary is None:
        pytest.skip("b4n1web binary not installed")
    return binary


@pytest.fixture
def light_browser():
    """Create a light mode browser and clean up after."""
    browser = AgentBrowser(mode=BrowserMode.LIGHT)
    yield browser
    browser.close()


@pytest.fixture
def render_browser():
    """Create a render mode browser and clean up after."""
    browser = AgentBrowser(mode=BrowserMode.RENDER)
    yield browser
    browser.close()
