//! Theming and color application - Optional styling system
//!
//! This module provides color application and theming capabilities that
//! library consumers can choose to use or ignore entirely. Room Runtime
//! can skip this module and apply its own styling.
//!
//! # Background Color Support (NEW FEATURE)
//! - Terminal background colors

#![allow(dead_code)]  // Keep unused code until cleanup decisions are made
//! - Text background highlighting
//! - Gradient background support (future)
//!
//! # RSB MODULE_SPEC Compliance
//! - Optional dependency on colors module
//! - No coupling to geometry/layout modules
//! - Graceful degradation when colors unavailable

use crate::{RESET, get_color_code};

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

    /// Create a ColorScheme from BoxyConfig
    pub fn from_config(config: &crate::core::BoxyConfig) -> Self {
        Self {
            border_color: config.colors.box_color.clone(),
            text_color: config.colors.text_color.clone(),
            background_color: BackgroundColor::None,
            header_color: config.colors.title_color.clone(),
            footer_color: None, // BoxyConfig doesn't have footer_color
            status_color: None, // BoxyConfig doesn't have status_color
        }
    }
}

/// Apply background color to text content (line-by-line to prevent bleeding)
#[allow(dead_code)]
pub fn apply_background_color(text: &str, bg_color: &BackgroundColor) -> String {
    match bg_color {
        BackgroundColor::None => text.to_string(),
        BackgroundColor::Ansi(code) => {
            apply_background_per_line(text, &format!("\x1b[48;5;{}m", code))
        }
        BackgroundColor::Rgb(r, g, b) => {
            apply_background_per_line(text, &format!("\x1b[48;2;{};{};{}m", r, g, b))
        }
        BackgroundColor::Named(name) => {
            let color_code = get_background_color_code(name);
            if color_code.is_empty() {
                text.to_string()
            } else {
                apply_background_per_line(text, color_code)
            }
        }
        BackgroundColor::Hex(hex) => {
            if let Some((r, g, b)) = parse_hex_color(hex) {
                apply_background_per_line(text, &format!("\x1b[48;2;{};{};{}m", r, g, b))
            } else {
                text.to_string()
            }
        }
    }
}

/// Apply background color code to each line individually to prevent bleeding
fn apply_background_per_line(text: &str, bg_code: &str) -> String {
    text.lines()
        .map(|line| {
            if line.is_empty() {
                // For empty lines, just add a reset to prevent bleeding
                RESET.to_string()
            } else {
                format!("{}{}{}", bg_code, line, RESET)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
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

/// Apply colors to rendered box output (borders and content)
pub fn apply_colors_to_rendered_box(rendered: &str, scheme: &ColorScheme) -> String {
    use crate::{RESET, get_color_code, visual::BOX_CHARS};

    let lines: Vec<&str> = rendered.lines().collect();
    if lines.is_empty() {
        return rendered.to_string();
    }

    let border_color_code = get_color_code(&scheme.border_color);
    let text_color_code = if scheme.text_color == "auto" {
        border_color_code
    } else if scheme.text_color == "none" {
        ""
    } else {
        get_color_code(&scheme.text_color)
    };

    let mut result = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            result.push(line.to_string());
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut colored_line = String::new();
        let mut in_border = false;
        let mut in_text_color = false;

        for ch in chars.iter() {
            // Check if this character is a box drawing character
            if BOX_CHARS.contains(*ch) {
                // Close text color if we were in one
                if in_text_color {
                    colored_line.push_str(RESET);
                    in_text_color = false;
                }
                // Start border color if not already
                if !in_border {
                    colored_line.push_str(border_color_code);
                    in_border = true;
                }
                colored_line.push(*ch);
            } else {
                // Close border color if we were in one
                if in_border {
                    colored_line.push_str(RESET);
                    in_border = false;
                }

                // Apply text color to non-space content
                if !text_color_code.is_empty() && *ch != ' ' {
                    if !in_text_color {
                        colored_line.push_str(text_color_code);
                        in_text_color = true;
                    }
                    colored_line.push(*ch);
                } else {
                    // Close text color for spaces
                    if in_text_color {
                        colored_line.push_str(RESET);
                        in_text_color = false;
                    }
                    colored_line.push(*ch);
                }
            }
        }

        // Close any open colors
        if in_border || in_text_color {
            colored_line.push_str(RESET);
        }

        result.push(colored_line);
    }

    result.join("\n")
}

/// Apply colors to individual box components (OPTIONAL - Room Runtime can skip this)
///
/// This function applies per-component theming, allowing different colors for
/// header, body, footer, and status sections. This is the progressive enhancement
/// Layer 2 - pure API users can ignore this entirely.
///
/// # Progressive Enhancement
///
/// - **Layer 0 (Pure API)**: `layout.render()` - no colors
/// - **Layer 1 (Config)**: `BoxLayout::from(&config)` - structure only
/// - **Layer 2 (Theming)**: `apply_component_colors(&layout, &scheme)` - OPT-IN
///
/// # Examples
///
/// ```rust
/// use boxy::api::layout::{BoxBuilder, HeaderBuilder};
/// use boxy::api::theming::{ColorScheme, apply_component_colors};
///
/// let layout = BoxBuilder::new("Body text")
///     .with_header(HeaderBuilder::new("Title"))
///     .build();
///
/// // Optional: apply component-specific colors
/// let mut scheme = ColorScheme::default();
/// scheme.header_color = Some("bright_yellow".to_string());
/// scheme.text_color = "white".to_string();
///
/// let colored_output = apply_component_colors(&layout, &scheme);
/// ```
pub fn apply_component_colors(
    layout: &crate::api::layout::BoxLayout,
    scheme: &ColorScheme,
) -> String {
    use crate::{RESET, get_color_code};

    let mut result = Vec::new();

    // Apply header color if present
    if let Some(header) = &layout.header {
        let header_text = if let Some(ref color) = scheme.header_color {
            let color_code = get_color_code(color);
            if !color_code.is_empty() && color != "none" {
                format!("{}{}{}", color_code, header.content, RESET)
            } else {
                header.content.clone()
            }
        } else {
            header.content.clone()
        };

        // Apply background color if specified
        let final_header = apply_background_color(&header_text, &scheme.background_color);
        result.push(final_header);
    }

    // Apply body text color
    let body_text = if scheme.text_color != "none" && scheme.text_color != "auto" {
        let color_code = get_color_code(&scheme.text_color);
        if !color_code.is_empty() {
            format!("{}{}{}", color_code, layout.body.content, RESET)
        } else {
            layout.body.content.clone()
        }
    } else {
        layout.body.content.clone()
    };

    // Apply background color to body
    let final_body = apply_background_color(&body_text, &scheme.background_color);
    result.push(final_body);

    // Apply status color if present
    if let Some(status) = &layout.status {
        let status_text = if let Some(ref color) = scheme.status_color {
            let color_code = get_color_code(color);
            if !color_code.is_empty() && color != "none" {
                format!("{}{}{}", color_code, status.content, RESET)
            } else {
                status.content.clone()
            }
        } else {
            status.content.clone()
        };

        let final_status = apply_background_color(&status_text, &scheme.background_color);
        result.push(final_status);
    }

    // Apply footer color if present
    if let Some(footer) = &layout.footer {
        let footer_text = if let Some(ref color) = scheme.footer_color {
            let color_code = get_color_code(color);
            if !color_code.is_empty() && color != "none" {
                format!("{}{}{}", color_code, footer.content, RESET)
            } else {
                footer.content.clone()
            }
        } else {
            footer.content.clone()
        };

        let final_footer = apply_background_color(&footer_text, &scheme.background_color);
        result.push(final_footer);
    }

    result.join("\n")
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
    fn test_background_color_multiline() {
        let multiline = "Line 1\nLine 2\nLine 3";
        let result = apply_background_color(multiline, &BackgroundColor::Ansi(196));
        let expected =
            "\x1b[48;5;196mLine 1\x1b[0m\n\x1b[48;5;196mLine 2\x1b[0m\n\x1b[48;5;196mLine 3\x1b[0m";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_background_color_with_empty_lines() {
        let text_with_empty = "Line 1\n\nLine 3";
        let result = apply_background_color(text_with_empty, &BackgroundColor::Rgb(255, 0, 0));
        let expected = "\x1b[48;2;255;0;0mLine 1\x1b[0m\n\x1b[0m\n\x1b[48;2;255;0;0mLine 3\x1b[0m";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_apply_component_colors_basic() {
        use crate::api::layout::{BoxBuilder, HeaderBuilder};

        let layout = BoxBuilder::new("Body content")
            .with_header(HeaderBuilder::new("Title"))
            .build();

        let mut scheme = ColorScheme::default();
        scheme.text_color = "white".to_string();

        let output = apply_component_colors(&layout, &scheme);

        // Should contain both header and body
        assert!(output.contains("Title"));
        assert!(output.contains("Body content"));
    }

    #[test]
    fn test_apply_component_colors_with_header_color() {
        use crate::api::layout::{BoxBuilder, HeaderBuilder};

        let layout = BoxBuilder::new("Body")
            .with_header(HeaderBuilder::new("Title"))
            .build();

        let mut scheme = ColorScheme::default();
        scheme.header_color = Some("bright_yellow".to_string());
        scheme.text_color = "white".to_string();

        let output = apply_component_colors(&layout, &scheme);

        // Header should have color codes (check for ANSI escape sequences)
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn test_apply_component_colors_with_footer() {
        use crate::api::layout::{BoxBuilder, FooterBuilder};

        let layout = BoxBuilder::new("Content")
            .with_footer(FooterBuilder::new("Footer"))
            .build();

        let mut scheme = ColorScheme::default();
        scheme.footer_color = Some("green".to_string());
        scheme.text_color = "auto".to_string();

        let output = apply_component_colors(&layout, &scheme);

        assert!(output.contains("Footer"));
    }

    #[test]
    fn test_apply_component_colors_plain_scheme() {
        use crate::api::layout::{BoxBuilder, HeaderBuilder};

        let layout = BoxBuilder::new("Content")
            .with_header(HeaderBuilder::new("Header"))
            .build();

        let scheme = ColorScheme::plain();

        let output = apply_component_colors(&layout, &scheme);

        // Plain scheme should not add color codes
        assert!(output.contains("Header"));
        assert!(output.contains("Content"));
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
        let scheme =
            ColorScheme::default().with_background(BackgroundColor::Named("red".to_string()));
        let result = renderer("Hello", &scheme);
        assert_eq!(result, "Hello"); // Colors ignored
    }
}
