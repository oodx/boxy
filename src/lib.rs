pub mod colors;
pub mod core;
pub mod visual;
pub mod width_plugin;
pub mod emoji_debug;
pub mod jynx_plugin;

// Import colors module public API
pub use colors::{
    get_color_code,
    validate_color,
    get_color_categories,
    generate_color_help,
    RESET,
};

// Import core module public API (consolidates config, parser, help)
pub use core::{
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

// Import visual module public API
pub use visual::{
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

// Import other modules
pub use width_plugin::*;
pub use jynx_plugin::*;

// Re-export external types that modules need
pub use std::fs::File;
pub use std::process::{Command, Stdio};
// pub use unicode_width::UnicodeWidthStr;  // No longer needed - using custom implementation
pub use std::collections::HashMap;
pub use regex::Regex;