<?php

declare(strict_types=1);

namespace B4n1Web;

/**
 * Structured page data returned by the b4n1web binary.
 */
final class Page
{
    /**
     * @param list<string> $links
     */
    public function __construct(
        public readonly string $url,
        public readonly string $markdown,
        public readonly array $links,
        public readonly ?string $screenshot = null,
    ) {
    }

    /**
     * Parses the binary's text protocol:
     *
     *   URL: <url>
     *   Markdown:
     *   <free-form markdown lines...>
     *   Links: ["https://...", ...]
     *   Screenshot: <path>            (optional)
     */
    public static function parse(string $url, string $output): self
    {
        $markdown = '';
        $links = [];
        $screenshot = null;
        $inMarkdown = false;

        foreach (preg_split('/\r\n|\r|\n/', $output) ?: [] as $line) {
            if (str_starts_with($line, 'URL:')) {
                continue;
            }
            if (str_starts_with($line, 'Markdown:')) {
                $inMarkdown = true;
                $rest = trim(substr($line, strlen('Markdown:')));
                if ($rest !== '') {
                    $markdown .= $rest . "\n";
                }
                continue;
            }
            if (str_starts_with($line, 'Links:')) {
                $inMarkdown = false;
                $json = trim(substr($line, strlen('Links:')));
                $decoded = $json === '' ? [] : json_decode($json, true);
                $links = is_array($decoded) ? $decoded : [];
                continue;
            }
            if (str_starts_with($line, 'Screenshot:')) {
                $inMarkdown = false;
                $shot = trim(substr($line, strlen('Screenshot:')));
                $screenshot = $shot === '' ? null : $shot;
                continue;
            }
            if ($inMarkdown) {
                $markdown .= $line . "\n";
            }
        }

        return new self($url, trim($markdown), $links, $screenshot);
    }

    /**
     * Main content heuristics: drop the first two lines (title + empty line).
     */
    public function getMainContent(): string
    {
        $lines = preg_split('/\r\n|\r|\n/', $this->markdown) ?: [];
        if (count($lines) > 2) {
            $lines = array_slice($lines, 2);
        }

        return trim(implode("\n", $lines));
    }

    /**
     * @return list<string>
     */
    public function findLinksByText(string $text): array
    {
        if ($text === '') {
            return [];
        }
        $needle = strtolower($text);
        $found = [];
        foreach ($this->links as $link) {
            if (str_contains(strtolower((string) $link), $needle)) {
                $found[] = $link;
            }
        }

        return $found;
    }
}
