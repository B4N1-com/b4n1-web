<?php

declare(strict_types=1);

namespace B4n1Web;

use B4n1Web\Exception\NavigationException;

/**
 * Thin wrapper over the b4n1web binary.
 *
 *   $browser = new AgentBrowser();               // mode: light, timeout: 30
 *   $page = $browser->goto('https://example.com');
 *   echo $page->markdown;
 *   $browser->close();                            // no-op (stateless binary)
 */
final class AgentBrowser
{
    private string $binary;
    private BrowserMode $mode;
    private int $timeout;

    /**
     * @param ?callable(list<string>, int): array{string, string, int|null} $runner
     *        injectable process runner (tests); defaults to proc_open runner
     * @param ?string $binary explicit binary path (skips auto-resolution)
     */
    public function __construct(
        BrowserMode $mode = BrowserMode::Light,
        int $timeout = 30,
        ?string $binary = null,
        ?callable $runner = null,
    ) {
        $this->mode = $mode;
        $this->timeout = $timeout;
        $this->binary = $binary ?? Binary::resolveOrThrow();
        $this->runner = $runner ?? [$this, 'runProcess'];
    }

    private $runner;

    public function binaryPath(): string
    {
        return $this->binary;
    }

    /**
     * Navigate + extract. waitFor: CSS selector (render mode).
     *
     * @throws NavigationException on non-zero exit or timeout
     */
    public function goto(string $url, ?string $waitFor = null): Page
    {
        $argv = [$this->binary, 'goto', $url, '--mode', $this->mode->value];
        if ($waitFor !== null && $waitFor !== '') {
            $argv[] = '--wait-for';
            $argv[] = $waitFor;
        }
        [$stdout, $stderr, $status] = ($this->runner)($argv, $this->timeout);
        if ($status === null) {
            throw new NavigationException("b4n1web timed out after {$this->timeout}s");
        }
        if ($status !== 0) {
            throw new NavigationException("b4n1web exited {$status}: " . trim($stderr));
        }

        return Page::parse($url, $stdout);
    }

    /**
     * Execute JavaScript in the page context, return raw result string.
     *
     * @throws NavigationException on non-zero exit or timeout
     */
    public function evaluate(string $url, string $js): string
    {
        $argv = [$this->binary, 'evaluate', $url, $js];
        [$stdout, $stderr, $status] = ($this->runner)($argv, $this->timeout);
        if ($status === null) {
            throw new NavigationException("b4n1web timed out after {$this->timeout}s");
        }
        if ($status !== 0) {
            throw new NavigationException("b4n1web exited {$status}: " . trim($stderr));
        }

        return trim($stdout);
    }

    /**
     * The binary is stateless: close is a no-op kept for API parity.
     */
    public function close(): void
    {
    }

    /**
     * Default runner: proc_open with hard timeout. Returns [stdout, stderr, exit|null].
     *
     * @param list<string> $argv
     * @return array{string, string, int|null}
     */
    private function runProcess(array $argv, int $timeout): array
    {
        $spec = [
            0 => ['pipe', 'r'],
            1 => ['pipe', 'w'],
            2 => ['pipe', 'w'],
        ];
        $cmd = implode(' ', array_map('escapeshellarg', $argv));
        $proc = proc_open($cmd, $spec, $pipes);
        if (!is_resource($proc)) {
            throw new NavigationException('failed to spawn b4n1web process');
        }
        fclose($pipes[0]);

        stream_set_blocking($pipes[1], false);
        stream_set_blocking($pipes[2], false);

        $stdout = '';
        $stderr = '';
        $exitCode = null;
        $deadline = microtime(true) + $timeout;
        while (true) {
            $read = [];
            if (isset($pipes[1]) && !feof($pipes[1])) {
                $read[] = $pipes[1];
            }
            if (isset($pipes[2]) && !feof($pipes[2])) {
                $read[] = $pipes[2];
            }
            if ($read === []) {
                break;
            }
            $write = null;
            $except = null;
            $sec = 0;
            $usec = 200_000;
            @stream_select($read, $write, $except, $sec, $usec);
            foreach ($read as $stream) {
                $chunk = (string) stream_get_contents($stream);
                if ($stream === ($pipes[1] ?? null)) {
                    $stdout .= $chunk;
                } else {
                    $stderr .= $chunk;
                }
            }
            $status = proc_get_status($proc);
            if (!$status['running']) {
                // proc_get_status returns the real exit code on the FIRST
                // "not running" read (later reads give -1, and so would proc_close).
                if ($status['exitcode'] >= 0 && $exitCode === null) {
                    $exitCode = $status['exitcode'];
                }
                if (feof($pipes[1]) && feof($pipes[2])) {
                    break;
                }
            }
            if (microtime(true) >= $deadline) {
                proc_terminate($proc, 9);
                fclose($pipes[1]);
                fclose($pipes[2]);
                proc_close($proc);

                return [$stdout, $stderr, null];
            }
        }

        $stdout .= (string) stream_get_contents($pipes[1]);
        $stderr .= (string) stream_get_contents($pipes[2]);
        fclose($pipes[1]);
        fclose($pipes[2]);
        $closeCode = proc_close($proc);
        if ($exitCode === null || $exitCode < 0) {
            $exitCode = $closeCode;
        }

        return [$stdout, $stderr, $exitCode < 0 ? 0 : $exitCode];
    }
}
