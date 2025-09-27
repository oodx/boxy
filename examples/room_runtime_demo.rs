//! Demo of the new boxy library API for Room Runtime integration
//!
//! This example shows how Room Runtime (or any layout engine) can use
//! the boxy API to get pure geometry and apply its own styling.

use boxy::api::{geometry, layout, theming};

fn main() {
    println!("🚀 Boxy Library API Demo for Room Runtime\n");

    // 1. Pure Geometry Calculations
    demo_geometry();

    // 2. Component Building (No Colors)
    demo_layout();

    // 3. Optional Theming (Room Runtime can skip this)
    demo_theming();

    // 4. Background Colors (New Feature)
    demo_background_colors();
}

fn demo_geometry() {
    println!("📐 Geometry Calculations (Pure)");
    println!("{}", "=".repeat(40));

    let text = "Hello 🌟 World 中文";
    let metrics = geometry::get_text_metrics(text);

    println!("Text: '{}'", text);
    println!("Display Width: {} columns", metrics.display_width);
    println!("Character Count: {} chars", metrics.char_count);
    println!("Has Wide Chars: {}", metrics.has_wide_chars);

    let dims = geometry::calculate_box_dimensions(
        text,
        geometry::get_box_styles().first().unwrap().1,
        2,    // h_padding
        1,    // v_padding
        None, // fixed_width
    );

    println!("Box Dimensions:");
    println!("  Total: {}×{}", dims.total_width, dims.total_height);
    println!("  Inner: {}×{}", dims.inner_width, dims.inner_height);
    println!();
}

fn demo_layout() {
    println!("🔨 Component Layout Building");
    println!("{}", "=".repeat(40));

    let content = "Hello 🌟 World\nThis is line 2\nAnd line 3 with 中文";

    let layout = layout::BoxBuilder::new(content)
        .with_header(layout::HeaderBuilder::new("📦 Room Runtime Demo").align_center())
        .with_footer(layout::FooterBuilder::new("Footer Text").align_right())
        .with_status(layout::StatusBuilder::new("Status: Ready").align_center())
        .build();

    println!(
        "Layout created with {} components:",
        layout.components().len()
    );
    for (i, component) in layout.components().iter().enumerate() {
        println!(
            "  Component {}: {}×{}",
            i + 1,
            component.width,
            component.height
        );
    }

    println!("\nRendered Layout (Pure Unicode, No Colors):");
    println!("{}", layout.render());
    println!();
}

fn demo_theming() {
    println!("🎨 Optional Theming");
    println!("{}", "=".repeat(40));

    let content = "Styled content";

    // Room Runtime would use plain renderer
    let plain_renderer = theming::create_plain_renderer();
    let scheme = theming::ColorScheme::default();
    let plain_result = plain_renderer(content, &scheme);

    println!("Plain Renderer (Room Runtime): '{}'", plain_result);

    // Traditional boxy would use themed renderer
    let themed_renderer = theming::create_themed_renderer();
    let themed_result = themed_renderer(content, &scheme);

    println!("Themed Renderer (Traditional): '{}'", themed_result);
    println!();
}

fn demo_background_colors() {
    println!("🌈 Background Colors (New Feature)");
    println!("{}", "=".repeat(40));

    let text = "Background Demo";

    // Different background color types
    let colors = vec![
        ("None", theming::BackgroundColor::None),
        (
            "Red (Named)",
            theming::BackgroundColor::Named("red".to_string()),
        ),
        ("ANSI 196", theming::BackgroundColor::Ansi(196)),
        ("RGB Red", theming::BackgroundColor::Rgb(255, 0, 0)),
        (
            "Hex Blue",
            theming::BackgroundColor::Hex("#0000FF".to_string()),
        ),
    ];

    for (name, bg_color) in colors {
        let styled = theming::apply_background_color(text, &bg_color);
        println!("{}: '{}'", name, styled);
    }
    println!();
}
