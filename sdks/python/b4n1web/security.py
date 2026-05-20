"""
B4n1Web SecurityShield — URL security validation with in-memory cache.

SecurityShield provides URL safety checking for agentic browser navigation:
- New domains default to safe=True, needs_api_check=True (unknown → verify)
- Mark domains as safe (whitelist) or unsafe (blacklist)
- Cache results with TTL (cache_days)
- Invalid/malformed URLs default to safe=True, no API check needed
"""

import time
from typing import Tuple


class SecurityShield:
    """URL security validation with in-memory caching.

    Args:
        cache_days: Number of days to cache domain results (default: 7).
    """

    def __init__(self, cache_days: int = 7) -> None:
        self._cache_days = cache_days
        self._cache: dict = {}  # domain → {"is_safe": bool, "expires": float}

    # ── Core API ──────────────────────────────────────────────

    def is_url_safe(self, raw_url: str) -> Tuple[bool, bool]:
        """Check if a URL is safe to navigate.

        Args:
            raw_url: The URL to check.

        Returns:
            Tuple of (is_safe, needs_api_check).
            - is_safe=True  → safe to navigate.
            - needs_api_check=False → result is cached and trusted (no external call).
            - Example: (True, False) = safe and cached.
            - Example: (True, True)  = safe but unverified (call API).
            - Example: (False, False) = explicitly blocked (cached).
        """
        domain = self._extract_domain(raw_url)
        if not domain:
            # Invalid URL — safe default, no API check needed
            return True, False

        now = time.time()
        entry = self._cache.get(domain)
        if entry is not None:
            if now < entry["expires"]:
                return entry["is_safe"], False
            # Expired — remove and re-check
            del self._cache[domain]

        # Unknown domain — safe but needs API verification
        return True, True

    def mark_domain(self, domain: str, is_safe: bool) -> None:
        """Explicitly mark a domain as safe or unsafe.

        Overwrites any previous entry for the domain and resets the TTL.

        Args:
            domain: Domain name (case-insensitive, e.g. "example.com").
            is_safe: True to whitelist, False to blacklist.
        """
        normalized = domain.lower().strip()
        expires = time.time() + self._cache_days * 86400
        self._cache[normalized] = {"is_safe": is_safe, "expires": expires}

    def clear_cache(self) -> None:
        """Remove all cached domain entries."""
        self._cache.clear()

    # ── Internal helpers ──────────────────────────────────────

    @staticmethod
    def _extract_domain(raw_url: str) -> str:
        """Extract hostname from URL, or '' on failure."""
        try:
            # Lightweight parse without external deps
            url = raw_url.strip()
            if "://" in url:
                url = url.split("://", 1)[1]
            host = url.split("/", 1)[0]
            host = host.split("?", 1)[0]
            host = host.split("@", 1)[-1]    # strip userinfo
            host = host.split(":", 1)[0]      # strip port
            if not host or "." not in host and host != "localhost":
                return ""
            return host.lower()
        except Exception:
            return ""
