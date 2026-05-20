//! Visual regression testing - screenshot comparison with diff
//!
//! Uses tiny-skia for PNG decode/encode and pixel-level comparison.

use std::path::Path;
use crate::Result;
use std::sync::atomic::{AtomicU32, Ordering};

/// Result of a visual comparison
pub struct VisualDiff {
    pub diff_pixels: u64,
    pub total_pixels: u64,
    pub diff_percent: f64,
    pub diff_image: Vec<u8>,
    pub passed: bool,
}

/// Compare two screenshot PNG files and generate a diff
pub fn compare<P: AsRef<Path>>(baseline: P, actual: P, threshold: f64) -> Result<VisualDiff> {
    let baseline_bytes = std::fs::read(baseline.as_ref())
        .map_err(|e| crate::Error::Other(format!("Read baseline: {}", e)))?;
    let actual_bytes = std::fs::read(actual.as_ref())
        .map_err(|e| crate::Error::Other(format!("Read actual: {}", e)))?;

    let baseline_pix = tiny_skia::Pixmap::decode_png(&baseline_bytes)
        .map_err(|e| crate::Error::Other(format!("Decode baseline: {}", e)))?;
    let actual_pix = tiny_skia::Pixmap::decode_png(&actual_bytes)
        .map_err(|e| crate::Error::Other(format!("Decode actual: {}", e)))?;

    let w = baseline_pix.width().max(actual_pix.width());
    let h = baseline_pix.height().max(actual_pix.height());

    let mut diff_pix = tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| crate::Error::Other("Create diff".to_string()))?;

    // Compare using pixel API
    let diff_data = diff_pix.pixels_mut();
    let mut diff_pixels: u64 = 0;

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let bp = baseline_pix.pixel(x, y);
            let ap = actual_pix.pixel(x, y);

            let same = match (bp, ap) {
                (Some(b), Some(a)) => b == a,
                (None, None) => true,
                _ => false,
            };

            if same {
                // Same pixel - copy with semi-transparency
                if let Some(p) = bp {
                    if idx < diff_data.len() {
                        diff_data[idx] = p;
                    }
                }
            } else {
                // Different - red
                if idx < diff_data.len() {
                    diff_data[idx] = tiny_skia::PremultipliedColorU8::from_rgba(128, 0, 0, 128)
                        .unwrap_or(tiny_skia::PremultipliedColorU8::TRANSPARENT);
                }
                diff_pixels += 1;
            }
        }
    }

    let total_pixels = (w as u64) * (h as u64);
    let diff_percent = if total_pixels > 0 { (diff_pixels as f64 / total_pixels as f64) * 100.0 } else { 0.0 };

    let diff_image = diff_pix.encode_png()
        .map_err(|e| crate::Error::Other(format!("PNG encode: {}", e)))?;

    let passed = diff_percent <= threshold;

    Ok(VisualDiff { diff_pixels, total_pixels, diff_percent, diff_image, passed })
}

/// Generate an HTML report from multiple visual diffs
pub fn generate_report(diffs: &[(&str, &VisualDiff)]) -> String {
    let mut rows = String::new();
    for (name, diff) in diffs {
        let icon = if diff.passed { "✅" } else { "❌" };
        rows.push_str(&format!(
            r#"<tr><td>{}</td><td>{}</td><td>{:.2}%</td><td>{}/{}px</td><td>{}</td></tr>"#,
            icon, name, diff.diff_percent, diff.diff_pixels, diff.total_pixels,
            if diff.passed { "PASS" } else { "FAIL" }
        ));
    }
    format!(
        r#"<!DOCTYPE html><html><head><title>Visual Regression Report</title>
        <style>body{{font:sans-serif;margin:20px}} table{{border-collapse:collapse;width:100%}}
        th,td{{border:1px solid #ccc;padding:10px}}</style></head>
        <body><h1>Visual Regression Report</h1>
        <table><tr><th></th><th>Name</th><th>Diff %</th><th>Pixels</th><th>Result</th></tr>
        {}</table></body></html>"#, rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_png(r: u8, g: u8, b: u8) -> Vec<u8> {
        let mut pix = tiny_skia::Pixmap::new(10, 10).unwrap();
        let c = tiny_skia::PremultipliedColorU8::from_rgba(r, g, b, 255).unwrap();
        let data = pix.pixels_mut();
        for px in data.iter_mut() {
            *px = c;
        }
        pix.encode_png().unwrap()
    }

    static FILE_ID: AtomicU32 = AtomicU32::new(0);
    fn write_test_png(data: &[u8]) -> std::path::PathBuf {
        let id = FILE_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("b4n1_vr_{}_{}.png", std::process::id(), id));
        std::fs::write(&path, data).unwrap();
        path
    }

    #[test]
    fn test_identical() {
        let data = create_test_png(255, 0, 0);
        let a = write_test_png(&data);
        let diff = compare(&a, &a, 0.0).unwrap();
        assert_eq!(diff.diff_pixels, 0);
        assert!(diff.passed);
        std::fs::remove_file(&a).ok();
    }

    #[test]
    fn test_different() {
        let mut red_pix = tiny_skia::Pixmap::new(1, 1).unwrap();
        let mut grn_pix = tiny_skia::Pixmap::new(1, 1).unwrap();
        red_pix.pixels_mut()[0] = tiny_skia::PremultipliedColorU8::from_rgba(255, 0, 0, 255).unwrap();
        grn_pix.pixels_mut()[0] = tiny_skia::PremultipliedColorU8::from_rgba(0, 255, 0, 255).unwrap();
        
        let r = write_test_png(&red_pix.encode_png().unwrap());
        let g = write_test_png(&grn_pix.encode_png().unwrap());
        
        let diff = compare(&r, &g, 0.0).unwrap();
        assert!(diff.diff_pixels > 0, "Expected >0 diff pixels, got {}", diff.diff_pixels);
        assert!(!diff.passed);
        std::fs::remove_file(&r).ok();
        std::fs::remove_file(&g).ok();
    }

    #[test]
    fn test_diff_percent() {
        let data = create_test_png(255, 255, 255);
        let a = write_test_png(&data);
        let diff = compare(&a, &a, 5.0).unwrap();
        assert_eq!(diff.diff_percent, 0.0);
        std::fs::remove_file(&a).ok();
    }

    #[test]
    fn test_report() {
        let d = VisualDiff { diff_pixels: 5, total_pixels: 100, diff_percent: 5.0, diff_image: vec![], passed: true };
        let html = generate_report(&[("t1", &d)]);
        assert!(html.contains("t1"));
        assert!(html.contains("PASS"));
    }

    #[test]
    fn test_different_sizes() {
        let mut small = tiny_skia::Pixmap::new(5, 10).unwrap();
        let mut large = tiny_skia::Pixmap::new(10, 5).unwrap();
        let red = tiny_skia::PremultipliedColorU8::from_rgba(255, 0, 0, 255).unwrap();
        let blue = tiny_skia::PremultipliedColorU8::from_rgba(0, 0, 255, 255).unwrap();
        for px in small.pixels_mut().iter_mut() { *px = red; }
        for px in large.pixels_mut().iter_mut() { *px = blue; }
        let sp = write_test_png(&small.encode_png().unwrap());
        let lp = write_test_png(&large.encode_png().unwrap());
        let diff = compare(&sp, &lp, 100.0).unwrap();
        assert!(diff.diff_pixels > 0);
        std::fs::remove_file(&sp).ok();
        std::fs::remove_file(&lp).ok();
    }
}
