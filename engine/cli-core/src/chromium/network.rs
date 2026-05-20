//! Network interception and mocking module
//!
//! Provides Playwright-style route() and waitForRequest()/waitForResponse()
//! using JavaScript injection for intercepting fetch/XMLHttpRequest.

use crate::Result;
use chromiumoxide::Page;

/// Route handler: return mocked response or None to pass through
pub enum RouteResult {
    /// Return a mocked response
    Mock { status: u16, headers: Vec<(String, String)>, body: String },
    /// Pass through to network
    Passthrough,
}

/// A pending network request/response wait
pub struct NetworkWait {
    page: Page,
    pattern: String,
    wait_type: NetworkWaitType,
    timeout_ms: u64,
}

enum NetworkWaitType {
    Request,
    Response,
}

impl NetworkWait {
    pub async fn wait(self) -> Result<bool> {
        let js = format!(
            r#"(async function() {{
                const pattern = '{}';
                const timeout = {};
                const start = Date.now();
                return new Promise((resolve) => {{
                    const check = setInterval(() => {{
                        if (Date.now() - start > timeout) {{
                            clearInterval(check);
                            resolve(false);
                        }}
                    }}, 100);
                    const handler = (event) => {{
                        const url = event.detail.url || '';
                        if (url.includes(pattern) || url.match(new RegExp(pattern))) {{
                            clearInterval(check);
                            document.removeEventListener('b4n1-request', handler);
                            resolve(true);
                        }}
                    }};
                    document.addEventListener('b4n1-request', handler);
                }});
            }})()"#,
            self.pattern.replace('\'', "\\'"),
            self.timeout_ms
        );

        let result = self.page.evaluate(js)
            .await
            .map_err(|e| crate::Error::Other(format!("Network wait error: {}", e)))?;

        let val: std::result::Result<bool, _> = result.into_value();
        Ok(val.unwrap_or(false))
    }
}

/// Install network interceptors on a page
pub async fn install_interceptors(page: &Page) -> Result<()> {
    let js = r#"
    (function() {
        if (window.__b4n1_interceptors) return;
        window.__b4n1_interceptors = { routes: [], blocked: [] };

        // Intercept fetch
        const originalFetch = window.fetch;
        window.fetch = async function(...args) {
            const url = typeof args[0] === 'string' ? args[0] : args[0].url;
            const method = args[1]?.method || 'GET';
            
            // Check blocked resources
            for (const block of window.__b4n1_interceptors.blocked) {
                if (url.match(block)) {
                    throw new Error(`Blocked by b4n1web: ${url}`);
                }
            }

            // Dispatch event for waitForRequest
            document.dispatchEvent(new CustomEvent('b4n1-request', { detail: { url, method } }));

            // Check routes
            for (const route of window.__b4n1_interceptors.routes) {
                if (url.match(route.pattern)) {
                    if (route.handler) {
                        const result = route.handler(url, method);
                        if (result) return new Response(result.body, {
                            status: result.status || 200,
                            headers: result.headers || {}
                        });
                    }
                }
            }

            try {
                const response = await originalFetch.apply(this, args);
                document.dispatchEvent(new CustomEvent('b4n1-response', { 
                    detail: { url, status: response.status } 
                }));
                return response;
            } catch(e) {
                throw e;
            }
        };

        // Intercept XMLHttpRequest
        const originalOpen = XMLHttpRequest.prototype.open;
        XMLHttpRequest.prototype.open = function(method, url, ...rest) {
            this._b4n1_url = typeof url === 'string' ? url : url.toString();
            this._b4n1_method = method;
            return originalOpen.apply(this, [method, url, ...rest]);
        };

        const originalSend = XMLHttpRequest.prototype.send;
        XMLHttpRequest.prototype.send = function(...args) {
            const url = this._b4n1_url || '';
            
            document.dispatchEvent(new CustomEvent('b4n1-request', { 
                detail: { url, method: this._b4n1_method } 
            }));

            const origOnLoad = this.onload;
            this.onload = function() {
                document.dispatchEvent(new CustomEvent('b4n1-response', {
                    detail: { url, status: this.status }
                }));
                if (origOnLoad) origOnLoad.call(this);
            };

            return originalSend.apply(this, args);
        };
    })();
    "#;

    page.evaluate(js)
        .await
        .map_err(|e| crate::Error::Other(format!("Interceptors install error: {}", e)))?;

    Ok(())
}

/// Block resource types matching the given pattern
pub async fn block_resources(page: &Page, patterns: &[&str]) -> Result<()> {
    let js = format!(
        r#"(function() {{
            if (!window.__b4n1_interceptors) window.__b4n1_interceptors = {{ routes: [], blocked: [] }};
            const patterns = [{}];
            window.__b4n1_interceptors.blocked.push(...patterns);
        }})()"#,
        patterns.iter()
            .map(|p| format!("/{}/", regex_lite::escape(p)))
            .collect::<Vec<_>>()
            .join(", ")
    );

    page.evaluate(js)
        .await
        .map_err(|e| crate::Error::Other(format!("Block resources error: {}", e)))?;

    Ok(())
}

/// Add a route handler
pub async fn add_route(page: &Page, pattern: &str) -> Result<()> {
    let js = format!(
        r#"(function() {{
            if (!window.__b4n1_interceptors) window.__b4n1_interceptors = {{ routes: [], blocked: [] }};
            window.__b4n1_interceptors.routes.push({{
                pattern: new RegExp('{}'),
                handler: null
            }});
        }})()"#,
        pattern.replace('\'', "\\'")
    );

    page.evaluate(js)
        .await
        .map_err(|e| crate::Error::Other(format!("Route add error: {}", e)))?;

    Ok(())
}

/// Remove a route handler
pub async fn remove_route(page: &Page, pattern: &str) -> Result<()> {
    let js = format!(
        r#"(function() {{
            if (!window.__b4n1_interceptors) return;
            window.__b4n1_interceptors.routes = window.__b4n1_interceptors.routes.filter(
                r => r.pattern.source !== new RegExp('{}').source
            );
        }})()"#,
        pattern.replace('\'', "\\'")
    );

    page.evaluate(js)
        .await
        .map_err(|e| crate::Error::Other(format!("Route remove error: {}", e)))?;

    Ok(())
}

/// Create a network wait for request matching pattern
pub fn wait_for_request(page: &Page, pattern: &str, timeout_ms: u64) -> NetworkWait {
    NetworkWait {
        page: page.clone(),
        pattern: pattern.to_string(),
        wait_type: NetworkWaitType::Request,
        timeout_ms,
    }
}

/// Create a network wait for response matching pattern
pub fn wait_for_response(page: &Page, pattern: &str, timeout_ms: u64) -> NetworkWait {
    NetworkWait {
        page: page.clone(),
        pattern: pattern.to_string(),
        wait_type: NetworkWaitType::Response,
        timeout_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_resources_js_generation() {
        // Test that patterns array is properly generated
        let patterns = vec!["*.jpg", "*.png", "*.gif"];
        let escaped: String = patterns.iter()
            .map(|p| format!("/{}/", regex_lite::escape(p)))
            .collect::<Vec<_>>()
            .join(", ");
        assert!(escaped.contains("jpg"));
        assert!(escaped.contains("png"));
    }

    #[test]
    fn test_network_wait_js_contains_pattern() {
        let js = format!(
            r#"const pattern = '{}';"#,
            "api/data".replace('\'', "\\'")
        );
        assert!(js.contains("api/data"));
    }

    #[test]
    fn test_route_escapes_quotes() {
        let js = format!("new RegExp('{}')", "test'123".replace('\'', "\\'"));
        assert_eq!(js, "new RegExp('test\\'123')");
    }
}
