"""
B4n1Web SDK
Zero-overhead, high-speed web execution for sovereign AI agents.

NOTE: The B4n1Web binary must be installed separately via:
    curl -sL https://github.com/B4N1-com/b4n1-web/releases/latest/download/b4n1web-v0.6.2-flat.tar.gz | tar -xz

Quick Start:
    # Using MCP (recommended for AI agents)
    from b4n1web.mcp import AsyncMcpClient

    async with AsyncMcpClient() as client:
        page = await client.goto("https://example.com")
        print(page.markdown)

    # Using subprocess (legacy)
    from b4n1web import AgentBrowser, BrowserMode

    browser = AgentBrowser(mode=BrowserMode.LIGHT)
    page = browser.goto("https://example.com")
    print(page.markdown)
"""

from .browser import AgentBrowser, BrowserMode, Page, SDK_VERSION
from .errors import BinaryNotFoundError
from .mcp import AsyncMcpClient, McpClient
from .mcp import Tool, ToolResult

__version__ = "0.6.2"
__author__ = "Bani Montoya"
__email__ = "banimontoya@gmail.com"

__all__ = [
    "AgentBrowser",
    "BrowserMode",
    "Page",
    "SDK_VERSION",
    "BinaryNotFoundError",
    "AsyncMcpClient",
    "McpClient",
    "Tool",
    "ToolResult",
]
