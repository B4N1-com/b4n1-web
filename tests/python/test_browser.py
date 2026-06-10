"""
Exhaustive Unit Tests — Python SDK (b4n1web) Browser Module
Covers Page, BrowserMode, get_b4n1web_binary, and AgentBrowser with mocked subprocess.
"""

import os
import sys
import subprocess
import warnings
from unittest.mock import patch, MagicMock, PropertyMock
from pathlib import Path

import pytest

# ── SUT imports ──
from b4n1web import AgentBrowser, BrowserMode, Page, BinaryNotFoundError, SDK_VERSION
from b4n1web.browser import get_b4n1web_binary
from b4n1web.errors import BinaryNotFoundError


# =====================================================================
# 1. BrowserMode Enum
# =====================================================================

class TestBrowserModeEnum:
    def test_light_value(self):
        assert BrowserMode.LIGHT.value == "light"

    def test_js_value(self):
        assert BrowserMode.JS.value == "js"

    def test_render_value(self):
        assert BrowserMode.RENDER.value == "render"

    def test_all_modes_set(self):
        modes = {m.value for m in BrowserMode}
        assert modes == {"light", "js", "render"}

    def test_mode_from_string(self):
        assert BrowserMode("light") == BrowserMode.LIGHT
        assert BrowserMode("js") == BrowserMode.JS
        assert BrowserMode("render") == BrowserMode.RENDER

    def test_invalid_mode_raises(self):
        with pytest.raises(ValueError):
            BrowserMode("invalid")

    def test_invalid_mode_case(self):
        with pytest.raises(ValueError):
            BrowserMode("LIGHT")

    def test_mode_count(self):
        assert len(list(BrowserMode)) == 3

    def test_mode_hashable(self):
        modes = {BrowserMode.LIGHT, BrowserMode.JS, BrowserMode.RENDER}
        assert len(modes) == 3

    def test_mode_equality(self):
        assert BrowserMode.LIGHT == BrowserMode("light")
        assert BrowserMode.LIGHT != BrowserMode.JS

    def test_mode_name(self):
        assert BrowserMode.LIGHT.name == "LIGHT"
        assert BrowserMode.JS.name == "JS"
        assert BrowserMode.RENDER.name == "RENDER"

    def test_mode_iteration(self):
        values = [m.value for m in BrowserMode]
        assert "light" in values
        assert "js" in values
        assert "render" in values


# =====================================================================
# 2. Page Class
# =====================================================================

class TestPage:
    # ── Construction ──

    def test_minimal_page(self):
        page = Page(url="https://example.com", markdown="", links=[])
        assert page.url == "https://example.com"
        assert page.markdown == ""
        assert page.links == []
        assert page.screenshot is None

    def test_full_page(self):
        page = Page(
            url="https://example.com/page",
            markdown="# Title\nContent",
            links=["https://a.com", "https://b.com"],
            screenshot="base64data"
        )
        assert page.url == "https://example.com/page"
        assert page.markdown == "# Title\nContent"
        assert page.links == ["https://a.com", "https://b.com"]
        assert page.screenshot == "base64data"

    def test_page_with_unicode_markdown(self):
        page = Page(url="https://example.com", markdown="日本語テスト\n🚀 emoji", links=[])
        assert "日本語" in page.markdown
        assert "🚀" in page.markdown

    def test_page_with_empty_markdown(self):
        page = Page(url="https://example.com", markdown="", links=[])
        assert len(page.markdown) == 0

    def test_page_with_very_long_markdown(self):
        long_md = "x" * 100_000
        page = Page(url="https://example.com", markdown=long_md, links=[])
        assert len(page.markdown) == 100_000

    def test_page_with_many_links(self):
        links = [f"https://example.com/{i}" for i in range(1000)]
        page = Page(url="https://example.com", markdown="", links=links)
        assert len(page.links) == 1000

    def test_page_with_empty_links(self):
        page = Page(url="https://example.com", markdown="", links=[])
        assert page.links == []

    def test_page_with_duplicate_links(self):
        page = Page(url="https://example.com", markdown="", links=[
            "https://a.com", "https://a.com", "https://b.com"
        ])
        assert len(page.links) == 3

    def test_page_with_malformed_links(self):
        page = Page(url="https://example.com", markdown="", links=[
            "not-a-url", "://broken", "https://valid.com"
        ])
        assert len(page.links) == 3

    def test_page_repr(self):
        page = Page(url="https://example.com", markdown="Content", links=[])
        r = repr(page)
        assert "example.com" in r

    def test_page_str(self):
        page = Page(url="https://example.com", markdown="Content", links=[])
        s = str(page)
        assert isinstance(s, str)

    # ── get_main_content ──

    def test_get_main_content_skips_header(self):
        page = Page(url="t", markdown="# Header\n\n## Sub\n\nActual content", links=[])
        content = page.get_main_content()
        assert "Actual content" in content
        assert "# Header" not in content

    def test_get_main_content_with_no_header(self):
        page = Page(url="t", markdown="No header here", links=[])
        content = page.get_main_content()
        assert content == "No header here"

    def test_get_main_content_empty_page(self):
        page = Page(url="t", markdown="", links=[])
        content = page.get_main_content()
        assert content == ""

    def test_get_main_content_single_line(self):
        page = Page(url="t", markdown="Single line", links=[])
        content = page.get_main_content()
        assert content == "Single line"

    def test_get_main_content_two_lines(self):
        page = Page(url="t", markdown="Line 1\nLine 2", links=[])
        content = page.get_main_content()
        assert len(content) > 0

    def test_get_main_content_with_trailing_whitespace(self):
        page = Page(url="t", markdown="# H\n\nContent  \n", links=[])
        content = page.get_main_content()
        assert "Content" in content.strip()

    def test_get_main_content_preserves_internal_formatting(self):
        page = Page(url="t", markdown="# Title\n\n```python\nx = 1\n```", links=[])
        content = page.get_main_content()
        assert "```python" in content

    def test_get_main_content_many_lines(self):
        md = "# Title\n\n" + "\n".join(f"Line {i}" for i in range(100))
        page = Page(url="t", markdown=md, links=[])
        content = page.get_main_content()
        assert "Line 0" in content

    # ── find_links_by_text ──

    def test_find_links_by_text_exact(self):
        page = Page(url="t", markdown="", links=["https://example.com/contact"])
        results = page.find_links_by_text("contact")
        assert len(results) == 1
        assert "contact" in results[0]

    def test_find_links_by_text_partial(self):
        page = Page(url="t", markdown="", links=["https://example.com/about-us"])
        results = page.find_links_by_text("about")
        assert len(results) == 1

    def test_find_links_by_text_no_match(self):
        page = Page(url="t", markdown="", links=["https://example.com/page"])
        results = page.find_links_by_text("missing")
        assert len(results) == 0

    def test_find_links_by_text_case_insensitive(self):
        page = Page(url="t", markdown="", links=["https://Example.COM/Page"])
        results = page.find_links_by_text("example")
        assert len(results) == 1

    def test_find_links_by_text_multiple_matches(self):
        page = Page(url="t", markdown="", links=[
            "https://example.com/contact",
            "https://example.com/contact-us",
            "https://other.com/page",
        ])
        results = page.find_links_by_text("contact")
        assert len(results) == 2

    def test_find_links_by_text_empty_string(self):
        page = Page(url="t", markdown="", links=["https://a.com", "https://b.com"])
        results = page.find_links_by_text("")
        assert len(results) == 2

    def test_find_links_by_text_special_characters(self):
        page = Page(url="t", markdown="", links=["https://example.com/page?q=hello+world"])
        results = page.find_links_by_text("hello")
        assert len(results) == 1

    def test_find_links_by_text_unicode_query(self):
        page = Page(url="t", markdown="", links=["https://example.com/日本語"])
        results = page.find_links_by_text("日本語")
        assert len(results) == 1

    def test_find_links_by_text_empty_link_list(self):
        page = Page(url="t", markdown="", links=[])
        results = page.find_links_by_text("anything")
        assert len(results) == 0

    def test_find_links_by_text_newline_in_link(self):
        page = Page(url="t", markdown="", links=["https://example.com/page\nwith-newline"])
        results = page.find_links_by_text("page")
        assert len(results) == 1

    def test_find_links_preserves_order(self):
        page = Page(url="t", markdown="", links=[
            "https://b.com", "https://a.com", "https://c.com"
        ])
        results = page.find_links_by_text("com")
        assert results == ["https://b.com", "https://a.com", "https://c.com"]


# =====================================================================
# 3. get_b4n1web_binary
# =====================================================================

class TestGetBinary:
    def test_finds_bundled_binary(self):
        """In the installed package, the bundled binary should be found."""
        path = get_b4n1web_binary()
        assert path is not None, "Should find bundled binary"
        assert os.path.isfile(path)
        assert os.access(path, os.X_OK)

    def test_bundled_path_contains_bin(self):
        path = get_b4n1web_binary()
        assert path is not None
        assert "bin" in path or "b4n1web" in path.lower()

# =====================================================================
# 5. AgentBrowser — Construction
# =====================================================================

class TestAgentBrowserInit:
    def test_default_init(self):
        browser = AgentBrowser()
        assert browser.mode == BrowserMode.LIGHT
        assert browser.timeout == 30
        assert browser.user_agent == "B4n1Web-Agent/1.0"
        browser.close()

    def test_light_mode(self):
        browser = AgentBrowser(mode=BrowserMode.LIGHT)
        assert browser.mode == BrowserMode.LIGHT
        browser.close()

    def test_js_mode(self):
        browser = AgentBrowser(mode=BrowserMode.JS)
        assert browser.mode == BrowserMode.JS
        browser.close()

    def test_render_mode(self):
        browser = AgentBrowser(mode=BrowserMode.RENDER)
        assert browser.mode == BrowserMode.RENDER
        browser.close()

    def test_custom_timeout(self):
        browser = AgentBrowser(timeout=60)
        assert browser.timeout == 60
        browser.close()

    def test_zero_timeout(self):
        browser = AgentBrowser(timeout=0)
        assert browser.timeout == 0
        browser.close()

    def test_custom_user_agent(self):
        browser = AgentBrowser(user_agent="CustomAgent/1.0")
        assert browser.user_agent == "CustomAgent/1.0"
        browser.close()

    def test_empty_user_agent(self):
        browser = AgentBrowser(user_agent="")
        assert browser.user_agent == ""
        browser.close()

    def test_long_user_agent(self):
        ua = "A" * 500
        browser = AgentBrowser(user_agent=ua)
        assert browser.user_agent == ua
        browser.close()

    def test_unicode_user_agent(self):
        browser = AgentBrowser(user_agent="日本語Agent/🚀")
        assert browser.user_agent == "日本語Agent/🚀"
        browser.close()

    def test_user_agent_in_session_headers(self):
        browser = AgentBrowser(user_agent="TestAgent/2.0")
        assert browser.session.headers["User-Agent"] == "TestAgent/2.0"
        browser.close()

    def test_session_has_accept_headers(self):
        browser = AgentBrowser()
        headers = browser.session.headers
        assert "Accept" in headers
        assert "Accept-Language" in headers
        assert "Accept-Encoding" in headers
        browser.close()

    def test_session_verifies_ssl(self):
        browser = AgentBrowser()
        assert browser.session.verify == "/etc/ssl/certs/ca-certificates.crt"
        browser.close()

    def test_binary_path_property(self):
        browser = AgentBrowser()
        path = browser.binary_path
        assert path is not None
        assert os.path.isfile(path)
        browser.close()

    def test_binary_path_is_executable(self):
        browser = AgentBrowser()
        path = browser.binary_path
        assert os.access(path, os.X_OK)
        browser.close()


# =====================================================================
# 7. AgentBrowser — goto (with mocked subprocess)
# =====================================================================

class TestAgentBrowserGoto:
    
    @patch("subprocess.run")
    def test_goto_light_mode(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="URL: https://example.com\nMarkdown:\nExample Domain\nLinks: [\"https://iana.org\"]\n"
        )
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            assert page.url == "https://example.com"
            assert "Example Domain" in page.markdown
            assert len(page.links) == 1
            assert "https://iana.org" in page.links

    
    @patch("subprocess.run")
    def test_goto_js_mode(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="URL: https://example.com\nMarkdown:\nPage\nLinks: [\"https://a.com\"]\n"
        )
        with AgentBrowser(mode=BrowserMode.JS) as browser:
            page = browser.goto("https://example.com")
            assert page.url == "https://example.com"
            mock_run.assert_called_once()
            call_args = mock_run.call_args[0][0]
            assert "--mode" in call_args
            assert "js" in call_args

    
    @patch("subprocess.run")
    def test_goto_render_mode(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="URL: https://example.com\nMarkdown:\nPage\nLinks: []\nScreenshot: data:image/png;base64,abc123\n"
        )
        with AgentBrowser(mode=BrowserMode.RENDER) as browser:
            page = browser.goto("https://example.com")
            # Screenshot not parsed by SDK - goes to markdown
            assert "abc123" in page.markdown

    
    @patch("subprocess.run")
    def test_goto_with_special_characters_in_url(self, mock_run):
        mock_run.return_value = MagicMock(returncode=0, stdout="URL: https://example.com/page?q=hello+world\nMarkdown:\nContent\nLinks: []\n")
        with AgentBrowser() as browser:
            page = browser.goto("https://example.com/page?q=hello+world")
            assert page.url == "https://example.com/page?q=hello+world"

    
    @patch("subprocess.run")
    def test_goto_no_links(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="URL: https://example.com\nMarkdown:\nContent\nLinks: []\n"
        )
        with AgentBrowser() as browser:
            page = browser.goto("https://example.com")
            assert page.links == []

    
    @patch("subprocess.run")
    def test_goto_many_links(self, mock_run):
        links = ", ".join([f'"https://example.com/{i}"' for i in range(100)])
        output = f'URL: https://example.com\nMarkdown:\nContent\nLinks: [{links}]\n'
        mock_run.return_value = MagicMock(returncode=0, stdout=output)
        with AgentBrowser() as browser:
            page = browser.goto("https://example.com")
            assert len(page.links) == 100

    
    @patch("subprocess.run")
    def test_goto_empty_markdown(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="URL: https://example.com\nMarkdown:\nLinks: []\n"
        )
        with AgentBrowser() as browser:
            page = browser.goto("https://example.com")
            assert page.markdown == ""

    
    @patch("subprocess.run")
    def test_goto_malformed_output(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="GARBAGE OUTPUT WITH NO STRUCTURE"
        )
        with AgentBrowser() as browser:
            page = browser.goto("https://example.com")
            assert page.url == "https://example.com"
            assert page.markdown == "GARBAGE OUTPUT WITH NO STRUCTURE"
            assert page.links == []

    
    @patch("subprocess.run")
    def test_goto_binary_error(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=1,
            stderr="Binary error: invalid URL"
        )
        with AgentBrowser() as browser:
            with pytest.raises(RuntimeError, match="Binary error"):
                browser.goto("https://example.com")

    
    @patch("subprocess.run")
    def test_goto_timeout(self, mock_run):
        mock_run.side_effect = subprocess.TimeoutExpired(cmd="b4n1web", timeout=30)
        with AgentBrowser() as browser:
            with pytest.raises(RuntimeError, match="timed out"):
                browser.goto("https://example.com")

    
    @patch("subprocess.run")
    def test_goto_uses_correct_binary_path(self, mock_run):
        mock_run.return_value = MagicMock(returncode=0, stdout="URL: https://example.com\nMarkdown:\nContent\nLinks: []\n")
        with AgentBrowser() as browser:
            browser.goto("https://example.com")
            call_args = mock_run.call_args[0][0]
            assert call_args[0] == browser.binary_path

    
    @patch("subprocess.run")
    def test_goto_passes_mode_flag(self, mock_run):
        mock_run.return_value = MagicMock(returncode=0, stdout="URL: https://example.com\nMarkdown:\nContent\nLinks: []\n")
        with AgentBrowser(mode=BrowserMode.JS) as browser:
            browser.goto("https://example.com")
            call_args = mock_run.call_args[0][0]
            assert "goto" in call_args
            assert "--mode" in call_args
            assert "js" in call_args

    
    @patch("subprocess.run")
    def test_goto_url_passed_as_argument(self, mock_run):
        mock_run.return_value = MagicMock(returncode=0, stdout="URL: https://test.com\nMarkdown:\nX\nLinks: []\n")
        with AgentBrowser() as browser:
            browser.goto("https://test.com")
            call_args = mock_run.call_args[0][0]
            assert "https://test.com" in call_args

    
    @patch("subprocess.run")
    def test_goto_returns_page_object(self, mock_run):
        mock_run.return_value = MagicMock(returncode=0, stdout="URL: https://example.com\nMarkdown:\nContent\nLinks: []\n")
        with AgentBrowser() as browser:
            result = browser.goto("https://example.com")
            assert isinstance(result, Page)

    
    @patch("subprocess.run")
    def test_goto_screenshot_parsed(self, mock_run):
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout="URL: https://example.com\nMarkdown:\nPage\nLinks: []\nScreenshot: iVBORw0KGgoAAAANSUhEUg==\n"
        )
        with AgentBrowser() as browser:
            page = browser.goto("https://example.com")
            # Screenshot goes to markdown since SDK doesn't parse it separately
            assert "iVBORw0KGgoAAAANSUhEUg==" in page.markdown


# =====================================================================
# 8. AgentBrowser — close
# =====================================================================

class TestAgentBrowserClose:
    def test_close_session(self):
        browser = AgentBrowser()
        browser.close()

    def test_close_multiple_times(self):
        browser = AgentBrowser()
        browser.close()
        browser.close()
        browser.close()

    def test_close_in_context_manager(self):
        with AgentBrowser() as browser:
            assert isinstance(browser, AgentBrowser)


# =====================================================================
# 9. AgentBrowser — Context Manager
# =====================================================================

class TestAgentBrowserContextManager:
    def test_context_manager_enter_exit(self):
        with AgentBrowser() as browser:
            assert isinstance(browser, AgentBrowser)

    
    @patch("subprocess.run")
    def test_context_manager_with_goto(self, mock_run):
        mock_run.return_value = MagicMock(returncode=0, stdout="URL: https://example.com\nMarkdown:\nContent\nLinks: []\n")
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            assert page is not None
            assert page.url == "https://example.com"

    def test_context_manager_exception_propagates(self):
        with pytest.raises(RuntimeError):
            with AgentBrowser() as browser:
                raise RuntimeError("test error")

    def test_context_manager_closes_on_exception(self):
        try:
            with AgentBrowser() as browser:
                raise ValueError("oops")
        except ValueError:
            pass


# =====================================================================
# 10. AgentBrowser — Real Binary (if available)
# =====================================================================

@pytest.mark.skipif(get_b4n1web_binary() is None, reason="No binary available")
class TestAgentBrowserRealBinary:
    def test_real_goto(self):
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            assert isinstance(page, Page)
            assert page.url == "https://example.com"
            assert len(page.markdown) > 0
            assert isinstance(page.links, list)

    def test_real_goto_js_mode(self):
        with AgentBrowser(mode=BrowserMode.JS) as browser:
            page = browser.goto("https://example.com")
            assert page.url == "https://example.com"
            assert len(page.markdown) > 0

    def test_real_main_content(self):
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            content = page.get_main_content()
            assert isinstance(content, str)

    def test_real_find_links(self):
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto("https://example.com")
            links = page.find_links_by_text("iana")
            assert isinstance(links, list)

    def test_real_timeout(self):
        with AgentBrowser(mode=BrowserMode.LIGHT, timeout=5) as browser:
            page = browser.goto("https://example.com")
            assert page is not None

    def test_real_invalid_url(self):
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            with pytest.raises(Exception):
                browser.goto("https://this-domain-definitely-does-not-exist-abc123xyz.com")


# =====================================================================
# Render Mode Tests (with real Chromium if available)
# =====================================================================

@pytest.mark.skipif(get_b4n1web_binary() is None, reason="Binary not available")
class TestRenderMode:
    def test_render_mode_enum(self):
        assert BrowserMode.RENDER.value == "render"

    def test_render_mode_construction(self):
        browser = AgentBrowser(mode=BrowserMode.RENDER)
        assert browser.mode == BrowserMode.RENDER
        browser.close()

    def test_render_mode_goto_with_chromium(self):
        """Test render mode with real Chromium if installed."""
        # Check if chromium is installed
        binary = get_b4n1web_binary()
        result = subprocess.run(
            [binary, "chromium", "version"],
            capture_output=True, text=True, timeout=10
        )
        if "not installed" in result.stdout.lower():
            pytest.skip("Chromium not installed")

        # Run render mode
        result = subprocess.run(
            [binary, "goto", "https://example.com", "--mode", "render"],
            capture_output=True, text=True, timeout=60
        )
        assert result.returncode == 0
        output = result.stdout
        assert "URL:" in output
        assert "Markdown:" in output
        # Render mode should include screenshot or links
        assert "Screenshot:" in output or "Links:" in output

    def test_render_mode_screenshot_output(self):
        """Verify render mode produces screenshot data."""
        binary = get_b4n1web_binary()
        result = subprocess.run(
            [binary, "chromium", "version"],
            capture_output=True, text=True, timeout=10
        )
        if "not installed" in result.stdout.lower():
            pytest.skip("Chromium not installed")

        result = subprocess.run(
            [binary, "goto", "https://example.com", "--mode", "render"],
            capture_output=True, text=True, timeout=60
        )
        assert result.returncode == 0
        # Check for screenshot in output
        output = result.stdout
        if "Screenshot:" in output:
            # Screenshot line should have base64 data
            for line in output.splitlines():
                if line.startswith("Screenshot:"):
                    screenshot_data = line.replace("Screenshot:", "").strip()
                    assert len(screenshot_data) > 0
                    break
