// Theming example showing optional color application
// Demonstrates how to use themes with the Boxy library API

use boxy::api::{layout, theming};

fn main() {
    println!("Boxy Theming Examples");
    println!("====================\n");

    // Create a basic layout
    let layout = layout::BoxBuilder::new("This box can be styled with different themes")
        .with_header(layout::HeaderBuilder::new("Theming Demo"))
        .with_footer(layout::FooterBuilder::new("Theme: Active"))
        .with_fixed_width(60)
        .build();

    // Example 1: Plain rendering (no colors)
    println!("1. Plain Rendering (No Colors)");
    println!("-------------------------------\n");

    let plain_renderer = theming::create_plain_renderer();
    let plain_output = plain_renderer.render(&layout);
    println!("{}", plain_output);
    println!();

    // Example 2: Default themed rendering
    println!("2. Default Theme");
    println!("----------------\n");

    let themed_renderer = theming::create_themed_renderer();
    let themed_output = themed_renderer.render_with_theme(&layout, "info");
    println!("{}", themed_output);
    println!();

    // Example 3: Background color support
    println!("3. Background Color Application");
    println!("-------------------------------\n");

    // Apply background color to layout
    let bg_color = theming::BackgroundColor::Rgb(40, 44, 52);  // Dark background
    let layout_with_bg = theming::apply_background_color(layout.clone(), &bg_color);
    println!("{}", layout_with_bg.render());
    println!();

    // Example 4: Color scheme customization
    println!("4. Custom Color Scheme");
    println!("----------------------\n");

    let mut custom_scheme = theming::ColorScheme::default();
    custom_scheme.primary = theming::Color::Hex("#00ff00".to_string());
    custom_scheme.secondary = theming::Color::Rgb(255, 255, 0);
    custom_scheme.accent = theming::Color::Named("cyan".to_string());

    let custom_layout = theming::apply_colors(layout.clone(), &custom_scheme);
    println!("{}", custom_layout.render());
    println!();

    // Example 5: Multiple rendering modes
    println!("5. Rendering Mode Comparison");
    println!("----------------------------\n");

    let content = "Same content, different rendering modes";
    let simple_layout = layout::BoxBuilder::new(content)
        .with_fixed_width(45)
        .build();

    println!("Plain mode:");
    println!("{}", plain_renderer.render(&simple_layout));
    println!();

    println!("Themed mode (using 'success' theme):");
    println!("{}", themed_renderer.render_with_theme(&simple_layout, "success"));
    println!();

    println!("With background color:");
    let bg = theming::BackgroundColor::Ansi(234);  // Dark gray
    let bg_layout = theming::apply_background_color(simple_layout, &bg);
    println!("{}", bg_layout.render());
}