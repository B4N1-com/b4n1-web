//! DOM Node types

use std::collections::HashMap;
use std::sync::Arc;

/// Node type in the DOM
#[derive(Debug, Clone)]
pub enum NodeType {
    Document,
    Element(Element),
    Text(Text),
    Comment(String),
    DocumentType(String),
}

/// Text node
#[derive(Debug, Clone)]
pub struct Text {
    pub content: String,
}

/// Element node
#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name: String,
    pub attrs: HashMap<String, String>,
    pub namespace: Option<String>,
}

impl Element {
    pub fn new(tag_name: String) -> Self {
        Self {
            tag_name,
            attrs: HashMap::new(),
            namespace: None,
        }
    }

    pub fn with_attrs(tag_name: String, attrs: Vec<(&str, &str)>) -> Self {
        let attrs = attrs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Self {
            tag_name,
            attrs,
            namespace: None,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.attrs.get("id").map(|s| s.as_str())
    }

    pub fn class_name(&self) -> Option<&str> {
        self.attrs.get("class").map(|s| s.as_str())
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(|s| s.as_str())
    }

    pub fn has_class(&self, class: &str) -> bool {
        self.class_name()
            .map(|c| c.split_whitespace().any(|x| x == class))
            .unwrap_or(false)
    }
}

/// Node data containing the node type and children
#[derive(Debug, Clone)]
pub struct NodeData {
    pub node_type: NodeType,
    pub children: Vec<Arc<Node>>,
}

impl NodeData {
    pub fn new_document() -> Self {
        Self {
            node_type: NodeType::Document,
            children: Vec::new(),
        }
    }

    pub fn new_element(tag: String) -> Self {
        Self {
            node_type: NodeType::Element(Element::new(tag)),
            children: Vec::new(),
        }
    }

    pub fn new_text(content: String) -> Self {
        Self {
            node_type: NodeType::Text(Text { content }),
            children: Vec::new(),
        }
    }

    pub fn new_comment(content: String) -> Self {
        Self {
            node_type: NodeType::Comment(content),
            children: Vec::new(),
        }
    }
}

/// A node in the DOM tree
#[derive(Debug, Clone)]
pub struct Node {
    pub data: NodeData,
    parent: Option<Arc<Node>>,
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self { data, parent: None }
    }

    pub fn with_parent(data: NodeData, parent: Arc<Node>) -> Self {
        Self {
            data,
            parent: Some(parent),
        }
    }

    pub fn parent(&self) -> Option<&Arc<Node>> {
        self.parent.as_ref()
    }

    pub fn is_element(&self) -> bool {
        matches!(self.data.node_type, NodeType::Element(_))
    }

    pub fn is_text(&self) -> bool {
        matches!(self.data.node_type, NodeType::Text(_))
    }

    pub fn as_element(&self) -> Option<&Element> {
        match &self.data.node_type {
            NodeType::Element(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&Text> {
        match &self.data.node_type {
            NodeType::Text(t) => Some(t),
            _ => None,
        }
    }

    pub fn tag_name(&self) -> Option<&str> {
        self.as_element().map(|e| e.tag_name.as_str())
    }

    pub fn text_content(&self) -> String {
        collect_text(self)
    }

    pub fn inner_html(&self) -> String {
        self.children()
            .iter()
            .filter_map(|child| match &child.data.node_type {
                NodeType::Text(t) => Some(t.content.clone()),
                NodeType::Element(e) => Some(format!("<{}>...</{}>", e.tag_name, e.tag_name)),
                _ => None,
            })
            .collect()
    }

    pub fn children(&self) -> &[Arc<Node>] {
        &self.data.children
    }
}

/// Collect all text content from a node and its descendants
fn collect_text(node: &Node) -> String {
    let mut result = String::new();
    if let NodeType::Text(t) = &node.data.node_type {
        result.push_str(&t.content);
    }
    for child in &node.data.children {
        result.push_str(&collect_text(child));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_new() {
        let el = Element::new("div".to_string());
        assert_eq!(el.tag_name, "div");
        assert!(el.attrs.is_empty());
        assert!(el.namespace.is_none());
    }

    #[test]
    fn test_element_with_attrs() {
        let el = Element::with_attrs(
            "a".to_string(),
            vec![("href", "https://example.com"), ("class", "link")],
        );
        assert_eq!(el.tag_name, "a");
        assert_eq!(el.get_attribute("href"), Some("https://example.com"));
        assert_eq!(el.get_attribute("class"), Some("link"));
        assert_eq!(el.get_attribute("id"), None);
    }

    #[test]
    fn test_element_id_and_class() {
        let el = Element::with_attrs(
            "div".to_string(),
            vec![("id", "main"), ("class", "container flex")],
        );
        assert_eq!(el.id(), Some("main"));
        assert_eq!(el.class_name(), Some("container flex"));
        assert!(el.has_class("container"));
        assert!(el.has_class("flex"));
        assert!(!el.has_class("other"));
    }

    #[test]
    fn test_node_data_constructors() {
        let doc = NodeData::new_document();
        assert!(matches!(doc.node_type, NodeType::Document));
        assert!(doc.children.is_empty());

        let el = NodeData::new_element("p".to_string());
        assert!(matches!(el.node_type, NodeType::Element(_)));

        let txt = NodeData::new_text("hello".to_string());
        assert!(matches!(txt.node_type, NodeType::Text(_)));

        let comment = NodeData::new_comment("note".to_string());
        assert!(matches!(comment.node_type, NodeType::Comment(_)));
    }

    #[test]
    fn test_node_type_checks() {
        let el_node = Node::new(NodeData::new_element("span".to_string()));
        assert!(el_node.is_element());
        assert!(!el_node.is_text());
        assert!(el_node.as_element().is_some());
        assert!(el_node.as_text().is_none());
        assert_eq!(el_node.tag_name(), Some("span"));

        let text_node = Node::new(NodeData::new_text("hello".to_string()));
        assert!(!text_node.is_element());
        assert!(text_node.is_text());
        assert!(text_node.as_element().is_none());
        assert!(text_node.as_text().is_some());
        assert!(text_node.tag_name().is_none());
    }

    #[test]
    fn test_text_content_collection() {
        let mut root = NodeData::new_element("div".to_string());
        root.children.push(Arc::new(Node::new(NodeData::new_text(
            "Hello ".to_string(),
        ))));

        let mut child = NodeData::new_element("span".to_string());
        child
            .children
            .push(Arc::new(Node::new(NodeData::new_text("World".to_string()))));
        root.children.push(Arc::new(Node::new(child)));

        let root_node = Node::new(root);
        assert_eq!(root_node.text_content(), "Hello World");
    }

    #[test]
    fn test_inner_html() {
        let mut root = NodeData::new_element("div".to_string());
        root.children
            .push(Arc::new(Node::new(NodeData::new_text("text".to_string()))));
        root.children
            .push(Arc::new(Node::new(NodeData::new_element("p".to_string()))));

        let root_node = Node::new(root);
        let html = root_node.inner_html();
        assert!(html.contains("text"));
        assert!(html.contains("<p>...</p>"));
    }
}
