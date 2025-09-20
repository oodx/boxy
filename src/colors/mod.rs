//! Colors module - RSB MODULE_SPEC compliant color system
//!
//! This module provides a comprehensive 90+ color palette system for terminal output.
//! Extracted from jynx architecture and optimized for boxy themes.
//!
//! Version: boxy v0.6.0+ (RSB MODULE_SPEC reorganization)

pub mod helpers;
pub mod utils;

// Re-export public API (curated, no wildcards per RSB MODULE_SPEC)
pub use utils::{RESET, generate_color_help, get_color_categories, get_color_code, validate_color};

// Test module access
#[cfg(test)]
pub use helpers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_colors_preserved() {
        // Ensure all v0.5.0 colors still work
        assert_eq!(get_color_code("red"), "\x1B[38;5;9m");
        assert_eq!(get_color_code("green"), "\x1B[38;5;10m");
        assert_eq!(get_color_code("blue2"), "\x1B[38;5;39m");
        assert_eq!(get_color_code("grey3"), "\x1B[38;5;237m");
    }

    #[test]
    fn test_extended_colors_available() {
        // Test new semantic colors
        assert_eq!(get_color_code("crimson"), "\x1B[38;5;196m");
        assert_eq!(get_color_code("emerald"), "\x1B[38;5;34m");
        assert_eq!(get_color_code("azure"), "\x1B[38;5;33m");
        assert_eq!(get_color_code("amber"), "\x1B[38;5;220m");
    }

    #[test]
    fn test_semantic_colors() {
        assert_eq!(get_color_code("error"), "\x1B[38;5;196m");
        assert_eq!(get_color_code("success"), "\x1B[38;5;46m");
        assert_eq!(get_color_code("warning"), "\x1B[38;5;220m");
        assert_eq!(get_color_code("info"), "\x1B[38;5;33m");
    }

    #[test]
    fn test_fallback_behavior() {
        // Unknown colors should return empty string
        assert_eq!(get_color_code("unknown"), "");
        assert_eq!(get_color_code("invalid_color"), "");

        // Control values
        assert_eq!(get_color_code("none"), "");
        assert_eq!(get_color_code("auto"), "");
    }

    #[test]
    fn test_color_validation() {
        // Valid colors
        assert!(validate_color("crimson").is_ok());
        assert!(validate_color("none").is_ok());

        // Invalid colors with suggestions
        let result = validate_color("redd");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("crimson"));
    }

    #[test]
    fn test_color_categories() {
        let categories = get_color_categories();
        assert!(!categories.is_empty());

        // Verify legacy colors are preserved
        let legacy_category = categories.iter().find(|(name, _)| name.contains("Legacy"));
        assert!(legacy_category.is_some());
    }

    #[test]
    fn test_ansi_stripping() {
        // Test ANSI code stripping helper
        let colored_text = "\x1B[38;5;196mHello\x1B[0m World";
        let stripped = strip_ansi_codes(colored_text);
        assert_eq!(stripped, "Hello World");
    }

    #[test]
    fn test_padding_with_ansi() {
        // Test padding with ANSI codes
        let colored_text = "\x1B[38;5;196m■ red\x1B[0m";
        let padded = pad_cell(colored_text, 10);
        // Should pad to account for invisible ANSI codes
        assert!(padded.len() > colored_text.len());
        assert!(padded.starts_with("\x1B[38;5;196m■ red\x1B[0m"));
    }
}
