//! Boxy - Unicode-aware text box library
//!
//! # Feature Flags
//!
//! - `cli` (default): Full CLI features, theme engine, builtin themes, stream parsing
//! - `api-only`: Lean library with just layout/color API (no CLI bloat)
//!
//! ## API-Only Mode (Minimal Dependencies)
//!
//! ```toml
//! [dependencies]
//! boxy = { version = "0.20", default-features = false, features = ["api-only"] }
//! ```
//!
//! Includes:
//! - Core API: geometry, layout, theming, room_runtime
//! - Colors: 112-color naming system
//! - Visual: Box styles and rendering
//! - Plugins: width_plugin, height_plugin (needed by API)
//!
//! Excludes CLI bloat:
//! - Argument parsing, help text, stream parsing
//! - Theme engine and YAML loading
//! - Builtin themes, emoji_debug, jynx_plugin

pub mod api;
pub mod colors;
pub mod core;
pub mod height_plugin;
pub mod visual;
pub mod width_plugin;

#[cfg(feature = "cli")]
pub mod plugins;

// Import colors module public API
pub use colors::{
    RESET, generate_color_help, get_color_categories, get_color_code, strip_ansi_codes,
    validate_color,
};

pub use core::{
    AlignmentConfig, BodyAlignment, BoxColors, BoxyConfig, DESCRIPTION, DividerConfig, NAME,
    PaddingConfig, ParsedContent, VERSION, WidthConfig, expand_variables, parse_content_stream,
    render_title_or_footer, resolve_box_config, truncate_with_ellipsis, unescape_stream_value,
    wrap_text_at_word_boundaries,
};

#[cfg(feature = "cli")]
pub use core::{show_comprehensive_help, show_usage_examples};

// Import visual module public API
pub use visual::{
    ASCII,
    BOX_CHARS,
    Body,
    // CRITICAL: Protected macros for width calculations are exported via #[macro_export]
    BoxStyle,
    COLON,
    DASHED,
    DOT,
    DOUBLE,
    Footer,
    HEAVY,
    // Component system
    Header,
    NORMAL,
    ROUNDED,
    RenderTarget,
    STAR,
    Status,
    THICKSII,
    // Drawing functions with PROTECTED width calculations
    calculate_box_width,
    draw_box,
    render_to_string,
    strip_box,
    // Box style system
    validate_box_style,
};

pub use height_plugin::*;
pub use width_plugin::*;

#[cfg(feature = "cli")]
pub use plugins::*;

// Re-export external types that modules need
pub use std::fs::File;
pub use std::process::{Command, Stdio};
// pub use unicode_width::UnicodeWidthStr;  // No longer needed - using custom implementation
pub use regex::Regex;
pub use std::collections::HashMap;
