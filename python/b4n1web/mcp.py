"""
B4n1Web SDK - MCP Client

MCP (Model Context Protocol) client for connecting to b4n1web MCP server.
Provides async support, typed interfaces, and full feature coverage.
"""

import asyncio
import json
from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Union
import socket
import threading

from .browser import BrowserMode, Page


@dataclass
class Tool:
    """MCP Tool definition."""

    name: str
    description: str
    input_schema: Dict[str, Any]


@dataclass
class ToolResult:
    """Result from a tool call."""

    content: List[Dict[str, Any]]
    is_error: bool = False

    @property
    def text(self) -> str:
        """Get text content from result."""
        return "".join(
            c.get("text", "") for c in self.content if c.get("type") == "text"
        )


@dataclass
class McpResponse:
    """JSON-RPC response from MCP server."""

    jsonrpc: str
    id: Union[int, None]
    result: Optional[Dict[str, Any]] = None
    error: Optional["McpError"] = None

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "McpResponse":
        """Create response from dictionary."""
        error = None
        if "error" in data:
            error = McpError.from_dict(data["error"])

        return cls(
            jsonrpc=data.get("jsonrpc", "2.0"),
            id=data.get("id"),
            result=data.get("result"),
            error=error,
        )


@dataclass
class McpError:
    """JSON-RPC error."""

    code: int
    message: str
    data: Optional[Any] = None

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "McpError":
        """Create error from dictionary."""
        return cls(
            code=data.get("code", -32603),
            message=data.get("message", "Unknown error"),
            data=data.get("data"),
        )


class McpClient:
    """
    B4n1Web MCP Client

    Connects to b4n1web MCP server and provides typed interface to all tools.

    Example:
        >>> client = McpClient(host="localhost", port=8765)
        >>> await client.connect()
        >>> page = await client.goto("https://example.com")
        >>> print(page.markdown)
    """

    def __init__(
        self,
        host: str = "localhost",
        port: int = 8765,
        timeout: float = 30.0,
    ):
        self.host = host
        self.port = port
        self.timeout = timeout
        self._socket: Optional[socket.socket] = None
        self._request_id = 0
        self._lock = threading.Lock()
        self._tools: List[Tool] = []
        self._protocol_version: Optional[str] = None
        self._server_version: Optional[str] = None

    def connect(self) -> None:
        """Connect to MCP server (synchronous)."""
        self._socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._socket.settimeout(self.timeout)
        self._socket.connect((self.host, self.port))

        # Initialize connection
        response = self._send_request(
            "initialize",
            {
                "protocolVersion": "2025-11-25",
                "clientInfo": {"name": "b4n1web-python-sdk", "version": "0.7.0"},
            },
        )

        if response.result:
            self._protocol_version = response.result.get("protocolVersion")
            server_info = response.result.get("serverInfo", {})
            self._server_version = server_info.get("version")

        # List available tools
        tools_response = self._send_request("tools/list", {})
        if tools_response.result:
            for tool in tools_response.result.get("tools", []):
                self._tools.append(
                    Tool(
                        name=tool["name"],
                        description=tool.get("description", ""),
                        input_schema=tool.get("inputSchema", {}),
                    )
                )

    async def connect_async(self) -> None:
        """Connect to MCP server (async)."""
        await asyncio.to_thread(self.connect)

    def disconnect(self) -> None:
        """Disconnect from MCP server."""
        if self._socket:
            self._socket.close()
            self._socket = None

    async def disconnect_async(self) -> None:
        """Disconnect from MCP server (async)."""
        await asyncio.to_thread(self.disconnect)

    def _send_request(self, method: str, params: Dict[str, Any]) -> McpResponse:
        """Send JSON-RPC request to server."""
        with self._lock:
            self._request_id += 1
            request = {
                "jsonrpc": "2.0",
                "id": self._request_id,
                "method": method,
                "params": params,
            }

            request_str = json.dumps(request) + "\n"
            self._socket.sendall(request_str.encode())

            # Read response
            response_str = self._socket.recv(4096).decode()
            response = json.loads(response_str)

            return McpResponse.from_dict(response)

    async def _send_request_async(
        self, method: str, params: Dict[str, Any]
    ) -> McpResponse:
        """Send JSON-RPC request to server (async)."""
        return await asyncio.to_thread(self._send_request, method, params)

    def goto(self, url: str, mode: BrowserMode = BrowserMode.LIGHT, wait_for: Optional[str] = None) -> Page:
        """Navigate to a URL and extract content.
        
        Args:
            url: URL to navigate to
            mode: Browser mode (LIGHT, JS, RENDER)
            wait_for: CSS selector to wait for before extracting content (render mode only)
        """
        args: Dict[str, Any] = {"url": url, "mode": mode.value}
        if wait_for:
            args["wait_for"] = wait_for
        
        response = self._send_request(
            "tools/call",
            {"name": "goto", "arguments": args},
        )

        if response.error:
            raise RuntimeError(f"MCP error: {response.error.message}")

        return self._parse_goto_result(response)

    async def goto_async(self, url: str, mode: BrowserMode = BrowserMode.LIGHT, wait_for: Optional[str] = None) -> Page:
        """Navigate to a URL and extract content (async).
        
        Args:
            url: URL to navigate to
            mode: Browser mode (LIGHT, JS, RENDER)
            wait_for: CSS selector to wait for before extracting content (render mode only)
        """
        args: Dict[str, Any] = {"url": url, "mode": mode.value}
        if wait_for:
            args["wait_for"] = wait_for
        
        response = await self._send_request_async(
            "tools/call",
            {"name": "goto", "arguments": args},
        )

        if response.error:
            raise RuntimeError(f"MCP error: {response.error.message}")

        return self._parse_goto_result(response)

    def _parse_goto_result(self, response: McpResponse) -> Page:
        """Parse goto tool result."""
        if not response.result:
            raise RuntimeError("Empty response from goto")

        content = response.result.get("content", [])
        text = "".join(c.get("text", "") for c in content if c.get("type") == "text")

        # Parse structured output
        url = ""
        markdown = ""
        links: List[str] = []
        screenshot: Optional[str] = None
        in_markdown = False

        for line in text.split("\n"):
            if line.startswith("URL:"):
                url = line[4:].strip()
            elif line.startswith("Markdown:"):
                in_markdown = True
                content_after = line[9:].strip()
                if content_after:
                    markdown += content_after + "\n"
            elif line.startswith("Links:"):
                in_markdown = False
                try:
                    links = json.loads(line[6:].strip())
                except Exception:
                    links = []
            elif line.startswith("Screenshot:"):
                in_markdown = False
                screenshot = line[12:].strip()
            else:
                if in_markdown:
                    markdown += line + "\n"
                elif line.strip():
                    markdown += line + "\n"

        return Page(
            url=url or "unknown",
            markdown=markdown.strip(),
            links=links,
            screenshot=screenshot,
        )

    def get_links(self) -> List[str]:
        """Get all links from current page."""
        response = self._send_request(
            "tools/call", {"name": "get_links", "arguments": {}}
        )

        if response.error:
            raise RuntimeError(f"MCP error: {response.error.message}")

        content = response.result.get("content", [])
        text = "".join(c.get("text", "") for c in content if c.get("type") == "text")

        try:
            return json.loads(text.strip())
        except Exception:
            return []

    async def get_links_async(self) -> List[str]:
        """Get all links from current page (async)."""
        response = await self._send_request_async(
            "tools/call", {"name": "get_links", "arguments": {}}
        )

        if response.error:
            raise RuntimeError(f"MCP error: {response.error.message}")

        content = response.result.get("content", [])
        text = "".join(c.get("text", "") for c in content if c.get("type") == "text")

        try:
            return json.loads(text.strip())
        except Exception:
            return []

    @property
    def tools(self) -> List[Tool]:
        """Get list of available tools."""
        return self._tools

    @property
    def is_connected(self) -> bool:
        """Check if connected to server."""
        return self._socket is not None

    @property
    def protocol_version(self) -> Optional[str]:
        """Get MCP protocol version."""
        return self._protocol_version

    @property
    def server_version(self) -> Optional[str]:
        """Get b4n1web server version."""
        return self._server_version

    def __enter__(self):
        """Context manager entry."""
        self.connect()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        self.disconnect()
        return False


class AsyncMcpClient:
    """
    Async B4n1Web MCP Client

    Full async implementation with context manager support.

    Example:
        >>> async with AsyncMcpClient() as client:
        ...     page = await client.goto("https://example.com")
        ...     print(page.markdown)
    """

    def __init__(
        self,
        host: str = "localhost",
        port: int = 8765,
        timeout: float = 30.0,
    ):
        self.client = McpClient(host=host, port=port, timeout=timeout)

    async def __aenter__(self):
        """Async context manager entry."""
        await self.client.connect_async()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit."""
        await self.client.disconnect_async()
        return False

    def goto(self, url: str, mode: BrowserMode = BrowserMode.LIGHT, wait_for: Optional[str] = None) -> Page:
        """Navigate to a URL (sync)."""
        return self.client.goto(url, mode, wait_for)

    async def goto_async(self, url: str, mode: BrowserMode = BrowserMode.LIGHT, wait_for: Optional[str] = None) -> Page:
        """Navigate to a URL (async)."""
        return await self.client.goto_async(url, mode, wait_for)

    def get_links(self) -> List[str]:
        """Get all links (sync)."""
        return self.client.get_links()

    async def get_links_async(self) -> List[str]:
        """Get all links (async)."""
        return await self.client.get_links_async()

    @property
    def tools(self) -> List[Tool]:
        """Get list of available tools."""
        return self.client.tools

    @property
    def is_connected(self) -> bool:
        """Check if connected."""
        return self.client.is_connected

    @property
    def protocol_version(self) -> Optional[str]:
        """Get MCP protocol version."""
        return self.client.protocol_version

    @property
    def server_version(self) -> Optional[str]:
        """Get b4n1web server version."""
        return self.client.server_version
