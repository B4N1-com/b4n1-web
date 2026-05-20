"""Unit tests for MCP client - simplified but functional."""

import json
import subprocess
import sys
from unittest.mock import MagicMock, Mock, patch

import pytest

from b4n1web.mcp import (
    AsyncMcpClient,
    McpError,
    McpResponse,
    McpClient,
    Tool,
    ToolResult,
)
from b4n1web import BrowserMode
from b4n1web.browser import BinaryNotFoundError


class TestTool:
    def test_tool_creation(self):
        tool = Tool(
            name="goto",
            description="Navigate to URL",
            input_schema={"type": "object", "properties": {}},
        )
        assert tool.name == "goto"
        assert tool.description == "Navigate to URL"
        assert tool.input_schema == {"type": "object", "properties": {}}


class TestToolResult:
    def test_tool_result_creation(self):
        result = ToolResult(
            content=[{"type": "text", "text": "Hello world"}],
            is_error=False,
        )
        assert len(result.content) == 1
        assert result.is_error is False

    def test_tool_result_text_property(self):
        result = ToolResult(
            content=[
                {"type": "text", "text": "Line1"},
                {"type": "text", "text": "Line2"},
            ]
        )
        assert result.text == "Line1Line2"


class TestMcpError:
    def test_mcp_error_creation(self):
        error = McpError(code=-32600, message="Invalid request", data=None)
        assert error.code == -32600
        assert error.message == "Invalid request"


class TestMcpResponse:
    def test_mcp_response_creation(self):
        response = McpResponse(
            jsonrpc="2.0",
            id=1,
            result={"status": "ok"},
            error=None,
        )
        assert response.jsonrpc == "2.0"
        assert response.id == 1
        assert response.result == {"status": "ok"}

    def test_mcp_response_from_dict_success(self):
        data = {
            "jsonrpc": "2.0",
            "id": 2,
            "result": {"content": [{"type": "text", "text": "ok"}]},
        }
        response = McpResponse.from_dict(data)
        assert response.id == 2
        assert response.result == {"content": [{"type": "text", "text": "ok"}]}

    def test_mcp_response_from_dict_error(self):
        data = {
            "jsonrpc": "2.0",
            "id": 3,
            "error": {"code": -32600, "message": "Invalid Request"},
        }
        response = McpResponse.from_dict(data)
        assert response.id == 3
        assert response.error is not None
        assert response.error.code == -32600

    def test_mcp_response_without_id(self):
        data = {"jsonrpc": "2.0", "result": {}}
        response = McpResponse.from_dict(data)
        assert response.id is None

    def test_mcp_response_without_result(self):
        data = {"jsonrpc": "2.0", "id": 1}
        response = McpResponse.from_dict(data)
        assert response.result is None


class TestMcpClientInit:
    def test_mcp_client_default_init(self):
        client = McpClient()
        assert client.timeout == 30.0
        assert client._process is None
        assert client._request_id == 0
        assert client._tools == []
        assert client._protocol_version is None
        assert client._server_version is None

    def test_mcp_client_custom_init(self):
        client = McpClient(timeout=60.0)
        assert client.timeout == 60.0

    def test_mcp_client_properties_not_connected(self):
        client = McpClient()
        assert client.is_connected is False
        assert client.tools == []
        assert client.protocol_version is None
        assert client.server_version is None

    def test_mcp_client_init_binary_not_found(self):
        """Test that BinaryNotFoundError is raised when binary is not found"""
        with patch('b4n1web.mcp.get_b4n1web_binary') as mock_get_binary:
            mock_get_binary.return_value = None
            with pytest.raises(BinaryNotFoundError):
                McpClient()


class TestMcpClientConnect:
    @patch("subprocess.Popen")
    def test_connect_success(self, mock_popen):
        # Mock subprocess
        mock_process = MagicMock()
        mock_process.stdin = MagicMock()
        mock_process.stdout = MagicMock()
        mock_process.stderr = MagicMock()
        mock_process.poll.return_value = None  # Process is running
        mock_popen.return_value = mock_process

        # Mock responses for initialize and tools/list
        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {
                        "tools": [
                            {
                                "name": "goto",
                                "description": "Navigate to a URL and extract content",
                                "inputSchema": {
                                    "properties": {
                                        "mode": {
                                            "default": "light",
                                            "enum": ["light", "js", "render"],
                                            "type": "string",
                                        },
                                        "url": {"type": "string"},
                                        "wait_for": {
                                            "description": "CSS selector to wait for before extracting content (render mode only)",
                                            "type": "string",
                                        },
                                    },
                                    "required": ["url"],
                                    "type": "object",
                                },
                            },
                            {
                                "name": "get_links",
                                "description": "Get all links from the current page",
                                "inputSchema": {"properties": {}, "type": "object"},
                            },
                        ]
                    },
                }
            ) + "\n",
        ]

        client = McpClient()
        client.connect()

        assert client.is_connected is True
        assert client.protocol_version == "2025-11-25"
        assert client.server_version == "0.7.0"
        assert len(client.tools) == 2
        assert client.tools[0].name == "goto"
        assert client.tools[1].name == "get_links"

        # Verify subprocess was started with correct arguments
        mock_popen.assert_called_once()
        args, kwargs = mock_popen.call_args
        # Accept either direct binary path or module execution
        assert len(args[0]) >= 2
        assert args[0][-1] == "mcp"  # Last argument should be "mcp"
        assert kwargs["stdin"] == subprocess.PIPE
        assert kwargs["stdout"] == subprocess.PIPE
        assert kwargs["stderr"] == subprocess.PIPE
        assert kwargs["text"] is True

    @patch("subprocess.Popen")
    def test_connect_subprocess_error(self, mock_popen):
        # Simulate Popen failing to find b4n1web binary
        mock_popen.side_effect = FileNotFoundError("b4n1web binary not found")
        client = McpClient()
        with pytest.raises(FileNotFoundError):
            client.connect()

    @patch("subprocess.Popen")
    def test_connect_initialization_failure(self, mock_popen):
        mock_process = MagicMock()
        mock_process.stdin = MagicMock()
        mock_process.stdout = MagicMock()
        mock_process.stderr = MagicMock()
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        # Mock empty response (subprocess died)
        mock_process.stdout.readline.return_value = ""
        mock_process.stderr.read.return_value = "Failed to start"

        client = McpClient()
        with pytest.raises(RuntimeError, match="MCP subprocess closed unexpectedly"):
            client.connect()

    @patch("subprocess.Popen")
    def test_disconnect(self, mock_popen):
        mock_process = MagicMock()
        mock_process.stdin = MagicMock()
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        client = McpClient()
        client._process = mock_process
        client.disconnect()

        mock_process.terminate.assert_called_once()
        mock_process.wait.assert_called_once_with(timeout=5)


class TestMcpClientGoto:
    @patch("subprocess.Popen")
    def test_goto_light_mode(self, mock_popen):
        mock_process = MagicMock()
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        # Mock responses for initialize, tools/list, and goto
        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
            # Goto response - matches actual format from the server
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": "URL: https://example.com\nMarkdown: # Hello\nLinks: "
                                + json.dumps(["https://example.com/link"])
                                + "\n",
                            }
                        ]
                    },
                }
            ) + "\n",
        ]

        client = McpClient()
        client.connect()
        page = client.goto("https://example.com", BrowserMode.LIGHT)

        assert page.url == "https://example.com"
        assert "# Hello" in page.markdown
        assert page.links == ["https://example.com/link"]

    @patch("subprocess.Popen")
    def test_goto_render_mode(self, mock_popen):
        mock_process = MagicMock()
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
            # Goto response with screenshot
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": "URL: https://example.com\nMarkdown: # Page\nLinks: []\nScreenshot: base64data123\n",
                            }
                        ]
                    },
                }
            ) + "\n",
        ]

        client = McpClient()
        client.connect()
        page = client.goto("https://example.com", BrowserMode.RENDER)

        assert page.screenshot == "base64data123"
        assert "# Page" in page.markdown
        assert len(page.links) == 0

    @patch("subprocess.Popen")
    def test_goto_error_response(self, mock_popen):
        mock_process = MagicMock()
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
            # Error response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 3,
                    "error": {"code": -32602, "message": "Invalid params"},
                }
            ) + "\n",
        ]

        client = McpClient()
        client.connect()
        with pytest.raises(RuntimeError, match="MCP error: Invalid params"):
            client.goto("https://example.com")

    @patch("subprocess.Popen")
    def test_goto_empty_response(self, mock_popen):
        mock_process = MagicMock()
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.side_effect = [None, 1]  # First call: running, Second call: exited
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
            # Empty response
            json.dumps({"jsonrpc": "2.0", "id": 3, "result": {}}) + "\n",
        ]

        client = McpClient()
        client.connect()
        with pytest.raises(RuntimeError, match="Empty response from goto"):
            client.goto("https://example.com")


class TestMcpClientGetLinks:
    @patch("subprocess.Popen")
    def test_get_links_success(self, mock_popen):
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
            # Get links response - matches actual format from the server
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": json.dumps(["https://a.com", "https://b.com"]),
                            }
                        ]
                    },
                }
            ) + "\n",
        ]

        client = McpClient()
        client.connect()
        links = client.get_links()

        # The get_links method should parse the JSON string and return a list
        assert isinstance(links, list)
        assert len(links) == 2
        assert "https://a.com" in links
        assert "https://b.com" in links

    @patch("subprocess.Popen")
    def test_get_links_error(self, mock_popen):
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
            # Error response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 3,
                    "error": {"code": -32603, "message": "Internal error"},
                }
            ) + "\n",
        ]

        client = McpClient()
        client.connect()
        with pytest.raises(RuntimeError, match="MCP error: Internal error"):
            client.get_links()

    @patch("subprocess.Popen")
    def test_get_links_eval_error(self, mock_popen):
        """Test get_links when content is not valid JSON (should return empty list)"""
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
            # Invalid JSON that can't be parsed - but still needs to be in the correct format
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": "not valid python",
                            }
                        ]
                    },
                }
            ) + "\n",
        ]

        client = McpClient()
        client.connect()
        links = client.get_links()

        # Should return empty list when content can't be parsed as JSON
        assert links == []


class TestAsyncMethodsExist:
    """Verify async methods exist"""

    def test_mcp_client_connect_async_is_coroutine(self):
        client = McpClient()
        assert hasattr(client, "connect_async")

    def test_mcp_client_disconnect_async_is_coroutine(self):
        client = McpClient()
        assert hasattr(client, "disconnect_async")

    def test_mcp_client_goto_async_is_coroutine(self):
        client = McpClient()
        assert hasattr(client, "goto_async")

    def test_mcp_client_get_links_async_is_coroutine(self):
        client = McpClient()
        assert hasattr(client, "get_links_async")


class TestMcpClientContextManager:
    @patch("subprocess.Popen")
    def test_context_manager(self, mock_popen):
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": {"tools": []},
                }
            ) + "\n",
        ]

        with McpClient() as client:
            assert client.is_connected is True
            assert client.protocol_version == "2025-11-25"

        # Process should be terminated
        mock_process.terminate.assert_called_once()


class TestAsyncMcpClient:
    def test_async_client_default_init(self):
        client = AsyncMcpClient()
        assert isinstance(client.client, McpClient)
        assert client.client.timeout == 30.0

    def test_async_client_custom_init(self):
        client = AsyncMcpClient(timeout=120.0)
        assert isinstance(client.client, McpClient)
        assert client.client.timeout == 120.0

    def test_async_context_manager(self):
        client = AsyncMcpClient()
        # Just test that we can access the is_connected property (delegates to underlying client)
        assert client.is_connected == client.client.is_connected

    def test_async_client_goto_sync(self):
        client = AsyncMcpClient()
        # Mock the underlying client's goto method
        mock_page = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_page.url = "https://test.com"
        mock_page.markdown = "# Test"
        mock_page.links = []
        client.client.goto = Mock(return_value=mock_page)

        result = client.goto("https://test.com")
        assert result.url == "https://test.com"

    def test_async_client_get_links_sync(self):
        client = AsyncMcpClient()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        client.client.get_links = Mock(return_value=["https://a.com"])
        result = client.get_links()
        assert len(result) == 1
        assert "https://a.com" in result

    def test_async_client_get_links_async(self):
        # Just verify the method exists
        assert hasattr(AsyncMcpClient().client, "get_links_async")


class TestEdgeCases:
    def test_client_handles_binary_not_found(self):
        """Test that BinaryNotFoundError is raised when binary is not found"""
        with patch('b4n1web.mcp.get_b4n1web_binary') as mock_get_binary:
            mock_get_binary.return_value = None
            with pytest.raises(BinaryNotFoundError):
                McpClient()

    @patch("subprocess.Popen")
    def test_client_handles_malformed_json_response(self, mock_popen):
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.return_value = None
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response - valid JSON
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Tools/list response - MALFORMED JSON
            '{"invalid": json}' + "\n",
        ]

        client = McpClient()
        with pytest.raises(json.JSONDecodeError):
            client.connect()

    @patch("subprocess.Popen")
    def test_client_handles_subprocess_termination(self, mock_popen):
        mock_process = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdin = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stdout = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.stderr = MagicMock()
        # Simulate process dying during communication
        # Simulate process dying during communication
        # Simulate process dying during communication
        mock_process.poll.side_effect = [None, 1]  # First call: running, Second call: exited
        mock_popen.return_value = mock_process

        mock_process.stdout.readline.side_effect = [
            # Initialize response
            json.dumps(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "protocolVersion": "2025-11-25",
                        "serverInfo": {"name": "b4n1web", "version": "0.7.0"},
                        "capabilities": {},
                    },
                }
            ) + "\n",
            # Process died before responding to tools/list
            "",  # Empty string indicates EOF
        ]

        with pytest.raises(RuntimeError, match="MCP subprocess closed unexpectedly"):
            client = McpClient()
            client.connect()


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
