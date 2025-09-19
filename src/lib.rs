pub mod colors;
pub mod config;
pub mod visual;
pub mod width_plugin;
pub mod parser;
pub mod emoji_debug;

// Import colors module public API
pub use colors::{
    get_color_code,
    validate_color,
    get_color_categories,
    generate_color_help,
    RESET,
};

// Import config module
pub use config::*;

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
pub use parser::*;

// Re-export external types that modules need
pub use std::fs::File;
pub use std::process::{Command, Stdio};
// pub use unicode_width::UnicodeWidthStr;  // No longer needed - using custom implementation
pub use std::collections::HashMap;
pub use regex::Regex;