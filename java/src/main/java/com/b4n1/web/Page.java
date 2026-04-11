package com.b4n1.web;

import java.util.List;
import java.util.stream.Collectors;

/**
 * Structured page data returned by B4n1Web.
 */
public class Page {
    private String url;
    private String markdown;
    private List<String> links;
    private String screenshot;

    public Page() {
    }

    public Page(String url, String markdown, List<String> links) {
        this.url = url;
        this.markdown = markdown;
        this.links = links;
    }

    public String getUrl() {
        return url;
    }

    public void setUrl(String url) {
        this.url = url;
    }

    public String getMarkdown() {
        return markdown;
    }

    public void setMarkdown(String markdown) {
        this.markdown = markdown;
    }

    public List<String> getLinks() {
        return links;
    }

    public void setLinks(List<String> links) {
        this.links = links;
    }

    public String getScreenshot() {
        return screenshot;
    }

    public void setScreenshot(String screenshot) {
        this.screenshot = screenshot;
    }

    /**
     * Extract main content from markdown, skipping headers.
     */
    public String getMainContent() {
        if (markdown == null || markdown.isEmpty()) {
            return "";
        }
        String[] lines = markdown.split("\n");
        if (lines.length > 2) {
            StringBuilder sb = new StringBuilder();
            for (int i = 2; i < lines.length; i++) {
                if (i > 2) sb.append("\n");
                sb.append(lines[i]);
            }
            return sb.toString().trim();
        }
        return markdown.trim();
    }

    /**
     * Find links containing specific text.
     */
    public List<String> findLinksByText(String text) {
        if (links == null || text == null) {
            return List.of();
        }
        String lowerText = text.toLowerCase();
        return links.stream()
                .filter(link -> link.toLowerCase().contains(lowerText))
                .collect(Collectors.toList());
    }
}
