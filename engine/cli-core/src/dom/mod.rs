//! DOM module - Minimal DOM implementation for AI agents
//!
//! Uses scraper for reliable HTML parsing and provides wrapper types.

pub mod node;
pub mod tree;

pub use node::{Element, Node, NodeData, NodeType, Text};
pub use tree::DomTree;

use crate::Result;
use scraper::{Html, Selector};

pub fn parse_html(html: &str) -> Result<DomTree> {
    let document = Html::parse_document(html);
    Ok(DomTree { document })
}

pub fn extract_links(html: &str) -> Result<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").map_err(|e| crate::Error::Parse(e.to_string()))?;

    let links: Vec<String> = document
        .select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .filter(|h| !h.is_empty() && !h.starts_with('#') && !h.starts_with("javascript:"))
        .map(String::from)
        .collect();

    Ok(links)
}

pub fn extract_text(html: &str) -> String {
    let document = Html::parse_document(html);
    if let Ok(selector) = Selector::parse("body") {
        if let Some(body) = document.select(&selector).next() {
            return body.text().collect::<Vec<_>>().join("\n");
        }
    }
    document
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html() {
        let dom = parse_html("<p>hi</p>").unwrap();
        assert!(dom.text_content().contains("hi"));
    }

    #[test]
    fn test_parse_html_empty() {
        let dom = parse_html("").unwrap();
        assert!(!dom.text_content().contains("error"));
    }

    #[test]
    fn test_extract_links_basic() {
        let links = extract_links(r#"<a href="https://x.com">x</a>"#).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://x.com");
    }

    #[test]
    fn test_extract_links_filters() {
        let links = extract_links(r##"<a href="">e</a><a href="#a">a</a><a href="javascript:">j</a>"##).unwrap();
        assert!(links.is_empty());
    }

    #[test]
    fn test_extract_text_with_body() {
        let text = extract_text("<html><body><p>Hello</p></body></html>");
        assert!(text.contains("Hello"));
    }

    #[test]
    fn test_extract_text_no_body() {
        let text = extract_text("<p>bare</p>");
        assert!(text.contains("bare"));
    }

    #[test]
    fn test_extract_text_empty() {
        let text = extract_text("");
        assert!(text.is_empty() || text.trim().is_empty());
    }
}
