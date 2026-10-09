//! Color tokens.
//!
//! Hex values use the splash `#xRRGGBB` form so they can be pasted straight
//! into `script_mod!` blocks (e.g. `draw_bg.color: #xFF6B35`). Helpers
//! [`hex_to_vec4`] / [`vec4_to_hex`] convert to the f32 form that
//! makepad's draw calls accept (e.g. `draw_bg.color: vec4(1.0, 0.42, 0.21, 1.0)`).

// Light mode — the canonical OctoStudio v0.5 design system.
pub const INK: &str = "#x1C1C1E";
pub const SECONDARY: &str = "#x8E8E93";
pub const ACCENT: &str = "#xFF6B35";
pub const ACCENT_HOVER: &str = "#xFF8866";
pub const ACCENT_SOFT: &str = "#xFFE9D6";
pub const WARM_HOVER: &str = "#xEADCD0";
pub const CARD_BG: &str = "#xFFF5EC";
pub const CHIP_BG: &str = "#xF5E6DD";
pub const CANVAS: &str = "#xF6F6F8";
pub const SURFACE: &str = "#xFFFFFF";
pub const HAIRLINE: &str = "#xE5E5EA";
pub const DANGER: &str = "#xB3541E";
pub const DANGER_HOVER: &str = "#xD66030";
pub const SUCCESS: &str = "#x2E7D5B";
pub const WARNING: &str = "#xB46A16";
pub const INFO: &str = "#x4A6FA5";
pub const PLACEHOLDER_FG: &str = "#xF5E6D2"; // splash `color_empty` for TextInputs
pub const PLACEHOLDER_HOVER: &str = "#xEFEEEF";

// Dark mode (reserved for v0.6.1 — declared now so widgets can branch).
pub const DARK_CANVAS: &str = "#x0B0B0D";
pub const DARK_SURFACE: &str = "#x17171A";
pub const DARK_INK: &str = "#xF5F5F7";
pub const DARK_SECONDARY: &str = "#x8E8E93";

/// Convert `#xRRGGBB` → `vec4(r, g, b, a)` f32 for makepad draw calls.
pub fn hex_to_vec4(hex: &str) -> [f32; 4] {
    let s = hex.trim_start_matches("#x").trim_start_matches('#');
    let v = u32::from_str_radix(s, 16).expect("hex color");
    [
        ((v >> 16) & 0xFF) as f32 / 255.0,
        ((v >>  8) & 0xFF) as f32 / 255.0,
        ( v        & 0xFF) as f32 / 255.0,
        1.0,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accent_parses_correctly() {
        let [r, g, b, a] = hex_to_vec4(ACCENT);
        assert!((r - 1.0).abs() < 1e-3);
        assert!((g - 0x6B as f32 / 255.0).abs() < 1e-3);
        assert!((b - 0x35 as f32 / 255.0).abs() < 1e-3);
        assert!((a - 1.0).abs() < 1e-3);
    }

    #[test]
    fn dark_mode_reserved_values() {
        assert!(DARK_CANVAS.starts_with("#x0"));
        assert!(DARK_SURFACE.starts_with("#x1"));
    }
}
