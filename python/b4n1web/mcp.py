"""
B4n1Web SDK - MCP Client

MCP (Model Context Protocol) client for connecting to b4n1web MCP server.
Provides async support, typed interfaces, and full feature coverage.
"""

import asyncio
import json
import subprocess
import threading
from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Union

from .browser import BinaryNotFoundError, BrowserMode, Page, get_b4n1web_binary


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

    Spawns b4n1web mcp subprocess and communicates via stdio JSON-RPC.

    Example:
        >>> client = McpClient()
        >>> await client.connect()
        >>> page = await client.goto("https://example.com")
        >>> print(page.markdown)
    """

    def __init__(
        self,
        binary_path: Optional[str] = None,
        timeout: float = 30.0,
    ):
        resolved = binary_path or get_b4n1web_binary()
        if not resolved:
            raise BinaryNotFoundError()
        self.binary_path = resolved
        self.timeout = timeout
        self._process: Optional[subprocess.Popen] = None
        self._request_id = 0
        self._lock = threading.Lock()
        self._tools: List[Tool] = []
        self._protocol_version: Optional[str] = None
        self._server_version: Optional[str] = None

    def connect(self) -> None:
        """Spawn b4n1web mcp subprocess and initialize."""
        self._process = subprocess.Popen(
            [self.binary_path, "mcp"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )

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
        """Terminate the MCP subprocess."""
        if self._process:
            self._process.terminate()
            try:
                self._process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self._process.kill()
            self._process = None

    async def disconnect_async(self) -> None:
        """Disconnect from MCP server (async)."""
        await asyncio.to_thread(self.disconnect)

    def _send_request(self, method: str, params: Dict[str, Any]) -> McpResponse:
        """Send JSON-RPC request via subprocess stdio."""
        with self._lock:
            if not self._process or not self._process.stdin or not self._process.stdout:
                raise RuntimeError("Not connected")

            self._request_id += 1
            request = {
                "jsonrpc": "2.0",
                "id": self._request_id,
                "method": method,
                "params": params,
            }

            request_str = json.dumps(request) + "\n"
            self._process.stdin.write(request_str)
            self._process.stdin.flush()

            response_str = self._process.stdout.readline()
            if not response_str:
                stderr = self._read_stderr()
                raise RuntimeError(
                    f"MCP subprocess closed unexpectedly. stderr: {stderr}"
                )

            response = json.loads(response_str.strip())
            return McpResponse.from_dict(response)

    def _read_stderr(self) -> str:
        """Read any available stderr output."""
        if not self._process or not self._process.stderr:
            return ""
        try:
            return self._process.stderr.read()
        except Exception:
            return ""

    async def _send_request_async(
        self, method: str, params: Dict[str, Any]
    ) -> McpResponse:
        """Send JSON-RPC request to server (async)."""
        return await asyncio.to_thread(self._send_request, method, params)

    def _call_tool(self, name: str, arguments: Dict[str, Any]) -> McpResponse:
        """Call an MCP tool and raise on error."""
        response = self._send_request(
            "tools/call", {"name": name, "arguments": arguments}
        )
        if response.error:
            raise RuntimeError(f"MCP error: {response.error.message}")
        return response

    async def _call_tool_async(
        self, name: str, arguments: Dict[str, Any]
    ) -> McpResponse:
        """Call an MCP tool and raise on error (async)."""
        response = await self._send_request_async(
            "tools/call", {"name": name, "arguments": arguments}
        )
        if response.error:
            raise RuntimeError(f"MCP error: {response.error.message}")
        return response

    @staticmethod
    def _get_text(response: McpResponse) -> str:
        """Extract text content from tool response."""
        content = response.result.get("content", []) if response.result else []
        return "".join(
            c.get("text", "") for c in content if c.get("type") == "text"
        )

    # --- Navigation ---

    def goto(
        self,
        url: str,
        mode: BrowserMode = BrowserMode.LIGHT,
        wait_for: Optional[str] = None,
    ) -> Page:
        """Navigate to a URL and extract content.

        Args:
            url: URL to navigate to
            mode: Browser mode (LIGHT, JS, RENDER)
            wait_for: CSS selector to wait for before extracting content (render mode only)
        """
        args: Dict[str, Any] = {"url": url, "mode": mode.value}
        if wait_for:
            args["wait_for"] = wait_for

        response = self._call_tool("goto", args)
        return self._parse_goto_result(response)

    async def goto_async(
        self,
        url: str,
        mode: BrowserMode = BrowserMode.LIGHT,
        wait_for: Optional[str] = None,
    ) -> Page:
        """Navigate to a URL and extract content (async).

        Args:
            url: URL to navigate to
            mode: Browser mode (LIGHT, JS, RENDER)
            wait_for: CSS selector to wait for before extracting content (render mode only)
        """
        args: Dict[str, Any] = {"url": url, "mode": mode.value}
        if wait_for:
            args["wait_for"] = wait_for

        response = await self._call_tool_async("goto", args)
        return self._parse_goto_result(response)

    def _parse_goto_result(self, response: McpResponse) -> Page:
        """Parse goto tool result."""
        if not response.result:
            raise RuntimeError("Empty response from goto")

        text = self._get_text(response)

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

    # --- Links ---

    def get_links(self) -> List[str]:
        """Get all links from current page."""
        response = self._call_tool("get_links", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return []

    async def get_links_async(self) -> List[str]:
        """Get all links from current page (async)."""
        response = await self._call_tool_async("get_links", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return []

    # --- Click ---

    def click(self, selector: str) -> str:
        """Click an element on the page.

        Args:
            selector: CSS selector for the element to click
        """
        response = self._call_tool("click", {"selector": selector})
        return self._get_text(response)

    async def click_async(self, selector: str) -> str:
        """Click an element on the page (async).

        Args:
            selector: CSS selector for the element to click
        """
        response = await self._call_tool_async("click", {"selector": selector})
        return self._get_text(response)

    # --- Type Text ---

    def type_text(
        self, selector: str, text: str, clear_first: bool = False
    ) -> str:
        """Type text into an element.

        Args:
            selector: CSS selector for the target element
            text: Text to type
            clear_first: Whether to clear existing content first
        """
        response = self._call_tool(
            "type_text",
            {"selector": selector, "text": text, "clear_first": clear_first},
        )
        return self._get_text(response)

    async def type_text_async(
        self, selector: str, text: str, clear_first: bool = False
    ) -> str:
        """Type text into an element (async).

        Args:
            selector: CSS selector for the target element
            text: Text to type
            clear_first: Whether to clear existing content first
        """
        response = await self._call_tool_async(
            "type_text",
            {"selector": selector, "text": text, "clear_first": clear_first},
        )
        return self._get_text(response)

    # --- Wait For Selector ---

    def wait_for_selector(
        self, selector: str, timeout_ms: int = 5000
    ) -> str:
        """Wait for an element to appear on the page.

        Args:
            selector: CSS selector to wait for
            timeout_ms: Maximum time to wait in milliseconds
        """
        response = self._call_tool(
            "wait_for_selector", {"selector": selector, "timeout": timeout_ms}
        )
        return self._get_text(response)

    async def wait_for_selector_async(
        self, selector: str, timeout_ms: int = 5000
    ) -> str:
        """Wait for an element to appear on the page (async).

        Args:
            selector: CSS selector to wait for
            timeout_ms: Maximum time to wait in milliseconds
        """
        response = await self._call_tool_async(
            "wait_for_selector", {"selector": selector, "timeout": timeout_ms}
        )
        return self._get_text(response)

    # --- Screenshot ---

    def screenshot(self, url: str) -> str:
        """Take a screenshot of a page.

        Args:
            url: URL to screenshot

        Returns:
            Base64-encoded PNG data URI
        """
        response = self._call_tool("screenshot", {"url": url})
        return self._get_text(response)

    async def screenshot_async(self, url: str) -> str:
        """Take a screenshot of a page (async).

        Args:
            url: URL to screenshot

        Returns:
            Base64-encoded PNG data URI
        """
        response = await self._call_tool_async("screenshot", {"url": url})
        return self._get_text(response)

    # --- Frames ---

    def frames(self) -> List[Dict[str, Any]]:
        """Get information about all iframes on the page."""
        response = self._call_tool("frames", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return []

    async def frames_async(self) -> List[Dict[str, Any]]:
        """Get information about all iframes on the page (async)."""
        response = await self._call_tool_async("frames", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return []

    # --- Iframe Text ---

    def iframe_text(self, index: int) -> str:
        """Get text content from an iframe.

        Args:
            index: Index of the iframe (0-based)
        """
        response = self._call_tool("iframe_text", {"index": index})
        return self._get_text(response)

    async def iframe_text_async(self, index: int) -> str:
        """Get text content from an iframe (async).

        Args:
            index: Index of the iframe (0-based)
        """
        response = await self._call_tool_async("iframe_text", {"index": index})
        return self._get_text(response)

    # --- Iframe Click ---

    def iframe_click(self, frame_index: int, selector: str) -> str:
        """Click an element inside an iframe.

        Args:
            frame_index: Index of the iframe (0-based)
            selector: CSS selector for the element to click
        """
        response = self._call_tool(
            "iframe_click",
            {"frame_index": frame_index, "selector": selector},
        )
        return self._get_text(response)

    async def iframe_click_async(
        self, frame_index: int, selector: str
    ) -> str:
        """Click an element inside an iframe (async).

        Args:
            frame_index: Index of the iframe (0-based)
            selector: CSS selector for the element to click
        """
        response = await self._call_tool_async(
            "iframe_click",
            {"frame_index": frame_index, "selector": selector},
        )
        return self._get_text(response)

    # --- Set Viewport ---

    def set_viewport(self, width: int, height: int) -> str:
        """Set the browser viewport size.

        Args:
            width: Viewport width in pixels
            height: Viewport height in pixels
        """
        response = self._call_tool(
            "set_viewport", {"width": width, "height": height}
        )
        return self._get_text(response)

    async def set_viewport_async(self, width: int, height: int) -> str:
        """Set the browser viewport size (async).

        Args:
            width: Viewport width in pixels
            height: Viewport height in pixels
        """
        response = await self._call_tool_async(
            "set_viewport", {"width": width, "height": height}
        )
        return self._get_text(response)

    # --- Set User Agent ---

    def set_user_agent(self, ua: str) -> str:
        """Set the browser user agent string.

        Args:
            ua: User agent string
        """
        response = self._call_tool("set_user_agent", {"ua": ua})
        return self._get_text(response)

    async def set_user_agent_async(self, ua: str) -> str:
        """Set the browser user agent string (async).

        Args:
            ua: User agent string
        """
        response = await self._call_tool_async(
            "set_user_agent", {"ua": ua}
        )
        return self._get_text(response)

    # --- Emulate Device ---

    def emulate_device(self, device: str) -> str:
        """Emulate a device.

        Args:
            device: Device name (e.g. "iPhone 12", "desktop")
        """
        response = self._call_tool("emulate_device", {"device": device})
        return self._get_text(response)

    async def emulate_device_async(self, device: str) -> str:
        """Emulate a device (async).

        Args:
            device: Device name (e.g. "iPhone 12", "desktop")
        """
        response = await self._call_tool_async(
            "emulate_device", {"device": device}
        )
        return self._get_text(response)

    # --- Cookies ---

    def cookies(self) -> Dict[str, Any]:
        """Get all cookies from the current page."""
        response = self._call_tool("cookies", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return {"raw": text.strip()}

    async def cookies_async(self) -> Dict[str, Any]:
        """Get all cookies from the current page (async)."""
        response = await self._call_tool_async("cookies", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return {"raw": text.strip()}

    # --- Upload File ---

    def upload_file(self, selector: str, path: str) -> str:
        """Upload a file to a file input element.

        Args:
            selector: CSS selector for the file input element
            path: Path to the file to upload
        """
        response = self._call_tool(
            "upload_file", {"selector": selector, "path": path}
        )
        return self._get_text(response)

    async def upload_file_async(self, selector: str, path: str) -> str:
        """Upload a file to a file input element (async).

        Args:
            selector: CSS selector for the file input element
            path: Path to the file to upload
        """
        response = await self._call_tool_async(
            "upload_file", {"selector": selector, "path": path}
        )
        return self._get_text(response)

    # --- Download File ---

    def download_file(self, url: str, output: str) -> str:
        """Download a file.

        Args:
            url: URL of the file to download
            output: Output file path
        """
        response = self._call_tool(
            "download_file", {"url": url, "output": output}
        )
        return self._get_text(response)

    async def download_file_async(self, url: str, output: str) -> str:
        """Download a file (async).

        Args:
            url: URL of the file to download
            output: Output file path
        """
        response = await self._call_tool_async(
            "download_file", {"url": url, "output": output}
        )
        return self._get_text(response)

    # --- Get Links From Page ---

    def get_links_from_page(self, url: str) -> List[str]:
        """Get all links from a URL.

        Args:
            url: URL to extract links from
        """
        response = self._call_tool(
            "get_links_from_page", {"url": url}
        )
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return []

    async def get_links_from_page_async(self, url: str) -> List[str]:
        """Get all links from a URL (async).

        Args:
            url: URL to extract links from
        """
        response = await self._call_tool_async(
            "get_links_from_page", {"url": url}
        )
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return []

    # --- Performance Metrics ---

    def performance_metrics(self) -> Dict[str, Any]:
        """Get performance metrics from the current page."""
        response = self._call_tool("performance_metrics", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return {"raw": text.strip()}

    async def performance_metrics_async(self) -> Dict[str, Any]:
        """Get performance metrics from the current page (async)."""
        response = await self._call_tool_async("performance_metrics", {})
        text = self._get_text(response)
        try:
            return json.loads(text.strip())
        except Exception:
            return {"raw": text.strip()}

    # --- Properties ---

    @property
    def tools(self) -> List[Tool]:
        """Get list of available tools."""
        return self._tools

    @property
    def is_connected(self) -> bool:
        """Check if subprocess is running."""
        return self._process is not None and self._process.poll() is None

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
        binary_path: Optional[str] = None,
        timeout: float = 30.0,
    ):
        self.client = McpClient(binary_path=binary_path, timeout=timeout)

    async def __aenter__(self):
        """Async context manager entry."""
        await self.client.connect_async()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit."""
        await self.client.disconnect_async()
        return False

    def goto(
        self,
        url: str,
        mode: BrowserMode = BrowserMode.LIGHT,
        wait_for: Optional[str] = None,
    ) -> Page:
        return self.client.goto(url, mode, wait_for)

    async def goto_async(
        self,
        url: str,
        mode: BrowserMode = BrowserMode.LIGHT,
        wait_for: Optional[str] = None,
    ) -> Page:
        return await self.client.goto_async(url, mode, wait_for)

    def get_links(self) -> List[str]:
        return self.client.get_links()

    async def get_links_async(self) -> List[str]:
        return await self.client.get_links_async()

    def click(self, selector: str) -> str:
        return self.client.click(selector)

    async def click_async(self, selector: str) -> str:
        return await self.client.click_async(selector)

    def type_text(
        self, selector: str, text: str, clear_first: bool = False
    ) -> str:
        return self.client.type_text(selector, text, clear_first)

    async def type_text_async(
        self, selector: str, text: str, clear_first: bool = False
    ) -> str:
        return await self.client.type_text_async(selector, text, clear_first)

    def wait_for_selector(
        self, selector: str, timeout_ms: int = 5000
    ) -> str:
        return self.client.wait_for_selector(selector, timeout_ms)

    async def wait_for_selector_async(
        self, selector: str, timeout_ms: int = 5000
    ) -> str:
        return await self.client.wait_for_selector_async(
            selector, timeout_ms
        )

    def screenshot(self, url: str) -> str:
        return self.client.screenshot(url)

    async def screenshot_async(self, url: str) -> str:
        return await self.client.screenshot_async(url)

    def frames(self) -> List[Dict[str, Any]]:
        return self.client.frames()

    async def frames_async(self) -> List[Dict[str, Any]]:
        return await self.client.frames_async()

    def iframe_text(self, index: int) -> str:
        return self.client.iframe_text(index)

    async def iframe_text_async(self, index: int) -> str:
        return await self.client.iframe_text_async(index)

    def iframe_click(self, frame_index: int, selector: str) -> str:
        return self.client.iframe_click(frame_index, selector)

    async def iframe_click_async(
        self, frame_index: int, selector: str
    ) -> str:
        return await self.client.iframe_click_async(frame_index, selector)

    def set_viewport(self, width: int, height: int) -> str:
        return self.client.set_viewport(width, height)

    async def set_viewport_async(self, width: int, height: int) -> str:
        return await self.client.set_viewport_async(width, height)

    def set_user_agent(self, ua: str) -> str:
        return self.client.set_user_agent(ua)

    async def set_user_agent_async(self, ua: str) -> str:
        return await self.client.set_user_agent_async(ua)

    def emulate_device(self, device: str) -> str:
        return self.client.emulate_device(device)

    async def emulate_device_async(self, device: str) -> str:
        return await self.client.emulate_device_async(device)

    def cookies(self) -> Dict[str, Any]:
        return self.client.cookies()

    async def cookies_async(self) -> Dict[str, Any]:
        return await self.client.cookies_async()

    def upload_file(self, selector: str, path: str) -> str:
        return self.client.upload_file(selector, path)

    async def upload_file_async(self, selector: str, path: str) -> str:
        return await self.client.upload_file_async(selector, path)

    def download_file(self, url: str, output: str) -> str:
        return self.client.download_file(url, output)

    async def download_file_async(self, url: str, output: str) -> str:
        return await self.client.download_file_async(url, output)

    def get_links_from_page(self, url: str) -> List[str]:
        return self.client.get_links_from_page(url)

    async def get_links_from_page_async(self, url: str) -> List[str]:
        return await self.client.get_links_from_page_async(url)

    def performance_metrics(self) -> Dict[str, Any]:
        return self.client.performance_metrics()

    async def performance_metrics_async(self) -> Dict[str, Any]:
        return await self.client.performance_metrics_async()

    @property
    def tools(self) -> List[Tool]:
        return self.client.tools

    @property
    def is_connected(self) -> bool:
        return self.client.is_connected

    @property
    def protocol_version(self) -> Optional[str]:
        return self.client.protocol_version

    @property
    def server_version(self) -> Optional[str]:
        return self.client.server_version
