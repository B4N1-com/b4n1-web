//! B4n1Web CLI
//!
//! Command-line interface for the B4n1Web agentic browser engine.

use clap::{Parser, Subcommand};
use b4n1web::{AgentBrowser, BrowserMode};
use b4n1web::mcp::{run_mcp_server_stdio, run_mcp_server_tcp};
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
        let _ = run_mcp_server_stdio().await?;
        return Ok(server.run_stdio_sync()?);
    }

    // All other commands need an async runtime
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        match cli.command {
            Commands::Goto { url, mode, wait_for } => {
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

                println!("URL: {}", page.url);
                println!("Markdown:\n{}", page.markdown);
                println!("Links: {:?}", page.links);
                if let Some(screenshot) = page.screenshot {
                    println!("Screenshot: {}", screenshot);
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
                let _ = run_mcp_server_tcp(port).await?;
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
        }
    })
}

async fn check_and_update(install: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = env!("CARGO_PKG_VERSION");
    
    println!("🔍 Checking for updates...");
    println!("Current version: v{}", current_version);
    
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.github.com/repos/B4N1-com/b4n1-web/releases/latest")
        .header("User-Agent", "b4n1web")
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
            println!("\n📦 Running installer...\n");
            
            // Run the install script - this handles the binary replacement correctly
            let result = std::process::Command::new("sh")
                .arg("-c")
                .arg("curl -sL https://web.b4n1.com/install | bash")
                .spawn();
            
            match result {
                Ok(_) => {
                    println!("✅ Installer started! Run 'b4n1web update' again in a few seconds to verify.");
                }
                Err(e) => {
                    println!("\n⚠️  Could not run installer: {}", e);
                    println!("\n💡 To update manually, run:");
                    println!("   curl -sL https://web.b4n1.com/install | bash");
                }
            }
            return Ok(());
        } else {
            println!("\n📦 To update, run:");
            println!("   b4n1web update --install");
        }
    } else {
        println!("\n✅ You're on the latest version!");
    }
    
    Ok(())
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