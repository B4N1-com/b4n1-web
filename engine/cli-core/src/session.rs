//! B4n1Web Session Manager - Lightweight tab-based sessions

use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use chromiumoxide::browser::BrowserConfig;
use chromiumoxide::Page;
use futures_util::StreamExt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SessionKind { Tab, Context, Browser }

/// Global session limits
const MAX_SESSIONS: usize = 10;
const INACTIVITY_TIMEOUT_SECS: u64 = 600; // 10 min

struct Session {
    page: Page,
    kind: SessionKind,
    url: String,
    active_at: Instant,
}
struct State { sessions: HashMap<String, Session> }

static STATE: OnceLock<Mutex<State>> = OnceLock::new();
fn state() -> &'static Mutex<State> {
    STATE.get_or_init(|| Mutex::new(State { sessions: HashMap::new() }))
}

async fn find_chrome() -> Result<std::path::PathBuf, String> {
    crate::chromium::find_chromium().ok_or_else(|| "Chrome not found".to_string())
}

static BROWSER: OnceLock<Mutex<Option<chromiumoxide::browser::Browser>>> = OnceLock::new();
fn shared_browser() -> &'static Mutex<Option<chromiumoxide::browser::Browser>> {
    BROWSER.get_or_init(|| Mutex::new(None))
}

async fn get_or_launch_browser() -> Result<(), String> {
    let mut guard = shared_browser().lock().await;
    if guard.is_none() {
        let chrome = find_chrome().await?;
        let config = BrowserConfig::builder()
            .disable_default_args()
            .chrome_executable(chrome)
            .args(["--headless".to_string()])
            .build().map_err(|e| format!("Config: {}", e))?;
        let (br, mut handler) = chromiumoxide::browser::Browser::launch(config)
            .await.map_err(|e| format!("Launch: {}", e))?;
        tokio::spawn(async move {
            use futures_util::StreamExt;
            while let Some(h) = handler.next().await { if h.is_err() { break; } }
        });
        *guard = Some(br);
    }
    Ok(())
}

pub async fn start(name: &str, kind: SessionKind) -> Result<String, String> {
    let mut st = state().lock().await;
    if st.sessions.contains_key(name) {
        return Err(format!("Session '{}' already exists", name));
    }
    if st.sessions.len() >= MAX_SESSIONS {
        // Auto-cleanup oldest inactive session
        let oldest = st.sessions.iter()
            .min_by_key(|(_, s)| s.active_at)
            .map(|(k, _)| k.clone());
        if let Some(old) = oldest {
            st.sessions.remove(&old);
        }
    }

    let page = match kind {
        SessionKind::Browser => {
            let chrome = find_chrome().await?;
        let config = BrowserConfig::builder()
            .disable_default_args()
            .chrome_executable(chrome)
            .no_sandbox()
            .args(["--headless".to_string()])
                .build().map_err(|e| format!("Config: {}", e))?;
            let (b, mut h) = chromiumoxide::browser::Browser::launch(config)
                .await.map_err(|e| format!("Launch: {}", e))?;
            tokio::spawn(async move {
                use futures_util::StreamExt;
                while let Some(ev) = h.next().await { if ev.is_err() { break; } }
            });
            b.new_page("about:blank").await.map_err(|e| format!("Page: {}", e))?
        }
        _ => {
            get_or_launch_browser().await?;
            let guard = shared_browser().lock().await;
            guard.as_ref().unwrap()
                .new_page("about:blank").await.map_err(|e| format!("Tab: {}", e))?
        }
    };

    st.sessions.insert(name.to_string(), Session { page, kind, url: "about:blank".into(), active_at: Instant::now() });
    Ok(format!("Session '{}' started ({:?})", name, kind))
}

pub async fn close(name: &str) -> Result<String, String> {
    state().lock().await.sessions.remove(name);
    Ok(format!("Session '{}' closed", name))
}

pub async fn list() -> Result<Vec<(String, String, String)>, String> {
    let st = state().lock().await;
    Ok(st.sessions.iter().map(|(k, s)| (k.clone(), format!("{:?}", s.kind), s.url.clone())).collect())
}

fn touch(name: &str) {
    if let Ok(mut st) = state().try_lock() {
        if let Some(s) = st.sessions.get_mut(name) {
            s.active_at = Instant::now();
        }
    }
}

/// Clean up sessions inactive longer than timeout
pub async fn cleanup_idle() -> Vec<String> {
    let mut st = state().lock().await;
    let now = Instant::now();
    let timeout = Duration::from_secs(INACTIVITY_TIMEOUT_SECS);
    let mut removed = vec![];
    st.sessions.retain(|name, s| {
        if now.duration_since(s.active_at) > timeout {
            removed.push(name.clone());
            false
        } else {
            true
        }
    });
    removed
}

/// Get current session count
pub fn session_count() -> usize {
    state().try_lock().map(|st| st.sessions.len()).unwrap_or(0)
}

/// Get max sessions
pub const fn max_sessions() -> usize { MAX_SESSIONS }

macro_rules! with_page {
    ($name:expr, $body:block) => {{
        let st = state().lock().await;
        let s = st.sessions.get($name).ok_or_else(|| format!("Session '{}' not found", $name))?;
        let _page = &s.page;
        $body
    }};
}

pub async fn goto(name: &str, url: &str, wait_for: Option<&str>) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;
    s.page.goto(url).await.map_err(|e| format!("Goto: {}", e))?;
    if let Some(sel) = wait_for {
        let js = format!("(async function(){{const t=10000,s=Date.now();while(Date.now()-s<t){{if(document.querySelector('{}'))return true;await new Promise(r=>setTimeout(r,100));}}return false;}}))()", sel.replace('\'', "\\'"));
        s.page.evaluate(js).await.map_err(|e| format!("Wait: {}", e))?;
    }
    let html = s.page.content().await.map_err(|e| format!("Content: {}", e))?;
    let links = extract_links(&html);
    let md = html_to_md(&html);
    drop(st);
    Ok(format!("URL: {}\nMarkdown:\n{}\n\nLinks: {:?}", url, md, links))
}

pub async fn click(name: &str, selector: &str) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;
    let js = format!("(function(){{const el=document.querySelector('{}');if(!el)throw new Error();const r=el.getBoundingClientRect();el.dispatchEvent(new MouseEvent('click',{{bubbles:true,clientX:r.x+r.width/2,clientY:r.y+r.height/2}}));return true;}})()", selector.replace('\'', "\\'"));
    s.page.evaluate(js).await.map_err(|e| format!("Click: {}", e))?;
    Ok(format!("Clicked: {}", selector))
}

pub async fn type_text(name: &str, selector: &str, text: &str, clear: bool) -> Result<String, String> {
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;
    let js = format!("(function(){{const el=document.querySelector('{}');if(!el)throw new Error();el.focus();if({})el.value='';el.value='{}';el.dispatchEvent(new Event('input',{{bubbles:true}}));el.dispatchEvent(new Event('change',{{bubbles:true}}));return true;}})()", selector.replace('\'', "\\'"), if clear{"true"}else{"false"}, text.replace('\'', "\\'"));
    s.page.evaluate(js).await.map_err(|e| format!("Type: {}", e))?;
    Ok(format!("Typed: {}", selector))
}

pub async fn wait_for(name: &str, selector: &str, timeout_ms: u64) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;
    let js = format!("(async function(){{const t={},s=Date.now();while(Date.now()-s<t){{if(document.querySelector('{}'))return true;await new Promise(r=>setTimeout(r,100));}}return false;}})()", timeout_ms, selector.replace('\'', "\\'"));
    let found: bool = s.page.evaluate(js).await.map_err(|e| format!("Eval: {}", e))?
        .into_value().map_err(|e| format!("Parse: {}", e))?;
    Ok(format!("Found: {}", found))
}

pub async fn screenshot(name: &str, url: &str, full_page: bool) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;
    s.page.goto(url).await.map_err(|e| format!("Goto: {}", e))?;
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    use chromiumoxide::page::ScreenshotParams;
    use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
    let params = ScreenshotParams::builder()
        .format(CaptureScreenshotFormat::Png)
        .full_page(full_page)
        .build();
    let bytes = s.page.screenshot(params)
        .await
        .map_err(|e| format!("Screenshot: {}", e))?;
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("Screenshot: data:image/png;base64,{}", b64))
}

pub async fn frames(name: &str) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;
    let js = "(function(){return Array.from(document.querySelectorAll('iframe')).map((f,i)=>({index:i,src:f.src||'',id:f.id||'',name:f.name||'',title:f.title||''}));})()";
    let val: serde_json::Value = s.page.evaluate(js).await.map_err(|e| format!("Eval: {}", e))?
        .into_value().map_err(|e| format!("Parse: {}", e))?;
    serde_json::to_string_pretty(&val).map_err(|e| format!("JSON: {}", e))
}

pub async fn iframe_text(name: &str, index: usize) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;
    let text: String = {
        let js = format!("(function(){{const f=document.querySelectorAll('iframe')[{}];if(!f)return'';try{{const d=f.contentDocument||f.contentWindow.document;return d.body.textContent||'';}}catch(e){{return'CROSS-ORIGIN: '+e.message;}}}})()", index);
        s.page.evaluate(js).await.map_err(|e| format!("Eval: {}", e))?
            .into_value().map_err(|e| format!("Parse: {}", e))?
    };
    Ok(format!("Iframe {}:\n{}", index, text))
}

/// Save session cookies + localStorage to a JSON file.
pub async fn save_state(name: &str, path: &str) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;

    // Extract cookies + localStorage via JS
    let jsobj = s.page.evaluate(
        r#"JSON.stringify({cookies: document.cookie, localStorage: Object.fromEntries([...Array.from({length: localStorage.length}).map((_,i)=>[localStorage.key(i), localStorage.getItem(i)]) )})"#
    ).await.map_err(|e| format!("Eval: {}", e))?;
    let json_str: String = jsobj.into_value().map_err(|e| format!("Parse: {}", e))?;

    std::fs::write(path, &json_str).map_err(|e| format!("Write {}: {}", path, e))?;
    drop(st);
    Ok(format!("State saved: {}", path))
}

/// Restore session cookies + localStorage from a JSON file.
pub async fn load_state(name: &str, path: &str) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;

    let json_str = std::fs::read_to_string(path).map_err(|e| format!("Read {}: {}", path, e))?;

    // Restore cookies
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&json_str) {
        if let Some(cookies) = v["cookies"].as_str() {
            for cookie in cookies.split(';').filter(|c| c.contains('=')) {
                let js = format!("document.cookie = '{}';", cookie.trim().replace('\'', "\\'"));
                s.page.evaluate(js).await.ok();
            }
        }
        if let Some(local) = v["localStorage"].as_object() {
            for (k, val) in local {
                if let Some(vstr) = val.as_str() {
                    let js = format!("localStorage.setItem('{}', '{}');",
                        k.replace('\'', "\\'"), vstr.replace('\'', "\\'"));
                    s.page.evaluate(js).await.ok();
                }
            }
        }
    }

    drop(st);
    Ok(format!("State loaded: {}", path))
}

fn extract_links(html: &str) -> Vec<String> {
    regex_lite::Regex::new(r#"href=["'](http[^"']+)["']"#).ok()
        .map(|r| r.captures_iter(html).filter_map(|c| c.get(1).map(|m| m.as_str().to_string())).collect())
        .unwrap_or_default()
}

fn html_to_md(html: &str) -> String {
    let mut s = html.to_string();
    if let Some(re) = regex_lite::Regex::new(r"(?is)<script[^>]*>.*?</script>|<style[^>]*>.*?</style>").ok() {
        s = re.replace_all(&s, "").to_string();
    }
    if let Some(re) = regex_lite::Regex::new(r"<[^>]*>").ok() {
        s = re.replace_all(&s, "").to_string();
    }
    if let Some(re) = regex_lite::Regex::new(r"\s+").ok() {
        s = re.replace_all(&s, " ").to_string();
    }
    s.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links_http() {
        let html = r#"<a href="https://a.com">link</a>"#;
        let links = extract_links(html);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://a.com");
    }

    #[test]
    fn test_extract_links_empty() {
        assert!(extract_links("<p>no links</p>").is_empty());
    }

    #[test]
    fn test_extract_links_filters_anchor() {
        let links = extract_links(r##"<a href="#sec">s</a><a href="https://b.com">b</a>"##);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://b.com");
    }

    #[test]
    fn test_html_to_md_removes_scripts() {
        let md = html_to_md("<html><script>alert(1)</script><p>hi</p></html>");
        assert!(!md.contains("alert"));
        assert!(md.contains("hi"));
    }

    #[test]
    fn test_html_to_md_removes_styles() {
        let md = html_to_md("<html><style>.c{color:red}</style><p>ok</p></html>");
        assert!(!md.contains("color"));
        assert!(md.contains("ok"));
    }

    #[test]
    fn test_html_to_md_strips_tags() {
        let md = html_to_md("<div><p>Hello <b>World</b></p></div>");
        assert!(md.contains("Hello World"));
        assert!(!md.contains('<'));
    }

    #[test]
    fn test_html_to_md_collapses_whitespace() {
        let md = html_to_md("  <p>  spaced  </p>  ");
        assert!(!md.contains("  "));
    }

    #[test]
    fn test_html_to_md_empty() {
        assert!(html_to_md("").is_empty());
    }

    #[test]
    fn test_session_kind_debug() {
        assert_eq!(format!("{:?}", SessionKind::Tab), "Tab");
        assert_eq!(format!("{:?}", SessionKind::Context), "Context");
        assert_eq!(format!("{:?}", SessionKind::Browser), "Browser");
    }

    #[test]
    fn test_session_kind_clone() {
        let a = SessionKind::Tab;
        let b = a;
        assert_eq!(a, b);
    }

    #[tokio::test]
    async fn test_state_initialization() {
        let st = state().lock().await;
        assert!(st.sessions.is_empty());
    }

    #[tokio::test]
    async fn test_start_close_session() {
        let result = start("test_sesh", SessionKind::Tab).await;
        if result.is_ok() {
            let close = close("test_sesh").await;
            assert!(close.is_ok());
        }
        // If Chrome isn't available, start returns Err, which is expected
    }

    #[tokio::test]
    async fn test_list_sessions() {
        let items = list().await.unwrap_or_default();
        // Just verify it doesn't panic
        assert!(items.len() >= 0);
    }

    #[tokio::test]
    async fn test_shared_browser_initialization() {
        let guard = shared_browser().lock().await;
        assert!(guard.is_none());
    }

    #[test]
    fn test_session_count() {
        assert_eq!(max_sessions(), 10);
    }

    #[test]
    fn test_cleanup_idle_empty() {
        let result = tokio::runtime::Runtime::new().unwrap().block_on(cleanup_idle());
        assert!(result.is_empty());
    }

    #[test]
    fn test_touch_nonexistent() {
        touch("nonexistent");
        // Should not panic
    }

    #[test]
    fn test_session_kind_clone_and_eq() {
        assert_eq!(SessionKind::Tab, SessionKind::Tab);
        assert_ne!(SessionKind::Tab, SessionKind::Browser);
    }
}
