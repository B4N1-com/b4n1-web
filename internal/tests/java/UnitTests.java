package com.b4n1.web;

import java.util.Arrays;

/**
 * Exhaustive tests for B4n1Web Java SDK.
 * SDK is a thin wrapper around the b4n1web binary.
 */
public class UnitTests {

    private static int passed = 0;
    private static int failed = 0;
    private static int skipped = 0;

    public static void main(String[] args) throws Exception {
        System.out.println("===========================================");
        System.out.println("  B4N1WEB JAVA SDK - TESTS");
        System.out.println("===========================================\n");

        // BrowserMode
        run("BrowserMode values", () -> {
            assertEq(BrowserMode.LIGHT.getValue(), "light", "LIGHT");
            assertEq(BrowserMode.JS.getValue(), "js", "JS");
            assertEq(BrowserMode.RENDER.getValue(), "render", "RENDER");
        });
        run("BrowserMode distinct", () -> {
            assertNeq(BrowserMode.LIGHT, BrowserMode.JS, "LIGHT!=JS");
            assertNeq(BrowserMode.LIGHT, BrowserMode.RENDER, "LIGHT!=RENDER");
            assertNeq(BrowserMode.JS, BrowserMode.RENDER, "JS!=RENDER");
        });
        run("BrowserMode valueOf", () -> {
            assertEq(BrowserMode.valueOf("LIGHT"), BrowserMode.LIGHT, "valueOf");
        });
        run("BrowserMode values()", () -> {
            BrowserMode[] modes = BrowserMode.values();
            assertEq(modes.length, 3, "3 modes");
        });

        // BrowserOptions
        run("BrowserOptions defaults", () -> {
            BrowserOptions opts = new BrowserOptions();
            assertEq(opts.getMode(), BrowserMode.LIGHT, "mode");
            assertEq(opts.getTimeout(), 30, "timeout");
            assertEq(opts.getUserAgent(), "B4N1Web-Agent/1.0", "userAgent");
        });
        run("BrowserOptions setMode", () -> {
            BrowserOptions opts = new BrowserOptions();
            opts.setMode(BrowserMode.JS);
            assertEq(opts.getMode(), BrowserMode.JS, "setMode");
        });
        run("BrowserOptions setTimeout", () -> {
            BrowserOptions opts = new BrowserOptions();
            opts.setTimeout(60);
            assertEq(opts.getTimeout(), 60, "setTimeout");
        });
        run("BrowserOptions setUserAgent", () -> {
            BrowserOptions opts = new BrowserOptions();
            opts.setUserAgent("Custom/1.0");
            assertEq(opts.getUserAgent(), "Custom/1.0", "setUserAgent");
        });

        // Page
        run("Page construction", () -> {
            Page p = new Page("https://example.com", "# Hello\nContent",
                Arrays.asList("https://a.com", "https://b.com"));
            assertEq(p.getUrl(), "https://example.com", "URL");
            assertEq(p.getMarkdown(), "# Hello\nContent", "markdown");
            assertEq(p.getLinks().size(), 2, "links count");
        });
        run("Page empty", () -> {
            Page p = new Page();
            assertTrue(p.getUrl() == null || p.getUrl().isEmpty(), "empty URL");
            assertTrue(p.getMarkdown() == null || p.getMarkdown().isEmpty(), "empty markdown");
        });
        run("Page unicode", () -> {
            Page p = new Page("https://example.com/日本語", "日本語テスト\n🚀",
                Arrays.asList("https://例.jp"));
            assertTrue(p.getMarkdown().contains("日本語"), "unicode");
        });
        run("Page setScreenshot", () -> {
            Page p = new Page("u", "Content", Arrays.asList());
            p.setScreenshot("base64");
            assertEq(p.getScreenshot(), "base64", "screenshot");
        });

        // GetMainContent
        run("GetMainContent empty", () -> {
            Page p = new Page("u", "", Arrays.asList());
            assertEq(p.getMainContent(), "", "empty");
        });
        run("GetMainContent single", () -> {
            Page p = new Page("u", "Only one line", Arrays.asList());
            assertEq(p.getMainContent(), "Only one line", "single");
        });
        run("GetMainContent skips header", () -> {
            Page p = new Page("u", "# Title\n\n## Sub\n\nActual content", Arrays.asList());
            String got = p.getMainContent();
            assertTrue(got.contains("Actual content"), "has content");
            assertTrue(!got.contains("# Title"), "skips header");
        });
        run("GetMainContent many lines", () -> {
            StringBuilder sb = new StringBuilder("# Header\n\n## Sub\n");
            for (int i = 0; i < 100; i++) sb.append("Line ").append(i).append("\n");
            Page p = new Page("u", sb.toString(), Arrays.asList());
            assertTrue(p.getMainContent().length() > 50, "many lines");
        });

        // FindLinksByText
        run("FindLinks exact", () -> {
            Page p = new Page("u", "", Arrays.asList("https://x.com/contact"));
            assertEq(p.findLinksByText("contact").size(), 1, "exact");
        });
        run("FindLinks partial", () -> {
            Page p = new Page("u", "", Arrays.asList("https://x.com/about-us"));
            assertEq(p.findLinksByText("about").size(), 1, "partial");
        });
        run("FindLinks no match", () -> {
            Page p = new Page("u", "", Arrays.asList("https://x.com/page"));
            assertEq(p.findLinksByText("missing").size(), 0, "no match");
        });
        run("FindLinks case insensitive", () -> {
            Page p = new Page("u", "", Arrays.asList("https://EXAMPLE.COM"));
            assertEq(p.findLinksByText("example").size(), 1, "case");
        });
        run("FindLinks multiple", () -> {
            Page p = new Page("u", "", Arrays.asList(
                "https://x.com/contact", "https://x.com/contact-us", "https://y.com"));
            assertEq(p.findLinksByText("contact").size(), 2, "multiple");
        });
        run("FindLinks empty search", () -> {
            Page p = new Page("u", "", Arrays.asList("https://a.com", "https://b.com"));
            assertEq(p.findLinksByText("").size(), 2, "empty search");
        });
        run("FindLinks empty list", () -> {
            Page p = new Page("u", "", Arrays.asList());
            assertEq(p.findLinksByText("x").size(), 0, "empty list");
        });
        run("FindLinks unicode", () -> {
            Page p = new Page("u", "", Arrays.asList("https://x.com/日本語"));
            assertEq(p.findLinksByText("日本語").size(), 1, "unicode");
        });

        // AgentBrowser
        run("AgentBrowser defaults", () -> {
            try {
                AgentBrowser b = new AgentBrowser();
                pass("construct");
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            }
        });
        run("AgentBrowser with mode", () -> {
            try {
                BrowserOptions opts = new BrowserOptions();
                opts.setMode(BrowserMode.JS);
                AgentBrowser b = new AgentBrowser(opts);
                pass("construct with mode");
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            }
        });
        run("AgentBrowser with timeout", () -> {
            try {
                BrowserOptions opts = new BrowserOptions();
                opts.setTimeout(60);
                AgentBrowser b = new AgentBrowser(opts);
                pass("construct with timeout");
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            }
        });
        run("AgentBrowser with userAgent", () -> {
            try {
                BrowserOptions opts = new BrowserOptions();
                opts.setUserAgent("Custom/1.0");
                AgentBrowser b = new AgentBrowser(opts);
                pass("construct with userAgent");
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            }
        });
        run("AgentBrowser close multiple", () -> {
            try {
                AgentBrowser b = new AgentBrowser();
                b.close(); b.close(); b.close();
                pass("multiple closes");
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            }
        });

        // Goto with real binary
        run("Goto success", () -> {
            try {
                AgentBrowser b = new AgentBrowser();
                Page p = b.goto_("https://example.com");
                assertEq(p.getUrl(), "https://example.com", "URL");
                assertTrue(p.getMarkdown().length() > 0, "markdown");
                assertTrue(p.getLinks() != null, "links");
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            } catch (Exception e) {
                fail("goto: " + e.getMessage());
            }
        });
        run("Goto JS mode", () -> {
            try {
                BrowserOptions opts = new BrowserOptions();
                opts.setMode(BrowserMode.JS);
                AgentBrowser b = new AgentBrowser(opts);
                Page p = b.goto_("https://example.com");
                assertEq(p.getUrl(), "https://example.com", "URL");
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            } catch (Exception e) {
                fail("goto JS: " + e.getMessage());
            }
        });
        run("Goto invalid URL", () -> {
            try {
                AgentBrowser b = new AgentBrowser();
                try {
                    b.goto_("not-a-valid-url");
                    fail("invalid URL should throw");
                } catch (NavigationException e) {
                    pass("invalid URL throws");
                }
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            }
        });
        run("Goto non-existent domain", () -> {
            try {
                AgentBrowser b = new AgentBrowser();
                try {
                    b.goto_("https://this-domain-definitely-does-not-exist-abc123xyz.com");
                    fail("non-existent should throw");
                } catch (NavigationException e) {
                    pass("non-existent throws");
                }
                b.close();
            } catch (BinaryNotFoundException e) {
                skip("binary not found");
            }
        });

        // Version
        run("GetVersion", () -> {
            String ver = AgentBrowser.getVersion();
            if (ver == null || ver.isEmpty() || ver.equals("unknown")) {
                skip("binary not in system path");
            } else {
                pass("version: " + ver);
            }
        });

        // Errors
        run("BinaryNotFoundException", () -> {
            try {
                throw new BinaryNotFoundException("test message");
            } catch (BinaryNotFoundException e) {
                assertTrue(e.getMessage().contains("test message"), "message");
                pass("BinaryNotFoundException");
            }
        });
        run("NavigationException", () -> {
            try {
                throw new NavigationException("https://x.com", "nav error");
            } catch (NavigationException e) {
                assertEq(e.getUrl(), "https://x.com", "URL");
                pass("NavigationException");
            }
        });

        // Results
        System.out.println("\n===========================================");
        System.out.println("  RESULTS: " + passed + " passed, " + failed + " failed, " + skipped + " skipped");
        System.out.println("===========================================");
        if (failed > 0) System.exit(1);
    }

    // Test helpers
    private static void run(String name, Runnable test) {
        try { test.run(); }
        catch (AssertionError e) { System.err.println("  ❌ " + name + ": " + e.getMessage()); failed++; }
        catch (Exception e) { System.err.println("  ❌ " + name + ": " + e.getClass().getSimpleName() + ": " + e.getMessage()); failed++; }
    }
    private static void pass(String m) { passed++; }
    private static void fail(String m) { System.err.println("  ❌ " + m); failed++; }
    private static void skip(String m) { System.out.println("  ⏭️  " + m); skipped++; }
    private static void assertEq(Object a, Object b, String n) {
        if (!a.equals(b)) throw new AssertionError(n + ": expected " + b + ", got " + a);
        passed++;
    }
    private static void assertNeq(Object a, Object b, String n) {
        if (a.equals(b)) throw new AssertionError(n + ": should differ");
        passed++;
    }
    private static void assertTrue(boolean c, String n) {
        if (!c) throw new AssertionError(n + ": expected true");
        passed++;
    }
}
