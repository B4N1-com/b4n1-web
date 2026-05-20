//! Chromium module - Chrome/Chromium detection and download
//!
//! Provides utilities for finding Chrome/Chromium and downloading it if needed.

pub mod browser;
pub mod download;
pub mod locator;
pub mod network;
pub mod trace;

pub use browser::ChromiumBrowser;
pub use download::{download_chromium, get_chromium_version, CHROMIUM_BASE_URL};

use std::path::PathBuf;

/// Find Chrome/Chromium binary in common locations
pub fn find_chromium() -> Option<PathBuf> {
    if let Ok(env_path) = std::env::var("B4N1WEB_CHROME_PATH") {
        let p = PathBuf::from(env_path);
        if p.exists() {
            tracing::info!("Found Chrome from B4N1WEB_CHROME_PATH: {:?}", p);
            return Some(p);
        }
    }

    // 1. Try PATH first
    if let Ok(path) = which::which("google-chrome") {
        tracing::info!("Found Chrome in PATH: {:?}", path);
        return Some(path);
    }
    if let Ok(path) = which::which("chromium") {
        tracing::info!("Found Chromium in PATH: {:?}", path);
        return Some(path);
    }
    if let Ok(path) = which::which("chromium-browser") {
        tracing::info!("Found chromium-browser in PATH: {:?}", path);
        return Some(path);
    }
    if let Ok(path) = which::which("chrome") {
        tracing::info!("Found chrome in PATH: {:?}", path);
        return Some(path);
    }

    // 2. Check common system paths
    #[cfg(target_os = "linux")]
    {
        let paths = vec![
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
            "/usr/bin/chromium-browser",
            "/snap/bin/chromium",
            "/opt/google/chrome/chrome",
        ];
        for path in paths {
            let p = PathBuf::from(path);
            if p.exists() {
                tracing::info!("Found Chrome at system path: {:?}", p);
                return Some(p);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let paths = vec![
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/System/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        ];
        for path in paths {
            let p = PathBuf::from(path);
            if p.exists() {
                tracing::info!("Found Chrome at macOS path: {:?}", p);
                return Some(p);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let paths = vec![
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files\Chromium\chromium.exe",
        ];
        for path in paths {
            let p = PathBuf::from(path);
            if p.exists() {
                tracing::info!("Found Chrome at Windows path: {:?}", p);
                return Some(p);
            }
        }
    }

    // 3. Check downloaded Chromium (~/.b4n1web/chromium/)
    if let Some(home) = dirs::home_dir() {
        let downloaded = home.join(".b4n1web").join("chromium").join("chrome");
        if downloaded.exists() {
            tracing::info!("Found downloaded Chromium: {:?}", downloaded);
            return Some(downloaded);
        }

        // Also check for linux/mac subdirs
        #[cfg(target_os = "linux")]
        {
            let downloaded = home.join(".b4n1web").join("chromium").join("linux64").join("chrome");
            if downloaded.exists() {
                return Some(downloaded);
            }
        }
    }

    None
}

/// Get the Chromium download directory
pub fn get_chromium_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".b4n1web").join("chromium"))
}

/// Get the Chromium binary path
pub fn get_chromium_path() -> Option<PathBuf> {
    get_chromium_dir().map(|d| d.join("chrome"))
}

/// Check if Chromium is available (either in system or downloaded)
pub fn is_chromium_available() -> bool {
    find_chromium().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chromium_dir() {
        let dir = get_chromium_dir();
        assert!(dir.is_some());
        let path = dir.unwrap();
        assert!(path.ends_with(".b4n1web/chromium"));
    }

    #[test]
    fn test_get_chromium_path() {
        let path = get_chromium_path();
        assert!(path.is_some());
        let p = path.unwrap();
        assert!(p.ends_with("chrome") || p.ends_with("chrome.exe"));
    }

    #[test]
    fn test_find_chromium_does_not_crash() {
        // Should not panic regardless of whether Chrome is installed
        let result = find_chromium();
        // Either returns Some(path) or None
        if let Some(path) = result {
            assert!(path.exists());
        }
    }

    #[test]
    fn test_is_chromium_available() {
        // Should not panic and return bool
        let _available = is_chromium_available();
    }
}
