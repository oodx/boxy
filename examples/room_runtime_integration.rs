// Room Runtime integration example
// Shows how Room Runtime can use Boxy's pure geometry API without colors

use boxy::api::geometry;
use boxy::api::layout::{BoxBuilder, FooterBuilder, HeaderBuilder};
use boxy::visual::ROUNDED;

fn main() {
    println!("Room Runtime Integration Example");
    println!("================================\n");

    // Example 1: Pure geometry calculations for Room Runtime
    println!("1. Pure Geometry Calculations");
    println!("-----------------------------\n");

    let content = "Room Runtime Layout Engine 🚀";

    // Get precise text width including Unicode/emoji
    let text_width = geometry::get_text_width(content);
    println!("Text: '{}'", content);
    println!("Calculated width: {} characters", text_width);

    // Calculate box dimensions with padding
    let box_dims = geometry::calculate_box_dimensions(
        content, ROUNDED, // box style
        2,       // horizontal padding
        1,       // vertical padding
        None,    // no fixed width
    );

    println!("Box dimensions:");
    println!("  - Total width: {}", box_dims.total_width);
    println!("  - Total height: {}", box_dims.total_height);
    println!("  - Inner width: {}", box_dims.inner_width);
    println!("  - Inner height: {}", box_dims.inner_height);
    println!();

    // Example 2: Building layouts without colors
    println!("2. Pure Unicode Box (No ANSI Colors)");
    println!("------------------------------------\n");

    let layout = BoxBuilder::new("Room Runtime can use this pure Unicode output")
        .with_header(HeaderBuilder::new("Layout Engine").align_center())
        .with_footer(FooterBuilder::new("v1.0.0"))
        .with_fixed_width(60)
        .build();

    // Render without any ANSI color codes
    let pure_unicode = layout.render();
    println!("{}", pure_unicode);
    println!();

    // Example 3: Component-based layout for Room Runtime
    println!("3. Component-Based Layout");
    println!("-------------------------\n");

    // Room Runtime can build layouts programmatically
    let header = HeaderBuilder::new("Component System")
        .align_left()
        .build_for_width(50);

    let footer = FooterBuilder::new("Status: Active")
        .align_right()
        .build_for_width(50);

    println!("Header: {}", header.content);
    println!("Footer: {}", footer.content);
    println!();

    // Example 4: Getting metrics for layout calculations
    println!("4. Text Metrics for Layout Engine");
    println!("---------------------------------\n");

    let complex_text = "Mixed: ASCII, 中文, 🎨 emoji, and काठमाडौं";
    let metrics = geometry::get_text_metrics(complex_text);

    println!("Text: '{}'", complex_text);
    println!("Metrics:");
    println!("  - Display width: {}", metrics.display_width);
    println!("  - Character count: {}", metrics.char_count);
    println!("  - Has wide chars: {}", metrics.has_wide_chars);
}
