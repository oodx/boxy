//! Theming and color application - Optional styling system
//!
//! This module provides color application and theming capabilities that
//! library consumers can choose to use or ignore entirely. Room Runtime
//! can skip this module and apply its own styling.
//!
//! # Background Color Support (NEW FEATURE)
//! - Terminal background colors
//! - Text background highlighting
//! - Gradient background support (future)
//!
//! # RSB MODULE_SPEC Compliance
//! - Optional dependency on colors module
//! - No coupling to geometry/layout modules
//! - Graceful degradation when colors unavailable

use crate::{get_color_code, RESET};

/// Background color specification
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundColor {
    /// No background color (transparent)
    None,
    /// ANSI color code (0-255)
    Ansi(u8),
    /// RGB color (r, g, b)
    Rgb(u8, u8, u8),
    /// Named color (maps to ANSI)
    Named(String),
    /// Hex color (#RRGGBB)
    Hex(String),
}

/// Complete color scheme for box components
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// Border color
    pub border_color: String,
    /// Text color
    pub text_color: String,
    /// Background color (NEW)
    pub background_color: BackgroundColor,
    /// Header text color
    pub header_color: Option<String>,
    /// Footer text color
    pub footer_color: Option<String>,
    /// Status text color
    pub status_color: Option<String>,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            border_color: "white".to_string(),
            text_color: "auto".to_string(),
            background_color: BackgroundColor::None,
            header_color: None,
            footer_color: None,
            status_color: None,
        }
    }
}

impl ColorScheme {
    /// Create a plain scheme (no colors)
    pub fn plain() -> Self {
        Self {
            border_color: "none".to_string(),
            text_color: "none".to_string(),
            background_color: BackgroundColor::None,
            header_color: None,
            footer_color: None,
            status_color: None,
        }
    }

    /// Create a scheme with background color
    pub fn with_background(mut self, bg_color: BackgroundColor) -> Self {
        self.background_color = bg_color;
        self
    }
}

/// Apply background color to text content
pub fn apply_background_color(text: &str, bg_color: &BackgroundColor) -> String {
    match bg_color {
        BackgroundColor::None => text.to_string(),
        BackgroundColor::Ansi(code) => {
            format!("\x1b[48;5;{}m{}{}", code, text, RESET)
        }
        BackgroundColor::Rgb(r, g, b) => {
            format!("\x1b[48;2;{};{};{}m{}{}", r, g, b, text, RESET)
        }
        BackgroundColor::Named(name) => {
            let color_code = get_background_color_code(name);
            if color_code.is_empty() {
                text.to_string()
            } else {
                format!("{}{}{}", color_code, text, RESET)
            }
        }
        BackgroundColor::Hex(hex) => {
            if let Some((r, g, b)) = parse_hex_color(hex) {
                format!("\x1b[48;2;{};{};{}m{}{}", r, g, b, text, RESET)
            } else {
                text.to_string()
            }
        }
    }
}

/// Get ANSI background color code for named colors
fn get_background_color_code(name: &str) -> &'static str {
    match name.to_lowercase().as_str() {
        "black" => "\x1b[40m",
        "red" => "\x1b[41m",
        "green" => "\x1b[42m",
        "yellow" => "\x1b[43m",
        "blue" => "\x1b[44m",
        "magenta" => "\x1b[45m",
        "cyan" => "\x1b[46m",
        "white" => "\x1b[47m",
        // Bright colors
        "bright_black" => "\x1b[100m",
        "bright_red" => "\x1b[101m",
        "bright_green" => "\x1b[102m",
        "bright_yellow" => "\x1b[103m",
        "bright_blue" => "\x1b[104m",
        "bright_magenta" => "\x1b[105m",
        "bright_cyan" => "\x1b[106m",
        "bright_white" => "\x1b[107m",
        _ => "",
    }
}

/// Parse hex color string to RGB values
fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

/// Apply color scheme to text content
pub fn apply_colors(content: &str, scheme: &ColorScheme) -> String {
    let mut result = content.to_string();

    // Apply background color first
    result = apply_background_color(&result, &scheme.background_color);

    // Apply text color if not "none"
    if scheme.text_color != "none" && scheme.text_color != "auto" {
        let color_code = get_color_code(&scheme.text_color);
        if !color_code.is_empty() {
            result = format!("{}{}{}", color_code, result, RESET);
        }
    }

    result
}

/// Create a renderer that ignores colors (for Room Runtime)
pub fn create_plain_renderer() -> impl Fn(&str, &ColorScheme) -> String {
    |content: &str, _scheme: &ColorScheme| content.to_string()
}

/// Create a renderer that applies full theming
pub fn create_themed_renderer() -> impl Fn(&str, &ColorScheme) -> String {
    |content: &str, scheme: &ColorScheme| apply_colors(content, scheme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_color_none() {
        let result = apply_background_color("Hello", &BackgroundColor::None);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_background_color_ansi() {
        let result = apply_background_color("Hello", &BackgroundColor::Ansi(196));
        assert_eq!(result, "\x1b[48;5;196mHello\x1b[0m");
    }

    #[test]
    fn test_background_color_rgb() {
        let result = apply_background_color("Hello", &BackgroundColor::Rgb(255, 0, 0));
        assert_eq!(result, "\x1b[48;2;255;0;0mHello\x1b[0m");
    }

    #[test]
    fn test_background_color_named() {
        let result = apply_background_color("Hello", &BackgroundColor::Named("red".to_string()));
        assert_eq!(result, "\x1b[41mHello\x1b[0m");
    }

    #[test]
    fn test_background_color_hex() {
        let result = apply_background_color("Hello", &BackgroundColor::Hex("#FF0000".to_string()));
        assert_eq!(result, "\x1b[48;2;255;0;0mHello\x1b[0m");
    }

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_hex_color("#FF0000"), Some((255, 0, 0)));
        assert_eq!(parse_hex_color("00FF00"), Some((0, 255, 0)));
        assert_eq!(parse_hex_color("#INVALID"), None);
        assert_eq!(parse_hex_color("#FFF"), None);
    }

    #[test]
    fn test_plain_renderer() {
        let renderer = create_plain_renderer();
        let scheme = ColorScheme::default().with_background(BackgroundColor::Named("red".to_string()));
        let result = renderer("Hello", &scheme);
        assert_eq!(result, "Hello"); // Colors ignored
    }
}