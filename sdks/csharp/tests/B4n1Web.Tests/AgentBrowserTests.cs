using B4N1Web;
using Xunit;

namespace B4n1Web.Tests;

public class AgentBrowserTests
{
    private const string ExpectedBinaryRelativePath = "src/native/linux-x64/b4n1web";

    /// <summary>
    /// The SDK project root directory (two levels up from the test project).
    /// </summary>
    private static readonly string SdkProjectDir = GetSdkProjectDir();

    private static string GetSdkProjectDir()
    {
        // Walk up from the test assembly output to find the SDK project root.
        // Test project is at: {sdk}/tests/B4n1Web.Tests/
        // SDK project is at:  {sdk}/src/
        var testDir = AppContext.BaseDirectory;
        var dir = new DirectoryInfo(testDir);

        // Walk up until we find a directory containing "src/B4n1Web.csproj"
        while (dir != null && !dir.GetFiles("B4n1Web.csproj", SearchOption.TopDirectoryOnly).Any())
        {
            // Check parent for submodule-like nesting
            if (dir.Parent != null)
            {
                var candidate = Path.Combine(dir.Parent.FullName, "src", "B4n1Web.csproj");
                if (File.Exists(candidate))
                    return dir.Parent.FullName;
            }

            dir = dir.Parent;
        }

        return dir?.FullName ?? throw new InvalidOperationException(
            "Could not locate SDK project root from test directory.");
    }

    // ── Binary existence smoke tests ──────────────────────────────────────

    [Fact]
    public void BinaryFile_ShouldExistAtExpectedPath()
    {
        var binaryPath = Path.Combine(SdkProjectDir, ExpectedBinaryRelativePath);
        var fileInfo = new FileInfo(binaryPath);

        Assert.True(fileInfo.Exists, $"Expected binary not found at: {binaryPath}");
    }

    [Fact]
    public void BinaryFile_ShouldBeNonEmpty()
    {
        var binaryPath = Path.Combine(SdkProjectDir, ExpectedBinaryRelativePath);
        var fileInfo = new FileInfo(binaryPath);

        Assert.True(fileInfo.Exists, $"Binary not found at: {binaryPath}");
        Assert.True(fileInfo.Length > 1024,
            $"Binary size ({fileInfo.Length} bytes) seems too small for a valid ELF binary.");
    }

    // ── Version smoke tests ───────────────────────────────────────────────

    [Fact]
    public void GetVersion_ShouldNotThrow()
    {
        var exception = Record.Exception(() => AgentBrowser.GetVersion());

        Assert.Null(exception);
    }

    [Fact]
    public void GetVersion_ReturnsString()
    {
        var version = AgentBrowser.GetVersion();

        Assert.NotNull(version);
        Assert.NotEmpty(version);
    }

    // ── Constructor smoke tests ───────────────────────────────────────────

    [Fact]
    public void Constructor_DefaultOptions_CreatesInstance()
    {
        using var browser = new AgentBrowser();

        Assert.NotNull(browser);
    }

    [Fact]
    public void Constructor_CustomOptions_CreatesInstance()
    {
        var options = new BrowserOptions
        {
            Mode = BrowserMode.JS,
            Timeout = 60,
            UserAgent = "TestAgent/1.0"
        };

        using var browser = new AgentBrowser(options);

        Assert.NotNull(browser);
    }

    [Fact]
    public void Constructor_LightModeOptions_CreatesInstance()
    {
        var options = new BrowserOptions
        {
            Mode = BrowserMode.Light,
            Timeout = 15
        };

        using var browser = new AgentBrowser(options);

        Assert.NotNull(browser);
    }

    [Fact]
    public void Constructor_RenderModeOptions_CreatesInstance()
    {
        var options = new BrowserOptions
        {
            Mode = BrowserMode.Render,
            Timeout = 120
        };

        using var browser = new AgentBrowser(options);

        Assert.NotNull(browser);
    }

    // ── Lifecycle smoke tests ─────────────────────────────────────────────

    [Fact]
    public void Close_ShouldNotThrow()
    {
        var browser = new AgentBrowser();

        var exception = Record.Exception(() => browser.Close());

        Assert.Null(exception);
    }

    [Fact]
    public void Dispose_ShouldNotThrow()
    {
        var browser = new AgentBrowser();

        var exception = Record.Exception(() => browser.Dispose());

        Assert.Null(exception);
    }

    [Fact]
    public void MultipleDispose_ShouldNotThrow()
    {
        var browser = new AgentBrowser();
        browser.Dispose();

        var exception = Record.Exception(() => browser.Dispose());

        Assert.Null(exception);
    }

    [Fact]
    public void UsingBlock_ShouldNotThrow()
    {
        var exception = Record.Exception(() =>
        {
            using var browser = new AgentBrowser();
        });

        Assert.Null(exception);
    }

    // ── GetLinks static smoke tests ───────────────────────────────────────

    [Fact]
    public void GetLinksFromPage_ShouldNotThrow_OnInvalidUrl()
    {
        // This should gracefully handle a bad URL rather than crash.
        var links = AgentBrowser.GetLinksFromPage("https://invalid.example.nonexistent");

        Assert.NotNull(links);
    }

    // ── GetVersion consistency ─────────────────────────────────────────────

    [Fact]
    public void GetVersion_ReturnsKnownValue_WhenBinaryIsFound()
    {
        var version = AgentBrowser.GetVersion();

        // The SDK version is 0.9.4, but the binary version could differ
        // (e.g. "b4n1web 0.9.4"). At minimum, verify it contains the
        // SDK version number.
        Assert.Contains("0.9.4", version);
    }
}
