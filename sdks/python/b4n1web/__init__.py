"""
B4n1Web SDK
Zero-overhead, high-speed web execution for sovereign AI agents.

NOTE: nothing has to be installed first. The wheel carries the engine for six
platforms and resolves the matching one at import time. Overrides:
    B4N1WEB_BIN_PATH=/path/to/b4n1web   exact binary, wins over the bundle
    B4N1WEB_BINARY=/path/to/b4n1web     same, kept for older callers
    python -m b4n1web install           optional: copy one onto PATH

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
from .security import SecurityShield
from .mcp import AsyncMcpClient, McpClient
from .mcp import Tool, ToolResult

__version__ = "0.16.2"
__author__ = "Bani Montoya"
__email__ = "banimontoya@gmail.com"

__all__ = [
    "AgentBrowser",
    "BrowserMode",
    "Page",
    "SDK_VERSION",
    "BinaryNotFoundError",
    "SecurityShield",
    "AsyncMcpClient",
    "McpClient",
    "Tool",
    "ToolResult",
]
