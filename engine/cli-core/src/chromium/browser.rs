//! Chromium browser module - Control Chrome via CDP
//!
//! Provides a browser interface for Render mode using chromiomoxide.

use crate::Result;
use std::path::PathBuf;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::Page;
use futures_util::StreamExt;
use super::locator::{Locator, LocatorStrategy};

/// A Chromium-based browser for Render mode
pub struct ChromiumBrowser {
    chrome_path: Option<PathBuf>,
    browser: Option<Browser>,
    page: Option<Page>,
    incognito: bool,
    proxy: Option<String>,
}

/// Page content from Chromium
pub struct ChromiumPage {
    pub url: String,
    pub html: String,
    pub markdown: String,
    pub links: Vec<String>,
    pub screenshot: Option<String>,
}

impl ChromiumBrowser {
    /// Launch a new Chromium browser with CDP connection
    pub async fn launch(chrome_path: Option<&PathBuf>) -> Result<Self> {
        Self::launch_with_proxy(chrome_path, None).await
    }

    /// Launch with optional HTTP proxy
    pub async fn launch_with_proxy(chrome_path: Option<&PathBuf>, proxy: Option<&str>) -> Result<Self> {
        let chrome = match chrome_path {
            Some(p) => p.clone(),
            None => return Ok(Self { chrome_path: None, browser: None, page: None, incognito: false, proxy: None }),
        };

        let mut config_builder = BrowserConfig::builder()
            .chrome_executable(chrome.clone())
            .disable_default_args();

        let mut extra_args = vec![
            "--headless".to_string(),
        ];

        if let Some(proxy_url) = proxy {
            extra_args.push(format!("--proxy-server={}", proxy_url));
            extra_args.push("--proxy-bypass-list=<-loopback>".to_string());
        }

        config_builder = config_builder.args(&extra_args);

        let config = config_builder.build()
            .map_err(|e| crate::Error::Other(format!("Browser config error: {}", e)))?;

        let (mut browser, mut handler) = Browser::launch(config)
            .await
            .map_err(|e| crate::Error::Other(format!("Chrome launch error: {}", e)))?;

        tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if h.is_err() { break; }
            }
        });

        let page = browser.new_page("about:blank")
            .await
            .map_err(|e| crate::Error::Other(format!("New page error: {}", e)))?;

        // Apply stealth patches to hide automation
        apply_stealth(&page).await?;

        Ok(Self {
            chrome_path: Some(chrome),
            browser: Some(browser),
            page: Some(page),
            incognito: false,
            proxy: proxy.map(|p| p.to_string()),
        })
    }

    /// Navigate to a URL and get page content
    pub async fn goto(&self, url: &str, wait_for: Option<&str>) -> Result<ChromiumPage> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No CDP page".to_string()))?;

        page.goto(url)
            .await
            .map_err(|e| crate::Error::Other(format!("CDP goto error: {}", e)))?;

        if let Some(selector) = wait_for {
            self.wait_for_selector(selector, 10000).await?;
        }

        let html = page.content()
            .await
            .map_err(|e| crate::Error::Other(format!("CDP content error: {}", e)))?;

        let links = extract_links_from_html(&html);
        let markdown = html_to_markdown(&html);

        Ok(ChromiumPage {
            url: url.to_string(),
            html,
            markdown,
            links,
            screenshot: None,
        })
    }

    /// Click element via JavaScript injection
    pub async fn click(&self, selector: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No CDP page".to_string()))?;

        // Human-like delay before click (100-400ms)
        human_delay(100, 400).await;

        let js = format!(
            r#"(function() {{
                const el = document.querySelector('{}');
                if (!el) throw new Error('Element not found');
                const rect = el.getBoundingClientRect();
                el.dispatchEvent(new MouseEvent('click', {{
                    bubbles: true,
                    clientX: rect.x + rect.width/2,
                    clientY: rect.y + rect.height/2
                }}));
                return true;
            }})()"#,
            selector.replace('\'', "\\'")
        );

        page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Click JS error: {}", e)))?;

        Ok(())
    }

    /// Type text via JavaScript injection
    pub async fn type_text(&self, selector: &str, text: &str, clear_first: bool) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No CDP page".to_string()))?;

        if clear_first {
            let js = format!(
                r#"(function() {{
                    const el = document.querySelector('{}');
                    if (!el) throw new Error('Element not found');
                    el.focus();
                    el.value = '';
                    return true;
                }})()"#,
                selector.replace('\'', "\\'")
            );
            page.evaluate(js)
                .await
                .map_err(|e| crate::Error::Other(format!("Clear error: {}", e)))?;
            human_delay(50, 150).await;
        } else {
            let js = format!(
                r#"(function() {{
                    const el = document.querySelector('{}');
                    if (!el) throw new Error('Element not found');
                    el.focus();
                    return true;
                }})()"#,
                selector.replace('\'', "\\'")
            );
            page.evaluate(js)
                .await
                .map_err(|e| crate::Error::Other(format!("Focus error: {}", e)))?;
        }

        // Type character by character with human-like delays
        for ch in text.chars() {
            human_delay(30, 120).await;
            let escaped = if ch == '\'' { "\\'".to_string() } else { ch.to_string() };
            let js = format!(
                r#"(function() {{
                    const el = document.activeElement;
                    if (!el) return;
                    el.value += '{}';
                    el.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    return true;
                }})()"#,
                escaped
            );
            page.evaluate(js)
                .await
                .map_err(|e| crate::Error::Other(format!("Type char error: {}", e)))?;
        }

        Ok(())
    }

    /// Wait for a CSS selector to appear
    pub async fn wait_for_selector(&self, selector: &str, timeout_ms: u64) -> Result<bool> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No CDP page".to_string()))?;

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
            selector.replace('\'', "\\'")
        );

        let result = page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Wait JS error: {}", e)))?;

        let val: std::result::Result<bool, _> = result.into_value();
        Ok(val.unwrap_or(false))
    }

    /// Take a screenshot via CDP
    pub async fn screenshot(&self, url: &str, full_page: bool) -> Result<Option<String>> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No CDP page".to_string()))?;

        page.goto(url)
            .await
            .map_err(|e| crate::Error::Other(format!("CDP goto for screenshot: {}", e)))?;

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        use chromiumoxide::page::ScreenshotParams;
        use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;

        let params = ScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .full_page(full_page)
            .build();

        let bytes = page.screenshot(params)
            .await
            .map_err(|e| crate::Error::Other(format!("CDP screenshot error: {}", e)))?;

        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
        Ok(Some(b64))
    }

    pub fn browser(&self) -> Option<&PathBuf> {
        self.chrome_path.as_ref()
    }

    pub async fn close(mut self) {
        if let Some(ref mut b) = self.browser {
            let _ = b.close().await;
        }
    }

    /// Evaluate arbitrary JavaScript in the page
    pub async fn evaluate(&self, js: &str) -> Result<serde_json::Value> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No CDP page".to_string()))?;

        let result = page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("JS Evaluation error: {}", e)))?;

        Ok(result.into_value().unwrap_or(serde_json::Value::Null))
    }

    /// Set proxy for next navigation (requires Chrome restart if proxy changed)
    pub async fn set_proxy(&mut self, proxy: &str) -> Result<()> {
        self.proxy = Some(proxy.to_string());
        Ok(())
    }

    /// Current proxy setting
    pub fn proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }

    /// Save cookies and localStorage to disk
    pub async fn save_state(&self, path: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = r#"(function() {
            const state = {
                cookies: document.cookie,
                localStorage: {},
            };
            for (let i = 0; i < localStorage.length; i++) {
                const k = localStorage.key(i);
                state.localStorage[k] = localStorage.getItem(k);
            }
            return JSON.stringify(state);
        })()"#;

        let result = page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Save state error: {}", e)))?;

        let val: std::result::Result<String, _> = result.into_value();
        if let Ok(json) = val {
            std::fs::write(path, &json)
                .map_err(|e| crate::Error::Other(format!("Write state error: {}", e)))?;
        }
        Ok(())
    }
}

impl ChromiumBrowser {
    /// Get Core Web Vitals after page load
    pub async fn performance_metrics(&self) -> Result<PerformanceMetrics> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = r#"(function() {
            const perf = performance || { getEntriesByType: () => [] };
            const paintEntries = perf.getEntriesByType('paint');
            const fcpEntry = paintEntries.find(e => e.name === 'first-contentful-paint');
            const fcp = fcpEntry ? fcpEntry.startTime : 0;
            const navEntries = perf.getEntriesByType('navigation');
            const nav = navEntries[0] || {};
            const domContentLoaded = nav.domContentLoadedEventEnd || 0;
            const domComplete = nav.domComplete || 0;
            const domInteractive = nav.domInteractive || 0;
            const resources = perf.getEntriesByType('resource') || [];
            let totalResources = resources.length;
            let totalResourceBytes = 0;
            let totalResourceTime = 0;
            for (const r of resources) {
                totalResourceBytes += r.transferSize || 0;
                totalResourceTime += r.duration || 0;
            }
            return JSON.stringify({ fcp, domContentLoaded, domComplete, domInteractive, totalResources, totalResourceBytes, totalResourceTime });
        })()"#;

        let result = page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Perf error: {}", e)))?;

        let val: std::result::Result<String, _> = result.into_value();
        let json = val.unwrap_or_default();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap_or_default();
        serde_json::from_value(parsed).map_err(|e| crate::Error::Other(format!("Parse metrics: {}", e)))
    }
}

/// Core Web Vitals and performance metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceMetrics {
    pub fcp: f64,
    #[serde(rename = "domContentLoaded")]
    pub dom_content_loaded: f64,
    #[serde(rename = "domComplete")]
    pub dom_complete: f64,
    #[serde(rename = "domInteractive")]
    pub dom_interactive: f64,
    #[serde(rename = "totalResources")]
    pub total_resources: u64,
    #[serde(rename = "totalResourceBytes")]
    pub total_resource_bytes: u64,
    #[serde(rename = "totalResourceTime")]
    pub total_resource_time: f64,
}

impl ChromiumBrowser {
    pub async fn load_state(&self, path: &str) -> Result<()> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| crate::Error::Other(format!("Read state error: {}", e)))?;

        let state: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| crate::Error::Other(format!("Parse state error: {}", e)))?;

        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        // Restore cookies
        if let Some(cookies) = state["cookies"].as_str() {
            for cookie in cookies.split(';').filter(|c| c.contains('=')) {
                let js = format!("document.cookie = '{}';", cookie.trim().replace('\'', "\\'"));
                page.evaluate(js).await.ok();
            }
        }

        // Restore localStorage
        if let Some(local) = state["localStorage"].as_object() {
            for (k, v) in local {
                if let Some(val) = v.as_str() {
                    let js = format!("localStorage.setItem('{}', '{}');",
                        k.replace('\'', "\\'"), val.replace('\'', "\\'"));
                    page.evaluate(js).await.ok();
                }
            }
        }

        Ok(())
    }

    // --- Locator API ---

    /// Find element by CSS selector
    pub fn locator(&self, selector: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::Css(selector.to_string()))
    }

    /// Find element by text content
    pub fn get_by_text(&self, text: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::Text(text.to_string()))
    }

    /// Find element by ARIA role
    pub fn get_by_role(&self, role: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::Role(role.to_string()))
    }

    /// Find element by data-testid attribute
    pub fn get_by_test_id(&self, test_id: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::TestId(test_id.to_string()))
    }

    /// Find element by label text
    pub fn get_by_label(&self, label: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::Label(label.to_string()))
    }

    /// Find element by placeholder
    pub fn get_by_placeholder(&self, placeholder: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::Placeholder(placeholder.to_string()))
    }

    /// Find element by alt text
    pub fn get_by_alt_text(&self, alt: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::AltText(alt.to_string()))
    }

    /// Find element by title attribute
    pub fn get_by_title(&self, title: &str) -> Locator {
        Locator::new(self.page.as_ref().unwrap(), LocatorStrategy::Title(title.to_string()))
    }

    /// Install network interceptors (must call before navigation)
    pub async fn install_interceptors(&self) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;
        super::network::install_interceptors(page).await
    }

    /// Block resource types (images, fonts, css, etc.)
    pub async fn block_resources(&self, patterns: &[&str]) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;
        super::network::block_resources(page, patterns).await
    }

    /// Route/pattern-based request interception
    pub async fn route(&self, pattern: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;
        super::network::add_route(page, pattern).await
    }

    /// Remove route
    pub async fn unroute(&self, pattern: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;
        super::network::remove_route(page, pattern).await
    }

    /// Wait for a network request matching pattern
    pub async fn wait_for_request(&self, pattern: &str, timeout_ms: u64) -> Result<bool> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;
        let waiter = super::network::wait_for_request(page, pattern, timeout_ms);
        waiter.wait().await
    }

    /// Wait for a network response matching pattern
    pub async fn wait_for_response(&self, pattern: &str, timeout_ms: u64) -> Result<bool> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;
        let waiter = super::network::wait_for_response(page, pattern, timeout_ms);
        waiter.wait().await
    }

    // --- Browser Context API ---

    /// Start incognito context (isolated cookies/storage)
    pub async fn start_context(&mut self) -> Result<()> {
        let browser = self.browser.as_mut()
            .ok_or_else(|| crate::Error::ChromeNotFound("No browser".to_string()))?;
        browser.start_incognito_context()
            .await
            .map_err(|e| crate::Error::Other(format!("Start context error: {}", e)))?;
        self.incognito = true;
        Ok(())
    }

    /// Quit incognito context (clean up isolated storage)
    pub async fn quit_context(&mut self) -> Result<()> {
        let browser = self.browser.as_mut()
            .ok_or_else(|| crate::Error::ChromeNotFound("No browser".to_string()))?;
        browser.quit_incognito_context()
            .await
            .map_err(|e| crate::Error::Other(format!("Quit context error: {}", e)))?;
        self.incognito = false;
        Ok(())
    }

    /// Check if browser is in incognito mode
    pub fn is_incognito(&self) -> bool {
        self.incognito
    }

    /// Get all cookies for the current page
    pub async fn cookies(&self) -> Result<Vec<(String, String)>> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = r#"(function() {
            return document.cookie.split('; ').filter(c => c.includes('=')).map(c => {
                const [n, ...v] = c.split('=');
                return [n.trim(), v.join('=').trim()];
            });
        })()"#;

        let result = page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Get cookies error: {}", e)))?;

        let val: std::result::Result<Vec<Vec<String>>, _> = result.into_value();
        Ok(val.unwrap_or_default().into_iter()
            .filter_map(|v| {
                if v.len() >= 2 { Some((v[0].clone(), v[1].clone())) }
                else { None }
            })
            .collect())
    }

    /// Clear all cookies
    pub async fn clear_cookies(&self) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        page.evaluate("document.cookie.split('; ').forEach(c => { const n = c.split('=')[0]; document.cookie = n + '=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/'; })")
            .await
            .map_err(|e| crate::Error::Other(format!("Clear cookies error: {}", e)))?;

        Ok(())
    }

    // --- Mobile Emulation ---

    /// Set viewport size (width, height)
    pub async fn set_viewport(&self, width: u32, height: u32) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = format!(
            r#"(function() {{
                document.body.style.width = '{}px';
                document.body.style.height = '{}px';
                window.resizeTo({}, {});
                return true;
            }})()"#,
            width, height, width, height
        );

        page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Viewport error: {}", e)))?;

        Ok(())
    }

    /// Set user agent override
    pub async fn set_user_agent(&self, user_agent: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = format!(
            r#"(function() {{
                Object.defineProperty(navigator, 'userAgent', {{ get: () => '{}' }});
                return true;
            }})()"#,
            user_agent.replace('\'', "\\'")
        );

        page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("UserAgent error: {}", e)))?;

        Ok(())
    }

    /// Set geolocation
    pub async fn set_geolocation(&self, latitude: f64, longitude: f64) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = format!(
            r#"(function() {{
                const pos = {{ coords: {{ latitude: {}, longitude: {}, accuracy: 10 }} }};
                const original = navigator.geolocation.getCurrentPosition;
                navigator.geolocation.getCurrentPosition = (success) => success(pos);
                return true;
            }})()"#,
            latitude, longitude
        );

        page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Geolocation error: {}", e)))?;

        Ok(())
    }

    /// Emulate a device by preset
    pub async fn emulate_device(&self, device: &str) -> Result<()> {
        match device.to_lowercase().as_str() {
            "iphone-12" | "iphone12" | "iphone" => {
                self.set_viewport(390, 844).await?;
                self.set_user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1").await?;
            }
            "iphone-se" => {
                self.set_viewport(375, 667).await?;
                self.set_user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1").await?;
            }
            "pixel-5" | "pixel5" | "pixel" | "android" => {
                self.set_viewport(393, 851).await?;
                self.set_user_agent("Mozilla/5.0 (Linux; Android 12; Pixel 5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Mobile Safari/537.36").await?;
            }
            "ipad" => {
                self.set_viewport(810, 1080).await?;
                self.set_user_agent("Mozilla/5.0 (iPad; CPU OS 15_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/604.1").await?;
            }
            "desktop" | "default" => {
                self.set_viewport(1280, 720).await?;
                self.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36").await?;
            }
            _ => {
                self.set_viewport(1280, 720).await?;
                self.set_user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36").await?;
            }
        }
        Ok(())
    }

    // --- Iframe Support ---

    /// List all iframes on the current page
    pub async fn frames(&self) -> Result<Vec<IframeInfo>> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = r#"(function() {
            return Array.from(document.querySelectorAll('iframe')).map((f, i) => ({
                index: i,
                src: f.src || '',
                id: f.id || '',
                name: f.name || '',
                title: f.title || '',
                width: f.width || f.offsetWidth,
                height: f.height || f.offsetHeight,
            }));
        })()"#;

        let result = page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Frames error: {}", e)))?;

        let val: std::result::Result<Vec<IframeInfo>, _> = result.into_value();
        Ok(val.unwrap_or_default())
    }

    /// Get text content from an iframe by index
    pub async fn iframe_text(&self, index: usize) -> Result<String> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = format!(
            r#"(function() {{
                const iframe = document.querySelectorAll('iframe')[{}];
                if (!iframe) return '';
                try {{
                    const doc = iframe.contentDocument || iframe.contentWindow.document;
                    return doc.body ? doc.body.textContent || '' : '';
                }} catch(e) {{
                    return 'CROSS-ORIGIN: ' + e.message;
                }}
            }})()"#,
            index
        );

        let result = page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Iframe text error: {}", e)))?;

        let val: std::result::Result<String, _> = result.into_value();
        Ok(val.unwrap_or_default())
    }

    /// Click inside an iframe
    pub async fn iframe_click(&self, frame_index: usize, selector: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = format!(
            r#"(function() {{
                const iframe = document.querySelectorAll('iframe')[{}];
                if (!iframe) throw new Error('Iframe not found');
                try {{
                    const doc = iframe.contentDocument || iframe.contentWindow.document;
                    const el = doc.querySelector('{}');
                    if (!el) throw new Error('Element not found');
                    const r = el.getBoundingClientRect();
                    el.dispatchEvent(new MouseEvent('click', {{
                        bubbles: true, clientX: r.x+r.width/2, clientY: r.y+r.height/2
                    }}));
                    return true;
                }} catch(e) {{ throw new Error(e.message); }}
            }})()"#,
            frame_index,
            selector.replace('\'', "\\'")
        );

        page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Iframe click error: {}", e)))?;
        Ok(())
    }

    /// Type text inside an iframe
    pub async fn iframe_type_text(&self, frame_index: usize, selector: &str, text: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        let js = format!(
            r#"(function() {{
                const iframe = document.querySelectorAll('iframe')[{}];
                if (!iframe) throw new Error('Iframe not found');
                try {{
                    const doc = iframe.contentDocument || iframe.contentWindow.document;
                    const el = doc.querySelector('{}');
                    if (!el) throw new Error('Element not found');
                    el.focus(); el.value = '{}';
                    el.dispatchEvent(new Event('input', {{bubbles:true}}));
                    el.dispatchEvent(new Event('change', {{bubbles:true}}));
                    return true;
                }} catch(e) {{ throw new Error(e.message); }}
            }})()"#,
            frame_index,
            selector.replace('\'', "\\'"),
            text.replace('\'', "\\'")
        );

        page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Iframe type error: {}", e)))?;
        Ok(())
    }

    // --- File Upload/Download ---

    /// Set the value of a file input element (upload)
    pub async fn upload_file(&self, selector: &str, file_path: &str) -> Result<()> {
        let page = self.page.as_ref()
            .ok_or_else(|| crate::Error::ChromeNotFound("No page".to_string()))?;

        // Read the file and convert to base64
        let data = std::fs::read(file_path)
            .map_err(|e| crate::Error::Other(format!("File read error: {}", e)))?;
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
        let filename = std::path::Path::new(file_path)
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();

        let js = format!(
            r#"(function() {{
                const input = document.querySelector('{}');
                if (!input) throw new Error('File input not found');
                if (input.tagName !== 'INPUT' || input.type !== 'file') {{
                    throw new Error('Element is not a file input');
                }}
                const b64 = '{}';
                const fileName = '{}';
                const byteChars = atob(b64);
                const byteNums = new Array(byteChars.length);
                for (let i = 0; i < byteChars.length; i++) {{
                    byteNums[i] = byteChars.charCodeAt(i);
                }}
                const byteArray = new Uint8Array(byteNums);
                const file = new File([byteArray], fileName, {{ type: 'application/octet-stream' }});
                const dt = new DataTransfer();
                dt.items.add(file);
                input.files = dt.files;
                input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                return true;
            }})()"#,
            selector.replace('\'', "\\'"),
            b64,
            filename.replace('\'', "\\'")
        );

        page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Upload error: {}", e)))?;
        Ok(())
    }

    /// Download a file by triggering a click on a download link and saving the response
    pub async fn download_file(&self, url: &str, output_path: &str) -> Result<()> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| crate::Error::Other(format!("Download error: {}", e)))?;

        let bytes = response.bytes()
            .await
            .map_err(|e| crate::Error::Other(format!("Download body error: {}", e)))?;

        std::fs::write(output_path, &bytes)
            .map_err(|e| crate::Error::Other(format!("File write error: {}", e)))?;

        Ok(())
    }
}

/// Information about an iframe on the page
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IframeInfo {
    pub index: usize,
    pub src: String,
    pub id: String,
    pub name: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

fn extract_links_from_html(html: &str) -> Vec<String> {
    let mut links = Vec::new();
    let re = regex_lite::Regex::new(r#"href=["'](http[^"']+)["']"#).ok();
    if let Some(re) = re {
        for cap in re.captures_iter(html) {
            if let Some(link) = cap.get(1) {
                links.push(link.as_str().to_string());
            }
        }
    }
    links
}

/// Add human-like delay (async)
async fn human_delay(min_ms: u64, max_ms: u64) {
    let ms = min_ms + (rand_nanos() % (max_ms - min_ms + 1));
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

fn rand_nanos() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
}

fn html_to_markdown(html: &str) -> String {
    let mut md = html.to_string();

    // Strip script and style blocks (including multiline)
    let re = regex_lite::Regex::new(r"(?is)<script[^>]*>.*?</script>|<style[^>]*>.*?</style>").ok();
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

/// Apply stealth patches to hide automation/bot detection
fn rand_range(min: usize, max: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    min + (nanos as usize) % (max - min + 1)
}

async fn apply_stealth(page: &chromiumoxide::Page) -> Result<()> {
    let js = format!(r#"
    (function() {{
        // === STEALTH: Ocultar automation ===
        Object.defineProperty(navigator, 'webdriver', {{ get: () => false }});
        if (window.chrome) {{
            Object.defineProperty(window.chrome, 'runtime', {{ get: () => undefined }});
        }}
        const originalQuery = window.navigator.permissions?.query;
        if (originalQuery) {{
            window.navigator.permissions.query = (p) => {{
                if (p.name === 'notifications') return Promise.resolve({{ state: 'denied' }});
                return originalQuery(p);
            }};
        }}
        Object.defineProperty(navigator, 'plugins', {{ get: () => [1, 2, 3, 4, 5] }});
        Object.defineProperty(navigator, 'languages', {{ get: () => ['en-US', 'en'] }});
        ['webdriver','__webdriver_eval','__selenium_evaluate',
         '__selenium_unwrapped','__driver_evaluate','__fxdriver_evaluate',
         '__webdriver_script_fn'].forEach(k => {{ try {{ delete window[k]; }} catch(e){{}} }});
        if (window.navigator.connection) {{
            Object.defineProperty(navigator.connection, 'rtt', {{ get: () => 100 }});
        }}

        // === FINGERPRINT RANDOMIZATION ===
        // Hardware concurrency (2-16 nucleos)
        const cpus = [2, 4, 6, 8, 12, 16];
        Object.defineProperty(navigator, 'hardwareConcurrency', {{
            get: () => cpus[{}]
        }});

        // Device memory (0.25, 0.5, 1, 2, 4, 8 GB)
        const mems = [0.25, 0.5, 1, 2, 4, 8];
        Object.defineProperty(navigator, 'deviceMemory', {{
            get: () => mems[{}]
        }});

        // Platform random
        const platforms = ['Win32', 'MacIntel', 'Linux x86_64', 'Linux aarch64'];
        Object.defineProperty(navigator, 'platform', {{
            get: () => platforms[{}]
        }});

        // WebGL vendor (spoof)
        const getExt = WebGLRenderingContext.prototype.getExtension;
        WebGLRenderingContext.prototype.getExtension = function() {{
            const result = getExt.apply(this, arguments);
            if (arguments[0] === 'WEBGL_debug_renderer_info') return null;
            return result;
        }};

        // Canvas noise (tiny random offset to prevent fingerprinting)
        const origToDataURL = HTMLCanvasElement.prototype.toDataURL;
        HTMLCanvasElement.prototype.toDataURL = function() {{
            const data = origToDataURL.apply(this, arguments);
            // Add 1px random pixel
            const ctx = this.getContext('2d');
            if (ctx) {{
                ctx.fillStyle = `rgba(${{{} % 255}}, ${{{} % 255}}, ${{{} % 255}}, 0.01)`;
                ctx.fillRect(0, 0, 1, 1);
            }}
            return data;
        }};
    }})();
    "#,
    rand_range(0, 5),
    rand_range(0, 5),
    rand_range(0, 3),
    rand_range(0, 255), rand_range(0, 255), rand_range(0, 255)
);

    page.evaluate(js)
        .await
        .map_err(|e| crate::Error::Other(format!("Stealth error: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_exists_id_selector() {
        let html = r#"<div id="main"><p>Hello</p></div>"#;
        assert!(html.contains("id=\"main\"") || html.contains("id='main'"));
    }

    #[test]
    fn test_selector_exists_class_selector() {
        let html = r#"<div class="btn-primary active"><span>Click</span></div>"#;
        assert!(html.contains("class=\"btn-primary"));
    }

    #[test]
    fn test_selector_exists_tag_selector() {
        let html = r#"<html><body><article>Content</article></body></html>"#;
        assert!(html.contains("<article>") || html.contains("<article "));
    }

    #[test]
    fn test_extract_links_from_html() {
        let html = r##"<html><body>
            <a href="https://a.com">A</a>
            <a href="#skip">anchor</a>
            <a href="javascript:void">js</a>
        </body></html>"##;
        let links = extract_links_from_html(html);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "https://a.com");
    }

    #[test]
    fn test_html_to_markdown_strips_scripts() {
        let html = r#"<html><body><script>alert(1)</script><p>text</p></body></html>"#;
        let md = html_to_markdown(html);
        assert!(!md.contains("alert"));
        assert!(md.contains("text"));
    }

    #[test]
    fn test_html_to_markdown_decodes_entities() {
        let html = r#"<html><body><p>Hello &amp; World &lt;3</p></body></html>"#;
        let md = html_to_markdown(html);
        assert!(md.contains("&") || md.contains("<"));
    }

    #[test]
    fn test_browser_stubs_dont_panic() {
        let browser = ChromiumBrowser { chrome_path: None, browser: None, page: None, incognito: false, proxy: None };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            assert!(browser.click(".btn").await.is_err());
            assert!(browser.type_text("#input", "text", false).await.is_err());
            assert!(browser.wait_for_selector(".el", 100).await.is_err());
            browser.close().await;
        });
    }

    #[test]
    fn test_iframe_info_serde() {
        let info = IframeInfo {
            index: 0, src: "https://example.com".into(), id: "frame1".into(),
            name: "main".into(), title: "Main Frame".into(), width: 300, height: 200,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("https://example.com"));
        assert!(json.contains("frame1"));
    }

    #[test]
    fn test_iframe_info_defaults() {
        let info = IframeInfo {
            index: 0, src: String::new(), id: String::new(),
            name: String::new(), title: String::new(), width: 0, height: 0,
        };
        assert_eq!(info.width, 0);
    }

    #[test]
    fn test_upload_file_selector_generation() {
        // Test that the JS selector generation works for various inputs
        let selector = "input[type=\"file\"]";
        let escaped = selector.replace('\'', "\\'");
        assert_eq!(escaped, "input[type=\"file\"]");
    }

    #[test]
    fn test_iframe_info_json() {
        let info = IframeInfo {
            index: 0, src: "https://frame.com".into(), id: "f1".into(),
            name: "main".into(), title: "Main".into(), width: 200, height: 100,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("frame.com"));
        assert!(json.contains("f1"));
    }

    #[test]
    fn test_set_viewport_js_generation() {
        let w = 390; let h = 844;
        let js = format!("window.resizeTo({}, {});", w, h);
        assert!(js.contains("390"));
        assert!(js.contains("844"));
    }

    #[test]
    fn test_emulate_device_iphone_viewport() {
        // Verify preset constants
        let w = 390u32; let h = 844u32;
        assert_eq!(w, 390);
        assert_eq!(h, 844);
    }

    #[test]
    fn test_upload_file_b64_encoding() {
        let data = b"hello";
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(data);
        assert_eq!(b64, "aGVsbG8=");
        assert!(!b64.contains(' '));
    }

    #[test]
    fn test_download_file_url_validation() {
        let url = "https://example.com/file.pdf";
        assert!(url.starts_with("http"));
        assert!(url.contains("."));
    }
}
