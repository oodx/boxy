//! Boxy Library API - Pure Rust text box rendering with Unicode support
//!
//! This module provides a clean, modular API for creating text-based UI boxes
//! with full Unicode, emoji, and multi-language support. Designed for both
//! standalone use and integration with layout engines like Room Runtime.
//!
//! # Features
//!
//! - 🌍 **Full Unicode Support**: Handles emoji, CJK characters, and complex scripts
//! - 📐 **Pure Geometry**: Layout calculations without color dependencies
//! - 🎨 **Optional Theming**: Apply colors only when needed
//! - 🧩 **Component System**: Composable headers, footers, and bodies
//! - 🎯 **Zero Dependencies**: Core geometry works without external crates
//!
//! # Quick Start
//!
//! ```rust
//! use boxy::api::{layout::BoxBuilder};
//!
//! // Simple box
//! let simple = BoxBuilder::new("Hello, World!").build();
//! println!("{}", simple.render());
//!
//! // Box with header and footer
//! let complete = BoxBuilder::new("Main content")
//!     .with_header("Title")
//!     .with_footer("Status: OK")
//!     .with_width(50)
//!     .build();
//! println!("{}", complete.render());
//! ```
//!
//! # Room Runtime Integration
//!
//! ```rust
//! use boxy::api::geometry;
//!
//! // Get precise text measurements
//! let width = geometry::get_text_width("Hello 🌟 World 中文");
//! assert_eq!(width, 19); // Handles emoji and CJK width
//!
//! // Calculate box dimensions
//! let dims = geometry::calculate_box_dimensions(
//!     "Content",
//!     "rounded", // box style
//!     2,        // h_padding
//!     1         // v_padding
//! );
//! ```
//!
//! # Advanced Usage
//!
//! ```rust
//! use boxy::api::{layout, theming};
//!
//! // Build complex layouts
//! let layout = layout::BoxBuilder::new("Content")
//!     .with_header(layout::HeaderBuilder::new("Title").align_center())
//!     .with_footer(layout::FooterBuilder::new("Footer").align_right())
//!     .with_padding(2)
//!     .build();
//!
//! // Apply optional theming
//! let scheme = theming::ColorScheme::default();
//! let colored = theming::apply_colors(layout, &scheme);
//! ```

pub mod geometry;
pub mod layout;
pub mod theming;
pub mod room_runtime;

// Re-export curated public API (RSB MODULE_SPEC pattern)
// Core geometry functions - always available
pub use geometry::{
    get_text_width,
    calculate_box_dimensions,
    BoxDimensions,
    TextMetrics,
    // ANSI overhead calculation
    calculate_ansi_overhead,
    compare_ansi_sizes,
    AnsiSizeComparison,
};

// Layout building blocks - no color coupling
pub use layout::{
    BoxBuilder,
    HeaderBuilder,
    FooterBuilder,
    StatusBuilder,
    BodyBuilder,
    ComponentLayout,
    // QOL convenience API
    BoxOptions,
    render_box,
    render_box_lines,
};

// Optional theming - consumers can ignore entirely
pub use theming::{
    ColorScheme,
    BackgroundColor,
    apply_colors,
    create_plain_renderer,  // For Room Runtime
    create_themed_renderer, // For traditional usage
};

// Room Runtime specific adapters
pub use room_runtime::{
    RoomRuntimeAdapter,
    ComponentPosition,
    ComponentType,
    LayoutMetadata,
};

// Protected macros re-export (CRITICAL for layout engines)
pub use crate::{
    status_padding_line,
    status_divider_line,
    status_content_line,
    max_width,
    inner_target_width,
};