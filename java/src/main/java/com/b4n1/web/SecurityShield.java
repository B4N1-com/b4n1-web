package com.b4n1.web;

import java.net.URI;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;

/**
 * SecurityShield provides URL security validation with caching.
 *
 * Fall-safe: returns safe=true if any error occurs.
 */
public class SecurityShield {
    private final String dbPath;
    private final int cacheDays;
    private final Map<String, CacheEntry> cache;

    private static class CacheEntry {
        boolean isSafe;
        long expires;

        CacheEntry(boolean isSafe, long expires) {
            this.isSafe = isSafe;
            this.expires = expires;
        }
    }

    public SecurityShield() {
        this(null, 7);
    }

    public SecurityShield(String dbPath, int cacheDays) {
        this.dbPath = dbPath != null ? dbPath 
            : System.getProperty("user.home") + "/.b4n1web/security.db";
        this.cacheDays = cacheDays;
        this.cache = new HashMap<>();
    }

    /**
     * Extract domain from URL.
     */
    private String extractDomain(String url) {
        try {
            URI uri = new URI(url);
            return uri.getHost().toLowerCase();
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Check if URL is safe to navigate.
     */
    public SecurityCheckResult isUrlSafe(String url) {
        String domain = extractDomain(url);
        if (domain == null) {
            return new SecurityCheckResult(true, false);
        }

        CacheEntry entry = cache.get(domain);
        if (entry == null) {
            return new SecurityCheckResult(true, true);
        }

        if (System.currentTimeMillis() > entry.expires) {
            cache.remove(domain);
            return new SecurityCheckResult(true, true);
        }

        return new SecurityCheckResult(entry.isSafe, false);
    }

    /**
     * Mark a domain as safe or unsafe.
     */
    public void markDomain(String domain, boolean isSafe) {
        String normalized = domain.toLowerCase();
        long expires = System.currentTimeMillis() + (cacheDays * 24L * 60L * 60L * 1000L);
        cache.put(normalized, new CacheEntry(isSafe, expires));
    }

    /**
     * Clear all cached domains.
     */
    public void clearCache() {
        cache.clear();
    }

    /**
     * Result of security check.
     */
    public static class SecurityCheckResult {
        public final boolean isSafe;
        public final boolean needsApiCheck;

        public SecurityCheckResult(boolean isSafe, boolean needsApiCheck) {
            this.isSafe = isSafe;
            this.needsApiCheck = needsApiCheck;
        }
    }
}
