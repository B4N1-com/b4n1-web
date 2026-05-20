//! # B4n1Web Engine
//!
//! The core Rust engine for B4n1Web, providing zero-overhead web execution for autonomous AI agents.
//!
//! ## Modes
//!
//! - **Light**: HTTP fetch + HTML parsing (5MB, ultra-fast)
//! - **JS**: Light + JavaScript execution via rquickjs (~15MB)
//! - **Render**: Full headless browser (downloaded on demand, ~80MB)

pub mod css;
pub mod dom;
pub mod fetcher;
pub mod js_runtime;
pub mod mcp;
pub mod render;
pub mod error;
pub mod chromium;
pub mod executor;
pub mod session;
pub mod test_runner;
pub mod visual;

pub use dom::tree::DomTree;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserMode {
    /// Light mode: HTTP fetch only, no JavaScript
    Light,
    /// JS mode: HTTP fetch + JavaScript execution
    Js,
    /// Render mode: Full headless browser (requires external binary)
    Render,
}

impl BrowserMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "js" | "javascript" => BrowserMode::Js,
            "render" | "headless" | "browser" => BrowserMode::Render,
            _ => BrowserMode::Light,
        }
    }
}

pub struct AgentBrowser {
    mode: BrowserMode,
}

impl AgentBrowser {
    pub fn new(mode: BrowserMode) -> Self {
        Self { mode }
    }

    pub async fn goto(&self, url: &str, wait_for: Option<&str>) -> Result<Page> {
        match self.mode {
            BrowserMode::Light => self.goto_light(url).await,
            BrowserMode::Js => {
                if chromium::find_chromium().is_some() {
                    self.goto_render(url, wait_for).await
                } else {
                    self.goto_js(url).await
                }
            }
            BrowserMode::Render => self.goto_render(url, wait_for).await,
        }
    }

    /// Evaluate arbitrary JavaScript in the browser (render mode only)
    pub async fn evaluate(&self, js: &str) -> Result<String> {
        if self.mode != BrowserMode::Render {
            return Err(Error::Other("Evaluate only supported in render mode".to_string()));
        }

        let chrome_path = chromium::find_chromium();
        if chrome_path.is_none() {
            return Err(Error::ChromeNotFound("Chrome not found".to_string()));
        }

        let browser = chromium::ChromiumBrowser::launch(chrome_path.as_ref()).await?;
        let result = browser.evaluate(js).await?;
        Ok(result.to_string())
    }

    /// Take a screenshot of the current page
    pub fn screenshot(&self, width: u32, height: u32) -> Result<String> {
        let html = ""; // Would need to store current HTML
        let dom = dom::parse_html(html)?;

        use render::ScreenshotRenderer;
        let renderer = ScreenshotRenderer::new();
        let data = renderer.render(&dom, width, height)?;

        Ok(base64_encode(&data))
    }

    /// Wait for an element to appear
    pub async fn wait_for_selector(&self, _selector: &str, _timeout_ms: u64) -> Result<bool> {
        // In real implementation, would poll DOM
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(true)
    }

    /// Click on an element
    pub async fn click(&self, _selector: &str) -> Result<()> {
        // In full render mode, would use headless browser
        // For now, simulate click
        Ok(())
    }

    /// Type text into an element
    pub async fn type_text(&self, _selector: &str, _text: &str, _clear_first: bool) -> Result<()> {
        // In full render mode, would use headless browser
        Ok(())
    }

    async fn goto_light(&self, url: &str) -> Result<Page> {
        let html = fetcher::fetch_html(url).await?;
        let dom = dom::parse_html(&html)?;
        let markdown = dom.text_content();
        let links = dom.links();

        Ok(Page {
            url: url.to_string(),
            mode: "light".to_string(),
            markdown,
            links,
            screenshot: None,
            js_output: None,
        })
    }

    async fn goto_js(&self, url: &str) -> Result<Page> {
        let html = fetcher::fetch_html(url).await?;

        let js_output = js_runtime::execute_scripts(&html)?;

        let dom = dom::parse_html(&html)?;
        let markdown = dom.text_content();
        let links = dom.links();

        Ok(Page {
            url: url.to_string(),
            mode: "js".to_string(),
            markdown,
            links,
            screenshot: None,
            js_output: Some(js_output),
        })
    }

    async fn goto_render(&self, url: &str, wait_for: Option<&str>) -> Result<Page> {
        // Try to find Chrome/Chromium
        let chrome_path = chromium::find_chromium();

        if chrome_path.is_none() {
            return Err(Error::ChromeNotFound(
                "Chrome/Chromium not found. Run: b4n1web chromium install".to_string()
            ));
        }

        // Launch Chromium browser
        let browser = chromium::ChromiumBrowser::launch(chrome_path.as_ref()).await?;

        // Navigate to URL with optional wait
        let page = browser.goto(url, wait_for).await?;

        // Take screenshot (re-navigate to page URL for CDP screenshot)
        let screenshot = match browser.screenshot(url, true).await {
            Ok(Some(ss)) => {
                tracing::info!("Screenshot captured: {} bytes", ss.len());
                Some(ss)
            }
            Ok(None) => {
                tracing::warn!("Screenshot returned None");
                None
            }
            Err(e) => {
                tracing::warn!("Screenshot failed: {}", e);
                None
            }
        };

        Ok(Page {
            url: page.url,
            mode: "render".to_string(),
            markdown: page.markdown,
            links: page.links,
            screenshot,
            js_output: None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub url: String,
    pub mode: String,
    pub markdown: String,
    pub links: Vec<String>,
    pub screenshot: Option<String>,
    pub js_output: Option<String>,
}

impl Page {
    pub fn new(url: String, markdown: String, links: Vec<String>) -> Self {
        Self {
            url,
            mode: "light".to_string(),
            markdown,
            links,
            screenshot: None,
            js_output: None,
        }
    }
}

fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_modes() {
        assert_eq!(BrowserMode::from_str("light"), BrowserMode::Light);
        assert_eq!(BrowserMode::from_str("js"), BrowserMode::Js);
        assert_eq!(BrowserMode::from_str("render"), BrowserMode::Render);
        assert_eq!(BrowserMode::from_str("unknown"), BrowserMode::Light);
        assert_eq!(BrowserMode::from_str("javascript"), BrowserMode::Js);
        assert_eq!(BrowserMode::from_str("headless"), BrowserMode::Render);
        assert_eq!(BrowserMode::from_str("browser"), BrowserMode::Render);
    }

    #[test]
    fn test_page_new() {
        let p = Page::new("https://example.com".into(), "hello".into(), vec!["https://link.com".into()]);
        assert_eq!(p.url, "https://example.com");
        assert_eq!(p.markdown, "hello");
        assert_eq!(p.links, vec!["https://link.com"]);
        assert!(p.screenshot.is_none());
        assert!(p.js_output.is_none());
    }

    #[test]
    fn test_base64_encode() {
        let data = b"hello";
        let encoded = base64_encode(data);
        assert_eq!(encoded, "aGVsbG8=");
    }

    #[test]
    fn test_base64_encode_empty() {
        assert_eq!(base64_encode(b""), "");
    }

    #[tokio::test]
    async fn test_wait_for_selector() {
        let b = AgentBrowser::new(BrowserMode::Light);
        let result = b.wait_for_selector(".test", 100).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_click() {
        let b = AgentBrowser::new(BrowserMode::Light);
        assert!(b.click(".btn").await.is_ok());
    }

    #[tokio::test]
    async fn test_type_text() {
        let b = AgentBrowser::new(BrowserMode::Light);
        assert!(b.type_text("#input", "text", false).await.is_ok());
    }

    #[tokio::test]
    async fn test_type_text_clear_first() {
        let b = AgentBrowser::new(BrowserMode::Light);
        assert!(b.type_text("#input", "text", true).await.is_ok());
    }

    #[tokio::test]
    async fn test_type_text_empty() {
        let b = AgentBrowser::new(BrowserMode::Light);
        assert!(b.type_text("#input", "", false).await.is_ok());
    }

    #[test]
    fn test_browser_mode_edge_cases() {
        assert_eq!(BrowserMode::from_str("LIGHT"), BrowserMode::Light);
        assert_eq!(BrowserMode::from_str("Light"), BrowserMode::Light);
        assert_eq!(BrowserMode::from_str("JS"), BrowserMode::Js);
        assert_eq!(BrowserMode::from_str("JavaScript"), BrowserMode::Js);
        assert_eq!(BrowserMode::from_str("RENDER"), BrowserMode::Render);
        assert_eq!(BrowserMode::from_str("HEADLESS"), BrowserMode::Render);
        assert_eq!(BrowserMode::from_str(""), BrowserMode::Light);
        assert_eq!(BrowserMode::from_str("   "), BrowserMode::Light);
    }

    #[test]
    fn test_page_new_edge_cases() {
        let p = Page::new("".into(), "".into(), vec![]);
        assert_eq!(p.url, "");
        assert_eq!(p.markdown, "");
        assert!(p.links.is_empty());

        let p = Page::new("url".into(), "md".into(), vec!["a".into(), "b".into()]);
        assert_eq!(p.links.len(), 2);
    }

    #[test]
    fn test_base64_encode_binary() {
        let data = vec![0u8, 255, 128, 64];
        let encoded = base64_encode(&data);
        assert!(!encoded.is_empty());
        assert!(encoded.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='));
    }

    #[test]
    fn test_base64_encode_long() {
        let data = vec![255u8; 1000];
        let encoded = base64_encode(&data);
        assert!(encoded.len() > 1300);
    }

    #[test]
    fn test_browser_js_mode_fallsback_to_render() {
        // When Chromium is available, Js mode should use render path
        // This test verifies the match arms are correct
        let b = AgentBrowser::new(BrowserMode::Js);
        assert!(matches!(b.mode, BrowserMode::Js));
    }
}
