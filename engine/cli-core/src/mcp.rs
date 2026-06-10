//! Model Context Protocol (MCP) server implementation
//!
//! Provides an MCP-compliant interface for AI agents to control the browser.

pub mod server;
pub mod tools;
pub mod types;

pub use server::McpServer;
pub use types::{McpRequest, McpResponse, McpError};
