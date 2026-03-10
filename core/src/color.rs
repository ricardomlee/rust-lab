/// Color conversion utilities for hex, RGB, and HSL formats.

/// Parse a hex color string (#RGB, #RRGGBB, #RRGGBBAA) into RGB(A) components.
pub fn hex_to_rgba(hex: &str) -> Result<(u8, u8, u8, u8), String> {
    let hex = hex.trim_start_matches('#');
    
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            Ok((r, g, b, 255))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            Ok((r, g, b, 255))
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            let a = u8::from_str_radix(&hex[6..8], 16)
                .map_err(|e| format!("Invalid hex: {}", e))?;
            Ok((r, g, b, a))
        }
        _ => Err(format!("Invalid hex length: expected 3, 6, or 8, got {}", hex.len())),
    }
}

/// Convert RGB to hex string (#RRGGBB).
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Convert RGBA to hex string with alpha (#RRGGBBAA).
pub fn rgba_to_hex(r: u8, g: u8, b: u8, a: u8) -> String {
    format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
}

/// RGB to HSL conversion.
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Lightness
    let l = (max + min) / 2.0;

    // Saturation
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    // Hue
    let h = if delta == 0.0 {
        0.0
    } else {
        let hue = match max {
            x if x == r => ((g - b) / delta + (if g < b { 6.0 } else { 0.0 })) / 6.0,
            x if x == g => ((b - r) / delta + 2.0) / 6.0,
            _ => ((r - g) / delta + 4.0) / 6.0,
        };
        hue
    };

    (h * 360.0, s * 100.0, l * 100.0)
}

/// HSL to RGB conversion.
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let h = (h % 360.0) / 360.0;
    let s = s / 100.0;
    let l = l / 100.0;

    if s == 0.0 {
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    let hue_to_rgb = |mut t: f64| {
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 1.0 / 2.0 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };

    let r = (hue_to_rgb(h + 1.0 / 3.0) * 255.0).round() as u8;
    let g = (hue_to_rgb(h) * 255.0).round() as u8;
    let b = (hue_to_rgb(h - 1.0 / 3.0) * 255.0).round() as u8;

    (r, g, b)
}

/// Format HSL as CSS string.
pub fn format_hsl(h: f64, s: f64, l: f64) -> String {
    format!("hsl({:.1}, {:.1}%, {:.1}%)", h, s, l)
}

/// Format RGBA as CSS string.
pub fn format_rgba(r: u8, g: u8, b: u8, a: u8) -> String {
    format!("rgba({}, {}, {}, {:.2})", r, g, b, a as f64 / 255.0)
}

/// Linear interpolation between two values.
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Blend two colors with a given ratio (0.0 = color1, 1.0 = color2).
pub fn blend_colors(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8, ratio: f64) -> (u8, u8, u8) {
    let t = ratio.clamp(0.0, 1.0);
    let r = lerp(r1 as f64, r2 as f64, t).round() as u8;
    let g = lerp(g1 as f64, g2 as f64, t).round() as u8;
    let b = lerp(b1 as f64, b2 as f64, t).round() as u8;
    (r, g, b)
}

/// Generate a color palette by interpolating between two colors.
pub fn generate_palette(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8, steps: usize) -> Vec<(u8, u8, u8)> {
    if steps <= 1 {
        return vec![(r1, g1, b1)];
    }
    (0..steps)
        .map(|i| {
            let t = if steps == 1 { 0.0 } else { i as f64 / (steps - 1) as f64 };
            blend_colors(r1, g1, b1, r2, g2, b2, t)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgba_short() {
        assert_eq!(hex_to_rgba("#FFF").unwrap(), (255, 255, 255, 255));
        assert_eq!(hex_to_rgba("#F00").unwrap(), (255, 0, 0, 255));
        assert_eq!(hex_to_rgba("#0F0").unwrap(), (0, 255, 0, 255));
    }

    #[test]
    fn test_hex_to_rgba_long() {
        assert_eq!(hex_to_rgba("#FFFFFF").unwrap(), (255, 255, 255, 255));
        assert_eq!(hex_to_rgba("#FF0000").unwrap(), (255, 0, 0, 255));
        assert_eq!(hex_to_rgba("#00FF00").unwrap(), (0, 255, 0, 255));
    }

    #[test]
    fn test_hex_to_rgba_alpha() {
        assert_eq!(hex_to_rgba("#FF000080").unwrap(), (255, 0, 0, 128));
        assert_eq!(hex_to_rgba("#00000000").unwrap(), (0, 0, 0, 0));
    }

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(255, 255, 255), "#FFFFFF");
        assert_eq!(rgb_to_hex(255, 0, 0), "#FF0000");
        assert_eq!(rgb_to_hex(0, 0, 0), "#000000");
    }

    #[test]
    fn test_rgb_to_hsl() {
        let (h, s, l) = rgb_to_hsl(255, 0, 0);
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 100.0).abs() < 1.0);
        assert!((l - 50.0).abs() < 1.0);

        let (h, s, l) = rgb_to_hsl(0, 255, 0);
        assert!((h - 120.0).abs() < 1.0);

        let (h, s, l) = rgb_to_hsl(0, 0, 255);
        assert!((h - 240.0).abs() < 1.0);
    }

    #[test]
    fn test_hsl_to_rgb() {
        let (r, g, b) = hsl_to_rgb(0.0, 100.0, 50.0);
        assert_eq!(r, 255);
        assert!(g == 0 || g == 1);
        assert!(b == 0 || b == 1);

        let (r, g, b) = hsl_to_rgb(120.0, 100.0, 50.0);
        assert!(r == 0 || r == 1);
        assert_eq!(g, 255);
        assert!(b == 0 || b == 1);
    }

    #[test]
    fn test_roundtrip_rgb_hsl() {
        let (r, g, b) = (128, 64, 192);
        let (h, s, l) = rgb_to_hsl(r, g, b);
        let (r2, g2, b2) = hsl_to_rgb(h, s, l);
        assert!((r as i16 - r2 as i16).abs() <= 2);
        assert!((g as i16 - g2 as i16).abs() <= 2);
        assert!((b as i16 - b2 as i16).abs() <= 2);
    }

    #[test]
    fn test_blend_colors() {
        // Blend white to black at 50%
        let (r, g, b) = blend_colors(255, 255, 255, 0, 0, 0, 0.5);
        assert!(r >= 127 && r <= 128);
        assert!(g >= 127 && g <= 128);
        assert!(b >= 127 && b <= 128);

        // Blend red to blue at 50%
        let (r, g, b) = blend_colors(255, 0, 0, 0, 0, 255, 0.5);
        assert!(r >= 127 && r <= 128);
        assert_eq!(g, 0);
        assert!(b >= 127 && b <= 128);

        // Ratio 0 = first color
        assert_eq!(blend_colors(100, 150, 200, 50, 75, 100, 0.0), (100, 150, 200));

        // Ratio 1 = second color
        assert_eq!(blend_colors(100, 150, 200, 50, 75, 100, 1.0), (50, 75, 100));
    }

    #[test]
    fn test_generate_palette() {
        let palette = generate_palette(255, 0, 0, 0, 0, 255, 5);
        assert_eq!(palette.len(), 5);
        
        // First should be red
        assert_eq!(palette[0], (255, 0, 0));
        
        // Last should be blue
        assert_eq!(palette[4], (0, 0, 255));
        
        // Middle should be purple-ish
        let (r, g, b) = palette[2];
        assert!(r >= 127 && r <= 128);
        assert_eq!(g, 0);
        assert!(b >= 127 && b <= 128);
    }

    #[test]
    fn test_generate_palette_single_step() {
        let palette = generate_palette(100, 150, 200, 50, 75, 100, 1);
        assert_eq!(palette.len(), 1);
        assert_eq!(palette[0], (100, 150, 200));
    }
}
