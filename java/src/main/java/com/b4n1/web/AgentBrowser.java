package com.b4n1.web;

import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

/**
 * B4n1Web Agent Browser.
 *
 * A browser instance optimized for AI agent workflows.
 * Requires B4n1Web binary to be installed.
 *
 * <pre>{@code
 * var browser = new AgentBrowser(BrowserMode.LIGHT);
 * var page = browser.goto("https://example.com");
 * System.out.println(page.getMarkdown());
 * browser.close();
 * }</pre>
 */
public class AgentBrowser implements AutoCloseable {
    private static final String SDK_VERSION = "0.6.0";
    private final BrowserOptions options;
    private final String binaryPath;

    public AgentBrowser() {
        this(new BrowserOptions());
    }

    public AgentBrowser(BrowserOptions options) {
        this.options = Objects.requireNonNull(options, "options cannot be null");

        String path = findBinary();
        if (path == null || path.isEmpty()) {
            throw new BinaryNotFoundException();
        }
        this.binaryPath = path;

        // Check version compatibility (non-fatal warning)
        checkVersionCompatibility();
    }

    /**
     * Check if binary version matches SDK version.
     * Prints warning to stderr if mismatch detected.
     */
    private static void checkVersionCompatibility() {
        String binaryVersion = getVersion();
        if (binaryVersion.equals("unknown")) {
            return;
        }

        if (!binaryVersion.equals(SDK_VERSION)) {
            System.err.println("⚠️  Version mismatch: SDK v" + SDK_VERSION + " requires binary v" + SDK_VERSION +
                ", but found v" + binaryVersion + ". Some features may not work correctly.");
        }
    }

    /**
     * Navigate to a URL and extract structured content.
     */
    public Page goto_(String url) throws NavigationException {
        try {
            ProcessBuilder pb = new ProcessBuilder(
                binaryPath, "goto", url, "--mode", options.getMode().getValue()
            );
            pb.redirectErrorStream(true);
            Process process = pb.start();

            String output = new BufferedReader(new InputStreamReader(process.getInputStream()))
                    .lines()
                    .collect(Collectors.joining("\n"));

            boolean finished = process.waitFor(options.getTimeout(), java.util.concurrent.TimeUnit.SECONDS);
            if (!finished) {
                process.destroyForcibly();
                throw new NavigationException(url, "Binary timed out after " + options.getTimeout() + "s");
            }

            if (process.exitValue() != 0) {
                throw new NavigationException(url, "Binary error: exit code " + process.exitValue());
            }

            return parseOutput(url, output);

        } catch (NavigationException e) {
            throw e;
        } catch (Exception e) {
            throw new NavigationException(url, e);
        }
    }

    /**
     * Navigate to URL (alias for goto to avoid reserved keyword).
     */
    public Page navigate(String url) {
        return goto_(url);
    }

    private Page parseOutput(String url, String output) {
        StringBuilder markdown = new StringBuilder();
        List<String> links = new ArrayList<>();

        for (String line : output.split("\n")) {
            if (line.startsWith("URL:")) {
                continue;
            } else if (line.startsWith("Markdown:")) {
                continue;
            } else if (line.startsWith("Links:")) {
                try {
                    String linksJson = line.substring(6).trim();
                    links = parseLinksJson(linksJson);
                } catch (Exception e) {
                    links = new ArrayList<>();
                }
            } else {
                if (markdown.length() > 0) {
                    markdown.append("\n");
                }
                markdown.append(line);
            }
        }

        Page page = new Page();
        page.setUrl(url);
        page.setMarkdown(markdown.toString().trim());
        page.setLinks(links);
        return page;
    }

    private List<String> parseLinksJson(String json) {
        if (json == null || json.isEmpty() || json.equals("[]")) {
            return new ArrayList<>();
        }
        json = json.trim();
        if (!json.startsWith("[")) {
            return new ArrayList<>();
        }
        List<String> result = new ArrayList<>();
        json = json.substring(1, json.length() - 1);
        for (String item : json.split(",")) {
            item = item.trim();
            if (item.startsWith("'") && item.endsWith("'")) {
                result.add(item.substring(1, item.length() - 1));
            } else if (item.startsWith("\"") && item.endsWith("\"")) {
                result.add(item.substring(1, item.length() - 1));
            }
        }
        return result;
    }

    private String findBinary() {
        // 1. Check bundled binary (bundled as native resource)
        try {
            String resourcePath = "/native/linux-x86_64/b4n1web-linux";
            java.net.URL url = getClass().getResource(resourcePath);
            if (url != null) {
                // Extract to temp directory
                File bundledBinary = extractBundledBinary(resourcePath);
                if (bundledBinary != null && bundledBinary.canExecute()) {
                    return bundledBinary.getAbsolutePath();
                }
            }
        } catch (Exception e) {
            // Bundled binary not available or couldn't be extracted
        }

        // 2. Check system install locations
        String[] possiblePaths = {
            "/usr/local/bin/b4n1web",
            "/usr/bin/b4n1web",
            System.getProperty("user.home") + "/.local/bin/b4n1web",
            System.getProperty("user.home") + "/.b4n1web/bin/b4n1web"
        };

        for (String path : possiblePaths) {
            File f = new File(path);
            if (f.exists() && f.canExecute()) {
                return path;
            }
        }
        return null;
    }

    /**
     * Extract bundled binary to temp directory
     */
    private File extractBundledBinary(String resourcePath) {
        try {
            File tempDir = new File(System.getProperty("java.io.tmpdir"), "b4n1web");
            tempDir.mkdirs();
            File tempBinary = new File(tempDir, "b4n1web");

            try (java.io.InputStream in = getClass().getResourceAsStream(resourcePath);
                 java.io.FileOutputStream out = new java.io.FileOutputStream(tempBinary)) {
                if (in == null) return null;
                byte[] buffer = new byte[8192];
                int bytesRead;
                while ((bytesRead = in.read(buffer)) != -1) {
                    out.write(buffer, 0, bytesRead);
                }
            }
            tempBinary.setExecutable(true);
            return tempBinary;
        } catch (Exception e) {
            return null;
        }
    }

    /**
     * Get B4n1Web binary version.
     */
    public static String getVersion() {
        String[] paths = {
            "/usr/local/bin/b4n1web",
            "/usr/bin/b4n1web",
            System.getProperty("user.home") + "/.local/bin/b4n1web",
            System.getProperty("user.home") + "/.b4n1web/bin/b4n1web"
        };

        for (String path : paths) {
            File f = new File(path);
            if (f.exists() && f.canExecute()) {
                try {
                    Process pb = new ProcessBuilder(path, "--version").start();
                    String version = new BufferedReader(new InputStreamReader(pb.getInputStream()))
                            .lines()
                            .collect(Collectors.joining("\n"));
                    pb.waitFor(5, java.util.concurrent.TimeUnit.SECONDS);
                    return version.trim();
                } catch (Exception e) {
                    // continue
                }
            }
        }
        return "unknown";
    }

    @Override
    public void close() {
        // No persistent session in current implementation
    }
}
