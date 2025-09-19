//! Themes module - RSB MODULE_SPEC compliant theme system
//!
//! This module provides comprehensive theme management for boxy including:
//! - Theme file validation and processing
//! - Interactive theme creation and editing
//! - Theme import/export functionality
//! - Legacy v0.5.0 compatibility layer
//! - Engine commands for theme configuration management
//!
//! Extracted from monolithic themes.rs and organized following RSB MODULE_SPEC.
//!
//! Version: boxy v0.16.0+ (RSB MODULE_SPEC reorganization)

pub mod utils;
pub mod helpers;

// Re-export public API (curated, no wildcards per RSB MODULE_SPEC)
pub use utils::{
    // Core validation functions
    validate_theme_file,
    validate_theme_file_with_duplicate_check,
    validate_theme_name,

    // Main command handlers
    handle_theme_command,
    handle_engine_command,

    // Interactive theme utilities
    create_theme_interactively,
    edit_theme_interactively,

    // File operations
    save_theme_to_file,
    export_theme_to_yaml,

    // Legacy compatibility
    Theme,
    get_themes,
    convert_boxy_theme_to_legacy,
    get_fallback_legacy_themes,

    // Help functions
    print_theme_help,
    print_engine_help,
};

// Test module access to helpers
#[cfg(test)]
pub use helpers::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_theme_name() {
        // Valid names
        assert!(validate_theme_name("my_theme").is_ok());
        assert!(validate_theme_name("test-theme").is_ok());
        assert!(validate_theme_name("theme.v1").is_ok());
        assert!(validate_theme_name("MyTheme123").is_ok());

        // Invalid names
        assert!(validate_theme_name("").is_err());
        assert!(validate_theme_name("theme with spaces").is_err());
        assert!(validate_theme_name("theme@special").is_err());
        assert!(validate_theme_name("builtin_theme").is_err());
        assert!(validate_theme_name("system_theme").is_err());

        // Too long
        let long_name = "a".repeat(51);
        assert!(validate_theme_name(&long_name).is_err());
    }

    #[test]
    fn test_legacy_themes_structure() {
        let themes = get_themes();

        // Verify core legacy themes are present
        assert!(themes.contains_key("error"));
        assert!(themes.contains_key("success"));
        assert!(themes.contains_key("warning"));
        assert!(themes.contains_key("info"));
        assert!(themes.contains_key("debug"));

        // The actual values depend on whether we're loading from theme engine or fallback
        // Since the theme engine may have different definitions, just verify structure exists
        if let Some(error_theme) = themes.get("error") {
            // Just verify that we have some values - don't check specific values
            // since theme engine vs fallback may differ
            assert!(!error_theme.color.is_empty());
            assert!(!error_theme.icon.is_empty());
        }
    }

    #[test]
    fn test_fallback_legacy_themes() {
        let themes = get_fallback_legacy_themes();

        // Should have at least the core themes
        assert!(themes.len() >= 15);

        // Verify specific themes
        assert!(themes.contains_key("fatal"));
        assert!(themes.contains_key("error"));
        assert!(themes.contains_key("warn"));
        assert!(themes.contains_key("success"));
        assert!(themes.contains_key("info"));
        assert!(themes.contains_key("debug"));
        assert!(themes.contains_key("magic"));
        assert!(themes.contains_key("silly"));
    }

    #[test]
    fn test_theme_file_validation_empty_file() {
        // Test validation of non-existent file
        let non_existent = PathBuf::from("/tmp/non_existent_theme.yml");
        let result = validate_theme_file(&non_existent);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to read theme file"));
    }

    #[test]
    fn test_export_theme_yaml_format() {
        use crate::theme_engine::BoxyTheme;

        let theme = BoxyTheme {
            color: "blue".to_string(),
            text_color: "white".to_string(),
            style: "rounded".to_string(),
            icon: Some("🎨".to_string()),
            width: Some(80),
            ..Default::default()
        };

        let yaml = export_theme_to_yaml("test_theme", &theme);

        // Should contain basic YAML structure
        assert!(yaml.contains("metadata:"));
        assert!(yaml.contains("themes:"));
        assert!(yaml.contains("test_theme:"));
        assert!(yaml.contains("color: blue"));
        assert!(yaml.contains("style: rounded"));
        assert!(yaml.contains("icon: 🎨"));
        assert!(yaml.contains("width: 80"));
    }

    #[test]
    fn test_boxy_theme_to_legacy_conversion() {
        use crate::theme_engine::BoxyTheme;

        let boxy_theme = BoxyTheme {
            color: "crimson".to_string(),
            text_color: "white".to_string(),
            style: "heavy".to_string(),
            icon: Some("⚡".to_string()),
            width: Some(60),
            ..Default::default()
        };

        let legacy_theme = convert_boxy_theme_to_legacy(boxy_theme);

        assert_eq!(legacy_theme.icon, "⚡");
        assert_eq!(legacy_theme.color, "crimson");
        assert_eq!(legacy_theme.width, Some(60));
    }

    #[test]
    fn test_legacy_theme_defaults() {
        use crate::theme_engine::BoxyTheme;

        // Test theme with no icon
        let boxy_theme = BoxyTheme {
            color: "blue".to_string(),
            icon: None,
            ..Default::default()
        };

        let legacy_theme = convert_boxy_theme_to_legacy(boxy_theme);

        // Should default to "📦" when no icon is provided
        assert_eq!(legacy_theme.icon, "📦");
        assert_eq!(legacy_theme.color, "blue");
    }
}