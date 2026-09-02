package com.b4n1.web;

import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
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
    private static final String SDK_VERSION = "0.13.0";
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
    }

    AgentBrowser(BrowserOptions options, String testBinaryPath) {
        this.options = Objects.requireNonNull(options, "options cannot be null");
        if (testBinaryPath != null) {
            this.binaryPath = testBinaryPath;
        } else {
            String path = findBinary();
            if (path == null || path.isEmpty()) {
                throw new BinaryNotFoundException();
            }
            this.binaryPath = path;
        }
    }

    /**
     * Navigate to a URL and extract structured content.
     */
    public Page goto_(String url) throws NavigationException {
        return goto_(url, null);
    }

    /**
     * Navigate to a URL with an optional CSS selector to wait for.
     */
    public Page goto_(String url, String waitFor) throws NavigationException {
        try {
            List<String> cmd = new ArrayList<>();
            cmd.add(binaryPath);
            cmd.add("goto");
            cmd.add(url);
            cmd.add("--mode");
            cmd.add(options.getMode().getValue());
            if (waitFor != null && !waitFor.isEmpty()) {
                cmd.add("--wait-for");
                cmd.add(waitFor);
            }

            ProcessBuilder pb = new ProcessBuilder(cmd);
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
     * Take a screenshot of the current page with specified dimensions.
     */
    public String screenshot(int width, int height) {
        try {
            ProcessBuilder pb = new ProcessBuilder(
                binaryPath, "screenshot", "--width", String.valueOf(width), "--height", String.valueOf(height),
                "--mode", options.getMode().getValue()
            );
            pb.redirectErrorStream(true);
            Process process = pb.start();

            String output = new BufferedReader(new InputStreamReader(process.getInputStream()))
                    .lines()
                    .collect(Collectors.joining("\n"));

            process.waitFor(options.getTimeout(), java.util.concurrent.TimeUnit.SECONDS);
            return output.trim();
        } catch (Exception e) {
            throw new RuntimeException("Screenshot failed", e);
        }
    }

    /**
     * Wait for a CSS selector to appear within the given timeout.
     */
    public boolean waitForSelector(String selector, int timeoutMs) {
        try {
            ProcessBuilder pb = new ProcessBuilder(
                binaryPath, "wait-selector", selector, "--timeout", String.valueOf(timeoutMs),
                "--mode", options.getMode().getValue()
            );
            pb.redirectErrorStream(true);
            Process process = pb.start();

            String output = new BufferedReader(new InputStreamReader(process.getInputStream()))
                    .lines()
                    .collect(Collectors.joining("\n"));

            process.waitFor(timeoutMs / 1000 + 1, java.util.concurrent.TimeUnit.SECONDS);
            return process.exitValue() == 0;
        } catch (Exception e) {
            return false;
        }
    }

    /**
     * Click an element matching the given CSS selector.
     */
    public void click(String selector) {
        try {
            ProcessBuilder pb = new ProcessBuilder(
                binaryPath, "click", selector, "--mode", options.getMode().getValue()
            );
            pb.redirectErrorStream(true);
            Process process = pb.start();
            process.waitFor(options.getTimeout(), java.util.concurrent.TimeUnit.SECONDS);
        } catch (Exception e) {
            throw new RuntimeException("Click failed for selector: " + selector, e);
        }
    }

    /**
     * Type text into an element matching the given CSS selector.
     */
    public void typeText(String selector, String text, boolean clearFirst) {
        try {
            List<String> cmd = new ArrayList<>();
            cmd.add(binaryPath);
            cmd.add("type");
            cmd.add(selector);
            cmd.add(text);
            cmd.add("--mode");
            cmd.add(options.getMode().getValue());
            if (clearFirst) {
                cmd.add("--clear-first");
            }

            ProcessBuilder pb = new ProcessBuilder(cmd);
            pb.redirectErrorStream(true);
            Process process = pb.start();
            process.waitFor(options.getTimeout(), java.util.concurrent.TimeUnit.SECONDS);
        } catch (Exception e) {
            throw new RuntimeException("TypeText failed for selector: " + selector, e);
        }
    }

    private Page parseOutput(String url, String output) {
        StringBuilder markdown = new StringBuilder();
        List<String> links = new ArrayList<>();
        String jsOutput = null;

        for (String line : output.split("\n")) {
            if (line.startsWith("URL:")) {
                continue;
            } else if (line.startsWith("Markdown:")) {
                continue;
            } else if (line.startsWith("Links:")) {
                String linksJson = line.substring(6).trim();
                links = parseLinksJson(linksJson);
            } else if (line.startsWith("JSOutput:")) {
                jsOutput = line.substring(9).trim();
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
        page.setJsOutput(jsOutput);
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
        try {
            String resourcePath = getPlatformResourcePath();
            if (resourcePath != null) {
                java.net.URL url = getClass().getResource(resourcePath);
                if (url != null) {
                    File bundledBinary = extractBundledBinary(resourcePath);
                    if (bundledBinary != null && bundledBinary.canExecute()) {
                        return bundledBinary.getAbsolutePath();
                    }
                }
            }
        } catch (Exception e) {
        }

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

    private static String getPlatformResourcePath() {
        String os = System.getProperty("os.name").toLowerCase(Locale.ROOT);
        String arch = System.getProperty("os.arch").toLowerCase(Locale.ROOT);

        String osPart;
        if (os.contains("linux")) {
            osPart = "linux";
        } else if (os.contains("mac") || os.contains("darwin")) {
            osPart = "macos";
        } else if (os.contains("windows")) {
            osPart = "windows";
        } else {
            return null;
        }

        String archPart;
        if (arch.equals("amd64") || arch.equals("x86_64")) {
            archPart = "amd64";
        } else if (arch.equals("aarch64") || arch.equals("arm64")) {
            archPart = "arm64";
        } else {
            return null;
        }

        String ext = os.contains("windows") ? ".exe" : "";
        return "/native/" + osPart + "-" + archPart + "/b4n1web" + ext;
    }

    private File extractBundledBinary(String resourcePath) {
        try {
            String ext = resourcePath.endsWith(".exe") ? ".exe" : "";
            File tempDir = new File(System.getProperty("java.io.tmpdir"), "b4n1web");
            tempDir.mkdirs();
            File tempBinary = new File(tempDir, "b4n1web" + ext);

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
                    String trimmed = version.trim();
                    String[] parts = trimmed.split(" ");
                    return parts.length >= 2 ? parts[parts.length - 1] : trimmed;
                } catch (Exception e) {
                }
            }
        }
        return "unknown";
    }

    @Override
    public void close() {
    }
}
