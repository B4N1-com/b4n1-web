import { describe, it, expect } from 'vitest';
import { SecurityShield } from '../../../sdks/javascript/src/security';

describe('SecurityShield', () => {
  describe('constructor', () => {
    it('should construct with default options', () => {
      const shield = new SecurityShield();
      expect(shield).toBeDefined();
    });

    it('should construct with custom dbPath', () => {
      const shield = new SecurityShield({ dbPath: '/custom/path/security.db' });
      expect(shield).toBeDefined();
    });

    it('should construct with custom cacheDays', () => {
      const shield = new SecurityShield({ cacheDays: 30 });
      expect(shield).toBeDefined();
    });

    it('should construct with both custom options', () => {
      const shield = new SecurityShield({
        dbPath: '/tmp/test.db',
        cacheDays: 14,
      });
      expect(shield).toBeDefined();
    });

    it('should construct with empty options object', () => {
      const shield = new SecurityShield({});
      expect(shield).toBeDefined();
    });

    it('should use default cacheDays of 7', () => {
      const shield = new SecurityShield();
      shield.markDomain('example.com', true);
      const result = shield.isUrlSafe('https://example.com');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });
  });

  describe('extractDomain', () => {
    it('should extract domain from http URL', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('http://example.com');
      expect(result).toBe('example.com');
    });

    it('should extract domain from https URL', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com');
      expect(result).toBe('example.com');
    });

    it('should extract domain from URL with path', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com/path/to/page');
      expect(result).toBe('example.com');
    });

    it('should extract domain from URL with query string', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com/search?q=test&page=1');
      expect(result).toBe('example.com');
    });

    it('should extract domain from URL with fragment', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com/page#section');
      expect(result).toBe('example.com');
    });

    it('should extract domain from URL with port', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com:8080/path');
      expect(result).toBe('example.com');
    });

    it('should extract domain from URL with auth', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://user:pass@example.com/path');
      expect(result).toBe('example.com');
    });

    it('should extract domain from URL with subdomain', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://sub.example.com');
      expect(result).toBe('sub.example.com');
    });

    it('should extract domain from URL with deep subdomain', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://a.b.c.example.com/path');
      expect(result).toBe('a.b.c.example.com');
    });

    it('should extract localhost', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('http://localhost:3000');
      expect(result).toBe('localhost');
    });

    it('should extract IP address', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('http://127.0.0.1:8080');
      expect(result).toBe('127.0.0.1');
    });

    it('should extract IPv6 address', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('http://[::1]:8080/path');
      expect(result).toBe('::1');
    });

    it('should handle URL with trailing slash', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com/');
      expect(result).toBe('example.com');
    });

    it('should handle URL with multiple slashes in path', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com/a/b/c//d');
      expect(result).toBe('example.com');
    });

    it('should return null for URL with no scheme', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('example.com/path');
      expect(result).toBe(null);
    });

    it('should return null for empty string', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('');
      expect(result).toBe(null);
    });

    it('should return null for null input', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain(null);
      expect(result).toBe(null);
    });

    it('should return null for just scheme', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://');
      expect(result).toBe(null);
    });

    it('should handle unicode in URL', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com/\u65e5\u672c\u8a9e');
      expect(result).toBe('example.com');
    });

    it('should handle encoded characters in URL', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://example.com/path%20with%20spaces');
      expect(result).toBe('example.com');
    });

    it('should return hostname for ftp URL', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('ftp://ftp.example.com/file.txt');
      expect(result).toBe('ftp.example.com');
    });

    it('should normalize domain to lowercase', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('https://EXAMPLE.COM/Path');
      expect(result).toBe('example.com');
    });

    it('should return empty string for data URI', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('data:text/plain,hello');
      expect(result).toBe('');
    });

    it('should return empty string for javascript URI', () => {
      const shield = new SecurityShield();
      const result = (shield as any).extractDomain('javascript:alert(1)');
      expect(result).toBe('');
    });
  });

  describe('isUrlSafe', () => {
    it('should return safe=true for unknown domain', () => {
      const shield = new SecurityShield();
      const result = shield.isUrlSafe('https://unknown-domain.com');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(true);
    });

    it('should return safe=false for marked unsafe domain', () => {
      const shield = new SecurityShield();
      shield.markDomain('evil.com', false);
      const result = shield.isUrlSafe('https://evil.com');
      expect(result.isSafe).toBe(false);
      expect(result.needsApiCheck).toBe(false);
    });

    it('should return safe=true for marked safe domain', () => {
      const shield = new SecurityShield();
      shield.markDomain('trusted.com', true);
      const result = shield.isUrlSafe('https://trusted.com');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });

    it('should treat subdomain as separate domain', () => {
      const shield = new SecurityShield();
      shield.markDomain('example.com', false);
      const result = shield.isUrlSafe('https://sub.example.com');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(true);
    });

    it('should return safe=true for empty URL', () => {
      const shield = new SecurityShield();
      const result = shield.isUrlSafe('');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });

    it('should return safe=true for null URL', () => {
      const shield = new SecurityShield();
      const result = shield.isUrlSafe(null as any);
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });

    it('should return safe=true for malformed URL', () => {
      const shield = new SecurityShield();
      const result = shield.isUrlSafe('not-a-valid-url-at-all');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });

    it('should return safe=true for very long URL', () => {
      const shield = new SecurityShield();
      const longUrl = 'https://example.com/' + 'a'.repeat(10000);
      const result = shield.isUrlSafe(longUrl);
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(true);
    });

    it('should return safe=true for unicode URL', () => {
      const shield = new SecurityShield();
      const result = shield.isUrlSafe('https://example.com/\u65e5\u672c\u8a9e/\u30c6\u30b9\u30c8');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(true);
    });

    it('should use cached domain result', () => {
      const shield = new SecurityShield();
      shield.markDomain('cached.com', true);
      const result1 = shield.isUrlSafe('https://cached.com/path1');
      const result2 = shield.isUrlSafe('https://cached.com/path2');
      expect(result1.isSafe).toBe(true);
      expect(result1.needsApiCheck).toBe(false);
      expect(result2.isSafe).toBe(true);
      expect(result2.needsApiCheck).toBe(false);
    });

    it('should use domain with port as same key (URL parsing ignores port for hostname)', () => {
      const shield = new SecurityShield();
      shield.markDomain('example.com', true);
      const result = shield.isUrlSafe('https://example.com:8080/path');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });
  });

  describe('markDomain', () => {
    it('should mark domain as unsafe', () => {
      const shield = new SecurityShield();
      shield.markDomain('evil.com', false);
      const result = shield.isUrlSafe('https://evil.com');
      expect(result.isSafe).toBe(false);
    });

    it('should mark domain as safe', () => {
      const shield = new SecurityShield();
      shield.markDomain('trusted.com', true);
      const result = shield.isUrlSafe('https://trusted.com');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });

    it('should overwrite existing domain marking', () => {
      const shield = new SecurityShield();
      shield.markDomain('example.com', true);
      expect(shield.isUrlSafe('https://example.com').isSafe).toBe(true);
      shield.markDomain('example.com', false);
      expect(shield.isUrlSafe('https://example.com').isSafe).toBe(false);
      shield.markDomain('example.com', true);
      expect(shield.isUrlSafe('https://example.com').isSafe).toBe(true);
    });

    it('should handle empty domain string', () => {
      const shield = new SecurityShield();
      expect(() => shield.markDomain('', true)).not.toThrow();
    });

    it('should handle unicode domain', () => {
      const shield = new SecurityShield();
      shield.markDomain('\u4f8b\u3048.jp', true);
      const result = shield.isUrlSafe('https://\u4f8b\u3048.jp');
      expect(result.isSafe).toBe(true);
      expect(result.needsApiCheck).toBe(false);
    });

    it('should handle marking many domains (100x)', () => {
      const shield = new SecurityShield();
      for (let i = 0; i < 100; i++) {
        shield.markDomain(`domain${i}.com`, i % 2 === 0);
      }
      expect(shield.isUrlSafe('https://domain0.com').isSafe).toBe(true);
      expect(shield.isUrlSafe('https://domain1.com').isSafe).toBe(false);
      expect(shield.isUrlSafe('https://domain98.com').isSafe).toBe(true);
      expect(shield.isUrlSafe('https://domain99.com').isSafe).toBe(false);
    });

    it('should return undefined', () => {
      const shield = new SecurityShield();
      const result = shield.markDomain('example.com', true);
      expect(result).toBeUndefined();
    });

    it('should persist across multiple isUrlSafe calls', () => {
      const shield = new SecurityShield();
      shield.markDomain('persistent.com', false);
      expect(shield.isUrlSafe('https://persistent.com/a').isSafe).toBe(false);
      expect(shield.isUrlSafe('https://persistent.com/b').isSafe).toBe(false);
      expect(shield.isUrlSafe('https://persistent.com/c').isSafe).toBe(false);
    });

    it('should normalize domain to lowercase', () => {
      const shield = new SecurityShield();
      shield.markDomain('EXAMPLE.COM', false);
      expect(shield.isUrlSafe('https://example.com').isSafe).toBe(false);
      expect(shield.isUrlSafe('https://EXAMPLE.COM').isSafe).toBe(false);
    });
  });

  describe('clearCache', () => {
    it('should remove all cached domains', () => {
      const shield = new SecurityShield();
      shield.markDomain('a.com', true);
      shield.markDomain('b.com', false);
      shield.markDomain('c.com', true);
      shield.clearCache();
      expect(shield.isUrlSafe('https://a.com').needsApiCheck).toBe(true);
      expect(shield.isUrlSafe('https://b.com').needsApiCheck).toBe(true);
      expect(shield.isUrlSafe('https://c.com').needsApiCheck).toBe(true);
    });

    it('should work on empty cache', () => {
      const shield = new SecurityShield();
      expect(() => shield.clearCache()).not.toThrow();
    });

    it('should allow remarking after clear', () => {
      const shield = new SecurityShield();
      shield.markDomain('example.com', true);
      shield.clearCache();
      shield.markDomain('example.com', false);
      expect(shield.isUrlSafe('https://example.com').isSafe).toBe(false);
    });

    it('should be idempotent', () => {
      const shield = new SecurityShield();
      shield.markDomain('example.com', true);
      shield.clearCache();
      shield.clearCache();
      shield.clearCache();
      expect(shield.isUrlSafe('https://example.com').needsApiCheck).toBe(true);
    });

    it('should not affect other shield instances', () => {
      const shield1 = new SecurityShield();
      const shield2 = new SecurityShield();
      shield1.markDomain('example.com', true);
      shield2.clearCache();
      expect(shield1.isUrlSafe('https://example.com').isSafe).toBe(true);
      expect(shield1.isUrlSafe('https://example.com').needsApiCheck).toBe(false);
    });
  });

  describe('cache expiration', () => {
    it('should expire cached entries with cacheDays=0', () => {
      const shield = new SecurityShield({ cacheDays: 0 });
      shield.markDomain('example.com', true);
      const result = shield.isUrlSafe('https://example.com');
      expect(result.needsApiCheck).toBe(true);
    });

    it('should not expire within cache window', () => {
      const shield = new SecurityShield({ cacheDays: 365 });
      shield.markDomain('example.com', true);
      const result = shield.isUrlSafe('https://example.com');
      expect(result.needsApiCheck).toBe(false);
    });
  });
});
