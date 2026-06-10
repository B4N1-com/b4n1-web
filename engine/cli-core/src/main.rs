//! B4n1Web CLI
//!
//! Command-line interface for the B4n1Web agentic browser engine.

use clap::{Parser, Subcommand};
use b4n1web::{AgentBrowser, BrowserMode};
use b4n1web::mcp::McpServer;
use b4n1web::session;
use serde_json::json;
use tracing_subscriber::fmt::init;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "b4n1web")]
#[command(about = "B4n1Web: The Agentic Browser Engine")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Navigate to a URL and extract content
    Goto {
        /// URL to navigate to
        url: String,
        /// Browser mode (light, js, or render)
        #[arg(short, long, default_value = "light")]
        mode: String,
        /// CSS selector to wait for before extracting content (render mode only)
        #[arg(long)]
        wait_for: Option<String>,
        /// JavaScript to execute after page load (render mode only)
        #[arg(long)]
        js: Option<String>,
    },
    /// Execute arbitrary JavaScript in a URL (render mode only)
    Evaluate {
        /// URL to navigate to
        url: String,
        /// JavaScript code to execute
        js: String,
    },
    /// Start MCP server (stdio mode by default)
    Mcp {
        /// Use TCP server mode instead of stdio
        #[arg(long)]
        tcp: bool,
        /// Port to listen on (TCP mode only)
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Install b4n1web skill/config for an AI agent
    Install {
        /// Target agent: opencode, antigravity, cursor, windsurf, claude-code
        agent: String,
    },
    /// Install render binary for headless browser functionality
    InstallRender {
        /// Actually download and install (without this, just checks)
        #[arg(short, long)]
        install: bool,
    },
    /// Check for updates
    Update {
        /// Actually download and install the update
        #[arg(short, long)]
        install: bool,
    },
    /// Manage Chromium browser for render mode
    Chromium {
        #[command(subcommand)]
        action: ChromiumAction,
    },
    /// Manage persistent browser sessions
    Session {
        #[command(subcommand)]
        action: SessionAction,
    },
    /// Start persistent browser daemon (keeps Chrome alive)
    Daemon {
        /// Port to listen on
        #[arg(short, long, default_value = "9876")]
        port: u16,
    },
    /// Execute JS on running daemon
    Exec {
        /// JavaScript code to execute
        js: String,
    },
}

#[derive(Subcommand)]
enum ChromiumAction {
    Install, Update, Version, Remove,
}

#[derive(Subcommand)]
enum SessionAction {
    /// Start a new browser session
    Start { name: String },
    /// Close a browser session
    Close { name: String },
    /// List active sessions
    List,
    /// Navigate to URL in a session
    Goto { name: String, url: String, #[arg(long)] wait_for: Option<String> },
    /// Click an element in a session
    Click { name: String, selector: String },
    /// Type text in a session
    Type { name: String, selector: String, text: String, #[arg(long)] clear_first: bool },
    /// Wait for selector in a session
    Wait { name: String, selector: String, #[arg(long, default_value = "5000")] timeout_ms: u64 },
    /// Screenshot a URL in a session
    Screenshot { name: String, url: String, #[arg(long)] full_page: bool },
    /// List iframes in a session
    Frames { name: String },
    /// Get iframe text
    IframeText { name: String, index: usize },
    /// Save session cookies + localStorage to a JSON file
    SaveState { name: String, path: String },
    /// Restore session cookies + localStorage from a JSON file
    LoadState { name: String, path: String },
}

fn get_b4n1web_path() -> PathBuf {
    std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("b4n1web"))
}

fn install_for_agent(agent: &str) -> Result<(), Box<dyn std::error::Error>> {
    let b4n1web_path = get_b4n1web_path();
    let agents: &[&str] = match agent.to_lowercase().as_str() {
        "all" => &["opencode", "antigravity", "cursor", "windsurf", "claude-code", "gemini", "kilo"],
        a => return install_single_agent(a, &b4n1web_path),
    };
    for a in agents {
        install_single_agent(a, &b4n1web_path)?;
    }
    Ok(())
}

fn merge_mcp_entry(path: &std::path::Path, top_key: &str, entry: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    let mut merged: serde_json::Value = serde_json::from_str(&existing).unwrap_or(json!({}));
    if let Some(obj) = merged.as_object_mut() {
        obj.entry(top_key)
            .or_insert_with(|| json!({}))
            .as_object_mut()
            .map(|m| m.insert("b4n1web".to_string(), entry));
    }
    fs::write(path, serde_json::to_string_pretty(&merged)?)?;
    Ok(())
}

fn install_single_agent(agent: &str, b4n1web_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    match agent.to_lowercase().as_str() {
        "opencode" => {
            let config_dir = dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config/opencode");
            
            fs::create_dir_all(&config_dir)?;
            
            let config_path = config_dir.join("opencode.json");
            
            let existing = fs::read_to_string(&config_path).unwrap_or_default();
            let mut merged: serde_json::Value = serde_json::from_str(&existing).unwrap_or(json!({
                "$schema": "https://opencode.ai/config.json"
            }));
            
            let b4n1web_entry = json!({
                "b4n1web": {
                    "type": "local",
                    "command": [b4n1web_path.to_string_lossy().to_string(), "mcp"],
                    "enabled": true
                }
            });

            if let Some(mcp) = merged.get_mut("mcp") {
                if let Some(obj) = mcp.as_object_mut() {
                    obj.insert("b4n1web".to_string(), b4n1web_entry["b4n1web"].clone());
                }
            } else {
                merged["mcp"] = json!({
                    "b4n1web": b4n1web_entry["b4n1web"]
                });
            }

            fs::write(&config_path, serde_json::to_string_pretty(&merged)?)?;
            println!("✅ OpenCode config installed to: {}", config_path.display());
            println!("   Restart OpenCode and use b4n1web in your prompts!");
        }
        
        "antigravity" | "cursor" | "windsurf" | "gemini" | "claude-code" | "claude" => {
            let (config_dir, config_file, top_key) = match agent {
                "antigravity" => (PathBuf::from("~/.config/antigravity"), "mcp.json", "mcpServers"),
                "cursor" => (PathBuf::from("~/.cursor"), "mcp.json", "mcpServers"),
                "windsurf" => (PathBuf::from("~/.windsurf"), "mcp.json", "mcpServers"),
                "gemini" => (PathBuf::from("~/.gemini"), "settings.json", "mcpServers"),
                _ => (PathBuf::from("~/.claude"), "settings.json", "mcpServers"),
            };
            let config_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~")).join(config_dir.strip_prefix("~/").unwrap_or(&config_dir));
            let config_path = config_dir.join(config_file);
            fs::create_dir_all(&config_dir)?;

            let entry = json!({
                "command": b4n1web_path.to_string_lossy().to_string(),
                "args": ["mcp"]
            });
            merge_mcp_entry(&config_path, top_key, entry)?;
            println!("✅ {} config installed to: {}", agent, config_path.display());
        }

        "kilo" => {
            let config_dir = dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("~"))
                .join(".config/kilo");
            fs::create_dir_all(&config_dir)?;

            let config_path = config_dir.join("kilo.jsonc");
            let entry = json!({
                "type": "local",
                "command": [b4n1web_path.to_string_lossy().to_string(), "mcp"],
                "enabled": true
            });
            merge_mcp_entry(&config_path, "mcp", entry)?;
            println!("✅ Kilo config installed to: {}", config_path.display());
        }

        _ => {
            println!("❌ Unknown agent: {}", agent);
            println!("\nSupported agents:");
            println!("  - opencode");
            println!("  - antigravity");
            println!("  - cursor");
            println!("  - windsurf");
            println!("  - claude-code");
            println!("  - gemini");
            println!("  - kilo");
            println!("\nUsage: b4n1web install <agent>");
            std::process::exit(1);
        }
    }
    
    println!("\n📝 Next steps:");
    println!("   1. Restart your agent");
    println!("   2. Use b4n1web in your prompts!");
    println!("");
    println!("   (MCP configured in stdio mode — no port needed)");
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init();

    let cli = Cli::parse();

    // For stdio MCP, run directly (no extra async runtime needed)
    if let Commands::Mcp { tcp: false, port } = &cli.command {
        let server = McpServer::new(*port);
        return Ok(server.run_stdio_sync()?);
    }

    // All other commands need an async runtime
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        match cli.command {
            Commands::Goto { url, mode, wait_for, js } => {
                let mode = match mode.as_str() {
                    "light" => BrowserMode::Light,
                    "js" => BrowserMode::Js,
                    "render" => BrowserMode::Render,
                    _ => {
                        eprintln!("Invalid mode. Use 'light', 'js', or 'render'");
                        std::process::exit(1);
                    }
                };

                let browser = AgentBrowser::new(mode);
                let page = browser.goto(&url, wait_for.as_deref()).await?;

                if let Some(js_code) = js {
                    let result = browser.evaluate(&js_code).await?;
                    println!("{}", result);
                } else {
                    println!("URL: {}", page.url);
                    println!("Markdown:\n{}", page.markdown);
                    println!("Links: {:?}", page.links);
                    if let Some(screenshot) = page.screenshot {
                        println!("Screenshot: {}", screenshot);
                    }
                }

                Ok(())
            }
            Commands::Evaluate { url, js } => {
                let browser = AgentBrowser::new(BrowserMode::Render);
                let _ = browser.goto(&url, None).await?;
                let result = browser.evaluate(&js).await?;
                println!("{}", result);
                Ok(())
            }
            Commands::Mcp { tcp: true, port } => {
                let server = McpServer::new(port);
                server.run().await?;
                Ok(())
            }
            // tcp:false handled above, unreachable here
            Commands::Mcp { .. } => unreachable!(),
            Commands::Install { agent } => {
                install_for_agent(&agent)?;
                Ok(())
            }
            Commands::InstallRender { install } => {
                install_render_binary(install).await?;
                Ok(())
            }
            Commands::Update { install } => {
                check_and_update(install).await?;
                Ok(())
            }
            Commands::Chromium { action } => {
                handle_chromium_action(action).await?;
                Ok(())
            }
            Commands::Session { action } => {
                handle_session_action(action).await?;
                Ok(())
            }
            Commands::Daemon { port } => {
                start_daemon(port).await?;
                Ok(())
            }
            Commands::Exec { js } => {
                exec_on_daemon(&js).await?;
                Ok(())
            }
        }
    })
}

async fn start_daemon(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    println!("Starting b4n1web daemon on port {}...", port);
    
    // Launch persistent browser
    let chrome_path = b4n1web::chromium::find_chromium()
        .ok_or("Chrome not found. Run: b4n1web chromium install")?;
    let browser = b4n1web::chromium::ChromiumBrowser::launch(Some(&chrome_path)).await?;
    let browser = Arc::new(Mutex::new(Some(browser)));
    
    println!("Browser launched. Listening on 0.0.0.0:{}", port);
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    loop {
        let (mut stream, _) = listener.accept().await?;
        let browser = browser.clone();
        
        tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            let n = stream.read(&mut buf).await.unwrap_or(0);
            if n == 0 { return; }
            
            let request = String::from_utf8_lossy(&buf[..n]);
            let request: serde_json::Value = serde_json::from_str(&request).unwrap_or_default();
            
            let action = request["action"].as_str().unwrap_or("");
            let js = request["js"].as_str().unwrap_or("");
            let url = request["url"].as_str().unwrap_or("about:blank");
            
            let response = match action {
                "goto" => {
                    let guard = browser.lock().await;
                    if let Some(br) = guard.as_ref() {
                        match br.goto(url, None).await {
                            Ok(page) => serde_json::json!({"ok": true, "url": page.url, "markdown": page.markdown}),
                            Err(e) => serde_json::json!({"error": e.to_string()}),
                        }
                    } else {
                        serde_json::json!({"error": "No browser"})
                    }
                }
                "eval" | "evaluate" => {
                    let guard = browser.lock().await;
                    if let Some(br) = guard.as_ref() {
                        match br.evaluate(js).await {
                            Ok(val) => serde_json::json!({"ok": true, "result": val.to_string()}),
                            Err(e) => serde_json::json!({"error": e.to_string()}),
                        }
                    } else {
                        serde_json::json!({"error": "No browser"})
                    }
                }
                "screenshot" => {
                    let guard = browser.lock().await;
                    if let Some(br) = guard.as_ref() {
                        match br.screenshot(url, true).await {
                            Ok(Some(ss)) => serde_json::json!({"ok": true, "screenshot": ss}),
                            _ => serde_json::json!({"error": "Screenshot failed"}),
                        }
                    } else {
                        serde_json::json!({"error": "No browser"})
                    }
                }
                "close" => {
                    let mut guard = browser.lock().await;
                    if let Some(br) = guard.take() {
                        br.close().await;
                    }
                    serde_json::json!({"ok": true})
                }
                _ => serde_json::json!({"error": "Unknown action"}),
            };
            
            let _ = stream.write_all(response.to_string().as_bytes()).await;
        });
    }
}

async fn exec_on_daemon(js: &str) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    
    let mut stream = TcpStream::connect("127.0.0.1:9876").await?;
    let request = serde_json::json!({"action": "eval", "js": js});
    stream.write_all(request.to_string().as_bytes()).await?;
    
    let mut buf = vec![0u8; 65536];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("{}", response);
    Ok(())
}

async fn check_and_update(install: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = env!("CARGO_PKG_VERSION");
    
    println!("🔍 Checking for updates...");
    println!("Current version: v{}", current_version);
    
    let client = reqwest::Client::builder()
        .user_agent("b4n1web")
        .build()?;
    
    let response = client
        .get("https://api.github.com/repos/B4N1-com/b4n1-web/releases/latest")
        .send()
        .await?;
    
    let release: serde_json::Value = response.json().await?;
    
    let latest_version = release["tag_name"]
        .as_str()
        .unwrap_or("v0.0.0")
        .trim_start_matches('v');
    
    let current = semver::Version::parse(current_version)?;
    let latest = semver::Version::parse(latest_version)?;
    
    if latest > current {
        println!("\n✅ New version available: v{}", latest);
        
        if install {
            println!("\n📦 Installing v{}...\n", latest);
            
            // Detect platform
            let os = std::env::consts::OS;
            let arch = std::env::consts::ARCH;
            let platform = match (os, arch) {
                ("linux", "x86_64") => "linux-x86_64",
                ("linux", "aarch64") => "linux-aarch64",
                ("macos", "x86_64") => "macos-x86_64",
                ("macos", "aarch64") => "macos-aarch64",
                _ => {
                    println!("\n⚠️  Unsupported platform: {}-{}", os, arch);
                    println!("\n💡 To update manually, run:");
                    println!("   curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash");
                    return Ok(());
                }
            };

            let tarball_url = format!(
                "https://github.com/B4N1-com/b4n1-web/releases/download/v{}/b4n1web-v{}-{}.tar.gz",
                latest, latest, platform
            );

            println!("Downloading from: {}", tarball_url);
            
            let tarball_response = client.get(&tarball_url).send().await;
            
            match tarball_response {
                Ok(resp) if resp.status().is_success() => {
                    let bytes = resp.bytes().await?;
                    let temp_dir = std::env::temp_dir().join(format!("b4n1web-update-{}", std::process::id()));
                    std::fs::create_dir_all(&temp_dir)?;
                    
                    let tarball_path = temp_dir.join("b4n1web.tar.gz");
                    std::fs::write(&tarball_path, &bytes)?;
                    
                    // Extract
                    let file = std::fs::File::open(&tarball_path)?;
                    let decoder = flate2::read::GzDecoder::new(file);
                    let mut archive = tar::Archive::new(decoder);
                    archive.unpack(&temp_dir)?;
                    
                    // Find the binary
                    let binary = find_extracted_binary(&temp_dir, "b4n1web");
                    
                    if let Some(binary_path) = binary {
                        // Determine install location (same dir as current binary)
                        let current_exe = std::env::current_exe()?;
                        let install_dir = current_exe.parent().unwrap_or(std::path::Path::new("/usr/local/bin"));
                        let dest = install_dir.join("b4n1web");
                        
                        std::fs::copy(&binary_path, &dest)?;
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
                        }
                        
                        let _ = std::fs::remove_dir_all(&temp_dir);
                        
                        println!("✅ Updated to v{}", latest);
                        println!("   Location: {}", dest.display());
                        return Ok(());
                    } else {
                        let _ = std::fs::remove_dir_all(&temp_dir);
                        println!("\n⚠️  Could not find binary in downloaded archive");
                    }
                }
                _ => {
                    println!("\n⚠️  Could not download from GitHub Releases");
                }
            }
            
            // Fallback: try install script from raw GitHub
            println!("\n📦 Trying install script fallback...");
            let result = std::process::Command::new("sh")
                .arg("-c")
                .arg("curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash")
                .spawn();
            
            match result {
                Ok(mut child) => {
                    let _ = child.wait();
                    println!("✅ Installer finished. Run 'b4n1web --version' to verify.");
                }
                Err(e) => {
                    println!("\n⚠️  Could not run installer: {}", e);
                    println!("\n💡 To update manually, run:");
                    println!("   curl -sL https://raw.githubusercontent.com/B4N1-com/b4n1-web/master/scripts/install.sh | bash");
                }
            }
        } else {
            println!("\n📦 To update, run:");
            println!("   b4n1web update --install");
        }
    } else {
        println!("\n✅ You're on the latest version!");
    }
    
    Ok(())
}

fn find_extracted_binary(dir: &std::path::Path, name: &str) -> Option<std::path::PathBuf> {
    // Check flat layout first
    let flat = dir.join(name);
    if flat.exists() {
        return Some(flat);
    }
    // Check nested directory layout (b4n1web-vX.Y.Z-platform/)
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let nested = path.join(name);
                if nested.exists() {
                    return Some(nested);
                }
            }
        }
    }
    None
}

async fn install_render_binary(install: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = env!("CARGO_PKG_VERSION");
    
    println!("🔍 Checking for render binary...");
    
    let render_path = dirs::home_dir()
        .ok_or("Cannot find home directory")?
        .join(".b4n1web")
        .join("bin")
        .join("b4n1web-render");
    
    if render_path.exists() {
        println!("✅ Render binary already installed at: {}", render_path.display());
        return Ok(());
    }
    
    if !install {
        println!("\n📦 Render binary not found.");
        println!("   To install, run: b4n1web install-render --install");
        return Ok(());
    }
    
    println!("\n⬇️  Downloading render binary (this may take a moment)...");
    
    let client = reqwest::Client::new();
    
    let download_url = format!(
        "https://github.com/B4N1-com/b4n1-web/releases/download/v{}/b4n1web-render-v{}-x86_64.tar.gz",
        current_version, current_version
    );
    
    let response = match client.get(&download_url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            println!("⚠️  Could not download render binary: {}", e);
            println!("   The render binary will be available in a future release.");
            return Ok(());
        }
    };
    
    if !response.status().is_success() {
        println!("⚠️  Render binary not available yet.");
        println!("   It will be included in a future release.");
        return Ok(());
    }
    
    let bytes = response.bytes().await?;
    
    if let Some(parent) = render_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let temp_dir = std::env::temp_dir();
    let tar_path = temp_dir.join("b4n1web-render.tar.gz");
    
    std::fs::write(&tar_path, &bytes)?;
    
    let tar_file = std::fs::File::open(&tar_path)?;
    let gz_decoder = flate2::read::GzDecoder::new(tar_file);
    let mut archive = tar::Archive::new(gz_decoder);
    archive.unpack(&temp_dir)?;
    
    let binary_path = temp_dir.join("b4n1web-render");
    std::fs::copy(&binary_path, &render_path)?;
    std::fs::set_permissions(&render_path, std::fs::Permissions::from_mode(0o755))?;
    
    std::fs::remove_file(&tar_path)?;
    std::fs::remove_file(&binary_path)?;
    
    println!("✅ Render binary installed to: {}", render_path.display());
    println!("\n📝 You can now use render mode:");
    println!("   b4n1web goto https://example.com --mode render");
    
    Ok(())
}

async fn handle_session_action(action: SessionAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        SessionAction::Start { name } => {
            println!("{}", session::start(&name, session::SessionKind::Tab).await?);
        }
        SessionAction::Close { name } => {
            println!("{}", session::close(&name).await?);
        }
        SessionAction::List => {
            for (name, kind, url) in session::list().await? {
                println!("  {} ({}) - {}", name, kind, url);
            }
        }
        SessionAction::Goto { name, url, wait_for } => {
            println!("{}", session::goto(&name, &url, wait_for.as_deref()).await?);
        }
        SessionAction::Click { name, selector } => {
            println!("{}", session::click(&name, &selector).await?);
        }
        SessionAction::Type { name, selector, text, clear_first } => {
            println!("{}", session::type_text(&name, &selector, &text, clear_first).await?);
        }
        SessionAction::Wait { name, selector, timeout_ms } => {
            println!("{}", session::wait_for(&name, &selector, timeout_ms).await?);
        }
        SessionAction::Screenshot { name, url, full_page } => {
            println!("{}", session::screenshot(&name, &url, full_page).await?);
        }
        SessionAction::Frames { name } => {
            println!("{}", session::frames(&name).await?);
        }
        SessionAction::IframeText { name, index } => {
            println!("{}", session::iframe_text(&name, index).await?);
        }
        SessionAction::SaveState { name, path } => {
            println!("{}", session::save_state(&name, &path).await?);
        }
        SessionAction::LoadState { name, path } => {
            println!("{}", session::load_state(&name, &path).await?);
        }
    }
    Ok(())
}

async fn handle_chromium_action(action: ChromiumAction) -> Result<(), Box<dyn std::error::Error>> {
    use b4n1web::chromium;
    
    match action {
        ChromiumAction::Install => {
            if let Some(path) = chromium::find_chromium() {
                println!("✅ Chromium already installed at: {:?}", path);
                println!("   Use 'b4n1web chromium update' to update.");
                return Ok(());
            }
            
            println!("🔍 No Chromium found.");
            println!("\n⬇️  Downloading Chromium (~150MB)...");
            
            let path = chromium::download_chromium().await?;
            println!("✅ Chromium installed to: {:?}", path);
            
            let version = chromium::get_chromium_version(&path)?;
            println!("   Version: {}", version);
        }
        
        ChromiumAction::Update => {
            let current = chromium::find_chromium().map(|p| {
                chromium::get_chromium_version(&p).ok()
            }).flatten();
            
            if let Some(ver) = current {
                println!("📍 Current Chromium version: {}", ver);
            } else {
                println!("ℹ️  No Chromium installed yet.");
            }
            
            println!("\n⬇️  Downloading latest Chromium (~150MB)...\n");
            
            let path = chromium::download_chromium().await?;
            println!("✅ Chromium updated to: {:?}", path);
            
            let version = chromium::get_chromium_version(&path)?;
            println!("   New version: {}", version);
        }
        
        ChromiumAction::Version => {
            if let Some(path) = chromium::find_chromium() {
                let version = chromium::get_chromium_version(&path)?;
                println!("Chromium: {}", version);
                println!("Path: {:?}", path);
            } else {
                println!("❌ No Chromium found.");
                println!("   Run 'b4n1web chromium install' to download.");
            }
        }
        
        ChromiumAction::Remove => {
            let base_dir = dirs::home_dir()
                .ok_or("Cannot find home directory")?
                .join(".b4n1web")
                .join("chromium");
            
            if base_dir.exists() {
                std::fs::remove_dir_all(&base_dir)?;
                println!("✅ Removed Chromium from: {:?}", base_dir);
            } else {
                println!("ℹ️  No Chromium installation found.");
            }
        }
    }
    
    Ok(())
}