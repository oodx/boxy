pub mod api;
pub mod colors;
pub mod core;
pub mod emoji_debug;
pub mod height_plugin;
pub mod jynx_plugin;
pub mod visual;
pub mod width_plugin;

// Import colors module public API
pub use colors::{
    RESET, generate_color_help, get_color_categories, get_color_code, validate_color,
    strip_ansi_codes,
};

// Import core module public API (consolidates config, parser, help)
pub use core::{
    AlignmentConfig,
    // Configuration types
    BodyAlignment,
    BoxColors,
    BoxyConfig,
    DESCRIPTION,
    DividerConfig,
    NAME,
    PaddingConfig,
    ParsedContent,
    // Constants
    VERSION,
    WidthConfig,
    // Parser functions with CRITICAL icon detection logic
    expand_variables,
    parse_content_stream,
    render_title_or_footer,
    // Configuration functions
    resolve_box_config,
    // Help functions
    show_comprehensive_help,
    show_usage_examples,
    truncate_with_ellipsis,
    unescape_stream_value,
    wrap_text_at_word_boundaries,
};

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

// Import other modules
pub use height_plugin::*;
pub use jynx_plugin::*;
pub use width_plugin::*;

// Re-export external types that modules need
pub use std::fs::File;
pub use std::process::{Command, Stdio};
// pub use unicode_width::UnicodeWidthStr;  // No longer needed - using custom implementation
pub use regex::Regex;
pub use std::collections::HashMap;
