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

        // Start async reads BEFORE waiting: ReadToEnd() blocks until the
        // stream closes, so reading synchronously first defeats the timeout
        // below (a long-lived child with open stdout hangs us until IT exits).
        // Found 2026-10-14 via the coverage gate: `sleep 30` + 300 ms timeout
        // never reported TimedOut. (RunAsync already had the right shape.)
        var outputTask = startInfo.RedirectStandardOutput
            ? process.StandardOutput.ReadToEndAsync() : Task.FromResult("");
        var errorTask = startInfo.RedirectStandardError
            ? process.StandardError.ReadToEndAsync() : Task.FromResult("");

        var exited = process.WaitForExit(timeoutMs);
        if (!exited)
        {
            try { process.Kill(); } catch { }
            process.WaitForExit();
            return new ProcessResult
            {
                TimedOut = true,
                StdOut = outputTask.GetAwaiter().GetResult(),
                StdErr = errorTask.GetAwaiter().GetResult()
            };
        }

        return new ProcessResult
        {
            ExitCode = process.ExitCode,
            StdOut = outputTask.GetAwaiter().GetResult(),
            StdErr = errorTask.GetAwaiter().GetResult(),
            TimedOut = false
        };
    }

    public async Task<ProcessResult> RunAsync(ProcessStartInfo startInfo, int timeoutMs)
    {
        using var process = new Process { StartInfo = startInfo };
        process.Start();

        var outputTask = startInfo.RedirectStandardOutput
            ? process.StandardOutput.ReadToEndAsync() : Task.FromResult("");
        var errorTask = startInfo.RedirectStandardError
            ? process.StandardError.ReadToEndAsync() : Task.FromResult("");

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