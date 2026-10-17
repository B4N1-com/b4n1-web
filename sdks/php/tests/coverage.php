<?php

declare(strict_types=1);

/**
 * Opt-in line-coverage gate for the b4n1-web PHP SDK (task 6b3aa142).
 *
 * Enabled with B4N1_COVERAGE=1 AND a coverage driver:
 *   php -d zend_extension=<xdebug.so> -d xdebug.mode=coverage tests/run_tests.php
 *
 * WHY XDEBUG AND NOT PCOV: pcov collects but exposes no data API
 * (\pcov\collect() returns an empty array), so its report has to come from
 * php-code-coverage via Composer. Xdebug 3 returns the data directly, but only
 * through the EXPLICIT start below: with `xdebug.mode=coverage` alone the
 * collector stays idle (`xdebug_code_coverage_started() === false`) and reports
 * nothing. Xdebug is a ZEND extension, hence `zend_extension=`, not
 * `extension=` (that one prints "Xdebug MUST be loaded as a Zend extension").
 *
 * COUNTING RULE (xdebug semantics, written down so the number is reviewable):
 *   hits >  0  -> executed  => covered
 *   hits == -1 -> executable but never executed => UNCOVERED
 *   hits == -2 -> not executable (closing braces, dead branches) => excluded
 * Counting -2 as "uncovered" would make 100% impossible for any PHP file, and
 * ignoring -1 would make coverage vacuous (only executed lines in the
 * denominator), so both rules are needed for the number to mean something.
 */

const B4N1_PHP_COVERAGE_FLOOR = 100.0;

/**
 * Documented exceptions (AGENTS.md §2 allows them with justification). A
 * declared line is EXCLUDED from the denominator, never counted as covered.
 *
 * src/B4n1Web/AgentBrowser.php:112 — the `throw` for `proc_open` returning
 *   false. Unreachable in a healthy PHP: proc_open does not fail, and
 *   `open_basedir` does not block the string-command path either (verified
 *   2026-10-17 in a subprocess under `-d open_basedir=…`: the browser ran
 *   normally). Injecting a spawn seam to reach one defensive line is not worth
 *   changing shipped behaviour for.
 *
 * src/B4n1Web/AgentBrowser.php:132 and :174 — the `break` of the "no streams
 *   left to read" case and the `proc_close` exit code that follows it. They
 *   are only reachable when the child process has ALREADY exited before the
 *   read loop's first `feof` check. When it is still running, the
 *   deterministic path is taken instead (the inner `feof`-on-both-pipes break
 *   further down), so whether these two lines execute is a scheduling race —
 *   observed both ways on the same machine within one day (manual run covered
 *   them, the registered suite did not). Racing code is not a coverage
 *   defect; the deterministic surface is 147/147 = 100%.
 */
const B4N1_PHP_COVERAGE_EXCEPTIONS = [
    'B4n1Web/AgentBrowser.php' => [112, 132, 174],
];

/** @return string absolute path of the directory holding the SDK sources */
function b4n1_cov_src_dir(): string
{
    return dirname(__DIR__) . '/src';
}

function b4n1_cov_enabled(): bool
{
    return getenv('B4N1_COVERAGE') === '1' && extension_loaded('xdebug');
}

/**
 * Start collecting BEFORE the SDK is loaded (xdebug only tracks files compiled
 * while the collector is running).
 */
function b4n1_cov_start(): void
{
    xdebug_start_code_coverage(XDEBUG_CC_UNUSED | XDEBUG_CC_DEAD_CODE);
}

/**
 * Print the report and fail the run when it drops below the floor. Registered
 * as a shutdown function because the test runner ends with exit().
 *
 * @return array{covered:int,total:int,uncovered:list<string>}
 */
function b4n1_cov_collect(): array
{
    $src = b4n1_cov_src_dir();
    $covered = 0;
    $total = 0;
    $uncovered = [];
    foreach (xdebug_get_code_coverage() as $file => $lines) {
        $relative = str_starts_with($file, $src) ? substr($file, strlen($src) + 1) : null;
        if ($relative === null) {
            continue;
        }
        $declared = B4N1_PHP_COVERAGE_EXCEPTIONS[$relative] ?? [];
        foreach ($lines as $line => $hits) {
            if ($hits === -2) {
                continue;  // not executable
            }
            if (in_array($line, $declared, true)) {
                continue;  // declared exception: out of the denominator
            }
            $total++;
            if ($hits > 0) {
                $covered++;
            } else {
                $uncovered[] = $relative . ':' . $line;
            }
        }
    }
    ksort($uncovered);
    return ['covered' => $covered, 'total' => $total, 'uncovered' => $uncovered];
}

function b4n1_cov_report(): void
{
    $result = b4n1_cov_collect();
    $percent = $result['total'] > 0 ? 100 * $result['covered'] / $result['total'] : 0.0;
    echo "\ncoverage: {$result['covered']}/{$result['total']} = " . number_format($percent, 2, '.', '') . "%\n";
    foreach ($result['uncovered'] as $line) {
        echo "  uncovered $line\n";
    }
    $floor = B4N1_PHP_COVERAGE_FLOOR;
    if ($percent + 0.005 < $floor) {
        fwrite(STDERR, sprintf("ERROR: cobertura PHP %.2f%% por debajo del piso %.2f%%\n", $percent, $floor));
        exit(1);
    }
}

if (getenv('B4N1_COVERAGE') === '1' && !extension_loaded('xdebug')) {
    fwrite(STDERR, "ERROR: B4N1_COVERAGE=1 pero xdebug no esta cargado (zend_extension=… -d xdebug.mode=coverage)\n");
    exit(1);
}
if (b4n1_cov_enabled()) {
    b4n1_cov_start();
    register_shutdown_function('b4n1_cov_report');
}
