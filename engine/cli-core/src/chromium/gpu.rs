//! GPU detection module for WebGL support in headless browsers

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GpuMode {
    /// GPU available - use hardware acceleration
    Native,
    /// No GPU - use SwiftShader software rendering
    SwiftShader,
    /// Use Xvfb virtual display with Chrome (not headless)
    Xvfb,
    /// Last resort - headless with SwiftShader
    None,
}

/// Detect available GPU mode for WebGL support
pub fn detect_gpu_mode() -> GpuMode {
    // Check if GPU is available via /sys or lspci
    let has_gpu = std::process::Command::new("lspci")
        .args(&["-v"])
        .output()
        .map(|o| {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.contains("VGA") || stdout.contains("3D") || stdout.contains("Display")
        })
        .unwrap_or(false);

    if has_gpu {
        return GpuMode::Native;
    }

    // Check if Xvfb is available
    let has_xvfb = std::process::Command::new("which")
        .arg("xvfb-run")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if has_xvfb {
        return GpuMode::Xvfb;
    }

    // Check if SwiftShader is available in Chrome
    let chrome_paths = &[
        "/opt/google/chrome",
        "/usr/bin/google-chrome",
        "/usr/lib/chromium",
    ];
    for path in chrome_paths {
        let swiftshader = std::path::Path::new(path).join("libvk_swiftshader.so");
        if swiftshader.exists() {
            return GpuMode::SwiftShader;
        }
    }

    GpuMode::None
}
