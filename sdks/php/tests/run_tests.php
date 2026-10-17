<?php

declare(strict_types=1);

// Offline test suite for the b4n1-web PHP SDK (no phpunit dependency).
// Run: php tests/run_tests.php   (exit 0 = all green)

spl_autoload_register(static function (string $class): void {
    $prefix = 'B4n1Web\\';
    if (!str_starts_with($class, $prefix)) {
        return;
    }
    $relative = substr($class, strlen($prefix));
    $file = __DIR__ . '/../src/B4n1Web/' . str_replace('\\', '/', $relative) . '.php';
    if (is_file($file)) {
        require $file;
    }
});

use B4n1Web\AgentBrowser;
use B4n1Web\Binary;
use B4n1Web\BrowserMode;
use B4n1Web\Exception\BinaryNotFoundException;
use B4n1Web\Exception\NavigationException;
use B4n1Web\Page;

// Coverage is OPT-IN and must be loaded BEFORE the SDK sources (xdebug only
// tracks files compiled while its collector runs). No-op unless
// B4N1_COVERAGE=1. See tests/coverage.php for the driver and counting rules.
require __DIR__ . '/coverage.php';

$pass = 0;
$fail = 0;
$failures = [];

function check(string $name, bool $ok, string $detail = ''): void
{
    global $pass, $fail, $failures;
    if ($ok) {
        $pass++;
        echo "  ok   {$name}\n";
    } else {
        $fail++;
        $failures[] = $name . ($detail !== '' ? " — {$detail}" : '');
        echo "  FAIL {$name}" . ($detail !== '' ? " — {$detail}" : '') . "\n";
    }
}

const SAMPLE_OUTPUT = "URL: https://example.com\n"
    . "Markdown:\n"
    . "# Example Domain\n"
    . "\n"
    . "This domain is for use in documentation examples.\n"
    . "Links: [\"https://example.com/a\", \"https://other.org/b\", \"/relative\"]\n"
    . "Screenshot: /tmp/shot.png\n";

// --- Page::parse -------------------------------------------------------------

$page = Page::parse('https://example.com', SAMPLE_OUTPUT);
check('parse: url', $page->url === 'https://example.com');
check('parse: markdown contains title', str_contains($page->markdown, '# Example Domain'));
check('parse: links count = 3', count($page->links) === 3, 'got ' . count($page->links));
check('parse: screenshot', $page->screenshot === '/tmp/shot.png');

$pageNoShot = Page::parse('https://x.io', "URL: https://x.io\nMarkdown:\ncontent\nLinks: []\n");
check('parse: no screenshot => null', $pageNoShot->screenshot === null);
check('parse: empty links => []', $pageNoShot->links === []);

$pageBadJson = Page::parse('u', "Markdown:\nx\nLinks: not-json\n");
check('parse: malformed Links JSON => []', $pageBadJson->links === []);

check('main content drops first two lines', !str_contains($page->getMainContent(), '# Example Domain'));
$shortPage = new Page('u', 'only', []);
check('main content short markdown', $shortPage->getMainContent() === 'only');

check('findLinksByText case-insensitive', $page->findLinksByText('EXAMPLE') === ['https://example.com/a']);
check('findLinksByText empty needle => []', $page->findLinksByText('') === []);

// Markdown can start on the same line as the header (the binary emits
// "Markdown:" followed by content when the page has no leading blank line).
$pageInline = Page::parse('https://x.io', "URL: https://x.io\nMarkdown:# Inline title\nLinks: []\n");
check('parse: inline markdown on the header line', str_starts_with($pageInline->markdown, '# Inline title'));

// --- Binary::resolve ---------------------------------------------------------

$saveBinPath = getenv('B4N1WEB_BIN_PATH');   // keep for the real-binary smoke below
$fake = tempnam(sys_get_temp_dir(), 'fakeb4n1');
chmod($fake, 0755);
putenv('B4N1WEB_BIN_PATH=' . $fake);
check('resolve: env override wins', Binary::resolve() === $fake);
putenv('B4N1WEB_BIN_PATH=/nonexistent/b4n1web');
// Falls through to other locations (may or may not find a system binary):
$fallthrough = Binary::resolve();
check('resolve: missing env override falls through', $fallthrough === null || is_file($fallthrough));
putenv('B4N1WEB_BIN_PATH');           // unset
putenv('B4N1WEB_BIN_PATH=');          // ensure getenv returns false
putenv('B4N1WEB_BIN_PATH');

// Isolated environment: nothing resolves => BinaryNotFoundException.
$savePath = getenv('PATH');
$saveHome = getenv('HOME');
$saveBinaryEnv = getenv('B4N1WEB_BINARY');
putenv('B4N1WEB_BINARY');
putenv('PATH=' . sys_get_temp_dir());
putenv('HOME=' . sys_get_temp_dir());
putenv('B4N1WEB_BIN_PATH');
$threw = false;
try {
    Binary::resolveOrThrow();
} catch (BinaryNotFoundException $e) {
    $threw = true;
    check('resolveOrThrow: message has install hint', str_contains($e->getMessage(), 'B4N1WEB_BIN_PATH'));
}
check('resolveOrThrow: throws when nothing found', $threw);
unset($threw);

$threwBrowser = false;
try {
    new AgentBrowser();
} catch (BinaryNotFoundException) {
    $threwBrowser = true;
}
check('AgentBrowser ctor: throws without binary', $threwBrowser);

// A runnable candidate from B4N1WEB_BINARY (not the BIN_PATH override): the
// resolver must return it, and resolveOrThrow must hand it back instead of
// throwing. Both run inside the isolated environment set above.
$fakeCandidate = tempnam(sys_get_temp_dir(), 'candb4n1');
chmod($fakeCandidate, 0755);
putenv('B4N1WEB_BINARY=' . $fakeCandidate);
check('resolve: B4N1WEB_BINARY candidate wins', Binary::resolve() === $fakeCandidate);
check('resolveOrThrow: returns the resolved path', Binary::resolveOrThrow() === $fakeCandidate);
unlink($fakeCandidate);
putenv('B4N1WEB_BINARY');

// Restore environment for the runner tests below.
if ($savePath !== false) {
    putenv('PATH=' . $savePath);
}
if ($saveHome !== false) {
    putenv('HOME=' . $saveHome);
}
if ($saveBinaryEnv !== false) {
    putenv('B4N1WEB_BINARY=' . $saveBinaryEnv);
}
if ($saveBinPath !== false) {
    putenv('B4N1WEB_BIN_PATH=' . $saveBinPath);
}
unlink($fake);

// --- AgentBrowser with injected runner ---------------------------------------

/** Fake runner mimicking the binary's text protocol. */
function fakeRunner(string $stdout, string $stderr, ?int $status, ?int $delay = null): callable
{
    return static function (array $argv, int $timeout) use ($stdout, $stderr, $status, $delay): array {
        $GLOBALS['last_argv'] = $argv;
        if ($delay !== null) {
            if ($delay > $timeout) {
                return [$stdout, $stderr, null];  // simulate timeout
            }
            sleep($delay);
        }

        return [$stdout, $stderr, $status];
    };
}

$browser = new AgentBrowser(mode: BrowserMode::Light, binary: '/fake/b4n1web', runner: fakeRunner(SAMPLE_OUTPUT, '', 0));
$page = $browser->goto('https://example.com');
check('goto: parses url', $page->url === 'https://example.com');
check('goto: argv = goto + url + --mode light', $GLOBALS['last_argv'] === ['/fake/b4n1web', 'goto', 'https://example.com', '--mode', 'light']);

$browser = new AgentBrowser(binary: '/fake/b4n1web', runner: fakeRunner(SAMPLE_OUTPUT, '', 0));
$browser->goto('https://x.io', waitFor: '#ready');
$argv = $GLOBALS['last_argv'];
check('goto: --wait-for appended', array_slice($argv, -2) === ['--wait-for', '#ready']);

$browser = new AgentBrowser(binary: '/fake/b4n1web', runner: fakeRunner('', 'boom', 3));
$threw = false;
try {
    $browser->goto('https://x.io');
} catch (NavigationException $e) {
    $threw = true;
    check('goto: nonzero exit message', str_contains($e->getMessage(), '3') && str_contains($e->getMessage(), 'boom'));
}
check('goto: nonzero exit throws', $threw);

$browser = new AgentBrowser(timeout: 0, binary: '/fake/b4n1web', runner: fakeRunner(SAMPLE_OUTPUT, '', 0, 5));
$threw = false;
try {
    $browser->goto('https://x.io');
} catch (NavigationException $e) {
    $threw = true;
    check('goto: timeout message', str_contains($e->getMessage(), 'timed out'));
}
check('goto: timeout throws', $threw);

$browser = new AgentBrowser(binary: '/fake/b4n1web', runner: fakeRunner("42\n", '', 0));
check('evaluate: returns trimmed stdout', $browser->evaluate('https://x.io', '6*7') === '42');
check('evaluate: argv', $GLOBALS['last_argv'] === ['/fake/b4n1web', 'evaluate', 'https://x.io', '6*7']);

$browser = new AgentBrowser(binary: '/fake/b4n1web', runner: fakeRunner('', 'js error', 1));
$threw = false;
try {
    $browser->evaluate('u', 'x');
} catch (NavigationException) {
    $threw = true;
}
check('evaluate: nonzero exit throws', $threw);

$browser = new AgentBrowser(binary: '/fake/b4n1web', runner: fakeRunner('', '', 0));
$browser->close();
check('close: no-op', true);
check('binaryPath: returns explicit override', $browser->binaryPath() === '/fake/b4n1web');

// --- BrowserMode enum --------------------------------------------------------

check('BrowserMode enum values', BrowserMode::Light->value === 'light'
    && BrowserMode::Js->value === 'js'
    && BrowserMode::Render->value === 'render');

// --- Real binary smoke (only when resolvable locally) ------------------------

$real = Binary::resolve();
if ($real !== null) {
    $browser = new AgentBrowser(binary: $real, timeout: 60);
    try {
        $page = $browser->goto('https://example.com');
        check('smoke: real binary goto url', $page->url === 'https://example.com');
        check('smoke: real binary markdown non-empty', strlen($page->markdown) > 0);
    } catch (NavigationException $e) {
        check('smoke: real binary goto', false, $e->getMessage());
    }
} else {
    echo "  note real-binary smoke skipped (no binary resolvable; unit tests still fully cover the SDK)\n";
}

// --- Real process runner (runProcess via public API, no network) -------------

$sleepWrapper = tempnam(sys_get_temp_dir(), 'b4w') ;
file_put_contents($sleepWrapper, "#!/bin/sh\nsleep 30\n");
chmod($sleepWrapper, 0755);
$browser = new AgentBrowser(timeout: 1, binary: $sleepWrapper);
$started = microtime(true);
$threw = false;
try {
    $browser->goto('https://x.io');
} catch (NavigationException $e) {
    $threw = true;
    check('real timeout: message says timed out', str_contains($e->getMessage(), 'timed out'));
}
$elapsed = microtime(true) - $started;
check('real timeout: throws', $threw);
check('real timeout: kills promptly', $elapsed < 5, sprintf('took %.1fs', $elapsed));
unlink($sleepWrapper);

$falseWrapper = tempnam(sys_get_temp_dir(), 'b4w');
file_put_contents($falseWrapper, "#!/bin/sh\necho boom >&2\nexit 3\n");
chmod($falseWrapper, 0755);
$browser = new AgentBrowser(timeout: 5, binary: $falseWrapper);
$threw = false;
try {
    $browser->goto('https://x.io');
} catch (NavigationException $e) {
    $threw = true;
    check('real nonzero: carries exit code + stderr', str_contains($e->getMessage(), '3') && str_contains($e->getMessage(), 'boom'));
}
check('real nonzero: throws', $threw);
$browserOk = new AgentBrowser(timeout: 5, binary: $falseWrapper);
try {
    $browserOk->evaluate('https://x.io', 'x');
    check('real nonzero evaluate: throws', false);
} catch (NavigationException) {
    check('real nonzero evaluate: throws', true);
}
unlink($falseWrapper);

// evaluate() has its own timeout branch, separate from goto()'s: a hanging
// process must surface as a NavigationException there too.
$sleepWrapper2 = tempnam(sys_get_temp_dir(), 'b4w');
file_put_contents($sleepWrapper2, "#!/bin/sh\nsleep 30\n");
chmod($sleepWrapper2, 0755);
$browserSleep = new AgentBrowser(timeout: 1, binary: $sleepWrapper2);
$threw = false;
try {
    $browserSleep->evaluate('https://x.io', '1+1');
} catch (NavigationException $e) {
    $threw = true;
    check('real evaluate timeout: message says timed out', str_contains($e->getMessage(), 'timed out'));
}
check('real evaluate timeout: throws', $threw);
unlink($sleepWrapper2);

// A command that writes nothing exits with both pipes already at EOF: the read
// loop must break out on the empty read set (not spin) and take the exit code
// from proc_close, since proc_get_status never reported one.
$silentWrapper = tempnam(sys_get_temp_dir(), 'b4w');
file_put_contents($silentWrapper, "#!/bin/sh\nexit 0\n");
chmod($silentWrapper, 0755);
$browserSilent = new AgentBrowser(timeout: 5, binary: $silentWrapper);
$silentOk = true;
try {
    $silentPage = $browserSilent->goto('https://x.io');
    $silentOk = $silentPage->url === 'https://x.io';
} catch (NavigationException $e) {
    $silentOk = false;
    echo '  detail: ' . $e->getMessage() . "\n";
}
check('real silent process: no output, no spin, exit 0', $silentOk);
unlink($silentWrapper);

// --- Summary -----------------------------------------------------------------

echo "\n{$pass} passed, {$fail} failed\n";
if ($fail > 0) {
    echo "Failures:\n";
    foreach ($failures as $failure) {
        echo "  - {$failure}\n";
    }
    exit(1);
}
exit(0);
