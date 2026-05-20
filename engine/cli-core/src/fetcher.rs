//! HTTP fetching module
//!
//! Handles secure HTTP requests with proper TLS.

use crate::{Error, Result};
use reqwest::Client;

pub async fn fetch_html(url: &str) -> Result<String> {
    let client = Client::builder()
        .user_agent("B4n1Web-Agent/1.0")
        .gzip(true)
        .build()
        .map_err(|e| Error::Http(e))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| Error::Http(e))?;

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");

    if !content_type.contains("text/html") {
        return Err(Error::Other(format!("Not HTML content: {}", content_type)));
    }

    let html = response.text().await.map_err(|e| Error::Http(e))?;
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_html_invalid_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(fetch_html("not a url"));
        assert!(result.is_err());
    }

    #[test]
    fn test_client_builds() {
        let client = Client::builder()
            .user_agent("B4n1Web-Agent/1.0")
            .gzip(true)
            .build();
        assert!(client.is_ok());
    }

    #[test]
    fn test_fetch_html_rejects_empty_url() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(fetch_html(""));
        assert!(result.is_err());
    }
}