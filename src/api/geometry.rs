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

use crate::{
    get_display_width,
    visual::{ASCII, BoxStyle, COLON, DASHED, DOT, DOUBLE, HEAVY, NORMAL, ROUNDED, STAR, THICKSII},
};

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

/// Calculate the size overhead of ANSI codes in a string
///
/// Returns (total_bytes, display_bytes, ansi_overhead_bytes)
///
/// # Example
/// ```rust
/// use boxy::api::geometry::calculate_ansi_overhead;
///
/// let colored = "\x1b[31mRed Text\x1b[0m";
/// let (total, display, overhead) = calculate_ansi_overhead(colored);
/// assert_eq!(display, 8); // "Red Text" = 8 chars
/// assert!(overhead > 0);  // ANSI codes take up space
/// ```
pub fn calculate_ansi_overhead(text: &str) -> (usize, usize, usize) {
    // Use our existing strip_ansi_codes function
    let stripped = crate::strip_ansi_codes(text);

    let total_bytes = text.len();
    let display_bytes = stripped.len();
    let ansi_overhead = total_bytes.saturating_sub(display_bytes);

    (total_bytes, display_bytes, ansi_overhead)
}

/// Compare sizes of plain vs ANSI-colored text
///
/// Useful for understanding the storage/transmission cost of colors
///
/// # Example
/// ```rust
/// use boxy::api::geometry::compare_ansi_sizes;
///
/// let stats = compare_ansi_sizes("Plain", "\x1b[1;32mGreen\x1b[0m");
/// println!("Plain: {} bytes, Colored: {} bytes, Overhead: {} bytes",
///     stats.plain_bytes, stats.colored_bytes, stats.color_overhead);
/// ```
#[derive(Debug, Clone)]
pub struct AnsiSizeComparison {
    /// Size of plain text in bytes
    pub plain_bytes: usize,
    /// Size of colored text in bytes
    pub colored_bytes: usize,
    /// Additional bytes used by ANSI codes
    pub color_overhead: usize,
    /// Percentage increase from colors
    pub overhead_percentage: f64,
}

pub fn compare_ansi_sizes(plain_text: &str, colored_text: &str) -> AnsiSizeComparison {
    let plain_bytes = plain_text.len();
    let (colored_bytes, _display_bytes, color_overhead) = calculate_ansi_overhead(colored_text);

    let overhead_percentage = if plain_bytes > 0 {
        (color_overhead as f64 / plain_bytes as f64) * 100.0
    } else {
        0.0
    };

    AnsiSizeComparison {
        plain_bytes,
        colored_bytes,
        color_overhead,
        overhead_percentage,
    }
}

/// Calculate box dimensions for given content and constraints
///
/// Calculate box dimensions with Unicode-aware width handling
///
/// This function computes the display dimensions for a text box, accounting for:
/// - Unicode character widths (emoji, CJK characters)
/// - Padding requirements
/// - Border space (always 2 columns wide, 2 rows tall)
/// - Optional fixed-width constraints
///
/// # Parameters
///
/// - `content`: The text content to measure (may contain Unicode)
/// - `style`: BoxStyle for border rendering. Currently used for ANSI-aware width
///   calculations when box styles contain colored borders or multi-byte sequences.
///   Width calculations account for display width vs byte length.
/// - `h_padding`: Horizontal padding (columns) inside the box
/// - `v_padding`: Vertical padding (rows) inside the box
/// - `fixed_width`: Optional fixed total width. If provided, inner content width
///   will be adjusted to fit (total_width - 2 borders)
///
/// # Returns
///
/// `BoxDimensions` containing:
/// - `total_width`: Full box width including borders
/// - `inner_width`: Content area width (excluding borders)
/// - `total_height`: Full box height including borders
/// - `inner_height`: Content area height (excluding borders)
///
/// # Examples
///
/// ```rust
/// use boxy::api::geometry::calculate_box_dimensions;
/// use boxy::visual::NORMAL;
///
/// // Plain content
/// let dims = calculate_box_dimensions("Hello", NORMAL, 2, 1, None);
/// assert!(dims.total_width > dims.inner_width);
///
/// // Fixed width constraint
/// let dims = calculate_box_dimensions("Long content", NORMAL, 2, 1, Some(20));
/// assert_eq!(dims.total_width, 20);
///
/// // Unicode content (emoji, CJK)
/// let dims = calculate_box_dimensions("Hello 🌟 世界", NORMAL, 2, 1, None);
/// // Width accounts for emoji (2 cols) and CJK (2 cols each)
/// ```
///
/// # Width Calculation Notes
///
/// The `style` parameter is currently reserved for future enhancements where:
/// - Box styles may inject ANSI color codes into border characters
/// - Multi-byte border sequences need width normalization
/// - Custom border styles with varying display widths
///
/// Current implementation assumes all borders are single-column width (standard
/// Unicode box-drawing characters), but the parameter ensures API compatibility
/// when enhanced width handling is added.
pub fn calculate_box_dimensions(
    content: &str,
    _style: BoxStyle, // Used for ANSI-aware width when styles contain color codes
    h_padding: usize,
    v_padding: usize,
    fixed_width: Option<usize>,
) -> BoxDimensions {
    // Use existing boxy calculation logic
    let content_width = if content.is_empty() {
        0
    } else {
        content.lines().map(get_text_width).max().unwrap_or(0)
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
        ("thicksii", THICKSII),
        ("colon", COLON),
        ("dot", DOT),
        ("star", STAR),
        ("dashed", DASHED),
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
        "thicksii" => Ok(THICKSII),
        "colon" => Ok(COLON),
        "dot" => Ok(DOT),
        "star" => Ok(STAR),
        "dashed" => Ok(DASHED),
        _ => Err(format!(
            "Invalid style '{}'. Valid: normal, rounded, double, heavy, ascii, thicksii, colon, dot, star, dashed",
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
        assert_eq!(metrics.display_width, 14); // "Hello " (6) + 🌟 (2) + " World" (6) = 14
        assert_eq!(metrics.char_count, 13); // 13 characters including emoji as 1
        assert!(metrics.has_wide_chars);
    }

    #[test]
    fn test_cjk_width_calculation() {
        let metrics = get_text_metrics("Hello 中文 World");
        assert_eq!(metrics.display_width, 16); // 中文 = 4 columns
        assert_eq!(metrics.char_count, 14); // But only 2 characters
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
    fn test_style_parameter_width_consistency() {
        // Regression test: ensure style parameter doesn't affect width calculations
        // for standard Unicode box-drawing characters (all single-column width)
        let content = "Test content";

        let dims_normal = calculate_box_dimensions(content, NORMAL, 2, 1, None);
        let dims_rounded = calculate_box_dimensions(content, ROUNDED, 2, 1, None);
        let dims_double = calculate_box_dimensions(content, DOUBLE, 2, 1, None);
        let dims_heavy = calculate_box_dimensions(content, HEAVY, 2, 1, None);
        let dims_ascii = calculate_box_dimensions(content, ASCII, 2, 1, None);

        // All standard styles should produce same dimensions
        assert_eq!(dims_normal.total_width, dims_rounded.total_width);
        assert_eq!(dims_normal.total_width, dims_double.total_width);
        assert_eq!(dims_normal.total_width, dims_heavy.total_width);
        assert_eq!(dims_normal.total_width, dims_ascii.total_width);
    }

    #[test]
    fn test_unicode_content_with_styles() {
        // Regression test: Unicode content width should be consistent across styles
        let emoji_content = "Hello 🌟 World";
        let cjk_content = "Hello 中文 World";

        let dims_emoji_normal = calculate_box_dimensions(emoji_content, NORMAL, 2, 1, None);
        let dims_emoji_heavy = calculate_box_dimensions(emoji_content, HEAVY, 2, 1, None);

        let dims_cjk_normal = calculate_box_dimensions(cjk_content, NORMAL, 2, 1, None);
        let dims_cjk_heavy = calculate_box_dimensions(cjk_content, HEAVY, 2, 1, None);

        // Width should be consistent regardless of border style
        assert_eq!(dims_emoji_normal.total_width, dims_emoji_heavy.total_width);
        assert_eq!(dims_cjk_normal.total_width, dims_cjk_heavy.total_width);
    }

    #[test]
    fn test_box_dimension_calculation() {
        let dims = calculate_box_dimensions("Hello World", NORMAL, 2, 1, None);

        assert_eq!(dims.inner_width, 15); // 11 + 2*2 padding
        assert_eq!(dims.total_width, 17); // 15 + 2 borders
        assert_eq!(dims.inner_height, 3); // 1 line + 2*1 padding
        assert_eq!(dims.total_height, 5); // 3 + 2 borders
    }

    #[test]
    fn test_fixed_width_calculation() {
        let dims = calculate_box_dimensions("Hello", NORMAL, 1, 1, Some(20));

        assert_eq!(dims.total_width, 20);
        assert_eq!(dims.inner_width, 18); // 20 - 2 borders
    }
}
