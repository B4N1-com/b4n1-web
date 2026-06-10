package com.b4n1.web;

import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Smoke tests for the {@link Page} data class.
 *
 * Verifies construction, getters/setters, main content extraction,
 * and link filtering logic.
 */
class PageTest {

    // ── Construction ──────────────────────────────────────────────────

    @Test
    void defaultConstructorShouldCreateEmptyPage() {
        Page page = new Page();
        assertNull(page.getUrl(), "URL should be null in empty page");
        assertNull(page.getMarkdown(), "Markdown should be null in empty page");
        assertNull(page.getLinks(), "Links list should be null in empty page");
        assertNull(page.getScreenshot(), "Screenshot should be null in empty page");
        assertNull(page.getJsOutput(), "JS output should be null in empty page");
    }

    @Test
    void parameterizedConstructorShouldSetFields() {
        List<String> links = Arrays.asList("https://example.com", "https://test.com");
        Page page = new Page("https://example.com", "# Hello\nWorld", links);

        assertEquals("https://example.com", page.getUrl());
        assertEquals("# Hello\nWorld", page.getMarkdown());
        assertEquals(links, page.getLinks());
    }

    // ── Setters / Getters ─────────────────────────────────────────────

    @Test
    void settersShouldUpdateFields() {
        Page page = new Page();

        page.setUrl("https://b4n1.com");
        assertEquals("https://b4n1.com", page.getUrl());

        page.setMarkdown("# B4N1\nFramework");
        assertEquals("# B4N1\nFramework", page.getMarkdown());

        List<String> links = Arrays.asList("/about", "/contact");
        page.setLinks(links);
        assertEquals(links, page.getLinks());

        page.setScreenshot("data:image/png;base64,abc");
        assertEquals("data:image/png;base64,abc", page.getScreenshot());

        page.setJsOutput("console.log('done')");
        assertEquals("console.log('done')", page.getJsOutput());
    }

    // ── getLinksArray ─────────────────────────────────────────────────

    @Test
    void getLinksArrayShouldReturnArray() {
        Page page = new Page();
        page.setLinks(Arrays.asList("a", "b", "c"));
        assertArrayEquals(new String[]{"a", "b", "c"}, page.getLinksArray());
    }

    @Test
    void getLinksArrayShouldReturnEmptyArrayWhenLinksAreNull() {
        Page page = new Page();
        assertArrayEquals(new String[0], page.getLinksArray());
    }

    @Test
    void getLinksArrayShouldReturnEmptyArrayWhenLinksAreEmpty() {
        Page page = new Page();
        page.setLinks(Collections.emptyList());
        assertArrayEquals(new String[0], page.getLinksArray());
    }

    // ── getMainContent ────────────────────────────────────────────────

    @Test
    void getMainContentShouldReturnEmptyStringWhenMarkdownIsNull() {
        Page page = new Page();
        assertEquals("", page.getMainContent());
    }

    @Test
    void getMainContentShouldReturnEmptyStringWhenMarkdownIsEmpty() {
        Page page = new Page();
        page.setMarkdown("");
        assertEquals("", page.getMainContent());
    }

    @Test
    void getMainContentShouldTrimMarkdownWithTwoOrFewerLines() {
        Page page = new Page();
        page.setMarkdown("# Single Header");
        assertEquals("# Single Header", page.getMainContent());

        page.setMarkdown("# Header\nContent");
        assertEquals("# Header\nContent", page.getMainContent());
    }

    @Test
    void getMainContentShouldSkipFirstTwoHeaderLines() {
        Page page = new Page();
        page.setMarkdown("# Title\n## Subtitle\nBody text here.");

        String result = page.getMainContent();
        assertEquals("Body text here.", result,
                "getMainContent() should skip the first two header lines");
    }

    @Test
    void getMainContentShouldJoinRemainingLines() {
        Page page = new Page();
        page.setMarkdown("# Title\n## Subtitle\nLine 1\nLine 2\nLine 3");

        String result = page.getMainContent();
        assertEquals("Line 1\nLine 2\nLine 3", result);
    }

    @Test
    void getMainContentShouldTrimWhitespaceFromResult() {
        Page page = new Page();
        page.setMarkdown("# Title\n## Subtitle\n  \nContent\n  ");

        String result = page.getMainContent();
        assertEquals("Content", result,
                "getMainContent() should trim whitespace from the result");
    }

    // ── findLinksByText ───────────────────────────────────────────────

    @Test
    void findLinksByTextShouldReturnMatchingLinks() {
        Page page = new Page();
        page.setLinks(Arrays.asList(
                "https://example.com/about",
                "https://example.com/contact",
                "https://example.com/blog"
        ));

        List<String> result = page.findLinksByText("about");
        assertEquals(1, result.size());
        assertTrue(result.contains("https://example.com/about"));
    }

    @Test
    void findLinksByTextShouldBeCaseInsensitive() {
        Page page = new Page();
        page.setLinks(Arrays.asList(
                "https://example.com/About",
                "https://example.com/CONTACT"
        ));

        List<String> result = page.findLinksByText("about");
        assertEquals(1, result.size());
        assertTrue(result.contains("https://example.com/About"));
    }

    @Test
    void findLinksByTextShouldReturnAllMatchingLinks() {
        Page page = new Page();
        page.setLinks(Arrays.asList(
                "https://example.com/doc",
                "https://docs.example.com",
                "https://example.com/api"
        ));

        List<String> result = page.findLinksByText("doc");
        assertEquals(2, result.size());
        assertTrue(result.contains("https://example.com/doc"));
        assertTrue(result.contains("https://docs.example.com"));
    }

    @Test
    void findLinksByTextShouldReturnEmptyListWhenNoMatch() {
        Page page = new Page();
        page.setLinks(Arrays.asList(
                "https://example.com/foo",
                "https://example.com/bar"
        ));

        List<String> result = page.findLinksByText("nonexistent");
        assertTrue(result.isEmpty());
    }

    @Test
    void findLinksByTextShouldReturnEmptyListWhenLinksAreNull() {
        Page page = new Page();
        assertTrue(page.findLinksByText("anything").isEmpty());
    }

    @Test
    void findLinksByTextShouldReturnEmptyListWhenTextIsNull() {
        Page page = new Page();
        page.setLinks(Arrays.asList("https://example.com"));
        assertTrue(page.findLinksByText(null).isEmpty());
    }

    @Test
    void findLinksByTextShouldReturnMutableList() {
        Page page = new Page();
        page.setLinks(Collections.singletonList("https://example.com"));
        List<String> result = page.findLinksByText("example");
        assertEquals(1, result.size());
        // Collectors.toList() in JDK 11 returns a mutable ArrayList
        result.add("https://added.com");
        assertEquals(2, result.size(), "Returned list should be mutable");
    }
}
