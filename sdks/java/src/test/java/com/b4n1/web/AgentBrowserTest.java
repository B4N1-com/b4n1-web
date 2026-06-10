package com.b4n1.web;

import org.junit.jupiter.api.Test;

import java.io.BufferedReader;
import java.io.File;
import java.io.InputStreamReader;
import java.net.URISyntaxException;
import java.net.URL;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Smoke tests for the {@link AgentBrowser} class.
 *
 * Verifies construction, bundled binary availability, and
 * version compatibility. These tests use the real bundled
 * binary shipped with the SDK.
 */
class AgentBrowserTest {

    // ── Construction (smoke) ──────────────────────────────────────────

    @Test
    void constructorWithNoArgsShouldSucceed() {
        assertDoesNotThrow(() -> {
            try (AgentBrowser browser = new AgentBrowser()) {
                assertNotNull(browser);
            }
        }, "AgentBrowser() should construct successfully when binary is available");
    }

    @Test
    void constructorWithDefaultOptionsShouldSucceed() {
        assertDoesNotThrow(() -> {
            try (AgentBrowser browser = new AgentBrowser(new BrowserOptions())) {
                assertNotNull(browser);
            }
        }, "AgentBrowser(BrowserOptions) should construct with default options");
    }

    @Test
    void constructorWithCustomOptionsShouldSucceed() {
        BrowserOptions options = new BrowserOptions();
        options.setMode(BrowserMode.JS);
        options.setTimeout(60);
        options.setUserAgent("TestAgent/1.0");

        assertDoesNotThrow(() -> {
            try (AgentBrowser browser = new AgentBrowser(options)) {
                assertNotNull(browser);
            }
        }, "AgentBrowser(BrowserOptions) should accept custom configuration");
    }

    @Test
    void constructorShouldRejectNullOptions() {
        assertThrows(NullPointerException.class, () -> new AgentBrowser(null),
                "Constructor should throw NullPointerException for null options");
    }

    // ── Bundled binary existence ──────────────────────────────────────

    @Test
    void bundledBinaryShouldExist() {
        File binary = resolveBundledBinary();
        assertNotNull(binary, "Bundled binary resource should be resolvable");
        assertTrue(binary.exists(),
                "Bundled binary file must exist at: " + binary.getAbsolutePath());
    }

    @Test
    void bundledBinaryShouldBeExecutable() {
        File binary = resolveBundledBinary();
        assertNotNull(binary);
        assertTrue(binary.canExecute(),
                "Bundled binary must be executable at: " + binary.getAbsolutePath());
    }

    // ── Binary version ────────────────────────────────────────────────

    @Test
    void binaryVersionShouldBe040() throws Exception {
        File binary = resolveBundledBinary();
        assertNotNull(binary, "Cannot test version: bundled binary not found");

        Process process = new ProcessBuilder(binary.getAbsolutePath(), "--version")
                .redirectErrorStream(true)
                .start();

        String output = new BufferedReader(new InputStreamReader(process.getInputStream()))
                .lines()
                .collect(Collectors.joining("\n"))
                .trim();

        boolean finished = process.waitFor(10, java.util.concurrent.TimeUnit.SECONDS);
        assertTrue(finished, "Binary --version command timed out");
        assertEquals(0, process.exitValue(),
                "Binary --version should exit with code 0");

        assertTrue(output.contains("0.9.4"),
                "Binary version should contain SDK version 0.9.4, but got: " + output);
    }

    @Test
    void binaryVersionShouldBeStableAndNotUnknown() throws Exception {
        File binary = resolveBundledBinary();
        assertNotNull(binary);

        Process process = new ProcessBuilder(binary.getAbsolutePath(), "--version")
                .redirectErrorStream(true)
                .start();

        String version = new BufferedReader(new InputStreamReader(process.getInputStream()))
                .lines()
                .collect(Collectors.joining("\n"))
                .trim();

        process.waitFor(5, java.util.concurrent.TimeUnit.SECONDS);

        assertNotNull(version);
        assertFalse(version.isEmpty(), "Version string must not be empty");
        assertFalse(version.equals("unknown"), "Version must not be 'unknown'");
    }

    // ── Helper methods ────────────────────────────────────────────────

    /**
     * Resolves the bundled native binary from the classpath resource
     * {@code /native/linux-x86_64/b4n1web}, falling back to the
     * project-relative file path for IDE runs.
     */
    private File resolveBundledBinary() {
        URL resource = getClass().getResource("/native/linux-x86_64/b4n1web");
        if (resource != null) {
            try {
                return new File(resource.toURI());
            } catch (URISyntaxException e) {
                return new File(resource.getPath());
            }
        }
        // Fallback for when running outside Maven (e.g. some IDE configurations)
        File fallback = new File("native/linux-x86_64/b4n1web");
        if (fallback.exists()) {
            return fallback;
        }
        return null;
    }
}
