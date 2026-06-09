"""
B4n1Web SDK - MCP Types
"""

from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Union


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


@dataclass
class McpResponse:
    """JSON-RPC response from MCP server."""

    jsonrpc: str
    id: Union[int, None]
    result: Optional[Dict[str, Any]] = None
    error: Optional[McpError] = None

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
