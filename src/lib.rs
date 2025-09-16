pub mod boxes;
pub mod colors;
pub mod components;
pub mod config;
pub mod draw;
pub mod width_plugin;
pub mod parser;
pub mod emoji_debug;

pub use boxes::*;
pub use colors::*;
pub use components::*;
pub use config::*;
pub use draw::*;
pub use width_plugin::*;
pub use parser::*;

// Re-export external types that modules need
pub use std::fs::File;
pub use std::process::{Command, Stdio};
// pub use unicode_width::UnicodeWidthStr;  // No longer needed - using custom implementation
pub use std::collections::HashMap;
pub use regex::Regex;