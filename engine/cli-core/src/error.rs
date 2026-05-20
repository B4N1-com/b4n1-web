//! Error types for B4n1Web

use thiserror::Error;

/// Main error type for all B4n1Web operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("HTML parsing failed: {0}")]
    Parse(String),

    #[error("URL parsing failed: {0}")]
    Url(#[from] url::ParseError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CSS parsing failed: {0}")]
    Css(String),

    #[error("JavaScript execution failed: {0}")]
    Js(String),

    #[error("Layout computation failed: {0}")]
    Layout(String),

    #[error("Rendering failed: {0}")]
    Render(String),

    #[error("Image encoding failed: {0}")]
    Image(String),

    #[error("DOM operation failed: {0}")]
    Dom(String),

    #[error("MCP protocol error: {0}")]
    Mcp(String),

    #[error("Render binary not found: {0}")]
    RenderBinaryNotFound(String),

    #[error("Chrome/Chromium not found: {0}")]
    ChromeNotFound(String),

    #[error("Zip extraction failed: {0}")]
    Zip(String),

    #[error("Generic error: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_parse() {
        let error = Error::Parse("bad HTML".to_string());
        assert_eq!(format!("{}", error), "HTML parsing failed: bad HTML");
    }

    #[test]
    fn test_error_display_url() {
        let result = url::Url::parse("not a url");
        if let Err(e) = result {
            let error = Error::Url(e);
            let display = format!("{}", error);
            assert!(display.contains("URL parsing failed"));
        }
    }

    #[test]
    fn test_error_display_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = Error::Io(io_err);
        let display = format!("{}", error);
        assert!(display.contains("I/O error"));
    }

    #[test]
    fn test_error_display_json() {
        let result = serde_json::from_str::<serde_json::Value>("not json");
        if let Err(e) = result {
            let error = Error::Json(e);
            let display = format!("{}", error);
            assert!(display.contains("JSON serialization error"));
        }
    }

    #[test]
    fn test_error_display_css() {
        let error = Error::Css("invalid selector".to_string());
        assert_eq!(format!("{}", error), "CSS parsing failed: invalid selector");
    }

    #[test]
    fn test_error_display_js() {
        let error = Error::Js("syntax error".to_string());
        assert_eq!(
            format!("{}", error),
            "JavaScript execution failed: syntax error"
        );
    }

    #[test]
    fn test_error_display_layout() {
        let error = Error::Layout("box overflow".to_string());
        assert_eq!(
            format!("{}", error),
            "Layout computation failed: box overflow"
        );
    }

    #[test]
    fn test_error_display_render() {
        let error = Error::Render("canvas error".to_string());
        assert_eq!(format!("{}", error), "Rendering failed: canvas error");
    }

    #[test]
    fn test_error_display_image() {
        let error = Error::Image("invalid format".to_string());
        assert_eq!(
            format!("{}", error),
            "Image encoding failed: invalid format"
        );
    }

    #[test]
    fn test_error_display_dom() {
        let error = Error::Dom("null reference".to_string());
        assert_eq!(format!("{}", error), "DOM operation failed: null reference");
    }

    #[test]
    fn test_error_display_mcp() {
        let error = Error::Mcp("timeout".to_string());
        assert_eq!(format!("{}", error), "MCP protocol error: timeout");
    }

    #[test]
    fn test_error_display_other() {
        let error = Error::Other("something broke".to_string());
        assert_eq!(format!("{}", error), "Generic error: something broke");
    }

    #[test]
    fn test_error_debug() {
        let error = Error::Parse("test".to_string());
        let debug = format!("{:?}", error);
        assert!(debug.contains("Parse"));
    }

    #[test]
    fn test_error_display_chrome_not_found() {
        let error = Error::ChromeNotFound("Chrome not found".to_string());
        let msg = format!("{}", error);
        assert!(msg.contains("Chrome") || msg.contains("chrome"));
    }

    #[test]
    fn test_error_display_render_binary_not_found() {
        let error = Error::RenderBinaryNotFound("render binary".to_string());
        let msg = format!("{}", error);
        assert!(msg.contains("render"));
    }

    #[test]
    fn test_error_display_zip() {
        let error = Error::Zip("corrupt archive".to_string());
        let msg = format!("{}", error);
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err: Error = io_err.into();
        assert!(format!("{}", err).contains("I/O error"));
    }

    #[test]
    fn test_error_from_json() {
        let json_err = serde_json::from_str::<()>("invalid").unwrap_err();
        let err: Error = json_err.into();
        assert!(format!("{}", err).contains("JSON"));
    }

    #[test]
    fn test_error_from_url() {
        let url_err = url::Url::parse("not a url").unwrap_err();
        let err: Error = url_err.into();
        assert!(format!("{}", err).contains("URL"));
    }
}
