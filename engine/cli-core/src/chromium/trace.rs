//! Tracing and recording module
//!
//! Captures screenshots at each step for test debugging and playback.

use crate::Result;
use std::path::PathBuf;

/// Trace entry for a single action
#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub step: usize,
    pub action: String,
    pub url: String,
    pub screenshot: Option<Vec<u8>>,
    pub timestamp: u64,
}

/// Test tracer that captures screenshots at each step
pub struct Tracer {
    entries: Vec<TraceEntry>,
    output_dir: PathBuf,
    step: usize,
    enabled: bool,
}

impl Tracer {
    pub fn new(output_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&output_dir).ok();
        Self { entries: vec![], output_dir, step: 0, enabled: true }
    }

    pub fn enable(&mut self) { self.enabled = true; }
    pub fn disable(&mut self) { self.enabled = false; }

    /// Start tracing
    pub fn start(&mut self) {
        self.entries.clear();
        self.step = 0;
    }

    /// Record an action and optionally take a screenshot
    pub async fn record(&mut self, action: &str, url: &str, screenshot_b64: Option<&str>) -> Result<()> {
        if !self.enabled { return Ok(()); }
        self.step += 1;

        let screenshot = screenshot_b64
            .and_then(|b64| {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.decode(b64).ok()
            });

        let entry = TraceEntry {
            step: self.step,
            action: action.to_string(),
            url: url.to_string(),
            screenshot: screenshot.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        // Save screenshot to disk
        if let Some(ref img) = screenshot {
            let path = self.output_dir.join(format!("step_{:03}_{}.png", self.step, sanitize_filename(action)));
            std::fs::write(&path, img).ok();
        }

        self.entries.push(entry);
        Ok(())
    }

    /// Generate HTML playback report
    pub fn playback_html(&self) -> Result<String> {
        let mut slides = String::new();
        for entry in &self.entries {
            let img_tag = if entry.screenshot.is_some() {
                let path = format!("step_{:03}_{}.png", entry.step, sanitize_filename(&entry.action));
                format!("<img src='{}' style='max-width:100%;border:1px solid #ccc;'/>", path)
            } else {
                String::new()
            };
            slides.push_str(&format!(
                r#"<div class="step">
                    <h3>Step {}: {}</h3>
                    <p><a href='{}'>{}</a></p>
                    {}
                </div>"#,
                entry.step, entry.action, entry.url, entry.url, img_tag
            ));
        }

        Ok(format!(
            r#"<!DOCTYPE html><html><head><title>Trace Playback</title>
            <style>body{{font-family:sans-serif;max-width:800px;margin:auto;padding:20px}}
            .step{{margin:20px 0;padding:15px;border-radius:8px;background:#f5f5f5}}</style>
            <body><h1>Trace Playback</h1>
            <p>{} steps recorded</p>
            {}</body></html>"#,
            self.entries.len(), slides
        ))
    }
}

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect::<String>()
        .chars()
        .take(50)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracer_new() {
        let t = Tracer::new(PathBuf::from("/tmp/b4n1-test-trace"));
        assert!(t.entries.is_empty());
        assert_eq!(t.step, 0);
    }

    #[test]
    fn test_sanitize_filename() {
        let s = sanitize_filename("click #button!");
        assert_eq!(s, "click__button_");
        assert!(s.len() <= 50);
    }

    #[test]
    fn test_trace_entry_creation() {
        let entry = TraceEntry {
            step: 1,
            action: "goto".into(),
            url: "https://example.com".into(),
            screenshot: None,
            timestamp: 1000,
        };
        assert_eq!(entry.step, 1);
        assert_eq!(entry.action, "goto");
    }

    #[test]
    fn test_playback_html_contains_steps() {
        let tracer = Tracer::new(PathBuf::from("/tmp/b4n1-test"));
        let mut t = tracer;
        t.entries.push(TraceEntry {
            step: 1, action: "navigate".into(), url: "https://x.com".into(),
            screenshot: None, timestamp: 1000,
        });
        let html = t.playback_html().unwrap();
        assert!(html.contains("1 steps"));
        assert!(html.contains("navigate"));
    }
}
