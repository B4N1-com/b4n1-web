//! Chromium download module
//!
//! Handles downloading Chromium from official sources.

use crate::Result;
use std::path::PathBuf;

pub const CHROMIUM_BASE_URL: &str = "https://storage.googleapis.com/chromium-browser-snapshots";

/// Get the appropriate Chromium download URL for the current platform
pub fn get_chromium_url() -> Option<(String, String)> {
    #[cfg(target_os = "linux")]
    {
        Some((
            format!("{}/Linux_x64/1187685/chrome-linux.zip", CHROMIUM_BASE_URL),
            "linux64".to_string(),
        ))
    }

    #[cfg(target_os = "macos")]
    {
        let arch = std::env::consts::ARCH;
        if arch == "x86_64" {
            Some((
                format!("{}/Mac/1187685/chrome-mac.zip", CHROMIUM_BASE_URL),
                "mac".to_string(),
            ))
        } else {
            Some((
                format!("{}/Mac_Arm/1187685/chrome-mac-arm64.zip", CHROMIUM_BASE_URL),
                "mac-arm64".to_string(),
            ))
        }
    }

    #[cfg(target_os = "windows")]
    {
        Some((
            format!("{}/Win/1187685/chrome-win.zip", CHROMIUM_BASE_URL),
            "win".to_string(),
        ))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

pub async fn download_chromium() -> Result<PathBuf> {
    let (url, platform) = get_chromium_url()
        .ok_or_else(|| crate::Error::Other("Unsupported platform for Chromium download".to_string()))?;

    println!("⬇️  Downloading Chromium for {} (~150MB)...", platform);

    let client = reqwest::Client::new();

    // Download
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| crate::Error::Other(format!("Failed to download Chromium: {}", e)))?;

    if !response.status().is_success() {
        return Err(crate::Error::Other(format!(
            "Failed to download Chromium: HTTP {}",
            response.status()
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| crate::Error::Other(format!("Failed to read Chromium: {}", e)))?;

    // Create directory
    let base_dir = dirs::home_dir()
        .ok_or_else(|| crate::Error::Other("Cannot find home directory".to_string()))?
        .join(".b4n1web")
        .join("chromium");

    std::fs::create_dir_all(&base_dir)?;

    // Write zip
    let zip_path = base_dir.join(format!("chrome-{}.zip", platform));
    std::fs::write(&zip_path, &bytes).map_err(|e| crate::Error::Other(e.to_string()))?;

    println!("📦 Extracting Chromium...");

    // Extract
    let file = std::fs::File::open(&zip_path).map_err(|e| crate::Error::Other(e.to_string()))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| crate::Error::Zip(e.to_string()))?;

    // Extract to temp first
    let temp_dir = std::env::temp_dir().join("b4n1web-chromium");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
    }
    std::fs::create_dir_all(&temp_dir)?;
    archive.extract(&temp_dir).map_err(|e| crate::Error::Zip(e.to_string()))?;

    let chrome_dir = std::fs::read_dir(&temp_dir)?
        .filter_map(|e| e.ok())
        .find(|e| e.file_type().map(|f| f.is_dir()).unwrap_or(false))
        .map(|e| e.path());

    if let Some(dir) = chrome_dir {
        let chrome_path = dir.join("chrome");
        if chrome_path.exists() {
            let final_path = base_dir.join("chrome");
            std::fs::copy(&chrome_path, &final_path)?;
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&final_path, std::fs::Permissions::from_mode(0o755))?;
            }

            // Clean up
            let _ = std::fs::remove_file(&zip_path);
            let _ = std::fs::remove_dir_all(&temp_dir);

            println!("✅ Chromium installed to: {:?}", final_path);
            return Ok(final_path);
        }
    }

    Err(crate::Error::Other("Failed to extract Chromium - chrome binary not found".to_string()))
}

/// Get the version of a Chromium binary
pub fn get_chromium_version(chrome_path: &PathBuf) -> Result<String> {
    let output = std::process::Command::new(chrome_path)
        .args(["--version"])
        .output()
        .map_err(|e| crate::Error::Other(format!("Failed to get version: {}", e)))?;

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chromium_base_url() {
        assert!(CHROMIUM_BASE_URL.contains("chromium-browser-snapshots"));
    }

    #[test]
    fn test_get_chromium_url_returns_some_on_supported() {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            let result = get_chromium_url();
            assert!(result.is_some());
            let (url, platform) = result.unwrap();
            assert!(url.starts_with(CHROMIUM_BASE_URL));
            assert!(!platform.is_empty());
            assert!(url.ends_with(".zip"));
        }
    }

    #[test]
    fn test_get_chromium_url_platform_format() {
        let result = get_chromium_url();
        if let Some((url, _platform)) = result {
            // URL should be a valid download link
            assert!(url.contains("chromium-browser-snapshots"));
            assert!(url.ends_with(".zip"));
        }
    }

    #[test]
    fn test_get_chromium_version_needs_chrome() {
        // get_chromium_version requires actual chrome binary
        // Just test it returns error when path doesn't exist
        let fake = PathBuf::from("/nonexistent/chrome");
        let result = get_chromium_version(&fake);
        assert!(result.is_err());
    }
}
