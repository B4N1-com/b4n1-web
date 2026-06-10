//! Locator API - Playwright-style element location and interaction
//!
//! Provides find-by-text, role, test-id, and selector-based element targeting.

use crate::Result;
use chromiumoxide::Page;

/// Strategies for finding elements
#[derive(Clone, Debug)]
pub enum LocatorStrategy {
    /// CSS selector
    Css(String),
    /// Text content (case-insensitive contains)
    Text(String),
    /// ARIA role
    Role(String),
    /// data-testid attribute
    TestId(String),
    /// Label text (for inputs via associated label or aria-label)
    Label(String),
    /// Title attribute
    Title(String),
    /// Placeholder attribute (for inputs)
    Placeholder(String),
    /// Alt text (for images)
    AltText(String),
}

impl LocatorStrategy {
    fn to_js_selector(&self) -> String {
        match self {
            LocatorStrategy::Css(sel) => sel.clone(),
            LocatorStrategy::Text(text) => {
                format!(
                    r#":is(button, a, label, span, div, h1, h2, h3, h4, h5, h6, p, li, td, th):not(:has(*)):not(input):not(textarea):not(select):not([role])[text()*="{}"]"#,
                    text.replace('"', "\\\"")
                )
            }
            LocatorStrategy::Role(role) => format!("[role=\"{}\"]", role.replace('"', "\\\"")),
            LocatorStrategy::TestId(id) => {
                format!(
                    "[data-testid=\"{}\"],[data-test-id=\"{}\"],[data-test=\"{}\"]",
                    id.replace('"', "\\\""),
                    id.replace('"', "\\\""),
                    id.replace('"', "\\\"")
                )
            }
            LocatorStrategy::Label(label) => {
                format!(
                    r#"//input[@id=//label[contains(normalize-space(.),"{}")]/@for] | //label[contains(normalize-space(.),"{}")]/input | //input[@aria-label="{}"]"#,
                    label.replace('"', "\\\""),
                    label.replace('"', "\\\""),
                    label.replace('"', "\\\"")
                )
            }
            LocatorStrategy::Title(title) => format!("[title=\"{}\"]", title.replace('"', "\\\"")),
            LocatorStrategy::Placeholder(placeholder) => {
                format!("[placeholder=\"{}\"]", placeholder.replace('"', "\\\""))
            }
            LocatorStrategy::AltText(alt) => format!("[alt=\"{}\"]", alt.replace('"', "\\\"")),
        }
    }

    fn to_js_function(&self) -> String {
        match self {
            LocatorStrategy::Text(text) => {
                format!(
                    r#"(function() {{
                        const elements = document.querySelectorAll('button, a, label, span, div, h1, h2, h3, h4, h5, h6, p, li, td, th, [role]');
                        for (const el of elements) {{
                            const text = (el.textContent || '').trim().toLowerCase();
                            if (text.includes('{}')) return el;
                        }}
                        return null;
                    }})()"#,
                    text.to_lowercase().replace('\'', "\\'")
                )
            }
            LocatorStrategy::Role(role) => {
                format!(
                    r#"(function() {{
                        const elements = document.querySelectorAll('[role="{}"]');
                        return elements.length > 0 ? elements[0] : null;
                    }})()"#,
                    role.replace('"', "\\\"")
                )
            }
            _ => format!("document.querySelector('{}')", self.to_js_selector().replace('\'', "\\'")),
        }
    }
}

/// A locator for finding and interacting with elements
pub struct Locator<'a> {
    page: &'a Page,
    strategy: LocatorStrategy,
}

impl<'a> Locator<'a> {
    pub fn new(page: &'a Page, strategy: LocatorStrategy) -> Self {
        Self { page, strategy }
    }

    /// Get the underlying selector string
    pub fn selector(&self) -> String {
        self.strategy.to_js_selector()
    }

    /// Click the located element
    pub async fn click(&self) -> Result<()> {
        let js = format!(
            r#"(function() {{
                const el = {};
                if (!el) throw new Error('Element not found');
                const rect = el.getBoundingClientRect();
                el.dispatchEvent(new MouseEvent('click', {{
                    bubbles: true, clientX: rect.x + rect.width/2, clientY: rect.y + rect.height/2
                }}));
                return true;
            }})()"#,
            self.strategy.to_js_function()
        );

        self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator click error: {}", e)))?;

        Ok(())
    }

    /// Type text into the located element
    pub async fn fill(&self, text: &str) -> Result<()> {
        let js = format!(
            r#"(function() {{
                const el = {};
                if (!el) throw new Error('Element not found');
                el.focus();
                el.value = '';
                el.value = '{}';
                el.dispatchEvent(new Event('input', {{ bubbles: true }}));
                el.dispatchEvent(new Event('change', {{ bubbles: true }}));
                return true;
            }})()"#,
            self.strategy.to_js_function(),
            text.replace('\'', "\\'")
        );

        self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator fill error: {}", e)))?;

        Ok(())
    }

    /// Wait for the element to appear
    pub async fn wait_for(&self, timeout_ms: u64) -> Result<bool> {
        let js = format!(
            r#"(async function() {{
                const timeout = {};
                const start = Date.now();
                while (Date.now() - start < timeout) {{
                    if (document.querySelector('{}')) return true;
                    await new Promise(r => setTimeout(r, 100));
                }}
                return false;
            }})()"#,
            timeout_ms,
            self.strategy.to_js_selector().replace('\'', "\\'")
        );

        let result = self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator wait error: {}", e)))?;

        let val: std::result::Result<bool, _> = result.into_value();
        Ok(val.unwrap_or(false))
    }

    /// Get text content of the element
    pub async fn inner_text(&self) -> Result<String> {
        let js = format!(
            r#"(function() {{
                const el = {};
                if (!el) return '';
                return el.textContent || '';
            }})()"#,
            self.strategy.to_js_function()
        );

        let result = self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator text error: {}", e)))?;

        let val: std::result::Result<String, _> = result.into_value();
        Ok(val.unwrap_or_default())
    }

    /// Get inner HTML
    pub async fn inner_html(&self) -> Result<String> {
        let js = format!(
            r#"(function() {{
                const el = {};
                if (!el) return '';
                return el.innerHTML || '';
            }})()"#,
            self.strategy.to_js_function()
        );

        let result = self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator html error: {}", e)))?;

        let val: std::result::Result<String, _> = result.into_value();
        Ok(val.unwrap_or_default())
    }

    /// Get attribute value
    pub async fn get_attribute(&self, name: &str) -> Result<Option<String>> {
        let js = format!(
            r#"(function() {{
                const el = {};
                if (!el) return null;
                const val = el.getAttribute('{}');
                return val;
            }})()"#,
            self.strategy.to_js_function(),
            name.replace('\'', "\\'")
        );

        let result = self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator attr error: {}", e)))?;

        let val: std::result::Result<Option<String>, _> = result.into_value();
        Ok(val.unwrap_or(None))
    }

    /// Check if element is visible
    pub async fn is_visible(&self) -> Result<bool> {
        let js = format!(
            r#"(function() {{
                const el = {};
                if (!el) return false;
                const style = window.getComputedStyle(el);
                return style.display !== 'none' && style.visibility !== 'hidden' && el.offsetWidth > 0 && el.offsetHeight > 0;
            }})()"#,
            self.strategy.to_js_function()
        );

        let result = self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator visible error: {}", e)))?;

        let val: std::result::Result<bool, _> = result.into_value();
        Ok(val.unwrap_or(false))
    }

    /// Count matching elements
    pub async fn count(&self) -> Result<usize> {
        let js = format!(
            r#"(function() {{
                const elements = document.querySelectorAll('{}');
                return elements.length;
            }})()"#,
            self.strategy.to_js_selector().replace('\'', "\\'")
        );

        let result = self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Locator count error: {}", e)))?;

        let val: std::result::Result<usize, _> = result.into_value();
        Ok(val.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locator_strategy_css() {
        let s = LocatorStrategy::Css(".my-class".into());
        assert_eq!(s.to_js_selector(), ".my-class");
    }

    #[test]
    fn test_locator_strategy_test_id() {
        let s = LocatorStrategy::TestId("submit-btn".into());
        let sel = s.to_js_selector();
        assert!(sel.contains("data-testid"));
        assert!(sel.contains("submit-btn"));
    }

    #[test]
    fn test_locator_strategy_role() {
        let s = LocatorStrategy::Role("button".into());
        assert_eq!(s.to_js_selector(), "[role=\"button\"]");
    }

    #[test]
    fn test_locator_strategy_text() {
        let s = LocatorStrategy::Text("Login".into());
        let sel = s.to_js_selector();
        assert!(sel.contains("text()"));
        assert!(sel.contains("Login"));
    }

    #[test]
    fn test_locator_strategy_placeholder() {
        let s = LocatorStrategy::Placeholder("Email".into());
        assert_eq!(s.to_js_selector(), "[placeholder=\"Email\"]");
    }

    #[test]
    fn test_locator_strategy_alt_text() {
        let s = LocatorStrategy::AltText("logo".into());
        assert_eq!(s.to_js_selector(), "[alt=\"logo\"]");
    }

    #[test]
    fn test_locator_strategy_title() {
        let s = LocatorStrategy::Title("Tooltip".into());
        assert_eq!(s.to_js_selector(), "[title=\"Tooltip\"]");
    }

    #[test]
    fn test_locator_strategy_escapes_quotes() {
        let s = LocatorStrategy::Role("but\"ton".into());
        assert!(s.to_js_selector().contains("\\\""));
    }
}
