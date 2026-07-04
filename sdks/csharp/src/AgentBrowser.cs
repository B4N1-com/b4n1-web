using System.Diagnostics;
using System.Runtime.InteropServices;

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
    private const string SdkVersion = "0.12.0";
    private readonly BrowserOptions _options;
    private readonly string _binaryPath;
    private readonly IProcessRunner _runner;
    private Page? _lastPage;
    private bool _disposed;

    public AgentBrowser(BrowserOptions? options = null) : this(options, new RealProcessRunner())
    {
    }

    public AgentBrowser(BrowserOptions? options, IProcessRunner runner)
    {
        _options = options ?? new BrowserOptions();
        _runner = runner ?? new RealProcessRunner();

        _binaryPath = FindBinary();
        if (string.IsNullOrEmpty(_binaryPath))
        {
            throw new BinaryNotFoundException();
        }

    }

    /// <summary>
    /// Navigate to a URL and extract structured content
    /// </summary>
    public async Task<Page> GotoAsync(string url, string? waitFor = null)
    {
        var args = $"goto {url} --mode {_options.Mode.ToString().ToLower()}";
        if (waitFor != null)
        {
            args += $" --wait-for \"{waitFor}\"";
        }

        var startInfo = CreateStartInfo(args);
        var result = await _runner.RunAsync(startInfo, _options.Timeout * 1000);

        if (result.TimedOut)
        {
            throw new Exception($"Binary timed out after {_options.Timeout}s");
        }

        if (result.ExitCode != 0)
        {
            throw new Exception($"Binary error: {result.StdErr}");
        }

        var page = ParseOutput(url, result.StdOut);
        _lastPage = page;
        return page;
    }

    /// <summary>
    /// Navigate to URL (synchronous version)
    /// </summary>
    public Page Goto(string url, string? waitFor = null)
    {
        return GotoAsync(url, waitFor).GetAwaiter().GetResult();
    }

    /// <summary>
    /// Take a screenshot of the current page
    /// </summary>
    public string Screenshot(int width, int height)
    {
        if (_lastPage == null)
            throw new InvalidOperationException("No page loaded. Call Goto or GotoAsync first.");

        var startInfo = CreateStartInfo($"screenshot --url {_lastPage.Url} --width {width} --height {height}");
        var result = _runner.Run(startInfo, _options.Timeout * 1000);

        if (result.TimedOut)
        {
            throw new Exception($"Binary timed out after {_options.Timeout}s");
        }

        if (result.ExitCode != 0)
        {
            throw new Exception($"Binary error: {result.StdErr}");
        }

        foreach (var line in result.StdOut.Split('\n'))
        {
            if (line.StartsWith("Screenshot:"))
            {
                var b64 = line[11..].Trim();
                if (!string.IsNullOrEmpty(b64))
                    return b64;
            }
        }

        throw new Exception("No screenshot data returned from binary");
    }

    /// <summary>
    /// Wait for a CSS selector to appear on the page
    /// </summary>
    public bool WaitForSelector(string selector, int timeoutMs)
    {
        if (_lastPage == null)
            throw new InvalidOperationException("No page loaded. Call Goto or GotoAsync first.");

        var startInfo = CreateStartInfo($"wait-for-selector \"{selector}\" --url {_lastPage.Url} --timeout {timeoutMs}");
        var result = _runner.Run(startInfo, _options.Timeout * 1000);

        if (result.TimedOut)
        {
            throw new Exception($"Binary timed out after {_options.Timeout}s");
        }

        if (result.ExitCode != 0)
        {
            throw new Exception($"Binary error: {result.StdErr}");
        }

        foreach (var line in result.StdOut.Split('\n'))
        {
            if (line.StartsWith("Found:"))
            {
                return line[6..].Trim() == "true";
            }
        }

        return false;
    }

    /// <summary>
    /// Click on an element by CSS selector
    /// </summary>
    public void Click(string selector)
    {
        if (_lastPage == null)
            throw new InvalidOperationException("No page loaded. Call Goto or GotoAsync first.");

        var startInfo = CreateStartInfo($"click \"{selector}\" --url {_lastPage.Url}");
        var result = _runner.Run(startInfo, _options.Timeout * 1000);

        if (result.TimedOut)
        {
            throw new Exception($"Binary timed out after {_options.Timeout}s");
        }

        if (result.ExitCode != 0)
        {
            throw new Exception($"Binary error: {result.StdErr}");
        }
    }

    /// <summary>
    /// Type text into an element by CSS selector
    /// </summary>
    public void TypeText(string selector, string text, bool clearFirst)
    {
        if (_lastPage == null)
            throw new InvalidOperationException("No page loaded. Call Goto or GotoAsync first.");

        var clearArg = clearFirst ? " --clear-first" : "";
        var startInfo = CreateStartInfo($"type-text \"{selector}\" \"{text}\" --url {_lastPage.Url}{clearArg}");
        var result = _runner.Run(startInfo, _options.Timeout * 1000);

        if (result.TimedOut)
        {
            throw new Exception($"Binary timed out after {_options.Timeout}s");
        }

        if (result.ExitCode != 0)
        {
            throw new Exception($"Binary error: {result.StdErr}");
        }
    }

    /// <summary>
    /// Get links from the last visited page
    /// </summary>
    public string[] GetLinks()
    {
        return _lastPage?.Links.ToArray() ?? Array.Empty<string>();
    }

    /// <summary>
    /// Fetch links from a URL without creating a browser instance (static)
    /// </summary>
    public static string[] GetLinksFromPage(string url, IProcessRunner? runner = null)
    {
        var binaryPath = FindBinary();
        if (string.IsNullOrEmpty(binaryPath))
            throw new BinaryNotFoundException();

        runner ??= new RealProcessRunner();
        var startInfo = new ProcessStartInfo
        {
            FileName = binaryPath,
            Arguments = $"goto {url} --mode light",
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true
        };

        var result = runner.Run(startInfo, 30000);

        if (result.ExitCode != 0)
            return Array.Empty<string>();

        foreach (var line in result.StdOut.Split('\n'))
        {
            if (line.StartsWith("Links:"))
            {
                try
                {
                    return System.Text.Json.JsonSerializer.Deserialize<List<string>>(line[6..])
                        ?.ToArray() ?? Array.Empty<string>();
                }
                catch
                {
                    return Array.Empty<string>();
                }
            }
        }

        return Array.Empty<string>();
    }

    /// <summary>
    /// Parse text output from the binary
    /// </summary>
    private Page ParseOutput(string url, string output)
    {
        var markdown = new List<string>();
        var links = new List<string>();
        string? screenshot = null;
        string? jsOutput = null;

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
            else if (line.StartsWith("Screenshot:"))
            {
                screenshot = line[11..].Trim();
                if (string.IsNullOrEmpty(screenshot)) screenshot = null;
            }
            else if (line.StartsWith("JsOutput:"))
            {
                jsOutput = line[9..].Trim();
                if (string.IsNullOrEmpty(jsOutput)) jsOutput = null;
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
            Links = links,
            Screenshot = screenshot,
            JsOutput = jsOutput
        };
    }

    /// <summary>
    /// Find b4n1web binary in bundled location or system install
    /// </summary>
    private static string FindBinary()
    {
        // 1. Check bundled binary (extracted from embedded resource for current platform)
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
    /// Get the platform-specific embedded resource name for the current OS/arch
    /// </summary>
    private static string? GetPlatformResourceName()
    {
        string os;
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
            os = "linux";
        else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            os = "macos";
        else if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            os = "windows";
        else
            return null;

        string arch = RuntimeInformation.OSArchitecture switch
        {
            Architecture.X64 => "amd64",
            Architecture.Arm64 => "arm64",
            _ => null!
        };
        if (arch == null) return null;

        string ext = os == "windows" ? ".exe" : "";
        return $"B4N1Web.native.{os}-{arch}.b4n1web{ext}";
    }

    /// <summary>
    /// Extract bundled binary for current platform from embedded resources to temp directory
    /// </summary>
    private static string? ExtractBundledBinary()
    {
        var resourceName = GetPlatformResourceName();
        if (resourceName == null) return null;

        string ext = RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ? ".exe" : "";
        var tempBinary = Path.Combine(Path.GetTempPath(), "b4n1web", $"b4n1web{ext}");

        if (File.Exists(tempBinary))
        {
            try
            {
                var psi = new ProcessStartInfo
                {
                    FileName = tempBinary,
                    Arguments = "--version",
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    RedirectStandardError = true,
                    CreateNoWindow = true
                };
                using var proc = new Process { StartInfo = psi };
                proc.Start();
                var output = proc.StandardOutput.ReadToEnd().Trim();
                proc.WaitForExit(3000);
                if (proc.ExitCode == 0 && !string.IsNullOrEmpty(output))
                    return tempBinary;
            }
            catch
            {
            }
        }

        try
        {
            var assembly = typeof(AgentBrowser).Assembly;
            using var stream = assembly.GetManifestResourceStream(resourceName);
            if (stream == null) return null;

            var tempDir = Path.Combine(Path.GetTempPath(), "b4n1web");
            Directory.CreateDirectory(tempDir);

            using var fs = new FileStream(tempBinary, FileMode.Create, FileAccess.Write);
            stream.CopyTo(fs);

            // Make executable on Unix
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux) ||
                RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            {
                var chmod = new Process
                {
                    StartInfo = new ProcessStartInfo("chmod", $"+x {tempBinary}")
                    {
                        UseShellExecute = false
                    }
                };
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
    public static string GetVersion(IProcessRunner? runner = null)
    {
        var path = FindBinary();
        if (string.IsNullOrEmpty(path))
        {
            return "unknown";
        }

        try
        {
            runner ??= new RealProcessRunner();
            var startInfo = new ProcessStartInfo
            {
                FileName = path,
                Arguments = "--version",
                UseShellExecute = false,
                RedirectStandardOutput = true,
                CreateNoWindow = true
            };

            var result = runner.Run(startInfo, 5000);
            return result.StdOut.Trim();
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

    private ProcessStartInfo CreateStartInfo(string arguments)
    {
        return new ProcessStartInfo
        {
            FileName = _binaryPath,
            Arguments = arguments,
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true
        };
    }
}