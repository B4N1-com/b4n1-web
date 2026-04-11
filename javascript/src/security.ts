/**
 * B4n1Web SecurityShield - URL Security Validation
 * 
 * Provides URL security validation with caching.
 * Fall-safe: returns safe=true if any error occurs.
 */

import { AgentBrowser, type Page } from './browser';
import { BrowserMode } from './types';
import type { SecurityShieldOptions, SecurityCheckResult } from './types';

/**
 * SecurityShield - Domain safety validation with caching
 * 
 * @example
 * ```typescript
 * import { SecurityShield, navigate } from 'b4n1-web';
 * 
 * // Simple usage
 * const result = await navigate('https://example.com');
 * 
 * // With custom shield
 * const shield = new SecurityShield({ cacheDays: 30 });
 * const { isSafe, needsApiCheck } = shield.isUrlSafe('https://example.com');
 * ```
 */
export class SecurityShield {
  private dbPath: string;
  private cacheDays: number;
  private cache: Map<string, { isSafe: boolean; expires: number }>;

  constructor(options: SecurityShieldOptions = {}) {
    this.dbPath = options.dbPath ?? `${process.env.HOME}/.b4n1web/security.db`;
    this.cacheDays = options.cacheDays ?? 7;
    this.cache = new Map();
  }

  /**
   * Extract domain from URL
   */
  private extractDomain(url: string): string | null {
    try {
      const parsed = new URL(url);
      return parsed.hostname.toLowerCase();
    } catch {
      return null;
    }
  }

  /**
   * Check if URL is safe to navigate
   */
  isUrlSafe(url: string): SecurityCheckResult {
    const domain = this.extractDomain(url);
    if (!domain) {
      return { isSafe: true, needsApiCheck: false };
    }

    const cached = this.cache.get(domain);
    if (cached) {
      if (Date.now() > cached.expires) {
        this.cache.delete(domain);
        return { isSafe: true, needsApiCheck: true };
      }
      return { isSafe: cached.isSafe, needsApiCheck: false };
    }

    return { isSafe: true, needsApiCheck: true };
  }

  /**
   * Mark a domain as safe or unsafe
   */
  markDomain(domain: string, isSafe: boolean): void {
    const normalizedDomain = domain.toLowerCase();
    const expires = Date.now() + this.cacheDays * 24 * 60 * 60 * 1000;
    this.cache.set(normalizedDomain, { isSafe, expires });
  }

  /**
   * Clear all cached domains
   */
  clearCache(): void {
    this.cache.clear();
  }
}

/**
 * Navigate to URL with optional security check
 * 
 * @example
 * ```typescript
 * import { navigate } from 'b4n1-web';
 * 
 * const result = await navigate('https://example.com');
 * if (result.success) {
 *   console.log(result.markdown);
 * }
 * ```
 */
export async function navigate(
  url: string,
  ignoreSecurity: boolean = false,
  securityShield?: SecurityShield
): Promise<{
  url: string;
  success: boolean;
  markdown?: string;
  links?: string[];
  error?: string;
}> {
  if (!securityShield) {
    securityShield = new SecurityShield();
  }

  if (!ignoreSecurity) {
    const { isSafe } = securityShield.isUrlSafe(url);
    if (!isSafe) {
      return {
        url,
        success: false,
        error: 'URL flagged as unsafe by security check',
      };
    }
  }

  try {
    const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
    const page: Page = await browser.goto(url);
    browser.close();

    return {
      url: page.url,
      success: true,
      markdown: page.markdown,
      links: page.links,
    };
  } catch (error: any) {
    return {
      url,
      success: false,
      error: error.message,
    };
  }
}
