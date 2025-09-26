//! Plugins module - CLI-only features
//!
//! This module contains all CLI-specific plugins that are NOT needed for API-only usage:
//! - jynx: Enhanced output formatting and coloring
//! - emoji_debug: Emoji debugging utilities
//! - theme_engine: YAML theme loading and management
//! - themes_builtin: Built-in theme registry
//! - themes: Theme commands and utilities

pub mod emoji_debug;
pub mod jynx;
pub mod theme_engine;
pub mod themes;
pub mod themes_builtin;

pub use emoji_debug::*;
pub use jynx::*;
pub use theme_engine::*;
pub use themes::*;
pub use themes_builtin::*;