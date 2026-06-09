//! MCP Tools module - tool registry and execution

use serde_json::Value;

pub fn list_tools() -> Value {
    serde_json::json!({
        "tools": [
            {
                "name": "goto",
                "description": "Navigate to a URL and extract content",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "url": { "type": "string" },
                        "mode": { "type": "string", "enum": ["light", "js", "render"] },
                        "wait_for": { "type": "string" }
                    },
                    "required": ["url"]
                }
            },
            {
                "name": "click",
                "description": "Click an element by CSS selector",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "selector": { "type": "string" }
                    },
                    "required": ["selector"]
                }
            },
            {
                "name": "type_text",
                "description": "Type text into an element",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "selector": { "type": "string" },
                        "text": { "type": "string" },
                        "clear_first": { "type": "boolean" }
                    },
                    "required": ["selector", "text"]
                }
            },
            {
                "name": "wait_for_selector",
                "description": "Wait for an element to appear",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "selector": { "type": "string" },
                        "timeout": { "type": "number" }
                    },
                    "required": ["selector"]
                }
            },
            {
                "name": "screenshot",
                "description": "Take a screenshot of a page",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "url": { "type": "string" },
                        "full_page": { "type": "boolean" }
                    }
                }
            }
        ]
    })
}
