//! DOM Tree - Top-level container for the DOM
//!
//! Uses scraper::Html directly for DOM operations.

use scraper::{Html, Selector};

#[derive(Clone)]
pub struct DomTree {
    pub document: Html,
}

impl DomTree {
    pub fn new() -> Self {
        Self {
            document: Html::parse_document("<html></html>"),
        }
    }

    pub fn from_document(document: Html) -> Self {
        Self { document }
    }

    pub fn root(&self) -> scraper::ElementRef<'_> {
        self.document.root_element()
    }

    pub fn select(&self, selector: &str) -> Vec<scraper::ElementRef<'_>> {
        Selector::parse(selector)
            .ok()
            .map(|sel| self.document.select(&sel).collect())
            .unwrap_or_default()
    }

    pub fn select_first(&self, selector: &str) -> Option<scraper::ElementRef<'_>> {
        Selector::parse(selector)
            .ok()
            .and_then(|sel| self.document.select(&sel).next())
    }

    pub fn text_content(&self) -> String {
        if let Some(body) = self.select_first("body") {
            body.text().collect::<Vec<_>>().join("\n")
        } else {
            self.document
                .root_element()
                .text()
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    pub fn links(&self) -> Vec<String> {
        self.select("a[href]")
            .into_iter()
            .filter_map(|el| el.value().attr("href"))
            .filter(|h| !h.is_empty() && !h.starts_with('#') && !h.starts_with("javascript:"))
            .map(String::from)
            .collect()
    }

    pub fn images(&self) -> Vec<String> {
        self.select("img[src]")
            .into_iter()
            .filter_map(|el| el.value().attr("src"))
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    }
}

impl Default for DomTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_tree_new() {
        let tree = DomTree::new();
        assert!(!tree.document.html().is_empty());
    }

    #[test]
    fn test_dom_tree_from_document() {
        let html = Html::parse_document("<html><body><p>Hello</p></body></html>");
        let tree = DomTree::from_document(html);
        assert!(tree.text_content().contains("Hello"));
    }

    #[test]
    fn test_dom_tree_text_content() {
        let html = Html::parse_document("<html><body><h1>Title</h1><p>Content</p></body></html>");
        let tree = DomTree::from_document(html);
        let text = tree.text_content();
        assert!(text.contains("Title"));
        assert!(text.contains("Content"));
    }

    #[test]
    fn test_dom_tree_links() {
        let html = Html::parse_document(
            r##"<html><body>
                <a href="https://example.com">Link 1</a>
                <a href="/page">Link 2</a>
                <a href="#anchor">Anchor</a>
                <a href="javascript:void(0)">JS</a>
            </body></html>"##,
        );
        let tree = DomTree::from_document(html);
        let links = tree.links();
        assert_eq!(links.len(), 2);
        assert!(links.contains(&"https://example.com".to_string()));
        assert!(links.contains(&"/page".to_string()));
    }

    #[test]
    fn test_dom_tree_images() {
        let html = Html::parse_document(
            r#"<html><body>
                <img src="/img1.png">
                <img src="https://example.com/img2.jpg">
                <img src="">
            </body></html>"#,
        );
        let tree = DomTree::from_document(html);
        let images = tree.images();
        assert_eq!(images.len(), 2);
        assert!(images.contains(&"/img1.png".to_string()));
        assert!(images.contains(&"https://example.com/img2.jpg".to_string()));
    }

    #[test]
    fn test_dom_tree_select() {
        let html = Html::parse_document(
            r#"<html><body>
                <div class="item">1</div>
                <div class="item">2</div>
                <div class="other">3</div>
            </body></html>"#,
        );
        let tree = DomTree::from_document(html);
        let items = tree.select(".item");
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_dom_tree_select_first() {
        let html =
            Html::parse_document(r#"<html><body><div id="target">found</div></body></html>"#);
        let tree = DomTree::from_document(html);
        let first = tree.select_first("#target");
        assert!(first.is_some());
        assert_eq!(first.unwrap().text().collect::<Vec<_>>().join(""), "found");
    }

    #[test]
    fn test_dom_tree_select_first_not_found() {
        let tree = DomTree::new();
        assert!(tree.select_first("#nonexistent").is_none());
    }
}
