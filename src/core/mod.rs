//! Core module - RSB MODULE_SPEC compliant core system
//!
//! This module provides comprehensive core functionality for boxy including:
//! - Configuration management (BoxyConfig and related types)
//! - Content parsing and processing with PROTECTED icon detection logic
//! - Text wrapping and formatting utilities
//! - Help system and comprehensive documentation
//!
//! Consolidated from config.rs, parser.rs, and help.rs following RSB MODULE_SPEC.
//!
//! CRITICAL: Preserves exact icon detection logic from parser.rs:385-410
//!
//! Version: boxy v0.16.0+ (RSB MODULE_SPEC reorganization)

pub mod utils;
pub mod helpers;

// Re-export public API (curated, no wildcards per RSB MODULE_SPEC)
pub use utils::{
    // Constants
    VERSION,
    NAME,
    DESCRIPTION,

    // Configuration types
    BodyAlignment,
    WidthConfig,
    BoxColors,
    DividerConfig,
    PaddingConfig,
    AlignmentConfig,
    BoxyConfig,
    ParsedContent,

    // Configuration functions
    resolve_box_config,

    // Parser functions with CRITICAL icon detection logic
    expand_variables,
    unescape_stream_value,
    parse_content_stream,
    wrap_text_at_word_boundaries,
    truncate_with_ellipsis,
    render_title_or_footer,

    // Help functions
    show_comprehensive_help,
    show_usage_examples,
};

// Test module access to helpers
#[cfg(test)]
pub use helpers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_module_integration() {
        // Test that all main components are accessible
        let config = BoxyConfig::default();
        assert_eq!(config.text, "");
        assert!(config.title.is_none());
    }

    #[test]
    fn test_config_creation() {
        use crate::visual::BoxStyle;

        let config = resolve_box_config(
            "test content",
            2, 1,
            &BoxStyle::default(),
            "blue",
            "white",
            Some("Test Title"),
            None, None, None, None, None,
            "left", "left", None,
            false, false, false, false,
            false, false, false, false,
            None, None,
            "left", false, false, false,
            None, None, false,
        );

        assert_eq!(config.text, "test content");
        assert_eq!(config.title, Some("Test Title".to_string()));
        assert_eq!(config.colors.box_color, "blue");
        assert_eq!(config.colors.text_color, "white");
        assert_eq!(config.width.h_padding, 2);
    }

    #[test]
    fn test_body_alignment_conversion() {
        assert_eq!(BodyAlignment::from("center"), BodyAlignment::Center);
        assert_eq!(BodyAlignment::from("right"), BodyAlignment::Right);
        assert_eq!(BodyAlignment::from("left"), BodyAlignment::Left);
        assert_eq!(BodyAlignment::from("invalid"), BodyAlignment::Left);
    }

    #[test]
    fn test_variable_expansion() {
        unsafe {
            std::env::set_var("TEST_VAR", "expanded");
        }
        let result = expand_variables("Hello $TEST_VAR world");
        assert_eq!(result, "Hello expanded world");
        unsafe {
            std::env::remove_var("TEST_VAR");
        }
    }

    #[test]
    fn test_unescape_stream_value() {
        assert_eq!(unescape_stream_value("hello\\nworld"), "hello\nworld");
        assert_eq!(unescape_stream_value("hello\\tworld"), "hello\tworld");
        assert_eq!(unescape_stream_value("hello/nworld"), "hello\nworld");
    }

    #[test]
    fn test_parse_content_stream() {
        let input = "tl='Test Title'; st='Status Text'; hd='Header';";
        let result = parse_content_stream(input).unwrap();

        assert_eq!(result.title, Some("Test Title".to_string()));
        assert_eq!(result.status, Some("Status Text".to_string()));
        assert_eq!(result.header, Some("Header".to_string()));
    }

    #[test]
    fn test_critical_icon_detection_logic() {
        // Test the PROTECTED icon detection logic in render_title_or_footer
        let result = render_title_or_footer("📦 Package Status", 20, "─", "left");

        // Should contain the icon and text properly formatted
        assert!(result.contains("📦 Package Status"));
        // The actual length may be different due to emoji width calculations
        assert!(result.chars().count() >= 18); // Allow some flexibility for emoji width
    }

    #[test]
    fn test_wrap_text_at_word_boundaries() {
        let result = wrap_text_at_word_boundaries("hello world test", 10);
        assert_eq!(result, vec!["hello", "world test"]);

        // Test with hints
        let result = wrap_text_at_word_boundaries("hello#W#world", 10);
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        let result = truncate_with_ellipsis("very long text", 8);
        assert!(result.ends_with("…"));
        // Check character count rather than byte length for Unicode safety
        assert!(result.chars().count() <= 8);
    }

    #[test]
    fn test_constants_accessible() {
        // Verify constants are properly exported
        assert!(!NAME.is_empty());
        assert!(!VERSION.is_empty());
        assert!(!DESCRIPTION.is_empty());
    }

    #[test]
    fn test_config_types_defaults() {
        // Test all config type defaults
        let width_config = WidthConfig::default();
        assert_eq!(width_config.h_padding, 1);
        assert_eq!(width_config.v_padding, 1);
        assert!(!width_config.enable_wrapping);

        let box_colors = BoxColors::default();
        assert_eq!(box_colors.box_color, "white");
        assert_eq!(box_colors.text_color, "none");

        let divider_config = DividerConfig::default();
        assert!(!divider_config.divider_after_title);
        assert!(!divider_config.divider_before_status);

        let padding_config = PaddingConfig::default();
        assert!(!padding_config.pad_before_title);
        assert!(!padding_config.pad_after_title);

        let alignment_config = AlignmentConfig::default();
        assert_eq!(alignment_config.header_align, "left");
        assert_eq!(alignment_config.footer_align, "left");
    }
}