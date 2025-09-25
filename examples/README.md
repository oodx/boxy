# Boxy Library Examples

This directory contains comprehensive examples demonstrating how to use the Boxy library API.

## Running Examples

To run any example, use:

```bash
cargo run --example <example_name>
```

## Available Examples

### 1. Basic Box Drawing (`basic_box.rs`)

Simple examples of creating boxes with different configurations.

```bash
cargo run --example basic_box
```

**Demonstrates:**
- Simple box creation
- Custom width settings
- Headers and footers
- Padding configuration

### 2. Room Runtime Integration (`room_runtime_integration.rs`)

Shows how layout engines like Room Runtime can use Boxy's pure geometry API.

```bash
cargo run --example room_runtime_integration
```

**Demonstrates:**
- Pure geometry calculations (no colors)
- Unicode/emoji width calculations
- Component-based layouts
- Text metrics for layout engines

### 3. Theming (`theming.rs`)

Optional color and theming application examples.

```bash
cargo run --example theming
```

**Demonstrates:**
- Plain rendering (no colors)
- Theme application
- Background colors
- Custom color schemes

### 4. Dynamic Layouts (`dynamic_layout.rs`)

Advanced examples of responsive and dynamic box layouts.

```bash
cargo run --example dynamic_layout
```

**Demonstrates:**
- Auto-sizing based on content
- Multi-column layouts
- Dynamic status updates
- Unicode-aware truncation

## Integration Patterns

### For CLI Applications

```rust
use boxy::api::layout::BoxBuilder;

let output = BoxBuilder::new("Your content")
    .with_header("App Name")
    .with_footer("v1.0.0")
    .build();

println!("{}", output.render());
```

### For Layout Engines (like Room Runtime)

```rust
use boxy::api::geometry;

// Get precise measurements without rendering
let metrics = geometry::get_text_metrics(content);
let width = metrics.display_width;

// Calculate dimensions for layout planning
let dims = geometry::calculate_box_dimensions(
    content,
    style,
    h_padding,
    v_padding
);
```

### For TUI Applications

```rust
use boxy::api::{layout, theming};

// Create layout
let layout = layout::BoxBuilder::new(content)
    .with_width(terminal_width)
    .build();

// Optionally apply colors
let colored = if supports_color {
    theming::apply_colors(layout, &theme)
} else {
    layout
};
```

## Key Features

- **Full Unicode Support**: Correctly handles emoji, CJK characters, and complex scripts
- **Modular Design**: Use only the parts you need
- **Zero Color Coupling**: Core functionality works without any ANSI codes
- **Background Color Support**: Modern terminal background color handling
- **Component System**: Build complex layouts from simple parts

## Performance Considerations

- Geometry calculations are cached where possible
- Regex patterns use lazy static compilation
- String allocations are minimized in hot paths
- Width calculations use efficient Unicode algorithms

## Migration from CLI

If you're migrating from using Boxy as a CLI tool:

1. Replace shell commands with `BoxBuilder` API calls
2. Use `geometry` module instead of manual width calculations
3. Apply theming only when needed (it's optional)
4. Leverage the component system for complex layouts

## Questions?

See the main [API documentation](../API_README.md) or check the source code for detailed rustdoc comments.