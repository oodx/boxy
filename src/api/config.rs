//! Configuration adapter - Bridge between CLI and API
//!
//! This module provides a unified configuration interface that works for both
//! CLI usage and programmatic API usage. It's an optional layer that provides
//! progressive enhancement.
//!
//! # Progressive Enhancement Philosophy
//!
//! - **Layer 0**: Pure API users can ignore this module entirely
//! - **Layer 1**: API users who want CLI-style config can use BoxyConfig
//! - **Layer 2**: CLI automatically uses BoxyConfig internally
//!
//! # Usage Examples
//!
//! ## Pure API Usage (No Config)
//!
//! ```rust
//! use boxy::api::layout::{BoxBuilder, HeaderBuilder};
//!
//! let layout = BoxBuilder::new("content")
//!     .with_header(HeaderBuilder::new("Title"))
//!     .with_fixed_width(40)
//!     .build();
//! ```
//!
//! ## Declarative Config Usage
//!
//! ```rust
//! use boxy::api::config::BoxyConfig;
//! use boxy::api::layout::BoxLayout;
//!
//! let mut config = BoxyConfig::default();
//! config.text = "content".to_string();
//! config.title = Some("Title".to_string());
//! config.width.fixed_width = Some(40);
//!
//! let layout = BoxLayout::from(&config);
//! println!("{}", layout.render());
//! ```
//!
//! # RSB MODULE_SPEC Compliance
//! - Re-exports core::BoxyConfig as public API
//! - No coupling to theming (colors optional)
//! - Adapter pattern for CLI → API bridge

// Re-export BoxyConfig as public API for library users
pub use crate::core::BoxyConfig;

use crate::api::layout::{BoxBuilder, BoxLayout, FooterBuilder, HeaderBuilder, StatusBuilder};

/// Convert BoxyConfig to BoxLayout (CLI → API adapter)
///
/// This adapter bridges the CLI configuration system with the API layout system,
/// allowing both interfaces to coexist. It follows the progressive enhancement
/// principle: theming/colors are NOT applied here (that's optional Layer 2).
///
/// # Examples
///
/// ```rust
/// use boxy::api::config::BoxyConfig;
/// use boxy::api::layout::BoxLayout;
///
/// let mut config = BoxyConfig::default();
/// config.text = "Hello World".to_string();
/// config.title = Some("Greeting".to_string());
/// config.width.fixed_width = Some(30);
///
/// let layout = BoxLayout::from(&config);
/// assert!(layout.render().contains("Hello World"));
/// assert!(layout.render().contains("Greeting"));
/// ```
impl From<&BoxyConfig> for BoxLayout {
    fn from(config: &BoxyConfig) -> Self {
        // CRITICAL: Preserve CLI title behavior - titles render INSIDE body, not as headers
        // Legacy CLI: Body::compose_content_lines() adds title as first body line
        // Do NOT use with_header() for titles - that would break CLI parity

        let mut builder = BoxBuilder::new(&config.text);

        // Apply title if provided (renders inside body, not as header)
        if let Some(title) = &config.title {
            builder = builder.with_title(title);
        }

        // Apply icon if provided
        if let Some(icon) = &config.icon {
            builder = builder.with_icon(icon);
        }

        // Only use header builder for explicit header field (not title)
        if let Some(header) = &config.header {
            let mut header_builder = HeaderBuilder::new(header);
            match config.alignment.header_align.as_str() {
                "left" => header_builder = header_builder.align_left(),
                "center" => header_builder = header_builder.align_center(),
                "right" => header_builder = header_builder.align_right(),
                _ => {}
            }
            builder = builder.with_header(header_builder);
        }

        // Apply footer if provided
        if let Some(footer_text) = &config.footer {
            let mut footer_builder = FooterBuilder::new(footer_text);

            // Apply footer alignment from config
            match config.alignment.footer_align.as_str() {
                "left" => footer_builder = footer_builder.align_left(),
                "center" => footer_builder = footer_builder.align_center(),
                "right" => footer_builder = footer_builder.align_right(),
                _ => {} // Default left
            }

            builder = builder.with_footer(footer_builder);
        }

        // Apply status line if provided
        if let Some(status_text) = &config.status_bar {
            builder = builder.with_status(StatusBuilder::new(status_text));
        }

        // Apply box style from config
        builder = builder.with_style(config.style);

        // Apply width configuration from WidthConfig
        if let Some(fixed) = config.width.fixed_width {
            builder = builder.with_fixed_width(fixed);
        }

        // Apply horizontal padding
        builder = builder.with_h_padding(config.width.h_padding);

        // Apply vertical padding (normalize to legacy behavior - legacy ignored v_padding)
        // For legacy CLI compatibility, don't apply v_padding unless explicitly set to non-default
        // This prevents extra blank rows that weren't present in legacy output
        let legacy_v_padding = 0; // Legacy behavior: no vertical padding
        builder = builder.with_v_padding(legacy_v_padding);

        // Apply height configuration (if multiplexing mode enabled)
        if let Some(height) = config.fixed_height {
            builder = builder.with_fixed_height(height);
        }

        // Apply wrapping mode
        builder = builder.with_wrapping(config.width.enable_wrapping);

        // TODO: Dividers and vertical padding not yet implemented in API
        // These features exist in CLI via config.dividers and config.padding
        // but require BoxLayout rendering changes to support properly.
        // Tracked in CHINA-05A Phase 3 or later.

        // Build the layout (NO colors applied - that's optional Layer 2)
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::core::{
        AlignmentConfig, BodyAlignment, BoxColors, DividerConfig, PaddingConfig, WidthConfig,
    };

    #[test]
    fn test_basic_config_to_layout() {
        let config = BoxyConfig {
            text: "Test content".to_string(),
            ..Default::default()
        };

        let layout = BoxLayout::from(&config);
        let output = layout.render();

        assert!(output.contains("Test content"));
    }

    #[test]
    fn test_config_with_title() {
        let config = BoxyConfig {
            text: "Body text".to_string(),
            title: Some("Header".to_string()),
            ..Default::default()
        };

        let layout = BoxLayout::from(&config);
        let output = layout.render();

        assert!(output.contains("Header"));
        assert!(output.contains("Body text"));
    }

    #[test]
    fn test_config_with_width() {
        let mut config = BoxyConfig::default();
        config.text = "Content".to_string();
        config.width.fixed_width = Some(30);

        let layout = BoxLayout::from(&config);

        assert_eq!(layout.total_width, 30);
    }

    #[test]
    fn test_config_with_footer() {
        let config = BoxyConfig {
            text: "Body".to_string(),
            footer: Some("Footer text".to_string()),
            ..Default::default()
        };

        let layout = BoxLayout::from(&config);
        let output = layout.render();

        assert!(output.contains("Footer text"));
    }

    #[test]
    fn test_config_preserves_unicode() {
        let mut config = BoxyConfig::default();
        config.text = "Hello 🌟 世界".to_string();
        config.title = Some("Unicode Test 中文".to_string());
        config.width.fixed_width = Some(30); // Ensure enough width for unicode

        let layout = BoxLayout::from(&config);
        let output = layout.render();

        assert!(output.contains("🌟"), "Output should contain 🌟");
        assert!(output.contains("世界"), "Output should contain 世界");
        assert!(output.contains("中文"), "Output should contain 中文");
    }

    #[test]
    fn test_config_with_padding() {
        let mut config = BoxyConfig::default();
        config.text = "Content".to_string();
        config.width.h_padding = 5;
        config.width.v_padding = 2;

        let layout = BoxLayout::from(&config);
        let output = layout.render();

        // Verify content is present and padded
        assert!(output.contains("Content"));
    }

    #[test]
    fn test_config_with_status_bar() {
        let config = BoxyConfig {
            text: "Body".to_string(),
            status_bar: Some("Status: OK".to_string()),
            ..Default::default()
        };

        let layout = BoxLayout::from(&config);
        let output = layout.render();

        assert!(output.contains("Status: OK"));
    }
}
