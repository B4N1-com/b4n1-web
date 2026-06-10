#!/usr/bin/env python3
"""B4n1Web SDK - Complete Feature Demonstration

Demonstrates ALL b4n1web features with graceful fallback
when the b4n1web binary is not installed.

Usage:
    pip install b4n1-web
    python3 examples/basic.py
"""

import asyncio
import os
import sys
from unittest.mock import MagicMock, patch

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from b4n1web import (
    BrowserMode,
    AgentBrowser,
    Page,
    BinaryNotFoundError,
    SDK_VERSION,
)
from b4n1web.mcp import McpClient, AsyncMcpClient

TEST_URL = "https://example.com"


def section(title: str):
    print(f"\n{'='*60}")
    print(f"  {title}")
    print(f"{'='*60}")


def demo_browser_mode():
    section("1. BrowserMode Enum")
    for mode in BrowserMode:
        print(f"  {mode.name:>8} = '{mode.value}'")
    assert BrowserMode.LIGHT.value == "light"
    assert BrowserMode.JS.value == "js"
    assert BrowserMode.RENDER.value == "render"
    print("  Enum values verified OK")


def mock_page(url: str = TEST_URL) -> Page:
    return Page(
        url=url,
        markdown="# Example Domain\n\nThis domain is for use in illustrative examples.",
        links=["https://www.iana.org/domains/example"],
    )


def demo_agent_browser():
    section("2. AgentBrowser Creation & Context Manager")
    try:
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            print(f"  Binary: {browser.binary_path}")
            print(f"  AgentBrowser mode={browser.mode.value}")
    except BinaryNotFoundError:
        print("  [SKIP] Binary not installed - using mock")
        with patch("b4n1web.browser.get_b4n1web_binary", return_value="/usr/bin/b4n1web"):
            with patch("b4n1web.browser.AgentBrowser.binary_path", new_callable=PropertyMock, return_value="/usr/bin/b4n1web"):
                with patch("b4n1web.browser.AgentBrowser.goto", return_value=mock_page()):
                    with AgentBrowser() as browser:
                        print(f"  AgentBrowser mode={browser.mode.value} (mocked)")
    print("  OK")


def demo_goto_modes():
    section("3. goto with Different Modes + Markdown & Links")
    try:
        with AgentBrowser(mode=BrowserMode.LIGHT) as browser:
            page = browser.goto(TEST_URL)
            print(f"  URL: {page.url}")
            print(f"  Markdown ({len(page.markdown)} chars): {page.markdown[:60]}...")
            print(f"  Links ({len(page.links)}): {page.links}")
    except BinaryNotFoundError:
        print("  [SKIP] Binary not installed - using mock")
        page = mock_page()
        print(f"  URL: {page.url}")
        print(f"  Markdown ({len(page.markdown)} chars): {page.markdown[:60]}...")
        print(f"  Links ({len(page.links)}): {page.links}")

    print("\n  --- JS Mode ---")
    try:
        with AgentBrowser(mode=BrowserMode.JS) as browser:
            page = browser.goto(TEST_URL)
            print(f"  JS mode page: {len(page.markdown)} chars, {len(page.links)} links")
    except BinaryNotFoundError:
        print("  [SKIP] Binary not installed")


def demo_page_methods():
    section("4. Page.get_main_content() & find_links_by_text()")
    try:
        with AgentBrowser() as browser:
            page = browser.goto(TEST_URL)
            _demo_page_api(page)
    except BinaryNotFoundError:
        print("  [DEMO] Using mock Page object")
        _demo_page_api(mock_page())


def _demo_page_api(page: Page):
    print(f"  URL: {page.url}")
    print(f"  Markdown: {page.markdown[:80]}...")

    main = page.get_main_content()
    print(f"  get_main_content(): {main[:60]}...")

    found = page.find_links_by_text("iana")
    print(f"  find_links_by_text('iana'): {found}")

    found = page.find_links_by_text("NONEXISTENT")
    print(f"  find_links_by_text('NONEXISTENT'): {found}")

    assert isinstance(page.url, str)
    assert isinstance(page.markdown, str)
    assert isinstance(page.links, list)
    print("  Page API verified OK")


def demo_error_handling():
    section("5. Error Handling (BinaryNotFoundError)")
    err = BinaryNotFoundError()
    print(f"  Exception type: {type(err).__name__}")
    print(f"  Message: {str(err)[:50]}...")
    assert issubclass(BinaryNotFoundError, RuntimeError), "Must inherit from RuntimeError"
    print("  Error handling OK")


def demo_mcp_client():
    section("6. McpClient (if server running)")
    try:
        with McpClient(timeout=2.0) as client:
            print(f"  Connected: {client.is_connected}")
            print(f"  Protocol: {client.protocol_version}")
            print(f"  Server: {client.server_version}")
            page = client.goto(TEST_URL)
            print(f"  Page: {page.url} - {len(page.markdown)} chars")
            links = client.get_links()
            print(f"  Links: {len(links)}")
    except (ConnectionRefusedError, OSError, TimeoutError) as e:
        print(f"  [SKIP] MCP server not running: {e}")
    except Exception as e:
        print(f"  [SKIP] MCP error: {e}")


async def demo_async_mcp():
    section("7. AsyncMcpClient (if server running)")
    try:
        async with AsyncMcpClient(timeout=2.0) as client:
            page = await client.goto_async(TEST_URL)
            print(f"  Page: {page.url} - {len(page.markdown)} chars")
            links = await client.get_links_async()
            print(f"  Links: {len(links)}")
    except (ConnectionRefusedError, OSError, TimeoutError) as e:
        print(f"  [SKIP] MCP server not running: {e}")
    except Exception as e:
        print(f"  [SKIP] MCP error: {e}")


def main():
    print(f"B4n1Web SDK v{SDK_VERSION} - Complete Feature Demo")
    print(f"Python {sys.version}")

    demo_browser_mode()
    demo_agent_browser()
    demo_goto_modes()
    demo_page_methods()
    demo_error_handling()
    demo_mcp_client()
    asyncio.run(demo_async_mcp())

    section("DONE")
    print("  All features demonstrated successfully!")


if __name__ == "__main__":
    main()
