<?php

declare(strict_types=1);

namespace B4n1Web;

use B4n1Web\Exception\BinaryNotFoundException;

/**
 * Resolves the b4n1web binary. Order mirrors the Python/JS/C#/Java/Ruby SDKs:
 *
 *   1. B4N1WEB_BIN_PATH (explicit override, must exist + be executable)
 *   2. well-known user/system locations
 *   3. PATH scan
 *
 * No silent fallback: resolveOrThrow() raises BinaryNotFoundException with an
 * actionable install hint when nothing resolves.
 */
final class Binary
{
    /** @var array<string, string> platform triple => bundled file name */
    public const PLATFORM_BINARIES = [
        'linux-x86_64' => 'b4n1web-linux-amd64',
        'linux-aarch64' => 'b4n1web-linux-arm64',
        'darwin-x86_64' => 'b4n1web-macos-x64',
        'darwin-arm64' => 'b4n1web-macos-arm64',
        'win-x64' => 'b4n1web-windows-amd64.exe',
    ];

    private const INSTALL_HINT = 'Install it (curl -sL https://raw.githubusercontent.com/'
        . 'B4N1-com/b4n1-web/master/scripts/install.sh | bash) or set B4N1WEB_BIN_PATH.';

    public static function resolve(): ?string
    {
        $env = getenv('B4N1WEB_BIN_PATH');
        if (is_string($env) && $env !== '' && self::isRunnable($env)) {
            return $env;
        }

        $candidates = [];
        $binaryEnv = getenv('B4N1WEB_BINARY');
        if (is_string($binaryEnv) && $binaryEnv !== '') {
            $candidates[] = $binaryEnv;
        }
        $home = getenv('HOME') ?: (getenv('USERPROFILE') ?: '');
        if ($home !== '') {
            $candidates[] = $home . '/.local/bin/b4n1web';
            $candidates[] = $home . '/.b4n1web/bin/b4n1web';
        }
        $candidates[] = '/usr/local/bin/b4n1web';
        $candidates[] = '/usr/bin/b4n1web';

        $path = getenv('PATH');
        if (is_string($path) && $path !== '') {
            foreach (explode(PATH_SEPARATOR, $path) as $dir) {
                $candidates[] = rtrim($dir, '/\\') . DIRECTORY_SEPARATOR . 'b4n1web';
            }
        }

        foreach ($candidates as $candidate) {
            if (self::isRunnable($candidate)) {
                return $candidate;
            }
        }

        return null;
    }

    /**
     * @throws BinaryNotFoundException when nothing resolves
     */
    public static function resolveOrThrow(): string
    {
        $path = self::resolve();
        if ($path === null) {
            throw new BinaryNotFoundException('b4n1web binary not found. ' . self::INSTALL_HINT);
        }

        return $path;
    }

    private static function isRunnable(string $path): bool
    {
        return is_file($path) && is_executable($path);
    }
}
