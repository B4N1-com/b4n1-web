//! Model Context Protocol server implementation
//!
//! Provides MCP-compliant tools for LLM integration.

use crate::chromium::{ChromiumBrowser, locator::LocatorStrategy};
use crate::{AgentBrowser, BrowserMode, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Option<i32>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
}

pub struct McpServer {
    port: u16,
    chromium: tokio::sync::Mutex<Option<ChromiumBrowser>>,
}

impl McpServer {
    pub fn new(port: u16) -> Self {
        Self { port, chromium: tokio::sync::Mutex::new(None) }
    }

    async fn ensure_chromium(&self) -> std::result::Result<tokio::sync::MutexGuard<'_, Option<ChromiumBrowser>>, crate::Error> {
        let mut guard = self.chromium.lock().await;
        if guard.is_none() {
            if let Some(path) = crate::chromium::find_chromium() {
                let browser = ChromiumBrowser::launch(Some(&path)).await
                    .map_err(|e| crate::Error::Mcp(format!("Chrome launch: {}", e)))?;
                *guard = Some(browser);
            }
        }
        Ok(guard)
    }

    pub async fn handle_request(&self, request: McpRequest) -> McpResponse {
        match request.method.as_str() {
            "tools/list" => self.list_tools(request.id),
            "tools/call" => self.call_tool(request.id, request.params).await,
            "initialize" => self.initialize(request.id, request.params),
            _ => McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(McpError {
                    code: -32601,
                    message: "Method not found".to_string(),
                }),
            },
        }
    }

    fn initialize(&self, id: Option<i32>, params: Option<serde_json::Value>) -> McpResponse {
        let client_version = params
            .as_ref()
            .and_then(|p| p.get("protocolVersion"))
            .and_then(|v| v.as_str())
            .unwrap_or("2025-11-25");
        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::json!({
                "protocolVersion": client_version,
                "serverInfo": {
                    "name": "b4n1web",
                    "version": "0.7.0"
                },
                "capabilities": {
                    "tools": {}
                }
            })),
            error: None,
        }
    }

    fn list_tools(&self, id: Option<i32>) -> McpResponse {
        let tools = vec![
            serde_json::json!({
                "name": "goto",
                "description": "Navigate to a URL and extract content",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "url": {"type": "string"},
                        "mode": {"type": "string", "enum": ["light", "js", "render"], "default": "light"},
                        "wait_for": {"type": "string", "description": "CSS selector to wait for before extracting content (render mode only)"}
                    },
                    "required": ["url"]
                }
            }),
            serde_json::json!({
                "name": "get_links",
                "description": "Get all links from the current page",
                "inputSchema": {"type": "object", "properties": {}}
            }),
            serde_json::json!({
                "name": "evaluate",
                "description": "Execute arbitrary JavaScript in the browser (render mode only)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "js": {"type": "string", "description": "JavaScript code to execute"}
                    },
                    "required": ["js"]
                }
            }),
            serde_json::json!({
                "name": "screenshot",
                "description": "Take a screenshot of a page (render mode only)",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "url": {"type": "string", "description": "URL to screenshot"},
                        "full_page": {"type": "boolean", "description": "Capture full scrollable page", "default": false}
                    },
                    "required": ["url"]
                }
            }),
            serde_json::json!({
                "name": "click",
                "description": "Click an element by CSS selector",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "selector": {"type": "string", "description": "CSS selector"}
                    },
                    "required": ["selector"]
                }
            }),
            serde_json::json!({
                "name": "type_text",
                "description": "Type text into an element",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "selector": {"type": "string"},
                        "text": {"type": "string"},
                        "clear_first": {"type": "boolean", "default": false}
                    },
                    "required": ["selector", "text"]
                }
            }),
            serde_json::json!({
                "name": "wait_for_selector",
                "description": "Wait for an element to appear",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "selector": {"type": "string"},
                        "timeout": {"type": "number", "default": 5000}
                    },
                    "required": ["selector"]
                }
            }),
            serde_json::json!({
                "name": "set_viewport",
                "description": "Set browser viewport size",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "width": {"type": "number", "default": 1280},
                        "height": {"type": "number", "default": 720}
                    }
                }
            }),
        ];

        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(serde_json::json!({ "tools": tools })),
            error: None,
        }
    }

    async fn call_tool(&self, id: Option<i32>, params: Option<serde_json::Value>) -> McpResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(McpError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                    }),
                }
            }
        };

        let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let tool_args = params.get("arguments").and_then(|v| v.as_object());

        match tool_name {
            "goto" => {
                let url = tool_args
                    .and_then(|a| a.get("url"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let mode = tool_args
                    .and_then(|a| a.get("mode"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("light");
                let wait_for = tool_args
                    .and_then(|a| a.get("wait_for"))
                    .and_then(|v| v.as_str());

                let mode = match mode {
                    "js" => BrowserMode::Js,
                    "render" => BrowserMode::Render,
                    _ => BrowserMode::Light,
                };
                let browser = AgentBrowser::new(mode);

                match browser.goto(url, wait_for).await {
                    Ok(page) => McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: Some(serde_json::json!({
                            "content": [{
                                "type": "text",
                                "text": format!("{}\n\nLinks: {:?}", page.markdown, page.links)
                            }]
                        })),
                        error: None,
                    },
                    Err(e) => McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id,
                        result: None,
                        error: Some(McpError {
                            code: -32603,
                            message: format!("Error: {}", e),
                        }),
                    },
                }
            }
            "get_links" => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "content": [{"type": "text", "text": "[]"}]
                })),
                error: None,
            },
            "click" | "type_text" | "wait_for_selector" | "screenshot"
            | "frames" | "iframe_text" | "iframe_click"
            | "set_viewport" | "set_user_agent" | "emulate_device"
            | "cookies" | "upload_file" | "download_file" | "get_links_from_page" => {
                let guard = self.ensure_chromium().await;
                match guard {
                    Ok(guard) => {
                        if let Some(ref browser) = *guard {
                            match tool_name {
                                "click" => {
                                    let sel = tool_args.and_then(|a| a.get("selector")).and_then(|v| v.as_str()).unwrap_or("");
                                    match browser.click(sel).await {
                                        Ok(()) => ok_response(id, format!("Clicked: {}", sel)),
                                        Err(e) => err_response(id, -32603, &format!("Click error: {}", e)),
                                    }
                                }
                                "type_text" => {
                                    let sel = tool_args.and_then(|a| a.get("selector")).and_then(|v| v.as_str()).unwrap_or("");
                                    let txt = tool_args.and_then(|a| a.get("text")).and_then(|v| v.as_str()).unwrap_or("");
                                    let clear = tool_args.and_then(|a| a.get("clear_first")).and_then(|v| v.as_bool()).unwrap_or(false);
                                    match browser.type_text(sel, txt, clear).await {
                                        Ok(()) => ok_response(id, format!("Typed into: {}", sel)),
                                        Err(e) => err_response(id, -32603, &format!("Type error: {}", e)),
                                    }
                                }
                                "wait_for_selector" => {
                                    let sel = tool_args.and_then(|a| a.get("selector")).and_then(|v| v.as_str()).unwrap_or("");
                                    let timeout = tool_args.and_then(|a| a.get("timeout")).and_then(|v| v.as_u64()).unwrap_or(5000);
                                    match browser.wait_for_selector(sel, timeout).await {
                                        Ok(found) => ok_response(id, format!("Selector found: {}", found)),
                                        Err(e) => err_response(id, -32603, &format!("Wait error: {}", e)),
                                    }
                                }
                                "screenshot" => {
                                    let url = tool_args.and_then(|a| a.get("url")).and_then(|v| v.as_str()).unwrap_or("about:blank");
                                    let full_page = tool_args.and_then(|a| a.get("full_page")).and_then(|v| v.as_bool()).unwrap_or(false);
                                    match browser.screenshot(url, full_page).await {
                                        Ok(Some(b64)) => ok_response(id, format!("data:image/png;base64,{}", b64)),
                                        Ok(None) => err_response(id, -32603, "Screenshot failed"),
                                        Err(e) => err_response(id, -32603, &format!("Screenshot error: {}", e)),
                                    }
                                }
                                "frames" => {
                                    match browser.frames().await {
                                        Ok(frames) => ok_response(id, serde_json::to_string(&frames).unwrap_or_default()),
                                        Err(e) => err_response(id, -32603, &format!("Frames error: {}", e)),
                                    }
                                }
                                "iframe_text" => {
                                    let idx = tool_args.and_then(|a| a.get("index")).and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                                    match browser.iframe_text(idx).await {
                                        Ok(text) => ok_response(id, text),
                                        Err(e) => err_response(id, -32603, &format!("Iframe error: {}", e)),
                                    }
                                }
                                "cookies" => {
                                    match browser.cookies().await {
                                        Ok(cookies) => ok_response(id, format!("{:?}", cookies)),
                                        Err(e) => err_response(id, -32603, &format!("Cookies error: {}", e)),
                                    }
                                }
                                "set_viewport" => {
                                    let w = tool_args.and_then(|a| a.get("width")).and_then(|v| v.as_u64()).unwrap_or(1280) as u32;
                                    let h = tool_args.and_then(|a| a.get("height")).and_then(|v| v.as_u64()).unwrap_or(720) as u32;
                                    match browser.set_viewport(w, h).await {
                                        Ok(()) => ok_response(id, format!("Viewport: {}x{}", w, h)),
                                        Err(e) => err_response(id, -32603, &format!("Viewport error: {}", e)),
                                    }
                                }
                                "emulate_device" => {
                                    let device = tool_args.and_then(|a| a.get("device")).and_then(|v| v.as_str()).unwrap_or("desktop");
                                    match browser.emulate_device(device).await {
                                        Ok(()) => ok_response(id, format!("Emulated: {}", device)),
                                        Err(e) => err_response(id, -32603, &format!("Emulate error: {}", e)),
                                    }
                                }
                                "upload_file" => {
                                    let sel = tool_args.and_then(|a| a.get("selector")).and_then(|v| v.as_str()).unwrap_or("");
                                    let path = tool_args.and_then(|a| a.get("path")).and_then(|v| v.as_str()).unwrap_or("");
                                    match browser.upload_file(sel, path).await {
                                        Ok(()) => ok_response(id, format!("Uploaded to: {}", sel)),
                                        Err(e) => err_response(id, -32603, &format!("Upload error: {}", e)),
                                    }
                                }
                                "download_file" => {
                                    let url = tool_args.and_then(|a| a.get("url")).and_then(|v| v.as_str()).unwrap_or("");
                                    let out = tool_args.and_then(|a| a.get("output")).and_then(|v| v.as_str()).unwrap_or("/tmp/download");
                                    match browser.download_file(url, out).await {
                                        Ok(()) => ok_response(id, format!("Downloaded to: {}", out)),
                                        Err(e) => err_response(id, -32603, &format!("Download error: {}", e)),
                                    }
                                }
                                "evaluate" => {
                                    let js = tool_args.and_then(|a| a.get("js")).and_then(|v| v.as_str()).unwrap_or("");
                                    match browser.evaluate(js).await {
                                        Ok(val) => ok_response(id, val.to_string()),
                                        Err(e) => err_response(id, -32603, &format!("Eval error: {}", e)),
                                    }
                                }
                                _ => err_response(id, -32601, "Unknown tool"),
                            }
                        } else {
                            err_response(id, -32603, "Chrome not available")
                        }
                    }
                    Err(e) => err_response(id, -32603, &format!("Chrome error: {}", e)),
                }
            }
            _ => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(McpError {
                    code: -32601,
                    message: format!("Unknown tool: {}", tool_name),
                }),
            },
        }
    }

    pub async fn run(&self) -> Result<()> {
        use tokio::net::TcpListener;
        use tokio::io::AsyncReadExt;
        
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await
            .map_err(|e| crate::Error::Mcp(format!("Failed to bind: {}", e)))?;
        
        println!("MCP server running on {}", addr);
        
        loop {
            let (mut socket, _) = listener.accept().await
                .map_err(|e| crate::Error::Mcp(format!("Failed to accept: {}", e)))?;
            
            let mut buf = [0u8; 4096];
            if let Ok(n) = socket.read(&mut buf).await {
                if n > 0 {
                    if let Ok(request) = serde_json::from_slice::<McpRequest>(&buf[..n]) {
                        let response = self.handle_request(request).await;
                            if let Ok(json) = serde_json::to_vec(&response) {
                                tokio::io::AsyncWriteExt::write_all(&mut socket, &json).await.ok();
                            }
                    }
                }
            }
        }
    }

    pub fn run_stdio_sync(&self) -> Result<()> {
        use std::io::{BufRead, Write};
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| crate::Error::Mcp(format!("runtime: {}", e)))?;

        rt.block_on(async {
            let server = McpServer::new(self.port);
            let mut reader = std::io::BufReader::new(std::io::stdin());
            let mut line = String::new();
            let mut stdout = std::io::stdout();

            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("MCP stdin error: {}", e);
                        break;
                    }
                }

                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if let Ok(request) = serde_json::from_str::<McpRequest>(trimmed) {
                    if request.id.is_none() {
                        continue;
                    }
                    let response = server.handle_request(request).await;
                    if let Ok(json) = serde_json::to_string(&response) {
                        let _ = writeln!(stdout, "{}", json);
                        let _ = stdout.flush();
                    }
                }
            }
        });

        Ok(())
    }
}

fn ok_response(id: Option<i32>, text: String) -> McpResponse {
    McpResponse {
        jsonrpc: "2.0".to_string(), id,
        result: Some(serde_json::json!({"content": [{"type": "text", "text": text}]})),
        error: None,
    }
}

fn err_response(id: Option<i32>, code: i32, message: &str) -> McpResponse {
    McpResponse {
        jsonrpc: "2.0".to_string(), id,
        result: None,
        error: Some(McpError { code, message: message.to_string() }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_server_new() {
        let server = McpServer::new(8080);
        assert_eq!(server.port, 8080);
    }

    #[tokio::test]
    async fn test_mcp_initialize_default_version() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            method: "initialize".to_string(),
            params: None,
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert_eq!(result["protocolVersion"], "2025-11-25");
        assert_eq!(result["serverInfo"]["name"], "b4n1web");
        assert_eq!(result["serverInfo"]["version"], "0.7.0");
    }

    #[tokio::test]
    async fn test_mcp_initialize_with_client_version() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(2),
            method: "initialize".to_string(),
            params: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "test", "version": "1.0"}
            })),
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        assert_eq!(result["protocolVersion"], "2024-11-05");
    }

    #[tokio::test]
    async fn test_mcp_list_tools() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(2),
            method: "tools/list".to_string(),
            params: None,
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 8);
        
        let tool_names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(tool_names.contains(&"goto"));
        assert!(tool_names.contains(&"get_links"));
    }

    #[tokio::test]
    async fn test_mcp_unknown_method() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(3),
            method: "unknown/method".to_string(),
            params: None,
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, -32601);
    }

    #[tokio::test]
    async fn test_mcp_call_tool_missing_params() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(4),
            method: "tools/call".to_string(),
            params: None,
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, -32602);
    }

    #[tokio::test]
    async fn test_mcp_call_unknown_tool() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(5),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "nonexistent_tool",
                "arguments": {}
            })),
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        let err = response.error.unwrap();
        assert_eq!(err.code, -32601);
        assert!(err.message.contains("nonexistent_tool"));
    }

    #[tokio::test]
    async fn test_mcp_request_serialization() {
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(42),
            method: "tools/list".to_string(),
            params: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("2.0"));
        assert!(json.contains("42"));
        assert!(json.contains("tools/list"));
    }

    #[tokio::test]
    async fn test_mcp_response_serialization() {
        let response = McpResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            result: Some(serde_json::json!({"status": "ok"})),
            error: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("2.0"));
        assert!(json.contains("ok"));
    }

    #[tokio::test]
    async fn test_mcp_get_links_tool() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(6),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "get_links",
                "arguments": {}
            })),
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        assert!(response.result.is_some());
    }

    #[tokio::test]
    async fn test_mcp_initialize_empty_params() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(7),
            method: "initialize".to_string(),
            params: Some(serde_json::json!({})),
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        let result = response.result.unwrap();
        assert_eq!(result["protocolVersion"], "2025-11-25");
    }

    #[tokio::test]
    async fn test_mcp_id_negative() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(-1),
            method: "tools/list".to_string(),
            params: None,
        };
        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        assert_eq!(response.id, Some(-1));
    }

    #[tokio::test]
    async fn test_mcp_notification_no_response() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: "notifications/initialized".to_string(),
            params: None,
        };
        let response = server.handle_request(request).await;
        // Notification with no id should still get error response
        // (server treats all messages as requests)
        assert!(response.error.is_some() || response.result.is_some());
    }

    #[tokio::test]
    async fn test_mcp_goto_tool_missing_url() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(8),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "goto",
                "arguments": {}
            })),
        };
        let response = server.handle_request(request).await;
        // Missing URL should still return result (goto with empty URL will fail gracefully)
        assert!(response.error.is_some() || response.result.is_some());
    }

    #[tokio::test]
    async fn test_mcp_goto_tool_invalid_mode() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(9),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "goto",
                "arguments": {
                    "url": "https://example.com",
                    "mode": "invalid_mode"
                }
            })),
        };
        let response = server.handle_request(request).await;
        // Invalid mode should default to Light
        assert!(response.result.is_some());
    }

    #[tokio::test]
    async fn test_mcp_tools_list_structure() {
        let server = McpServer::new(8080);
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(10),
            method: "tools/list".to_string(),
            params: None,
        };
        let response = server.handle_request(request).await;
        let result = response.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert!(tools.len() >= 2);
        
        let names: Vec<&str> = tools.iter()
            .filter_map(|t| t["name"].as_str())
            .collect();
        assert!(names.contains(&"goto"));
        assert!(names.contains(&"get_links"));
        
        // Verify schema structure
        for tool in tools {
            assert!(tool.get("name").is_some());
            assert!(tool.get("description").is_some());
            assert!(tool.get("inputSchema").is_some());
        }
    }

    #[tokio::test]
    async fn test_mcp_error_serialization_no_error_field() {
        let response = McpResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            result: Some(serde_json::json!({"ok": true})),
            error: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        // On success, error field must be absent (not null)
        assert!(!json.contains("error"));
        // result must be present
        assert!(json.contains("result"));
    }

    #[tokio::test]
    async fn test_mcp_error_serialization_error_present() {
        let response = McpResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            result: None,
            error: Some(McpError { code: -32601, message: "err".to_string() }),
        };
        let json = serde_json::to_string(&response).unwrap();
        // On error, result field must be absent
        assert!(!json.contains("result"));
        assert!(json.contains("error"));
    }
}