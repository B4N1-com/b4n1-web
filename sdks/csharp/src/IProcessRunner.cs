using System.Diagnostics;

namespace B4N1Web;

/// <summary>
/// Process runner abstraction for testability
/// </summary>
public interface IProcessRunner
{
    /// <summary>
    /// Run a process synchronously
    /// </summary>
    ProcessResult Run(ProcessStartInfo startInfo, int timeoutMs);

    /// <summary>
    /// Run a process asynchronously
    /// </summary>
    Task<ProcessResult> RunAsync(ProcessStartInfo startInfo, int timeoutMs);
}

/// <summary>
/// Process execution result
/// </summary>
public class ProcessResult
{
    public int ExitCode { get; set; }
    public string StdOut { get; set; } = string.Empty;
    public string StdErr { get; set; } = string.Empty;
    public bool TimedOut { get; set; }
}

/// <summary>
/// Real process runner using System.Diagnostics.Process
/// </summary>
public class RealProcessRunner : IProcessRunner
{
    public ProcessResult Run(ProcessStartInfo startInfo, int timeoutMs)
    {
        using var process = new Process { StartInfo = startInfo };
        process.Start();

        var output = process.StandardOutput.ReadToEnd();
        var error = process.StandardError.ReadToEnd();

        var timedOut = process.WaitForExit(timeoutMs);

        if (!timedOut)
        {
            try { process.Kill(); } catch { }
            return new ProcessResult { TimedOut = true, StdOut = output, StdErr = error };
        }

        return new ProcessResult
        {
            ExitCode = process.ExitCode,
            StdOut = output,
            StdErr = error,
            TimedOut = false
        };
    }

    public async Task<ProcessResult> RunAsync(ProcessStartInfo startInfo, int timeoutMs)
    {
        using var process = new Process { StartInfo = startInfo };
        process.Start();

        var outputTask = process.StandardOutput.ReadToEndAsync();
        var errorTask = process.StandardError.ReadToEndAsync();

        var timedOut = await Task.Run(() => process.WaitForExit(timeoutMs));

        var output = await outputTask;
        var error = await errorTask;

        if (!timedOut)
        {
            try { process.Kill(); } catch { }
            return new ProcessResult { TimedOut = true, StdOut = output, StdErr = error };
        }

        return new ProcessResult
        {
            ExitCode = process.ExitCode,
            StdOut = output,
            StdErr = error,
            TimedOut = false
        };
    }
}