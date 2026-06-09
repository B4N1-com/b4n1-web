//! MCP Server core logic

use crate::chromium::ChromiumBrowser;
use crate::{AgentBrowser, BrowserMode, Result};
use super::types::{McpRequest, McpResponse, ok_response, err_response};
use super::tools;

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
        let needs_launch = match &*guard {
            None => true,
            Some(b) => !b.is_alive(),
        };
        if needs_launch {
            *guard = None;
            if let Some(path) = crate::chromium::find_chromium() {
                match ChromiumBrowser::launch(Some(&path)).await {
                    Ok(browser) => { *guard = Some(browser); }
                    Err(e) => return Err(crate::Error::Mcp(format!("Chrome launch: {}", e))),
                }
            }
        }
        Ok(guard)
    }

    pub async fn handle_request(&self, request: McpRequest) -> McpResponse {
        match request.method.as_str() {
            "tools/list" => McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(tools::list_tools()),
                error: None,
            },
            "tools/call" => self.call_tool(request.id, request.params).await,
            "initialize" => self.initialize(request.id, request.params),
            _ => err_response(request.id, -32601, "Method not found"),
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
                    "version": "0.8.0"
                },
                "capabilities": {
                    "tools": {}
                }
            })),
            error: None,
        }
    }

    async fn call_tool(&self, id: Option<i32>, params: Option<serde_json::Value>) -> McpResponse {
        let params = match params {
            Some(p) => p,
            None => return err_response(id, -32602, "Invalid params"),
        };

        let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let tool_args = params.get("arguments").and_then(|v| v.as_object());

        match tool_name {
            "goto" => {
                let url = tool_args.and_then(|a| a.get("url")).and_then(|v| v.as_str()).unwrap_or("");
                let mode = tool_args.and_then(|a| a.get("mode")).and_then(|v| v.as_str()).unwrap_or("light");
                let wait_for = tool_args.and_then(|a| a.get("wait_for")).and_then(|v| v.as_str());

                let b_mode = match mode {
                    "js" => BrowserMode::Js,
                    "render" => BrowserMode::Render,
                    _ => BrowserMode::Light,
                };
                let browser = AgentBrowser::new(b_mode);

                match browser.goto(url, wait_for).await {
                    Ok(page) => ok_response(id, format!("{}\n\nLinks: {:?}", page.markdown, page.links)),
                    Err(e) => err_response(id, -32603, &format!("Error: {}", e)),
                }
            }
            "click" | "type_text" | "wait_for_selector" | "screenshot" => {
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
                                _ => err_response(id, -32601, "Unknown tool"),
                            }
                        } else {
                            err_response(id, -32603, "Chrome not available")
                        }
                    }
                    Err(e) => err_response(id, -32603, &format!("Chrome error: {}", e)),
                }
            }
            _ => err_response(id, -32601, &format!("Unknown tool: {}", tool_name)),
        }
    }

    pub async fn run(&self) -> Result<()> {
        use tokio::net::TcpListener;
        use tokio::io::AsyncReadExt;
        
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await
            .map_err(|e| crate::Error::Mcp(format!("Failed to bind: {}", e)))?;
            
        tracing::info!("MCP server listening on {}", addr);
        
        loop {
            let (mut socket, _) = listener.accept().await
                .map_err(|e| crate::Error::Mcp(format!("Accept error: {}", e)))?;
                
            let mut buf = [0; 4096];
            let n = socket.read(&mut buf).await
                .map_err(|e| crate::Error::Mcp(format!("Read error: {}", e)))?;
                
            if n == 0 { continue; }
            
            if let Ok(request) = serde_json::from_slice::<McpRequest>(&buf[..n]) {
                let response = self.handle_request(request).await;
                if let Ok(json) = serde_json::to_vec(&response) {
                    use tokio::io::AsyncWriteExt;
                    let _ = socket.write_all(&json).await;
                }
            }
        }
    }

    pub fn run_stdio_sync(&self) -> Result<()> {
        use std::io::{BufRead, Write};
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| crate::Error::Mcp(format!("runtime: {}", e)))?;

        rt.block_on(async {
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
                    let response = self.handle_request(request).await;
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
