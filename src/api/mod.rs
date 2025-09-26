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
//!     .with_header(HeaderBuilder::new("Title"))
//!     .with_footer(FooterBuilder::new("Status: OK"))
//!     .with_fixed_width(50)
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
//! use boxy::visual::ROUNDED;
//! let dims = geometry::calculate_box_dimensions(
//!     "Content",
//!     ROUNDED,  // box style
//!     2,        // h_padding
//!     1,        // v_padding
//!     None      // fixed_width
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

// Layout building blocks - no color coupling

// Optional theming - consumers can ignore entirely

// Room Runtime specific adapters

// Protected macros re-export (CRITICAL for layout engines)
