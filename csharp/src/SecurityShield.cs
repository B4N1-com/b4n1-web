using System.Text.Json;

namespace B4N1Web;

/// <summary>
/// SecurityShield provides URL security validation with caching
/// </summary>
public class SecurityShield
{
    private readonly string _dbPath;
    private readonly int _cacheDays;
    private readonly Dictionary<string, CacheEntry> _cache;

    private class CacheEntry
    {
        public bool IsSafe { get; set; }
        public DateTime Expires { get; set; }
    }

    public SecurityShield(string? dbPath = null, int cacheDays = 7)
    {
        _dbPath = dbPath ?? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".b4n1web/security.db");
        _cacheDays = cacheDays;
        _cache = new Dictionary<string, CacheEntry>();
    }

    /// <summary>
    /// Extract domain from URL
    /// </summary>
    private string? ExtractDomain(string url)
    {
        try
        {
            if (Uri.TryCreate(url, UriKind.Absolute, out var uri))
            {
                return uri.Host.ToLower();
            }
        }
        catch
        {
        }
        return null;
    }

    /// <summary>
    /// Check if URL is safe to navigate
    /// </summary>
    public (bool isSafe, bool needsApiCheck) IsUrlSafe(string url)
    {
        var domain = ExtractDomain(url);
        if (domain == null)
        {
            return (true, false);
        }

        if (!_cache.TryGetValue(domain, out var entry))
        {
            return (true, true);
        }

        if (DateTime.Now > entry.Expires)
        {
            _cache.Remove(domain);
            return (true, true);
        }

        return (entry.IsSafe, false);
    }

    /// <summary>
    /// Mark a domain as safe or unsafe
    /// </summary>
    public void MarkDomain(string domain, bool isSafe)
    {
        var normalized = domain.ToLower();
        _cache[normalized] = new CacheEntry
        {
            IsSafe = isSafe,
            Expires = DateTime.Now.AddDays(_cacheDays)
        };
    }

    /// <summary>
    /// Clear all cached domains
    /// </summary>
    public void ClearCache()
    {
        _cache.Clear();
    }
}

/// <summary>
/// Navigation result
/// </summary>
public class NavigateResult
{
    public string Url { get; set; } = string.Empty;
    public bool Success { get; set; }
    public string? Markdown { get; set; }
    public List<string>? Links { get; set; }
    public string? Error { get; set; }
}

/// <summary>
/// Navigation helper with security check
/// </summary>
public static class NavigationHelper
{
    /// <summary>
    /// Navigate to URL with optional security check
    /// </summary>
    public static async Task<NavigateResult> NavigateAsync(
        string url,
        bool ignoreSecurity = false,
        SecurityShield? shield = null,
        BrowserOptions? browserOptions = null)
    {
        if (!ignoreSecurity)
        {
            shield ??= new SecurityShield();
            var (isSafe, _) = shield.IsUrlSafe(url);
            if (!isSafe)
            {
                return new NavigateResult
                {
                    Url = url,
                    Success = false,
                    Error = "URL flagged as unsafe by security check"
                };
            }
        }

        try
        {
            var browser = new AgentBrowser(browserOptions);
            var page = await browser.GotoAsync(url);
            browser.Close();

            return new NavigateResult
            {
                Url = page.Url,
                Success = true,
                Markdown = page.Markdown,
                Links = page.Links
            };
        }
        catch (Exception ex)
        {
            return new NavigateResult
            {
                Url = url,
                Success = false,
                Error = ex.Message
            };
        }
    }
}
