# Boxy Library API

## Overview

Boxy is a flexible, modular Rust library for creating Unicode-aware text boxes and layouts. Designed with layout engines like Room Runtime in mind, Boxy provides pure geometry calculations, dynamic component building, and optional theming.

Key Design Goals:
- 📐 Pure geometry calculations
- 🧩 Modular, decoupled components
- 🌈 Optional theming
- 🌟 Full Unicode and emoji support

## Installation

Add Boxy to your `Cargo.toml`:

```toml
[dependencies]
boxy = { git = "https://github.com/your-repo/boxy" }
```

## Core Modules

### Geometry Module

Provides precise text and box dimension calculations with Unicode awareness.

Key Features:
- Emoji and CJK character width handling
- Flexible box dimension calculations
- Metrics for text display

```rust
use boxy::api::geometry;

let text = "Hello 🌟 World 中文";
let metrics = geometry::get_text_metrics(text);
let dims = geometry::calculate_box_dimensions(text, style, h_padding, v_padding);
```

### Layout Module

Create dynamic, composable box layouts without color coupling.

Components:
- `BoxBuilder`: Main layout constructor
- `HeaderBuilder`: Configurable headers
- `FooterBuilder`: Flexible footers
- `StatusBuilder`: Status line components
- `BodyBuilder`: Content rendering

```rust
use boxy::api::layout;

let layout = layout::BoxBuilder::new(content)
    .with_header(layout::HeaderBuilder::new("Title").align_center())
    .with_footer(layout::FooterBuilder::new("Footer"))
    .build();
```

### Theming Module

Optional color application with flexible rendering strategies.

Features:
- Multiple color application modes
- Background color support
- Plain and themed renderers

```rust
use boxy::api::theming;

let plain_renderer = theming::create_plain_renderer();
let themed_renderer = theming::create_themed_renderer();
let bg_color = theming::BackgroundColor::Rgb(255, 0, 0);
```

## Usage Examples

### Room Runtime (Pure Geometry)

```rust
use boxy::api::{geometry, layout};

// Calculate dimensions without colors
let dims = geometry::calculate_box_dimensions(content, style);
let layout = layout::BoxBuilder::new(content).build();
let rendered = layout.render(); // Pure Unicode
```

### Traditional Usage with Theming

```rust
use boxy::api::{layout, theming};

let layout = layout::BoxBuilder::new(content)
    .with_header(layout::HeaderBuilder::new("Title"))
    .build();

let scheme = theming::ColorScheme::default();
let styled_layout = theming::apply_colors(layout, &scheme);
```

## Key Features

- 📏 Precise Unicode width calculations
- 🔧 Dynamic component updates
- 🎨 Background color support
- 🔒 Protected calculation macros
- 🌐 Emoji and multi-language support

## API Reference

### Main Types
- `geometry::BoxDimensions`
- `geometry::TextMetrics`
- `layout::ComponentLayout`
- `theming::ColorScheme`
- `theming::BackgroundColor`

### Key Functions
- `geometry::get_text_width()`
- `geometry::calculate_box_dimensions()`
- `layout::BoxBuilder::new()`
- `theming::apply_colors()`
- `theming::apply_background_color()`

## Migration Guide

### From CLI to Library

1. Replace direct terminal printing with `layout` and `geometry` module calls
2. Use `theming` module for optional color application
3. Leverage `BoxBuilder` for dynamic layouts
4. Use `get_text_metrics()` instead of manual width calculations

## Limitations & Considerations

- Requires Rust 1.70+ for full Unicode support
- Performance may vary with complex Unicode strings
- Background color support is optional

## Contributing

Contributions welcome! Please check our GitHub repository for guidelines.

## License

[Insert your project's license here]