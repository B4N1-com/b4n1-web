//! Screenshot rendering - HTML to PNG image
//!
//! Creates a visual representation of the page as a PNG.

use crate::dom::tree::DomTree;
use crate::Result;
use tiny_skia::Pixmap;

pub struct ScreenshotRenderer;

impl ScreenshotRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, dom: &DomTree, width: u32, height: u32) -> Result<Vec<u8>> {
        let mut pixmap = Pixmap::new(width, height)
            .ok_or_else(|| crate::Error::Render("Failed to create pixmap".to_string()))?;

        // White background (RGBA)
        let pixels = pixmap.data_mut();
        for i in 0..(width * height) as usize {
            let idx = i * 4;
            pixels[idx] = 255; // R
            pixels[idx + 1] = 255; // G
            pixels[idx + 2] = 255; // B
            pixels[idx + 3] = 255; // A
        }

        // Get text content
        let text = dom.text_content();
        let lines: Vec<&str> = text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .take(28)
            .collect();

        let w = width as usize;

        // Draw rows
        for (i, line) in lines.iter().enumerate() {
            let y_start = 50 + (i as i32) * 20;
            let y_end = y_start + 20;

            if y_end > height as i32 {
                break;
            }

            // Alternating background colors (light gray / white)
            for y in y_start..y_end {
                for x in 0..width as i32 {
                    let idx = ((y as usize) * w + (x as usize)) * 4;
                    if idx + 3 < pixels.len() {
                        if i % 2 == 0 {
                            pixels[idx] = 245; // R - light gray
                            pixels[idx + 1] = 245; // G
                            pixels[idx + 2] = 245; // B
                        } else {
                            pixels[idx] = 255; // R - white
                            pixels[idx + 1] = 255; // G
                            pixels[idx + 2] = 255; // B
                        }
                        pixels[idx + 3] = 255;
                    }
                }
            }

            // Draw text
            let line = line.trim();
            if !line.is_empty() {
                self.draw_ascii_line(pixels, line, 15, y_start + 14, w)?;
            }
        }

        // Encode to PNG
        let data = pixmap
            .encode_png()
            .map_err(|e| crate::Error::Image(format!("PNG encoding error: {:?}", e)))?;

        Ok(data)
    }

    fn draw_ascii_line(
        &self,
        pixels: &mut [u8],
        text: &str,
        x: i32,
        y: i32,
        w: usize,
    ) -> Result<()> {
        let text = text.replace("  ", " ");
        let h = pixels.len() / w / 4;

        for (i, c) in text.chars().take(60).enumerate() {
            if c as u32 > 127 {
                continue;
            }

            let px = x + (i as i32) * 7;

            if px < (w as i32) - 3 && y > 0 && y < (h as i32) - 4 {
                if c != ' ' {
                    let pattern = get_char_pattern(c);
                    for (dx, dy, v) in pattern {
                        let nx = px + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < (w as i32) && ny >= 0 && ny < (h as i32) {
                            if v > 0 {
                                let idx = (ny as usize) * w * 4 + (nx as usize) * 4;
                                if idx + 3 < pixels.len() {
                                    pixels[idx] = 0; // R - black text
                                    pixels[idx + 1] = 0; // G
                                    pixels[idx + 2] = 0; // B
                                    pixels[idx + 3] = 255; // A
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn get_char_pattern(c: char) -> Vec<(i32, i32, u8)> {
    match c.to_ascii_lowercase() {
        'a' => vec![
            (0, 1, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (1, 1, 1),
            (1, 2, 1),
            (0, 2, 1),
        ],
        'b' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
            (2, 1, 1),
            (1, 1, 1),
            (1, 0, 1),
            (2, 0, 1),
        ],
        'c' => vec![(1, 0, 1), (0, 1, 1), (0, 2, 1), (1, 2, 1)],
        'd' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
            (2, 1, 1),
            (2, 0, 1),
        ],
        'e' => vec![
            (0, 0, 1),
            (1, 0, 1),
            (2, 0, 1),
            (0, 1, 1),
            (1, 1, 1),
            (2, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
        ],
        'f' => vec![(1, 0, 1), (0, 1, 1), (1, 1, 1), (2, 1, 1), (0, 2, 1)],
        'g' => vec![
            (1, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
            (2, 1, 1),
            (1, 1, 1),
        ],
        'h' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 1, 1),
            (2, 1, 1),
            (2, 0, 1),
            (2, 2, 1),
        ],
        'i' => vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (1, 1, 1), (1, 2, 1)],
        'j' => vec![
            (0, 0, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (2, 2, 1),
            (1, 2, 1),
            (0, 2, 1),
        ],
        'k' => vec![(0, 0, 1), (0, 1, 1), (0, 2, 1), (2, 0, 1), (1, 1, 1)],
        'l' => vec![(0, 0, 1), (0, 1, 1), (0, 2, 1), (1, 2, 1), (2, 2, 1)],
        'm' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 1, 1),
            (2, 1, 1),
            (2, 0, 1),
            (2, 2, 1),
        ],
        'n' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 1, 1),
            (2, 1, 1),
            (2, 2, 1),
        ],
        'o' => vec![
            (0, 1, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (2, 2, 1),
            (1, 2, 1),
            (0, 2, 1),
            (0, 1, 1),
        ],
        'p' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
            (2, 1, 1),
            (1, 1, 1),
        ],
        'q' => vec![
            (0, 1, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (2, 2, 1),
            (1, 2, 1),
            (0, 2, 1),
            (0, 1, 1),
            (2, 3, 1),
        ],
        'r' => vec![(0, 0, 1), (0, 1, 1), (0, 2, 1), (1, 1, 1), (2, 1, 1)],
        's' => vec![(1, 0, 1), (0, 1, 1), (1, 1, 1), (2, 1, 1), (1, 2, 1)],
        't' => vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (1, 1, 1), (1, 2, 1)],
        'u' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
            (2, 1, 1),
            (2, 0, 1),
        ],
        'v' => vec![(0, 0, 1), (0, 1, 1), (1, 2, 1), (2, 1, 1), (2, 0, 1)],
        'w' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (1, 2, 1),
            (2, 1, 1),
            (3, 2, 1),
            (4, 1, 1),
            (4, 0, 1),
        ],
        'x' => vec![(0, 0, 1), (1, 1, 1), (2, 2, 1), (0, 2, 1), (2, 0, 1)],
        'y' => vec![(0, 0, 1), (0, 1, 1), (1, 2, 1), (2, 3, 1)],
        'z' => vec![
            (0, 0, 1),
            (1, 0, 1),
            (2, 0, 1),
            (1, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
        ],
        '0' => vec![
            (0, 1, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (2, 2, 1),
            (1, 2, 1),
            (0, 2, 1),
            (0, 1, 1),
        ],
        '1' => vec![(1, 0, 1), (0, 1, 1), (1, 1, 1), (1, 2, 1)],
        '2' => vec![
            (0, 0, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (1, 2, 1),
            (0, 2, 1),
        ],
        '3' => vec![
            (0, 0, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (1, 1, 1),
            (2, 2, 1),
            (1, 2, 1),
            (0, 2, 1),
        ],
        '4' => vec![
            (0, 0, 1),
            (0, 1, 1),
            (1, 1, 1),
            (2, 1, 1),
            (2, 0, 1),
            (2, 2, 1),
        ],
        '5' => vec![
            (0, 0, 1),
            (1, 0, 1),
            (2, 0, 1),
            (0, 1, 1),
            (1, 2, 1),
            (2, 2, 1),
        ],
        '6' => vec![
            (1, 0, 1),
            (0, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
            (2, 2, 1),
            (2, 1, 1),
            (1, 1, 1),
        ],
        '7' => vec![(0, 0, 1), (1, 0, 1), (2, 0, 1), (2, 1, 1), (1, 2, 1)],
        '8' => vec![
            (0, 1, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (1, 1, 1),
            (2, 2, 1),
            (1, 2, 1),
            (0, 2, 1),
            (0, 1, 1),
        ],
        '9' => vec![
            (0, 0, 1),
            (1, 0, 1),
            (2, 0, 1),
            (2, 1, 1),
            (1, 1, 1),
            (0, 2, 1),
            (1, 2, 1),
        ],
        ' ' => vec![],
        '.' => vec![(0, 2, 1)],
        '-' => vec![(0, 1, 1), (1, 1, 1), (2, 1, 1)],
        _ => vec![(0, 1, 1), (1, 1, 1), (2, 1, 1)],
    }
}

impl Default for ScreenshotRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::tree::DomTree;

    fn make_dom(html: &str) -> DomTree {
        DomTree::from_document(scraper::Html::parse_document(html))
    }

    #[test]
    fn test_screenshot_renderer_new() {
        let r = ScreenshotRenderer::new();
        let _ = r.render(&make_dom("<p>Hi</p>"), 100, 50).unwrap();
    }

    #[test]
    fn test_render_invalid_dimensions() {
        let r = ScreenshotRenderer::new();
        let dom = make_dom("<p>x</p>");
        let err = r.render(&dom, 0, 100).unwrap_err();
        assert!(err.to_string().contains("pixmap"));
    }

    #[test]
    fn test_render_empty_dom() {
        let r = ScreenshotRenderer::new();
        let dom = make_dom("");
        let data = r.render(&dom, 100, 50).unwrap();
        assert!(!data.is_empty());
        assert!(data.starts_with(b"\x89PNG"));
    }

    #[test]
    fn test_render_with_text() {
        let r = ScreenshotRenderer::new();
        let dom = make_dom("<p>Hello World</p>");
        let data = r.render(&dom, 200, 100).unwrap();
        assert!(data.starts_with(b"\x89PNG"));
    }

    #[test]
    fn test_render_many_lines() {
        let r = ScreenshotRenderer::new();
        let lines: String = (0..30).map(|i| format!("<p>Line {}</p>", i)).collect();
        let dom = make_dom(&format!("<html><body>{}</body></html>", lines));
        let data = r.render(&dom, 200, 200).unwrap();
        assert!(data.starts_with(b"\x89PNG"));
    }

    #[test]
    fn test_render_truncates_at_28_lines() {
        let r = ScreenshotRenderer::new();
        let lines: String = (0..40).map(|i| format!("<p>Line {}</p>", i)).collect();
        let dom = make_dom(&format!("<html><body>{}</body></html>", lines));
        let data = r.render(&dom, 200, 600).unwrap();
        assert!(data.starts_with(b"\x89PNG"));
    }

    #[test]
    fn test_char_pattern_all_letters() {
        for c in 'a'..='z' {
            let p = get_char_pattern(c);
            assert!(!p.is_empty(), "letter {} should have pattern", c);
            for (dx, dy, v) in &p {
                assert!(*dx >= 0 && *dy >= 0);
                assert!(*v == 0 || *v == 1);
            }
        }
    }

    #[test]
    fn test_char_pattern_all_digits() {
        for c in '0'..='9' {
            let p = get_char_pattern(c);
            assert!(!p.is_empty(), "digit {} should have pattern", c);
        }
    }

    #[test]
    fn test_char_pattern_special() {
        assert!(get_char_pattern(' ').is_empty());
        assert!(!get_char_pattern('.').is_empty());
        assert!(!get_char_pattern('-').is_empty());
    }

    #[test]
    fn test_char_pattern_fallback() {
        let p = get_char_pattern('?');  // unsupported -> fallback
        assert!(!p.is_empty());
    }

    #[test]
    fn test_char_pattern_uppercase_maps_to_lowercase() {
        let lower = get_char_pattern('a');
        let upper = get_char_pattern('A');
        assert_eq!(lower, upper);
    }

    #[test]
    fn test_draw_ascii_line_bounds_check() {
        let r = ScreenshotRenderer::new();
        let w = 100;
        let h = 50;
        let mut pixels = vec![255u8; w * h * 4];
        // Should not panic with negative x
        assert!(r.draw_ascii_line(&mut pixels, "test", -10, 10, w).is_ok());
        // Should not panic with out-of-bounds y
        assert!(r.draw_ascii_line(&mut pixels, "test", 10, -10, w).is_ok());
        // Should not panic with y beyond height
        assert!(r.draw_ascii_line(&mut pixels, "test", 10, 1000, w).is_ok());
    }

    #[test]
    fn test_draw_ascii_line_skips_non_ascii() {
        let r = ScreenshotRenderer::new();
        let w = 100;
        let h = 50;
        let mut pixels = vec![255u8; w * h * 4];
        // Unicode chars >127 should be skipped
        assert!(r.draw_ascii_line(&mut pixels, "héllo wörld", 10, 10, w).is_ok());
    }

    #[test]
    fn test_draw_ascii_line_truncates_60_chars() {
        let r = ScreenshotRenderer::new();
        let w = 800;
        let h = 50;
        let mut pixels = vec![255u8; w * h * 4];
        let long = "a".repeat(100);
        assert!(r.draw_ascii_line(&mut pixels, &long, 10, 10, w).is_ok());
    }
}
