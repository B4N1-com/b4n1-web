"""
B4n1Web SDK - Browser automation for AI agents

This module provides an interface to the B4n1Web Rust engine.
The B4n1Web binary is bundled with this package.
"""

import ast
import os
import subprocess
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import List, Optional
import requests

# Use system CA certificates instead of certifi bundle
os.environ.setdefault("REQUESTS_CA_BUNDLE", "/etc/ssl/certs/ca-certificates.crt")
os.environ.setdefault("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt")


class BrowserMode(Enum):
    """Browser execution modes."""

    LIGHT = "light"
    JS = "js"
    RENDER = "render"


@dataclass
class Page:
    """Structured page data returned by B4n1Web."""

    url: str
    markdown: str
    links: List[str]
    screenshot: Optional[str] = None

    def get_main_content(self) -> str:
        """Extract main content from markdown."""
        lines = self.markdown.split("\n")
        content_lines = lines[2:] if len(lines) > 2 else lines
        return "\n".join(content_lines).strip()

    def find_links_by_text(self, text: str) -> List[str]:
        """Find links containing specific text."""
        return [link for link in self.links if text.lower() in link.lower()]


SDK_VERSION = "0.6.0"


def get_b4n1web_binary() -> Optional[str]:
    """Find b4n1web binary in bundled location or system install."""
    bundled = Path(__file__).parent / "bin" / "b4n1web-linux"
    if bundled.exists() and os.access(bundled, os.X_OK):
        return str(bundled)

    possible = [
        "/usr/local/bin/b4n1web",
        "/usr/bin/b4n1web",
        str(Path.home() / ".local/bin/b4n1web"),
        str(Path.home() / ".b4n1web/bin/b4n1web"),
    ]
    for d in os.environ.get("PATH", "").split(":"):
        possible.append(os.path.join(d, "b4n1web"))

    for loc in possible:
        if os.path.isfile(loc) and os.access(loc, os.X_OK):
            return loc

    return None


def get_b4n1web_version() -> Optional[str]:
    """Get the version of the bundled b4n1web binary."""
    binary = get_b4n1web_binary()
    if not binary:
        return None
    try:
        result = subprocess.run(
            [binary, "--version"],
            capture_output=True,
            text=True,
            timeout=5,
        )
        output = result.stdout.strip()
        if output:
            parts = output.split()
            if len(parts) >= 2:
                return parts[-1]
        return None
    except Exception:
        return None


from .errors import BinaryNotFoundError


class AgentBrowser:
    """
    B4n1Web Agent Browser

    A browser instance optimized for AI agent workflows.
    The B4n1Web binary is bundled with this package.

    Example:
        >>> browser = AgentBrowser(mode=BrowserMode.LIGHT)
        >>> page = browser.goto("https://example.com")
        >>> print(page.markdown)
    """

    def __init__(
        self,
        mode: BrowserMode = BrowserMode.LIGHT,
        timeout: int = 30,
        user_agent: str = "B4n1Web-Agent/1.0",
    ):
        self.mode = mode
        self.timeout = timeout
        self.user_agent = user_agent
        self.session = requests.Session()
        self.session.verify = "/etc/ssl/certs/ca-certificates.crt"
        self.session.headers.update(
            {
                "User-Agent": user_agent,
                "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
                "Accept-Language": "en-US,en;q=0.5",
                "Accept-Encoding": "gzip, deflate",
                "DNT": "1",
                "Connection": "keep-alive",
                "Upgrade-Insecure-Requests": "1",
            }
        )
        binary_path = get_b4n1web_binary()
        if not binary_path:
            raise BinaryNotFoundError()

    @property
    def binary_path(self) -> str:
        """Get b4n1web binary path."""
        binary_path = get_b4n1web_binary()
        if not binary_path:
            raise BinaryNotFoundError()
        return binary_path

    def goto(self, url: str, wait_for: Optional[str] = None) -> Page:
        """Navigate to a URL and extract structured content.

        Args:
            url: URL to navigate to
            wait_for: CSS selector to wait for before extracting content (render mode only)
        """
        cmd = [self.binary_path, "goto", url, "--mode", self.mode.value]
        if wait_for:
            cmd.extend(["--wait-for", wait_for])

        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=self.timeout,
            )

            if result.returncode != 0:
                raise RuntimeError(f"Binary error: {result.stderr.strip()}")

            return self._parse_output(url, result.stdout)

        except subprocess.TimeoutExpired:
            raise RuntimeError(f"Binary timed out after {self.timeout}s")

    def _parse_output(self, url: str, output: str) -> Page:
        """Parse text output from the Rust binary."""
        markdown = ""
        links = []

        for line in output.splitlines():
            if line.startswith("URL:"):
                continue
            elif line.startswith("Markdown:"):
                continue
            elif line.startswith("Links:"):
                try:
                    links = ast.literal_eval(line[6:].strip())
                except (ValueError, SyntaxError):
                    links = []
            else:
                markdown += line + "\n"

        return Page(
            url=url,
            markdown=markdown.strip(),
            links=links,
        )

    def close(self):
        """Close the browser session."""
        self.session.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
        return False
