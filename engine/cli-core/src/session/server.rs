//! Session manager core logic implementation

use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use chromiumoxide::browser::BrowserConfig;
use futures_util::StreamExt;

use super::types::{Session, SessionKind, State};
use crate::chromium::{find_chromium, utils};

/// Global session limits
pub const MAX_SESSIONS: usize = 10;
pub const INACTIVITY_TIMEOUT_SECS: u64 = 600; // 10 min

static STATE: OnceLock<Mutex<State>> = OnceLock::new();
pub fn state() -> &'static Mutex<State> {
    STATE.get_or_init(|| Mutex::new(State { sessions: HashMap::new() }))
}

static BROWSER: OnceLock<Mutex<Option<chromiumoxide::browser::Browser>>> = OnceLock::new();
pub fn shared_browser() -> &'static Mutex<Option<chromiumoxide::browser::Browser>> {
    BROWSER.get_or_init(|| Mutex::new(None))
}

async fn get_or_launch_browser() -> Result<(), String> {
    let mut guard = shared_browser().lock().await;
    if guard.is_none() {
        let chrome = find_chromium().ok_or_else(|| "Chrome not found".to_string())?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let tmp_dir = std::env::temp_dir().join(format!("b4n1web-shared-{}", now));
        let _ = std::fs::create_dir_all(&tmp_dir);
        let config = BrowserConfig::builder()
            .chrome_executable(chrome)
            .no_sandbox()
            .user_data_dir(tmp_dir)
            .args([
                "--headless".to_string(),
                "--disable-gpu".to_string(),
                "--disable-dev-shm-usage".to_string(),
                "--no-first-run".to_string(),
                "--disable-default-apps".to_string(),
            ])
            .build().map_err(|e| format!("Config: {}", e))?;
        let (br, mut handler) = chromiumoxide::browser::Browser::launch(config)
            .await.map_err(|e| format!("Launch: {}", e))?;
        tokio::spawn(async move {
            while handler.next().await.is_some() {}
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
        let oldest = st.sessions.iter()
            .min_by_key(|(_, s)| s.active_at)
            .map(|(k, _)| k.clone());
        if let Some(old) = oldest {
            st.sessions.remove(&old);
        }
    }

    let page = match kind {
        SessionKind::Browser => {
            let chrome = find_chromium().ok_or_else(|| "Chrome not found".to_string())?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            let tmp_dir = std::env::temp_dir().join(format!("b4n1web-sess-{}-{}", now, name));
            let _ = std::fs::create_dir_all(&tmp_dir);
            let config = BrowserConfig::builder()
                .chrome_executable(chrome)
                .no_sandbox()
                .user_data_dir(tmp_dir)
                .args([
                    "--headless".to_string(),
                    "--disable-gpu".to_string(),
                    "--disable-dev-shm-usage".to_string(),
                    "--no-first-run".to_string(),
                    "--disable-default-apps".to_string(),
                ])
                .build().map_err(|e| format!("Config: {}", e))?;
            let (b, mut h) = chromiumoxide::browser::Browser::launch(config)
                .await.map_err(|e| format!("Launch: {}", e))?;
            tokio::spawn(async move {
                while h.next().await.is_some() {}
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

pub fn touch(name: &str) {
    if let Ok(mut st) = state().try_lock() {
        if let Some(s) = st.sessions.get_mut(name) {
            s.active_at = Instant::now();
        }
    }
}

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
    let links = utils::extract_links_from_html(&html);
    let md = utils::html_to_markdown(&html);
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

pub async fn save_state(name: &str, path: &str) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;

    let jsobj = s.page.evaluate(
        r#"JSON.stringify({cookies: document.cookie, localStorage: Object.fromEntries([...Array.from({length: localStorage.length}).map((_,i)=>[localStorage.key(i), localStorage.getItem(i)]) )})"#
    ).await.map_err(|e| format!("Eval: {}", e))?;
    let json_str: String = jsobj.into_value().map_err(|e| format!("Parse: {}", e))?;

    std::fs::write(path, &json_str).map_err(|e| format!("Write {}: {}", path, e))?;
    drop(st);
    Ok(format!("State saved: {}", path))
}

pub async fn load_state(name: &str, path: &str) -> Result<String, String> {
    touch(name);
    let st = state().lock().await;
    let s = st.sessions.get(name).ok_or_else(|| format!("Session '{}' not found", name))?;

    let json_str = std::fs::read_to_string(path).map_err(|e| format!("Read {}: {}", path, e))?;

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
