//! Actions for browser automation in Render mode
//!
//! Provides interactive actions like click, type, wait-for-selector


/// Click action - click on an element by selector
pub struct ClickAction {
    pub selector: String,
    pub index: usize,
}

impl ClickAction {
    pub fn new(selector: &str) -> Self {
        Self {
            selector: selector.to_string(),
            index: 0,
        }
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
}

/// Type action - type text into an element
pub struct TypeAction {
    pub selector: String,
    pub text: String,
    pub clear_first: bool,
}

impl TypeAction {
    pub fn new(selector: &str, text: &str) -> Self {
        Self {
            selector: selector.to_string(),
            text: text.to_string(),
            clear_first: false,
        }
    }

    pub fn clear_first(mut self) -> Self {
        self.clear_first = true;
        self
    }
}

/// Wait action - wait for selector to appear
pub struct WaitAction {
    pub selector: String,
    pub timeout_ms: u64,
}

impl WaitAction {
    pub fn new(selector: &str) -> Self {
        Self {
            selector: selector.to_string(),
            timeout_ms: 5000,
        }
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

/// Screenshot options
#[derive(Debug, Clone)]
pub struct ScreenshotOptions {
    pub width: u32,
    pub height: u32,
    pub full_page: bool,
}

impl Default for ScreenshotOptions {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            full_page: false,
        }
    }
}

impl ScreenshotOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn full_page(mut self) -> Self {
        self.full_page = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_action_new() {
        let a = ClickAction::new("#btn");
        assert_eq!(a.selector, "#btn");
        assert_eq!(a.index, 0);
    }

    #[test]
    fn test_click_action_with_index() {
        let a = ClickAction::new(".item").with_index(2);
        assert_eq!(a.index, 2);
    }

    #[test]
    fn test_type_action_new() {
        let a = TypeAction::new("#input", "hello");
        assert_eq!(a.selector, "#input");
        assert_eq!(a.text, "hello");
        assert!(!a.clear_first);
    }

    #[test]
    fn test_type_action_clear_first() {
        let a = TypeAction::new("#input", "text").clear_first();
        assert!(a.clear_first);
    }

    #[test]
    fn test_wait_action_new() {
        let a = WaitAction::new(".loaded");
        assert_eq!(a.selector, ".loaded");
        assert_eq!(a.timeout_ms, 5000);
    }

    #[test]
    fn test_wait_action_with_timeout() {
        let a = WaitAction::new(".el").with_timeout(10000);
        assert_eq!(a.timeout_ms, 10000);
    }

    #[test]
    fn test_screenshot_options_default() {
        let o = ScreenshotOptions::default();
        assert_eq!(o.width, 1280);
        assert_eq!(o.height, 720);
        assert!(!o.full_page);
    }

    #[test]
    fn test_screenshot_options_with_size() {
        let o = ScreenshotOptions::new().with_size(800, 600);
        assert_eq!(o.width, 800);
        assert_eq!(o.height, 600);
    }

    #[test]
    fn test_screenshot_options_full_page() {
        let o = ScreenshotOptions::new().full_page();
        assert!(o.full_page);
    }

    #[test]
    fn test_screenshot_options_debug() {
        let o = ScreenshotOptions::new().with_size(100, 200);
        let d = format!("{:?}", o);
        assert!(d.contains("100"));
        assert!(d.contains("200"));
    }
}
