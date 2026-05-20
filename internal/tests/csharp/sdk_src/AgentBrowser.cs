using System.Diagnostics;

namespace B4N1Web;

/// <summary>
/// B4n1Web Agent Browser
/// 
/// A browser instance optimized for AI agent workflows.
/// Requires B4n1Web binary to be installed.
/// </summary>
/// <example>
/// <code>
/// using B4N1Web;
/// 
/// var browser = new AgentBrowser(new BrowserOptions { Mode = BrowserMode.Light });
/// var page = await browser.GotoAsync("https://example.com");
/// Console.WriteLine(page.Markdown);
/// browser.Close();
/// </code>
/// </example>
public class AgentBrowser : IDisposable
{
    private const string SdkVersion = "0.4.0";
    private readonly BrowserOptions _options;
    private readonly string _binaryPath;
    private bool _disposed;

    public AgentBrowser(BrowserOptions? options = null)
    {
        _options = options ?? new BrowserOptions();

        _binaryPath = FindBinary();
        if (string.IsNullOrEmpty(_binaryPath))
        {
            throw new BinaryNotFoundException();
        }

        // Check version compatibility (non-fatal warning)
        CheckVersionCompatibility();
    }

    /// <summary>
    /// Check if binary version matches SDK version.
    /// Prints warning to stderr if mismatch detected.
    /// </summary>
    private static void CheckVersionCompatibility()
    {
        var binaryVersion = GetVersion();
        if (binaryVersion == "unknown")
        {
            return;
        }

        if (binaryVersion != SdkVersion)
        {
            Console.Error.WriteLine(
                $"⚠️  Version mismatch: SDK v{SdkVersion} requires binary v{SdkVersion}, " +
                $"but found v{binaryVersion}. Some features may not work correctly.");
        }
    }

    /// <summary>
    /// Navigate to a URL and extract structured content
    /// </summary>
    public async Task<Page> GotoAsync(string url)
    {
        var startInfo = new ProcessStartInfo
        {
            FileName = _binaryPath,
            Arguments = $"goto {url} --mode {_options.Mode.ToString().ToLower()}",
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true
        };

        using var process = new Process { StartInfo = startInfo };
        process.Start();

        var output = await process.StandardOutput.ReadToEndAsync();
        var error = await process.StandardError.ReadToEndAsync();
        
        var timedOut = await Task.Run(() => process.WaitForExit(_options.Timeout * 1000));
        
        if (!timedOut)
        {
            process.Kill();
            throw new Exception($"Binary timed out after {_options.Timeout}s");
        }

        if (process.ExitCode != 0)
        {
            throw new Exception($"Binary error: {error}");
        }

        return ParseOutput(url, output);
    }

    /// <summary>
    /// Navigate to URL (synchronous version)
    /// </summary>
    public Page Goto(string url)
    {
        return GotoAsync(url).GetAwaiter().GetResult();
    }

    /// <summary>
    /// Parse text output from the binary
    /// </summary>
    private Page ParseOutput(string url, string output)
    {
        var markdown = new List<string>();
        var links = new List<string>();

        foreach (var line in output.Split('\n'))
        {
            if (line.StartsWith("URL:"))
            {
                continue;
            }
            else if (line.StartsWith("Markdown:"))
            {
                continue;
            }
            else if (line.StartsWith("Links:"))
            {
                try
                {
                    links = System.Text.Json.JsonSerializer.Deserialize<List<string>>(line[6..]) 
                        ?? new List<string>();
                }
                catch
                {
                    links = new List<string>();
                }
            }
            else
            {
                markdown.Add(line);
            }
        }

        return new Page
        {
            Url = url,
            Markdown = string.Join("\n", markdown).Trim(),
            Links = links
        };
    }

    /// <summary>
    /// Find b4n1web binary in bundled location or system install
    /// </summary>
    private static string FindBinary()
    {
        // 1. Check bundled binary (bundled as embedded resource)
        var bundledPath = ExtractBundledBinary();
        if (!string.IsNullOrEmpty(bundledPath))
        {
            return bundledPath;
        }

        // 2. Check system install locations
        var possiblePaths = new[]
        {
            "/usr/local/bin/b4n1web",
            "/usr/bin/b4n1web",
            Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".local/bin/b4n1web"),
            Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), ".b4n1web/bin/b4n1web"),
        };

        foreach (var path in possiblePaths)
        {
            if (File.Exists(path))
            {
                return path;
            }
        }

        return string.Empty;
    }

    /// <summary>
    /// Extract bundled binary from embedded resources to temp directory
    /// </summary>
    private static string? ExtractBundledBinary()
    {
        try
        {
            var assembly = typeof(AgentBrowser).Assembly;
            var resourceName = "B4N1Web.native.linux-x64.b4n1web-linux";
            using var stream = assembly.GetManifestResourceStream(resourceName);
            if (stream == null) return null;

            var tempDir = Path.Combine(Path.GetTempPath(), "b4n1web");
            Directory.CreateDirectory(tempDir);
            var tempBinary = Path.Combine(tempDir, "b4n1web");

            using var fs = new FileStream(tempBinary, FileMode.Create, FileAccess.Write);
            stream.CopyTo(fs);

            // Make executable on Unix
            if (Environment.OSVersion.Platform == PlatformID.Unix || Environment.OSVersion.Platform == PlatformID.MacOSX)
            {
                var chmod = new Process { StartInfo = new ProcessStartInfo("chmod", $"755 {tempBinary}") { UseShellExecute = false } };
                chmod.Start();
                chmod.WaitForExit();
            }

            return tempBinary;
        }
        catch
        {
            return null;
        }
    }

    /// <summary>
    /// Get B4n1Web binary version
    /// </summary>
    public static string GetVersion()
    {
        var path = FindBinary();
        if (string.IsNullOrEmpty(path))
        {
            return "unknown";
        }

        try
        {
            var startInfo = new ProcessStartInfo
            {
                FileName = path,
                Arguments = "--version",
                UseShellExecute = false,
                RedirectStandardOutput = true,
                CreateNoWindow = true
            };

            using var process = new Process { StartInfo = startInfo };
            process.Start();
            var output = process.StandardOutput.ReadToEnd();
            process.WaitForExit(5000);
            
            return output.Trim();
        }
        catch
        {
            return "unknown";
        }
    }

    /// <summary>
    /// Close the browser session
    /// </summary>
    public void Close()
    {
        Dispose();
    }

    public void Dispose()
    {
        if (!_disposed)
        {
            _disposed = true;
        }
        GC.SuppressFinalize(this);
    }
}
