//! JavaScript Module - Script extraction from HTML
//!
//! Extracts and analyzes JavaScript from HTML pages.

use crate::Result;
use scraper::Html;

pub struct JsEngine {
    dom: Html,
}

impl JsEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            dom: Html::parse_document(""),
        })
    }

    pub fn set_html(&mut self, html: &str) {
        self.dom = Html::parse_document(html);
    }

    pub fn execute(&self, _code: &str) -> Result<String> {
        Ok("JS execution not yet implemented - use render mode to extract scripts".to_string())
    }

    pub fn run_scripts(&self) -> Result<String> {
        let selector =
            scraper::Selector::parse("script").map_err(|e| crate::Error::Js(e.to_string()))?;

        let mut results = Vec::new();

        for element in self.dom.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                results.push(format!("<script src=\"{}\"> (external - not loaded)", src));
            } else {
                let code = element.text().collect::<String>();
                if !code.trim().is_empty() {
                    let preview = if code.len() > 100 {
                        format!("{}...", &code[..100])
                    } else {
                        code.clone()
                    };
                    results.push(format!("<script>{}</script>", preview));
                }
            }
        }

        if results.is_empty() {
            Ok("No script tags found in document".to_string())
        } else {
            Ok(results.join("\n"))
        }
    }

    pub fn get_dom(&self) -> &Html {
        &self.dom
    }
}

impl Default for JsEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create JS engine")
    }
}

pub struct JsResult {
    pub value: Option<String>,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_engine_new() {
        let engine = JsEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_js_engine_set_html() {
        let mut engine = JsEngine::new().unwrap();
        engine.set_html("<html><body><script>console.log('hi');</script></body></html>");
        assert!(!engine.get_dom().html().is_empty());
    }

    #[test]
    fn test_js_engine_run_scripts_no_scripts() {
        let mut engine = JsEngine::new().unwrap();
        engine.set_html("<html><body><p>No scripts here</p></body></html>");
        let result = engine.run_scripts().unwrap();
        assert_eq!(result, "No script tags found in document");
    }

    #[test]
    fn test_js_engine_run_scripts_with_src() {
        let mut engine = JsEngine::new().unwrap();
        engine.set_html(r#"<html><body><script src="app.js"></script></body></html>"#);
        let result = engine.run_scripts().unwrap();
        assert!(result.contains("app.js"));
        assert!(result.contains("external - not loaded"));
    }

    #[test]
    fn test_js_engine_run_scripts_inline() {
        let mut engine = JsEngine::new().unwrap();
        engine.set_html("<html><body><script>var x = 1;</script></body></html>");
        let result = engine.run_scripts().unwrap();
        assert!(result.contains("var x = 1;"));
    }

    #[test]
    fn test_js_engine_run_scripts_multiple() {
        let mut engine = JsEngine::new().unwrap();
        engine.set_html(
            r#"<html><body>
                <script src="lib.js"></script>
                <script>init();</script>
            </body></html>"#,
        );
        let result = engine.run_scripts().unwrap();
        assert!(result.contains("lib.js"));
        assert!(result.contains("init();"));
    }

    #[test]
    fn test_js_engine_run_scripts_empty() {
        let mut engine = JsEngine::new().unwrap();
        engine.set_html("<html><body><script></script></body></html>");
        let result = engine.run_scripts().unwrap();
        assert_eq!(result, "No script tags found in document");
    }

    #[test]
    fn test_js_engine_execute() {
        let engine = JsEngine::new().unwrap();
        let result = engine.execute("console.log('test')").unwrap();
        assert!(result.contains("not yet implemented"));
    }

    #[test]
    fn test_js_engine_default() {
        let engine = JsEngine::default();
        assert!(engine.get_dom().html().is_empty() || engine.get_dom().html().contains("html"));
    }
}
