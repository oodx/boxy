//! Background Color Demo
//!
//! Demonstrates the comprehensive background color support in Boxy.
//! Shows all 5 background color types: None, ANSI, RGB, Named, and Hex.

use boxy::api::layout::{BoxBuilder, FooterBuilder, HeaderBuilder};
use boxy::api::theming::{BackgroundColor, ColorScheme, apply_background_color};

fn main() {
    println!("🎨 Boxy Background Color Demo");
    println!("=============================\n");

    let content = "This text has different\nbackground colors applied";
    let header = "Background Demo";
    let footer = "Color Support";

    // Create a base layout
    let layout = BoxBuilder::new(content)
        .with_header(HeaderBuilder::new(header))
        .with_footer(FooterBuilder::new(footer))
        .with_fixed_width(40)
        .build();

    // Demo 1: No background (default)
    println!("1. No Background (Default)");
    println!("--------------------------");
    let scheme_none = ColorScheme::default().with_background(BackgroundColor::None);
    let result = apply_background_color(&layout.render(), &scheme_none.background_color);
    println!("{}", result);
    println!();

    // Demo 2: ANSI color codes (0-255)
    println!("2. ANSI Background Colors");
    println!("-------------------------");

    let ansi_colors = [
        (52, "Dark Red"),
        (22, "Dark Green"),
        (17, "Dark Blue"),
        (58, "Dark Yellow"),
        (54, "Dark Magenta"),
        (23, "Dark Cyan"),
    ];

    for (code, name) in ansi_colors {
        println!("ANSI {} ({}):", code, name);
        let bg_color = BackgroundColor::Ansi(code);
        let result = apply_background_color(&layout.render(), &bg_color);
        println!("{}", result);
        println!();
    }

    // Demo 3: RGB colors
    println!("3. RGB Background Colors");
    println!("------------------------");

    let rgb_colors = [
        ((40, 44, 52), "Dark Gray"),
        ((75, 0, 130), "Indigo"),
        ((139, 69, 19), "Saddle Brown"),
        ((0, 100, 0), "Dark Green"),
        ((25, 25, 112), "Midnight Blue"),
    ];

    for ((r, g, b), name) in rgb_colors {
        println!("RGB({}, {}, {}) - {}:", r, g, b, name);
        let bg_color = BackgroundColor::Rgb(r, g, b);
        let result = apply_background_color(&layout.render(), &bg_color);
        println!("{}", result);
        println!();
    }

    // Demo 4: Named colors
    println!("4. Named Background Colors");
    println!("--------------------------");

    let named_colors = [
        "red",
        "green",
        "blue",
        "yellow",
        "magenta",
        "cyan",
        "black",
        "white",
        "bright_red",
        "bright_green",
        "bright_blue",
    ];

    for name in named_colors {
        println!("Named color '{}':", name);
        let bg_color = BackgroundColor::Named(name.to_string());
        let result = apply_background_color(&layout.render(), &bg_color);
        println!("{}", result);
        println!();
    }

    // Demo 5: Hex colors
    println!("5. Hex Background Colors");
    println!("------------------------");

    let hex_colors = [
        ("#FF6B6B", "Coral Red"),
        ("#4ECDC4", "Turquoise"),
        ("#45B7D1", "Sky Blue"),
        ("#96CEB4", "Mint Green"),
        ("#FFEAA7", "Light Yellow"),
        ("#DDA0DD", "Plum"),
    ];

    for (hex, name) in hex_colors {
        println!("Hex {} ({}):", hex, name);
        let bg_color = BackgroundColor::Hex(hex.to_string());
        let result = apply_background_color(&layout.render(), &bg_color);
        println!("{}", result);
        println!();
    }

    // Demo 6: Color Scheme integration
    println!("6. Color Scheme Integration");
    println!("---------------------------");

    let mut scheme = ColorScheme::default();
    scheme.background_color = BackgroundColor::Rgb(30, 30, 30); // Dark background
    scheme.text_color = "bright_white".to_string(); // Light text
    scheme.border_color = "cyan".to_string(); // Cyan borders

    println!("Complete color scheme (dark theme):");
    // Note: apply_colors function doesn't exist yet, so we just show background
    let result = apply_background_color(&layout.render(), &scheme.background_color);
    println!("{}", result);
    println!();

    // Demo 7: Invalid colors (graceful degradation)
    println!("7. Invalid Colors (Graceful Degradation)");
    println!("----------------------------------------");

    println!("Invalid hex color '#INVALID':");
    let invalid_hex = BackgroundColor::Hex("#INVALID".to_string());
    let result = apply_background_color(&layout.render(), &invalid_hex);
    println!("{}", result);
    println!();

    println!("Invalid named color 'nonexistent':");
    let invalid_named = BackgroundColor::Named("nonexistent".to_string());
    let result = apply_background_color(&layout.render(), &invalid_named);
    println!("{}", result);
    println!();

    println!("✨ Background color demo complete!");
    println!("\nSupported formats:");
    println!("  • None - No background (transparent)");
    println!("  • ANSI(code) - 256-color ANSI codes (0-255)");
    println!("  • RGB(r,g,b) - True color RGB values");
    println!("  • Named(name) - Predefined color names");
    println!("  • Hex(#RRGGBB) - Hex color codes");
}
