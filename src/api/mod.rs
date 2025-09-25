//! Boxy Library API - RSB MODULE_SPEC compliant public interface
//!
//! This module provides a clean, decoupled API for library consumers like Room Runtime.
//! Follows RSB MODULE_SPEC patterns for modularity and cross-module integration.
//!
//! # Design Goals
//! - Pure geometry calculations (emoji/glyph width, box dimensions)
//! - Modular theming (optional color application)
//! - Dynamic component system (Header, Footer, Status, Body)
//! - Background color support
//! - Bridge to internal boxy modernization
//!
//! # Usage Patterns
//! ```rust
//! use boxy::api::{geometry, layout, theming};
//!
//! // Pure geometry (no colors)
//! let dims = geometry::calculate_box_dimensions(content, style);
//! let width = geometry::get_text_width("Hello 🌟 World");
//!
//! // Component building (no color coupling)
//! let header = layout::HeaderBuilder::new("Title").build();
//! let body = layout::BodyBuilder::new(content).build();
//!
//! // Optional theming (library consumers can skip)
//! let styled = theming::apply_colors(layout, theme);
//! ```

pub mod geometry;
pub mod layout;
pub mod theming;

// Re-export curated public API (RSB MODULE_SPEC pattern)
// Core geometry functions - always available
pub use geometry::{
    get_text_width,
    calculate_box_dimensions,
    BoxDimensions,
    TextMetrics,
};

// Layout building blocks - no color coupling
pub use layout::{
    BoxBuilder,
    HeaderBuilder,
    FooterBuilder,
    StatusBuilder,
    BodyBuilder,
    ComponentLayout,
};

// Optional theming - consumers can ignore entirely
pub use theming::{
    ColorScheme,
    BackgroundColor,
    apply_colors,
    create_plain_renderer,  // For Room Runtime
    create_themed_renderer, // For traditional usage
};

// Protected macros re-export (CRITICAL for layout engines)
pub use crate::{
    status_padding_line,
    status_divider_line,
    status_content_line,
    max_width,
    inner_target_width,
};