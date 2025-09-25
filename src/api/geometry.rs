//! Geometry calculations - Pure width/height/emoji handling
//!
//! This module provides text measurement and box dimension calculations
//! without any color/theme dependencies. Ideal for layout engines that
//! need precise measurements and apply their own styling.
//!
//! # Unicode Support
//! - Emoji width calculation (🌟 = 2 columns)
//! - CJK character handling (中 = 2 columns)
//! - Grapheme cluster awareness
//! - ANSI code stripping before calculation
//!
//! # RSB MODULE_SPEC Compliance
//! - No cross-module dependencies
//! - Pure calculation functions
//! - Curated re-exports from existing boxy internals

use crate::{get_display_width, visual::{BoxStyle, NORMAL, ROUNDED, DOUBLE, HEAVY, ASCII}};

/// Text width and display metrics
#[derive(Debug, Clone, PartialEq)]
pub struct TextMetrics {
    /// Display width in terminal columns (handles emoji/CJK)
    pub display_width: usize,
    /// Character count (may differ from display_width)
    pub char_count: usize,
    /// Byte length (may differ from both above)
    pub byte_length: usize,
    /// Contains emoji or wide characters
    pub has_wide_chars: bool,
}

/// Box dimensions and layout constraints
#[derive(Debug, Clone, PartialEq)]
pub struct BoxDimensions {
    /// Total box width including borders
    pub total_width: usize,
    /// Inner content width (total_width - 2)
    pub inner_width: usize,
    /// Total box height including borders
    pub total_height: usize,
    /// Inner content height
    pub inner_height: usize,
    /// Horizontal padding applied
    pub h_padding: usize,
    /// Vertical padding applied
    pub v_padding: usize,
}

/// Calculate comprehensive text metrics
/// Handles emoji, CJK, grapheme clusters, ANSI stripping
pub fn get_text_metrics(text: &str) -> TextMetrics {
    let display_width = get_display_width(text);
    let char_count = text.chars().count();
    let byte_length = text.len();
    let has_wide_chars = display_width != char_count;

    TextMetrics {
        display_width,
        char_count,
        byte_length,
        has_wide_chars,
    }
}

/// Get text display width (main function)
/// Strips ANSI codes then calculates Unicode width
pub fn get_text_width(text: &str) -> usize {
    get_display_width(text)
}

/// Calculate box dimensions for given content and constraints
pub fn calculate_box_dimensions(
    content: &str,
    _style: BoxStyle,
    h_padding: usize,
    v_padding: usize,
    fixed_width: Option<usize>,
) -> BoxDimensions {
    // Use existing boxy calculation logic
    let content_width = if content.is_empty() {
        0
    } else {
        content.lines()
            .map(get_text_width)
            .max()
            .unwrap_or(0)
    };

    let inner_width = match fixed_width {
        Some(fw) => fw.saturating_sub(2), // Account for borders
        None => content_width + (2 * h_padding),
    };

    let total_width = inner_width + 2; // Add border width

    let content_lines = content.lines().count().max(1);
    let inner_height = content_lines + (2 * v_padding);
    let total_height = inner_height + 2; // Add border height

    BoxDimensions {
        total_width,
        inner_width,
        total_height,
        inner_height,
        h_padding,
        v_padding,
    }
}

/// Get available box styles (pure Unicode, no colors)
pub fn get_box_styles() -> Vec<(&'static str, BoxStyle)> {
    vec![
        ("normal", NORMAL),
        ("rounded", ROUNDED),
        ("double", DOUBLE),
        ("heavy", HEAVY),
        ("ascii", ASCII),
    ]
}

/// Validate if a box style name is supported
pub fn validate_box_style_name(style_name: &str) -> Result<BoxStyle, String> {
    match style_name {
        "normal" => Ok(NORMAL),
        "rounded" => Ok(ROUNDED),
        "double" => Ok(DOUBLE),
        "heavy" => Ok(HEAVY),
        "ascii" => Ok(ASCII),
        _ => Err(format!(
            "Invalid style '{}'. Valid: normal, rounded, double, heavy, ascii",
            style_name
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_width_calculation() {
        let metrics = get_text_metrics("Hello 🌟 World");
        assert_eq!(metrics.display_width, 13); // 🌟 takes 2 columns
        assert_eq!(metrics.char_count, 12);    // But only 1 character
        assert!(metrics.has_wide_chars);
    }

    #[test]
    fn test_cjk_width_calculation() {
        let metrics = get_text_metrics("Hello 中文 World");
        assert_eq!(metrics.display_width, 16); // 中文 = 4 columns
        assert_eq!(metrics.char_count, 14);    // But only 2 characters
        assert!(metrics.has_wide_chars);
    }

    #[test]
    fn test_ascii_text() {
        let metrics = get_text_metrics("Hello World");
        assert_eq!(metrics.display_width, 11);
        assert_eq!(metrics.char_count, 11);
        assert!(!metrics.has_wide_chars);
    }

    #[test]
    fn test_box_dimension_calculation() {
        let dims = calculate_box_dimensions("Hello World", NORMAL, 2, 1, None);

        assert_eq!(dims.inner_width, 15);  // 11 + 2*2 padding
        assert_eq!(dims.total_width, 17);  // 15 + 2 borders
        assert_eq!(dims.inner_height, 3);  // 1 line + 2*1 padding
        assert_eq!(dims.total_height, 5);  // 3 + 2 borders
    }

    #[test]
    fn test_fixed_width_calculation() {
        let dims = calculate_box_dimensions("Hello", NORMAL, 1, 1, Some(20));

        assert_eq!(dims.total_width, 20);
        assert_eq!(dims.inner_width, 18); // 20 - 2 borders
    }
}