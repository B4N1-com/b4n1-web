//! CSS Styles - Style computation and cascade
//!
//! TODO: Implement CSS cascade, computed styles, and inline/style tag parsing

use std::collections::HashMap;

/// Computed styles for an element
#[derive(Debug, Clone, Default)]
pub struct ComputedStyles {
    pub display: String,
    pub position: String,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub margin: HashMap<String, f32>,
    pub padding: HashMap<String, f32>,
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub font_size: Option<String>,
    pub font_family: Option<String>,
    pub border: HashMap<String, String>,
}

impl ComputedStyles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_element(_element: &super::super::dom::node::Element) -> Self {
        // TODO: Parse inline style attribute
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computed_styles_default() {
        let styles = ComputedStyles::new();
        assert_eq!(styles.display, "");
        assert_eq!(styles.position, "");
        assert!(styles.width.is_none());
        assert!(styles.height.is_none());
        assert!(styles.margin.is_empty());
        assert!(styles.padding.is_empty());
        assert!(styles.color.is_none());
        assert!(styles.background_color.is_none());
        assert!(styles.font_size.is_none());
        assert!(styles.font_family.is_none());
        assert!(styles.border.is_empty());
    }

    #[test]
    fn test_computed_styles_default_trait() {
        let styles = ComputedStyles::default();
        assert_eq!(styles.display, "");
    }

    #[test]
    fn test_computed_styles_clone() {
        let styles = ComputedStyles::new();
        let cloned = styles.clone();
        assert_eq!(cloned.display, styles.display);
    }
}
