//! HTML parsing and extraction module
//!
//! Converts HTML to structured data, extracting clean content and links.

use crate::{Error, Result};
use scraper::{Html, Selector};

/// Parse HTML string into a scraper document
pub fn parse_html(html: &str) -> Result<Html> {
    Ok(Html::parse_document(html))
}

/// Convert HTML document to clean markdown
pub fn html_to_markdown(document: &Html) -> Result<String> {
    // Remove script and style elements
    let mut cleaned_html = document.html();

    // Simple cleaning: remove script and style tags
    cleaned_html = cleaned_html
        .lines()
        .filter(|line: &&str| {
            !line.trim().starts_with("<script") &&
            !line.trim().starts_with("</script>") &&
            !line.trim().starts_with("<style") &&
            !line.trim().starts_with("</style")
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Use html2md for conversion
    let markdown = html2md::parse_html(&cleaned_html);
    Ok(markdown)
}

/// Extract all links from HTML document
pub fn extract_links(document: &Html) -> Result<Vec<String>> {
    let selector = Selector::parse("a[href]").map_err(|e| Error::Parse(e.to_string()))?;

    let links = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter(|href| !href.is_empty() && !href.starts_with('#') && !href.starts_with("javascript:"))
        .map(|href| href.to_string())
        .collect();

    Ok(links)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_valid() {
        let html = "<html><body><p>Hello</p></body></html>";
        let doc = parse_html(html).unwrap();
        assert!(doc.html().contains("Hello"));
    }

    #[test]
    fn test_parse_html_empty() {
        let doc = parse_html("").unwrap();
        assert!(!doc.html().is_empty());
    }

    #[test]
    fn test_html_to_markdown_removes_scripts() {
        let html = "<html><body><script>alert(1)</script><p>text</p></body></html>";
        let doc = parse_html(html).unwrap();
        let md = html_to_markdown(&doc).unwrap();
        assert!(!md.contains("alert"));
        assert!(md.contains("text"));
    }

    #[test]
    fn test_html_to_markdown_removes_styles() {
        let html = "<html><head><style>.cls{color:red}</style></head><body><p>content</p></body></html>";
        let doc = parse_html(html).unwrap();
        let md = html_to_markdown(&doc).unwrap();
        assert!(!md.contains("color:red"));
        assert!(md.contains("content"));
    }

    #[test]
    fn test_html_to_markdown_empty() {
        let doc = parse_html("").unwrap();
        let md = html_to_markdown(&doc).unwrap();
        assert!(md.is_empty() || !md.contains('<'));
    }

    #[test]
    fn test_extract_links_basic() {
        let html = r#"<html><body><a href="https://example.com">link</a></body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://example.com");
    }

    #[test]
    fn test_extract_links_filters_anchor() {
        let html = r#"<html><body><a href="#section">anchor</a><a href="https://site.com">real</a></body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://site.com");
    }

    #[test]
    fn test_extract_links_filters_javascript() {
        let html = r#"<html><body><a href="javascript:void(0)">js</a></body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert!(links.is_empty());
    }

    #[test]
    fn test_extract_links_filters_empty() {
        let html = r#"<html><body><a href="">empty</a><a href="https://ok.com">ok</a></body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://ok.com");
    }

    #[test]
    fn test_extract_links_multiple() {
        let html = r#"<html><body>
            <a href="https://a.com">A</a>
            <a href="https://b.com">B</a>
            <a href="https://c.com">C</a>
        </body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert_eq!(links.len(), 3);
    }

    #[test]
    fn test_extract_links_no_links() {
        let html = "<html><body><p>no links</p></body></html>";
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert!(links.is_empty());
    }

    #[test]
    fn test_parse_html_malicious() {
        let html = r#"<html><body><script>while(true){}</script><img src=x onerror=alert(1)><p>text</p></body></html>"#;
        let doc = parse_html(html).unwrap();
        let md = html_to_markdown(&doc).unwrap();
        assert!(md.contains("text"));
        assert!(!md.contains("while(true)"));
        assert!(!md.contains("alert(1)"));
    }

    #[test]
    fn test_parse_html_xss_injection() {
        let html = r#"<html><body><a href="javascript:alert('xss')">click</a><p>safe</p></body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert!(links.is_empty());
        let md = html_to_markdown(&doc).unwrap();
        assert!(md.contains("safe"));
    }

    #[test]
    fn test_parse_html_utf8() {
        let html = "<html><body><p>Hello 世界 👋 ñ á é í ó ú</p></body></html>";
        let doc = parse_html(html).unwrap();
        let md = html_to_markdown(&doc).unwrap();
        assert!(md.contains("Hello"));
        assert!(md.contains("世界"));
    }

    #[test]
    fn test_parse_html_nested_scripts() {
        let html = "<html><body><div><script>function a(){<script>b();</script>}</script><p>after</p></div></body></html>";
        let doc = parse_html(html).unwrap();
        let md = html_to_markdown(&doc).unwrap();
        assert!(md.contains("after"));
    }

    #[test]
    fn test_parse_html_very_deep() {
        let mut html = String::from("<html><body>");
        for i in 0..1000 {
            html.push_str(&format!("<div id='{}'>", i));
        }
        html.push_str("deep");
        for _ in 0..1000 {
            html.push_str("</div>");
        }
        html.push_str("</body></html>");
        let doc = parse_html(&html).unwrap();
        let md = html_to_markdown(&doc).unwrap();
        assert!(md.contains("deep"));
    }

    #[test]
    fn test_extract_links_duplicates() {
        let html = r#"<html><body>
            <a href="https://same.com">A</a>
            <a href="https://same.com">B</a>
            <a href="https://same.com">C</a>
        </body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert_eq!(links.len(), 3); // Preserves duplicates
        assert_eq!(links[0], links[1]);
    }

    #[test]
    fn test_extract_links_protocol_relative() {
        let html = r#"<html><body><a href="//example.com/path">rel</a></body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "//example.com/path");
    }

    #[test]
    fn test_extract_links_mailto() {
        let html = r#"<html><body><a href="mailto:test@test.com">email</a></body></html>"#;
        let doc = parse_html(html).unwrap();
        let links = extract_links(&doc).unwrap();
        assert_eq!(links.len(), 1);
        assert!(links[0].contains("mailto:"));
    }
}
