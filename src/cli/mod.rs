//! CLI module - RSB MODULE_SPEC compliant CLI system
//!
//! This module provides RSB-compliant CLI argument processing including:
//! - bootstrap! macro for initialization
//! - options! macro for declarative argument parsing
//! - dispatch! macro for command routing
//! - Global context management for parsed options
//!
//! Version: boxy v0.22.0+ (RSB MODULE_SPEC CLI implementation)

pub mod macros;
pub mod options;

// Re-export CLI API
pub use options::{options, has_option, get_option_value};