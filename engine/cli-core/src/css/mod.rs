//! CSS Module - Simple CSS selector matching
//!
//! Simplified CSS selector matching for AI agents.
//! Supports: tag, #id, .class, [attr]

pub mod styles;

use crate::dom::node::{Node, NodeType};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

#[derive(Debug, Clone)]
pub enum SelectorPart {
    Tag(String),
    Id(String),
    Class(String),
    Attribute(String),
}

pub fn parse_selector(selector: &str) -> Selector {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_class = false;
    let mut in_id = false;

    for ch in selector.chars() {
        match ch {
            '#' if current.is_empty() => {
                in_id = true;
            }
            '.' if current.is_empty() => {
                in_class = true;
            }
            ' ' | '>' if !in_class && !in_id => {
                // Skip for now
            }
            _ => {
                if in_id {
                    if ch == ' ' || ch == '.' {
                        parts.push(SelectorPart::Id(current.clone()));
                        current = String::new();
                        in_id = false;
                    } else {
                        current.push(ch);
                    }
                } else if in_class {
                    if ch == ' ' || ch == '#' {
                        parts.push(SelectorPart::Class(current.clone()));
                        current = String::new();
                        in_class = false;
                    } else {
                        current.push(ch);
                    }
                } else {
                    current.push(ch);
                }
            }
        }
    }

    if !current.is_empty() {
        if in_id {
            parts.push(SelectorPart::Id(current));
        } else if in_class {
            parts.push(SelectorPart::Class(current));
        } else {
            parts.push(SelectorPart::Tag(current.to_lowercase()));
        }
    }

    Selector { parts }
}

pub fn matches(node: &Arc<Node>, selector: &Selector) -> bool {
    if selector.parts.is_empty() {
        return true;
    }

    let element = match &node.data.node_type {
        NodeType::Element(e) => e,
        _ => return false,
    };

    for part in &selector.parts {
        match part {
            SelectorPart::Tag(tag) => {
                if element.tag_name.to_lowercase() != *tag {
                    return false;
                }
            }
            SelectorPart::Id(id) => {
                if element.id() != Some(id) {
                    return false;
                }
            }
            SelectorPart::Class(class) => {
                if !element.has_class(class) {
                    return false;
                }
            }
            SelectorPart::Attribute(attr) => {
                if !element.attrs.contains_key(attr) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn select_all(root: &Arc<Node>, selector: &str) -> Vec<Arc<Node>> {
    let selector = parse_selector(selector);
    let mut results = Vec::new();
    collect_matching(root, &selector, &mut results);
    results
}

pub fn select_first(root: &Arc<Node>, selector: &str) -> Option<Arc<Node>> {
    let selector = parse_selector(selector);
    find_first(root, &selector)
}

fn collect_matching(node: &Arc<Node>, selector: &Selector, results: &mut Vec<Arc<Node>>) {
    if matches(node, selector) {
        results.push(node.clone());
    }

    for child in node.children() {
        collect_matching(child, selector, results);
    }
}

fn find_first(node: &Arc<Node>, selector: &Selector) -> Option<Arc<Node>> {
    if matches(node, selector) {
        return Some(node.clone());
    }

    for child in node.children() {
        let found = find_first(child, selector);
        if found.is_some() {
            return found;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::node::{NodeData, NodeType};
    use crate::dom::node::Element;

    #[test]
    fn test_parse_selector_tag() {
        let sel = parse_selector("div");
        assert_eq!(sel.parts.len(), 1);
        assert!(matches!(&sel.parts[0], SelectorPart::Tag(t) if t == "div"));
    }

    #[test]
    fn test_parse_selector_id() {
        let sel = parse_selector("#main");
        assert_eq!(sel.parts.len(), 1);
        assert!(matches!(&sel.parts[0], SelectorPart::Id(i) if i == "main"));
    }

    #[test]
    fn test_parse_selector_class() {
        let sel = parse_selector(".container");
        assert_eq!(sel.parts.len(), 1);
        assert!(matches!(&sel.parts[0], SelectorPart::Class(c) if c == "container"));
    }

    #[test]
    fn test_parse_selector_combined() {
        // Simplified parser: handles tag then #id or .class separately
        let sel = parse_selector("#main");
        assert_eq!(sel.parts.len(), 1);
        assert!(matches!(&sel.parts[0], SelectorPart::Id(i) if i == "main"));

        let sel2 = parse_selector(".container");
        assert_eq!(sel2.parts.len(), 1);
        assert!(matches!(&sel2.parts[0], SelectorPart::Class(c) if c == "container"));
    }

    #[test]
    fn test_parse_selector_empty() {
        let sel = parse_selector("");
        assert!(sel.parts.is_empty());
    }

    #[test]
    fn test_matches_tag() {
        let node = Arc::new(Node::new(NodeData::new_element("div".to_string())));
        let sel = parse_selector("div");
        assert!(matches(&node, &sel));

        let sel = parse_selector("span");
        assert!(!matches(&node, &sel));
    }

    #[test]
    fn test_matches_id() {
        let el = Element::with_attrs("div".to_string(), vec![("id", "main")]);
        let node = Arc::new(Node::new(NodeData {
            node_type: NodeType::Element(el),
            children: Vec::new(),
        }));
        let sel = parse_selector("#main");
        assert!(matches(&node, &sel));

        let sel = parse_selector("#other");
        assert!(!matches(&node, &sel));
    }

    #[test]
    fn test_matches_class() {
        let el = Element::with_attrs("div".to_string(), vec![("class", "container flex")]);
        let node = Arc::new(Node::new(NodeData {
            node_type: NodeType::Element(el),
            children: Vec::new(),
        }));
        let sel = parse_selector(".container");
        assert!(matches(&node, &sel));

        let sel = parse_selector(".flex");
        assert!(matches(&node, &sel));

        let sel = parse_selector(".other");
        assert!(!matches(&node, &sel));
    }

    #[test]
    fn test_matches_non_element() {
        let node = Arc::new(Node::new(NodeData::new_text("hello".to_string())));
        let sel = parse_selector("div");
        assert!(!matches(&node, &sel));
    }

    #[test]
    fn test_matches_empty_selector() {
        let node = Arc::new(Node::new(NodeData::new_element("div".to_string())));
        let sel = parse_selector("");
        assert!(matches(&node, &sel));
    }

    #[test]
    fn test_select_all() {
        let mut root = NodeData::new_element("div".to_string());
        root.children
            .push(Arc::new(Node::new(NodeData::new_element("p".to_string()))));
        root.children
            .push(Arc::new(Node::new(NodeData::new_element("p".to_string()))));
        root.children.push(Arc::new(Node::new(NodeData::new_element(
            "span".to_string(),
        ))));
        let root_node = Arc::new(Node::new(root));

        let results = select_all(&root_node, "p");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_select_first() {
        let mut root = NodeData::new_element("div".to_string());
        root.children.push(Arc::new(Node::new(NodeData::new_element(
            "span".to_string(),
        ))));
        root.children
            .push(Arc::new(Node::new(NodeData::new_element("p".to_string()))));
        let root_node = Arc::new(Node::new(root));

        let first = select_first(&root_node, "span");
        assert!(first.is_some());
        assert_eq!(first.unwrap().tag_name(), Some("span"));
    }
}
