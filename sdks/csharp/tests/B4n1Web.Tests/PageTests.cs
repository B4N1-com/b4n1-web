using B4N1Web;
using Xunit;

namespace B4n1Web.Tests;

public class PageTests
{
    [Fact]
    public void Constructor_ShouldSetProperties()
    {
        var page = new Page
        {
            Url = "https://example.com",
            Markdown = "# Title\n\nSome content here.",
            Links = new List<string> { "https://example.com/link1", "https://example.com/link2" },
            Screenshot = "base64data",
            JsOutput = "console output"
        };

        Assert.Equal("https://example.com", page.Url);
        Assert.Equal("# Title\n\nSome content here.", page.Markdown);
        Assert.Equal(2, page.Links.Count);
        Assert.Equal("base64data", page.Screenshot);
        Assert.Equal("console output", page.JsOutput);
    }

    [Fact]
    public void Constructor_DefaultValues_ShouldBeEmpty()
    {
        var page = new Page();

        Assert.Equal(string.Empty, page.Url);
        Assert.Equal(string.Empty, page.Markdown);
        Assert.NotNull(page.Links);
        Assert.Empty(page.Links);
        Assert.Null(page.Screenshot);
        Assert.Null(page.JsOutput);
    }

    [Fact]
    public void GetMainContent_WithMoreThanTwoLines_SkipsFirstTwo()
    {
        var page = new Page
        {
            Markdown = "# Header Line\nIntro paragraph\nActual content line 1\nActual content line 2"
        };

        var content = page.GetMainContent();

        Assert.Equal("Actual content line 1\nActual content line 2", content);
    }

    [Fact]
    public void GetMainContent_WithExactlyTwoLines_ReturnsFullContent()
    {
        var page = new Page
        {
            Markdown = "# Title\nContent line"
        };

        var content = page.GetMainContent();

        Assert.Equal("# Title\nContent line", content);
    }

    [Fact]
    public void GetMainContent_WithSingleLine_ReturnsLine()
    {
        var page = new Page
        {
            Markdown = "Just one line"
        };

        var content = page.GetMainContent();

        Assert.Equal("Just one line", content);
    }

    [Fact]
    public void GetMainContent_WithEmptyMarkdown_ReturnsEmpty()
    {
        var page = new Page
        {
            Markdown = string.Empty
        };

        var content = page.GetMainContent();

        Assert.Equal(string.Empty, content);
    }

    [Fact]
    public void GetMainContent_TrimsResult()
    {
        var page = new Page
        {
            Markdown = "# Title\n\n  Content with surrounding spaces  "
        };

        var content = page.GetMainContent();

        Assert.Equal("Content with surrounding spaces", content);
    }

    [Fact]
    public void FindLinksByText_ShouldMatchCaseInsensitive()
    {
        var page = new Page
        {
            Links = new List<string>
            {
                "https://example.com/About",
                "https://example.com/contact",
                "https://example.com/ABOUT-US",
                "https://example.com/services"
            }
        };

        var result = page.FindLinksByText("about");

        Assert.Equal(2, result.Count);
        Assert.Contains("https://example.com/About", result);
        Assert.Contains("https://example.com/ABOUT-US", result);
    }

    [Fact]
    public void FindLinksByText_WithNoMatch_ReturnsEmpty()
    {
        var page = new Page
        {
            Links = new List<string>
            {
                "https://example.com/foo",
                "https://example.com/bar"
            }
        };

        var result = page.FindLinksByText("nonexistent");

        Assert.Empty(result);
    }

    [Fact]
    public void FindLinksByText_WithEmptyLinks_ReturnsEmpty()
    {
        var page = new Page
        {
            Links = new List<string>()
        };

        var result = page.FindLinksByText("anything");

        Assert.Empty(result);
    }

    [Fact]
    public void FindLinksByText_WithPartialMatch_ReturnsMatchingLinks()
    {
        var page = new Page
        {
            Links = new List<string>
            {
                "https://example.com/user/profile",
                "https://example.com/user/settings",
                "https://example.com/admin"
            }
        };

        var result = page.FindLinksByText("/user/");

        Assert.Equal(2, result.Count);
        Assert.Contains("https://example.com/user/profile", result);
        Assert.Contains("https://example.com/user/settings", result);
    }
}
