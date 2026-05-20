"""Unit tests for BrowserMode enum."""

import pytest
from b4n1web import BrowserMode


class TestBrowserMode:
    def test_light_mode(self):
        assert BrowserMode.LIGHT.value == "light"

    def test_js_mode(self):
        assert BrowserMode.JS.value == "js"

    def test_render_mode(self):
        assert BrowserMode.RENDER.value == "render"

    def test_mode_count(self):
        assert len(BrowserMode) == 3

    def test_mode_from_string(self):
        assert BrowserMode("light") == BrowserMode.LIGHT
        assert BrowserMode("js") == BrowserMode.JS
        assert BrowserMode("render") == BrowserMode.RENDER
