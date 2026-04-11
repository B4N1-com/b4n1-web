"""
SecurityShield Module for b4n1-web

Provides URL security validation with persistent caching using SQLite.
"""

import sqlite3
import logging
import os
from datetime import datetime, timedelta
from typing import Optional, Tuple
from urllib.parse import urlparse

logger = logging.getLogger(__name__)

DEFAULT_CACHE_DAYS = 7
DEFAULT_DB_PATH = os.path.expanduser("~/.b4n1web/security.db")


class SecurityShield:
    """
    Security module for validating URLs before navigation.

    Uses SQLite cache to track domain safety with 7-day TTL.
    Fall-safe: returns True (safe) if any error occurs.
    """

    def __init__(
        self, db_path: Optional[str] = None, cache_days: int = DEFAULT_CACHE_DAYS
    ):
        self.db_path = db_path or DEFAULT_DB_PATH
        self.cache_days = cache_days
        self._init_db()

    def _init_db(self) -> None:
        """Initialize SQLite database with domain_cache table."""
        try:
            os.makedirs(os.path.dirname(self.db_path), exist_ok=True)
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            cursor.execute("""
                CREATE TABLE IF NOT EXISTS domain_cache (
                    domain TEXT PRIMARY KEY,
                    is_safe BOOLEAN NOT NULL,
                    last_checked TIMESTAMP NOT NULL
                )
            """)
            conn.commit()
            conn.close()
            logger.debug(f"SecurityShield initialized: {self.db_path}")
        except Exception as e:
            logger.warning(f"Failed to initialize database: {e}")

    def _extract_domain(self, url: str) -> Optional[str]:
        """Extract domain from URL."""
        try:
            parsed = urlparse(url)
            if not parsed.netloc:
                return None
            return parsed.netloc.lower()
        except Exception:
            return None

    def is_url_safe(self, url: str) -> Tuple[bool, bool]:
        """
        Check if URL is safe to navigate.

        Args:
            url: The URL to check

        Returns:
            Tuple of (is_safe, needs_api_check)
            - is_safe: True if URL is safe (cached or default)
            - needs_api_check: True if should verify via external API
        """
        domain = self._extract_domain(url)
        if not domain:
            logger.warning(f"Could not extract domain from URL: {url}")
            return True, False

        try:
            return self._check_cache(domain)
        except Exception as e:
            logger.warning(f"Database error in security check: {e}")
            return True, False

    def _check_cache(self, domain: str) -> Tuple[bool, bool]:
        """Check cache for domain."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()

        try:
            cursor.execute(
                "SELECT is_safe, last_checked FROM domain_cache WHERE domain = ?",
                (domain,),
            )
            row = cursor.fetchone()

            if row is None:
                conn.close()
                return True, True

            is_safe, last_checked = row
            last_checked_dt = datetime.fromisoformat(last_checked)
            expires_at = last_checked_dt + timedelta(days=self.cache_days)

            if datetime.now() > expires_at:
                conn.close()
                return True, True

            conn.close()
            return bool(is_safe), False

        except Exception as e:
            conn.close()
            logger.warning(f"Cache check error: {e}")
            return True, False

    def mark_domain(self, domain: str, is_safe: bool) -> None:
        """Mark a domain as safe or unsafe."""
        domain = domain.lower()

        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            cursor.execute(
                """
                INSERT OR REPLACE INTO domain_cache (domain, is_safe, last_checked)
                VALUES (?, ?, ?)
                """,
                (domain, is_safe, datetime.now().isoformat()),
            )
            conn.commit()
            conn.close()
            logger.info(f"Marked domain {domain} as {'safe' if is_safe else 'unsafe'}")
        except Exception as e:
            logger.warning(f"Failed to mark domain: {e}")

    def clear_cache(self) -> None:
        """Clear all cached domains."""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            cursor.execute("DELETE FROM domain_cache")
            conn.commit()
            conn.close()
            logger.info("Security cache cleared")
        except Exception as e:
            logger.warning(f"Failed to clear cache: {e}")


def navigate(
    url: str,
    ignore_security: bool = False,
    security_shield: Optional[SecurityShield] = None,
) -> dict:
    """
    Navigate to URL with optional security check.

    Args:
        url: URL to navigate to
        ignore_security: Skip security check if True
        security_shield: SecurityShield instance (creates default if None)

    Returns:
        Dictionary with navigation result

    Example:
        >>> result = navigate("https://example.com")
        >>> print(result)
        {'url': 'https://example.com', 'success': True, ...}
    """
    from b4n1web.browser import AgentBrowser, BrowserMode

    if security_shield is None:
        security_shield = SecurityShield()

    if not ignore_security:
        is_safe, needs_check = security_shield.is_url_safe(url)

        if not is_safe:
            return {
                "url": url,
                "success": False,
                "error": "URL flagged as unsafe by security check",
            }

        if needs_check:
            logger.info(f"Domain {url} needs external verification (placeholder)")

    try:
        browser = AgentBrowser(mode=BrowserMode.LIGHT)
        page = browser.goto(url)
        browser.close()

        return {
            "url": page.url,
            "markdown": page.markdown,
            "links": page.links,
            "success": True,
        }
    except Exception as e:
        return {"url": url, "success": False, "error": str(e)}


if __name__ == "__main__":
    logging.basicConfig(level=logging.DEBUG)

    shield = SecurityShield()

    result = shield.is_url_safe("https://example.com")
    print(f"example.com: safe={result[0]}, needs_check={result[1]}")

    shield.mark_domain("example.com", True)
    result = shield.is_url_safe("https://example.com")
    print(f"example.com (cached): safe={result[0]}, needs_check={result[1]}")
