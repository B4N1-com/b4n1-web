//! JavaScript Runtime Module
//!
//! Extracts and analyzes JavaScript from HTML pages.

use crate::Result;
use scraper::{Html, Selector};

pub fn execute_scripts(html: &str) -> Result<String> {
    let scripts = extract_scripts(html)?;

    if scripts.is_empty() {
        return Ok("No script tags found in document".to_string());
    }

    let inline_count = scripts
        .iter()
        .filter(|s| !s.starts_with("// External"))
        .count();
    let external_count = scripts.len() - inline_count;

    Ok(format!(
        "Found {} script tags ({} inline, {} external)",
        scripts.len(),
        inline_count,
        external_count
    ))
}

fn extract_scripts(html: &str) -> Result<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("script").map_err(|e| crate::Error::Parse(e.to_string()))?;

    let mut scripts = Vec::new();

    for element in document.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            scripts.push(format!("// External script: {}", src));
        } else {
            let code = element.text().collect::<String>();
            if !code.trim().is_empty() {
                let preview = if code.len() > 100 {
                    format!("{}...", &code[..100])
                } else {
                    code
                };
                scripts.push(format!("<script>{}</script>", preview));
            }
        }
    }

    Ok(scripts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_scripts() {
        let html = r#"<html><body>
            <script>const x = 1;</script>
            <script src="external.js"></script>
        </body></html>"#;

        let scripts = extract_scripts(html).unwrap();
        assert_eq!(scripts.len(), 2);
        assert!(scripts[0].contains("const x = 1"));
        assert!(scripts[1].contains("external.js"));
    }

    #[test]
    fn test_extract_scripts_no_scripts() {
        let html = "<html><body><p>no scripts</p></body></html>";
        let scripts = extract_scripts(html).unwrap();
        assert!(scripts.is_empty());
    }

    #[test]
    fn test_extract_scripts_empty_script() {
        let html = "<html><body><script></script></body></html>";
        let scripts = extract_scripts(html).unwrap();
        assert!(scripts.is_empty());
    }

    #[test]
    fn test_extract_scripts_truncation() {
        let long = "x".repeat(200);
        let html = format!("<script>{}</script>", long);
        let scripts = extract_scripts(&html).unwrap();
        assert_eq!(scripts.len(), 1);
        // Content after <script> tag should be truncated
        assert!(scripts[0].len() <= 220);
    }

    #[test]
    fn test_extract_scripts_exact_100_chars() {
        let exact = "x".repeat(100);
        let html = format!("<script>{}</script>", exact);
        let scripts = extract_scripts(&html).unwrap();
        assert_eq!(scripts.len(), 1);
        assert!(!scripts[0].contains("..."));
    }

    #[test]
    fn test_extract_scripts_mixed() {
        let html = r#"<html><body>
            <script>a</script>
            <script src="ext1.js"></script>
            <script>b</script>
            <script src="ext2.js"></script>
        </body></html>"#;
        let scripts = extract_scripts(html).unwrap();
        assert_eq!(scripts.len(), 4);
    }

    #[test]
    fn test_execute_scripts_no_scripts() {
        let result = execute_scripts("<html></html>").unwrap();
        assert!(result.contains("No script tags"));
    }

    #[test]
    fn test_execute_scripts_counts() {
        let html = r#"<script src="a.js"></script><script>b</script>"#;
        let result = execute_scripts(html).unwrap();
        assert!(result.contains("2 script tags"));
        assert!(result.contains("1 inline"));
        assert!(result.contains("1 external"));
    }
}
