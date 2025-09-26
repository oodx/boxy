// Theming example showing optional color application
// Demonstrates how to use themes with the Boxy library API

use boxy::api::{layout, theming};

fn main() {
    println!("Boxy Theming Examples");
    println!("====================\n");

    // Create a basic layout
    let box_layout = layout::BoxBuilder::new("This box can be styled with different themes")
        .with_header(layout::HeaderBuilder::new("Theming Demo"))
        .with_footer(layout::FooterBuilder::new("Theme: Active"))
        .with_fixed_width(60)
        .build();

    // Example 1: Plain rendering (no colors)
    println!("1. Plain Rendering (No Colors)");
    println!("-------------------------------\n");

    let plain_output = box_layout.render();
    println!("{}", plain_output);
    println!();

    // Example 2: Background color application
    println!("2. Background Color Application");
    println!("-------------------------------\n");

    // Apply background color to rendered text
    let bg_color = theming::BackgroundColor::Rgb(40, 44, 52);  // Dark background
    let with_bg = theming::apply_background_color(&plain_output, &bg_color);
    println!("{}", with_bg);
    println!();

    // Example 3: Different background color options
    println!("3. Background Color Options");
    println!("---------------------------\n");

    let content = layout::BoxBuilder::new("Testing background colors")
        .with_fixed_width(40)
        .build()
        .render();

    println!("No background:");
    let no_bg = theming::apply_background_color(&content, &theming::BackgroundColor::None);
    println!("{}", no_bg);
    println!();

    println!("ANSI color (234 - dark gray):");
    let ansi_bg = theming::apply_background_color(&content, &theming::BackgroundColor::Ansi(234));
    println!("{}", ansi_bg);
    println!();

    println!("Hex color (#1e1e2e):");
    let hex_bg = theming::apply_background_color(&content, &theming::BackgroundColor::Hex("#1e1e2e".to_string()));
    println!("{}", hex_bg);
    println!();

    println!("Named color (dark_blue):");
    let named_bg = theming::apply_background_color(&content, &theming::BackgroundColor::Named("dark_blue".to_string()));
    println!("{}", named_bg);
    println!();

    // Example 4: Renderer closures
    println!("4. Renderer Functions");
    println!("---------------------\n");

    let scheme = theming::ColorScheme {
        border_color: "#00ff00".to_string(),
        text_color: "#ffffff".to_string(),
        background_color: theming::BackgroundColor::Hex("#000000".to_string()),
        header_color: Some("#00ffff".to_string()),
        footer_color: Some("#ffff00".to_string()),
        status_color: Some("#ff00ff".to_string()),
    };

    // Plain renderer (no colors applied)
    let plain_renderer = theming::create_plain_renderer();
    let text = "Sample text";
    println!("Plain: {}", plain_renderer(text, &scheme));
    println!();

    // Custom renderer with colors
    let themed_renderer = theming::create_themed_renderer();
    println!("Themed: {}", themed_renderer(text, &scheme));
    println!();

    // Example 5: Real-world usage pattern
    println!("5. Real-World Pattern");
    println!("---------------------\n");

    let data_box = layout::BoxBuilder::new("Production Data:\n- Users: 1,234\n- Active Sessions: 56\n- Requests/sec: 789")
        .with_header(layout::HeaderBuilder::new("Dashboard"))
        .with_status(layout::StatusBuilder::new("Last Updated: 2s ago"))
        .with_fixed_width(50)
        .build();

    let rendered = data_box.render();

    println!("Without background:");
    println!("{}", rendered);
    println!();

    println!("With dark background:");
    let dark_bg = theming::BackgroundColor::Rgb(24, 24, 37);
    println!("{}", theming::apply_background_color(&rendered, &dark_bg));
}