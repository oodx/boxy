//! Visual module - RSB MODULE_SPEC compliant visual system
//!
//! This module provides comprehensive visual rendering functionality for boxy including:
//! - Box style definitions and validation
//! - Drawing and rendering functions with PROTECTED width calculation macros
//! - Component-based rendering architecture (Header, Footer, Status, Body)
//! - Box content stripping utilities
//!
//! Consolidated from boxes.rs, draw.rs, and components.rs following RSB MODULE_SPEC.
//!
//! CRITICAL: Preserves exact width calculation macros (max_width!, inner_target_width!)
//! from components.rs:284-298 - these are essential for width calculations.
//!
//! Version: boxy v0.16.0+ (RSB MODULE_SPEC reorganization)

pub mod utils;
pub mod helpers;
pub mod calc_macros;

// Re-export public API (curated, no wildcards per RSB MODULE_SPEC)
pub use utils::{
    // Box style system
    validate_box_style,
    BoxStyle,
    BOX_CHARS,
    NORMAL,
    ROUNDED,
    DOUBLE,
    HEAVY,
    ASCII,

    // Drawing functions with PROTECTED width calculations
    calculate_box_width,
    draw_box,
    strip_box,

    // Component system
    Header,
    Footer,
    Status,
    Body,

    // CRITICAL: Protected macros for width calculations are exported via #[macro_export]
};

// Test module access
#[cfg(test)]
pub use helpers::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_module_integration() {
        // Test that all main components are accessible
        assert!(validate_box_style("normal").is_ok());
        assert!(validate_box_style("rounded").is_ok());
        assert!(validate_box_style("invalid").is_err());
    }

    #[test]
    fn test_box_style_constants_available() {
        // Verify all style constants are properly exported
        assert_eq!(NORMAL.top_left, "┌");
        assert_eq!(ROUNDED.top_left, "╭");
        assert_eq!(DOUBLE.top_left, "╔");
        assert_eq!(HEAVY.top_left, "┏");
        assert_eq!(ASCII.top_left, "+");
    }

    #[test]
    fn test_strip_box_functionality() {
        // Test strip_box function is properly exported
        let boxed_content = "┌─────┐\n│ Hi  │\n└─────┘";
        let stripped = strip_box(boxed_content, false);
        assert_eq!(stripped, "Hi");
    }

    #[test]
    fn test_critical_macros_available() {
        // Test that the critical width calculation macros are accessible
        let lines = vec!["Hello world".to_string(), "Test".to_string()];

        // max_width! macro should be available
        let max_w = crate::max_width!(lines);
        assert_eq!(max_w, 11); // "Hello world" is 11 characters

        // inner_target_width! macro should be available
        let inner_w = crate::inner_target_width!(80, 2);
        assert_eq!(inner_w, 76); // 80 - 2*2 = 76
    }

    #[test]
    fn test_component_structs_available() {
        // Create a basic config to test component creation
        use crate::core::BoxyConfig;

        let config = BoxyConfig::default();

        // Test that component structs can be created
        let _header = Header::new(&config);
        let _footer = Footer::new(&config);
        let _status = Status::new(&config);
        let _body = Body::new(&config);

        // If we get here without compilation errors, the components are properly exported
        assert!(true);
    }
}
