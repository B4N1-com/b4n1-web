//! Render module - Lightweight rendering for screenshots

pub mod actions;
pub mod screenshot;

pub use actions::{ClickAction, ScreenshotOptions, TypeAction, WaitAction};
pub use screenshot::ScreenshotRenderer;

/// Render session for visual rendering
pub struct RenderSession {
    pub dom: Option<crate::dom::DomTree>,
    pub screenshot_renderer: ScreenshotRenderer,
}

impl RenderSession {
    pub fn new() -> Result<Self, crate::Error> {
        Ok(Self {
            dom: None,
            screenshot_renderer: ScreenshotRenderer::new(),
        })
    }

    pub fn set_dom(&mut self, dom: crate::dom::DomTree) {
        self.dom = Some(dom);
    }

    pub fn screenshot(&mut self, width: u32, height: u32) -> Result<Vec<u8>, crate::Error> {
        let dom = self
            .dom
            .as_ref()
            .ok_or(crate::Error::Render("No DOM set".to_string()))?;
        self.screenshot_renderer.render(dom, width, height)
    }
}

impl Default for RenderSession {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::DomTree;

    #[test]
    fn test_render_session_new() {
        let session = RenderSession::new().unwrap();
        assert!(session.dom.is_none());
    }

    #[test]
    fn test_render_session_default() {
        let session = RenderSession::default();
        assert!(session.dom.is_none());
    }

    #[test]
    fn test_render_session_set_dom() {
        let mut session = RenderSession::new().unwrap();
        let html = "<p>test</p>";
        let dom = DomTree::from_document(scraper::Html::parse_document(html));
        session.set_dom(dom);
        assert!(session.dom.is_some());
    }

    #[test]
    fn test_render_session_screenshot_no_dom() {
        let mut session = RenderSession::new().unwrap();
        let result = session.screenshot(100, 50);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No DOM"));
    }

    #[test]
    fn test_actions_rexported() {
        let _ = ClickAction::new("#btn");
        let _ = TypeAction::new("#in", "t");
        let _ = WaitAction::new(".el");
        let _ = ScreenshotOptions::new();
    }
}
