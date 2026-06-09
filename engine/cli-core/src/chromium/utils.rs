//! Utility functions for HTML processing and content extraction

use serde::{Deserialize, Serialize};

/// Iframe information extracted from a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IframeInfo {
    pub index: usize,
    pub src: String,
    pub id: String,
    pub name: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

/// Extract all links from HTML content
pub fn extract_links_from_html(html: &str) -> Vec<String> {
    let mut links = Vec::new();
    let re = regex_lite::Regex::new(r#"href=["']([^"']*)["']"#).ok();
    if let Some(re) = re {
        for cap in re.captures_iter(html) {
            let link = cap[1].to_string();
            if !link.is_empty() && !link.starts_with('#') && !link.starts_with("javascript:") {
                links.push(link);
            }
        }
    }
    links
}

/// Convert HTML content to a clean Markdown representation
pub fn html_to_markdown(html: &str) -> String {
    let mut md = html.to_string();
    
    // Simple regex-based cleaning (in a real browser we use more complex logic)
    // Strip scripts
    let re = regex_lite::Regex::new(r"(?s)<script.*?>.*?</script>").ok();
    if let Some(re) = re {
        md = re.replace_all(&md, "").to_string();
    }
    // Decode HTML entities
    md = md.replace("&amp;", "&")
           .replace("&lt;", "<")
           .replace("&gt;", ">")
           .replace("&quot;", "\"")
           .replace("&#39;", "'");
    // Strip HTML tags
    let re = regex_lite::Regex::new(r"<[^>]*>").ok();
    if let Some(re) = re {
        md = re.replace_all(&md, "").to_string();
    }
    // Collapse whitespace
    let re = regex_lite::Regex::new(r"\s+").ok();
    if let Some(re) = re {
        md = re.replace_all(&md, " ").to_string();
    }
    md.trim().to_string()
}

/// Add human-like delay (async)
pub async fn human_delay(min_ms: u64, max_ms: u64) {
    let ms = min_ms + (rand_nanos() % (max_ms - min_ms + 1));
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

pub fn rand_nanos() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
}
