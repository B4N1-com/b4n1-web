namespace B4N1Web;

/// <summary>
/// Browser execution modes
/// </summary>
public enum BrowserMode
{
    Light,
    JS,
    Render
}

/// <summary>
/// Options for AgentBrowser
/// </summary>
public class BrowserOptions
{
    public BrowserMode Mode { get; set; } = BrowserMode.Light;
    public int Timeout { get; set; } = 30;
    public string UserAgent { get; set; } = "B4N1Web-Agent/1.0";
}

/// <summary>
/// Structured page data returned by B4n1Web
/// </summary>
public class Page
{
    public string Url { get; set; } = string.Empty;
    public string Markdown { get; set; } = string.Empty;
    public List<string> Links { get; set; } = new();
    public string? Screenshot { get; set; }

    /// <summary>
    /// Extract main content from markdown, skipping headers
    /// </summary>
    public string GetMainContent()
    {
        var lines = Markdown.Split('\n');
        if (lines.Length > 2)
        {
            return string.Join("\n", lines.Skip(2)).Trim();
        }
        return Markdown.Trim();
    }

    /// <summary>
    /// Find links containing specific text
    /// </summary>
    public List<string> FindLinksByText(string text)
    {
        var lowerText = text.ToLower();
        return Links.Where(link => link.ToLower().Contains(lowerText)).ToList();
    }
}

/// <summary>
/// Exception thrown when binary is not found
/// </summary>
public class BinaryNotFoundException : Exception
{
    public BinaryNotFoundException() : base(
        "B4n1Web binary not found. Please install it first:\n  curl -sL https://web.b4n1.com/install | bash")
    {
    }
}
