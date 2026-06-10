/**
 * B4n1Web SecurityShield — URL security validation with in-memory cache.
 *
 * SecurityShield provides URL safety checking for agentic browser navigation.
 *
 * API:
 *   new SecurityShield({ dbPath?, cacheDays? })
 *   shield.isUrlSafe(url)        → { isSafe: boolean, needsApiCheck: boolean }
 *   shield.markDomain(domain, isSafe)
 *   shield.clearCache()
 */

import type { SecurityShieldOptions, SecurityCheckResult } from './types';

/** Default cache TTL in days */
const DEFAULT_CACHE_DAYS = 7;

/** In-memory cache: domain → result + expiry ms timestamp */
type CacheEntry = { isSafe: boolean; expires: number };

export class SecurityShield {
  private cacheDays: number;
  private cache: Map<string, CacheEntry>;

  /**
   * Create a SecurityShield instance.
   * @param opts - Optional configuration
   * @param opts.cacheDays - TTL of a cached result in days (default 7)
   */
  constructor(opts: SecurityShieldOptions = {}) {
    this.cacheDays = opts.cacheDays ?? DEFAULT_CACHE_DAYS;
    this.cache = new Map();
  }

  /**
   * Check whether a URL is safe to navigate.
   *
   * @param rawURL - The URL to check.
   * @returns `{ isSafe, needsApiCheck }`
   *   isSafe         — `true` if the domain is not blacklisted.
   *   needsApiCheck  — `false` when the result was previously cached.
   *
   * @example
   *   const { isSafe, needsApiCheck } = shield.isUrlSafe('https://unknown.com');
   *   // → isSafe=true, needsApiCheck=true  (unknown domain, needs screening)
   *
   *   shield.markDomain('evil.com', false);
   *   shield.isUrlSafe('https://evil.com');
   *   // → isSafe=false, needsApiCheck=false (explicitly blacklisted)
   */
  isUrlSafe(rawURL: string): SecurityCheckResult {
    const host = this._extractHost(rawURL);
    if (!host) {
      // Invalid URL — safe default, no API check needed
      return { isSafe: true, needsApiCheck: false };
    }

    const now = Date.now();
    const entry = this.cache.get(host);
    if (entry) {
      if (now < entry.expires) {
        // Cached and fresh — does not need an external API call
        return { isSafe: entry.isSafe, needsApiCheck: false };
      }
      // Expired — re-check
      this.cache.delete(host);
    }

    // New domain — safe but needs external API verification
    return { isSafe: true, needsApiCheck: true };
  }

  /**
   * Explicitly mark a domain as safe (whitelist) or unsafe (blacklist).
   * Overwrites any previous entry and resets the TTL timer.
   *
   * @param domain  - The domain to mark, e.g. `"example.com"`
   * @param isSafe  - `true` to whitelist, `false` to blacklist
   *
   * @example
   *   shield.markDomain('trusted.com', true);   // always allow
   *   shield.markDomain('malware.com', false);  // always block
   */
  markDomain(domain: string, isSafe: boolean): void {
    const normalized = domain.toLowerCase().trim();
    const expires = Date.now() + this.cacheDays * 86400_000; // days → ms
    this.cache.set(normalized, { isSafe, expires });
  }

  /**
   * Remove all cached domain entries.
   * After clearing, every next `isUrlSafe()` call for a previously-known domain
   * will return `needsApiCheck=true` again.
   */
  clearCache(): void {
    this.cache.clear();
  }

  // ── helpers ────────────────────────────────────────────────

  /**
   * Extract the hostname from a URL string.
   * Returns `''` for invalid / missing URLs.
   * @internal — exposed for testing as `extractDomain`.
   */
  extractDomain(rawURL: string): string | null {
    try {
      const url = rawURL?.trim();
      if (!url) return null;
      let rest: string;
      
      // Check for scheme (handles both :// and : schemes like data:, javascript:)
      const schemeIndex = url.indexOf(':');
      if (schemeIndex === -1) {
        // no scheme
        return null;
      }
      
      const scheme = url.slice(0, schemeIndex);
      rest = url.slice(schemeIndex + 1);
      
      // Handle special cases for schemes
      if (scheme === 'data' || scheme === 'javascript') {
        return '';
      }
      
      // Handle // authority format (like //example.com/path)
      if (rest.startsWith('//')) {
        rest = rest.slice(2);
      }
      // For other schemes like http:, https:, ftp:, etc., we already removed the scheme above
      
      // Strip userinfo "user:pass@"
      const atIdx = rest.lastIndexOf('@');
      const afterAuth = atIdx >= 0 ? rest.slice(atIdx + 1) : rest;
      // Strip path/query/fragment
      const [rawHost, ..._] = afterAuth.split(/[\/?#]/, 1);
      if (!rawHost) return null;
      let host = rawHost.trim();
      if (!host) return null;
      // IPv6 in brackets: [::1]:8080 → ::
      if (host.startsWith('[')) {
        const bracketEnd = host.indexOf(']');
        if (bracketEnd === -1) return null;
        host = host.slice(1, bracketEnd); // Extract content between brackets
        return host.toLowerCase(); // ::1 is final, no port to strip
      }
      // Strip port after last ':' — works for 'example.com:8080'
      const colonIdx = host.lastIndexOf(':');
      if (colonIdx > 0) {
        host = host.slice(0, colonIdx);
      }
      // Accept localhost / single-label hosts too
      return host.toLowerCase() || null;
    } catch {
      return null;
    }
  }

  private _extractHost(rawURL: string): string {
    const r = this.extractDomain(rawURL);
    return r ?? '';
  }
}

function _stripPort(host: string): string {
  const at = host.indexOf('@');
  const colon = host.indexOf(':', at >= 0 ? at + 1 : 0);
  if (colon >= 0) return host.slice(0, colon);
  return host;
}