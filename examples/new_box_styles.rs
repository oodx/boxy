// Test the new box styles: THICKSII, COLON, DOT, and STAR
use boxy::api::layout::{BoxBuilder, FooterBuilder, HeaderBuilder};
use boxy::visual::{ASCII, COLON, DASHED, DOT, STAR, THICKSII};

fn main() {
    println!("=== New Box Styles Demonstration ===\n");

    let width = 40;
    let content = "This is sample content\nwith multiple lines";

    // Test each new style in both box and barmode
    let styles = [
        ("THICKSII", THICKSII),
        ("COLON", COLON),
        ("DOT", DOT),
        ("STAR", STAR),
        ("DASHED", DASHED),
        ("ASCII (comparison)", ASCII),
    ];

    for (name, style) in styles {
        println!("{}:", name);

        // Regular box mode
        let box_mode = BoxBuilder::new(content)
            .with_header(HeaderBuilder::new("Header"))
            .with_footer(FooterBuilder::new("Footer"))
            .with_style(style)
            .with_fixed_width(width)
            .build();

        println!("Box mode:");
        println!("{}", box_mode.render());

        // Barmode
        let bar_mode = BoxBuilder::new(content)
            .with_header(HeaderBuilder::new("Header"))
            .with_footer(FooterBuilder::new("Footer"))
            .with_style(style)
            .with_fixed_width(width)
            .with_barmode()
            .build();

        println!("Bar mode:");
        println!("{}", bar_mode.render());
        println!("{}", "─".repeat(50));
        println!();
    }

    println!("Style Characteristics:");
    println!("• THICKSII: Uses '#' for corners/verticals, '=' for horizontals");
    println!("• COLON: Uses ':' for all characters");
    println!("• DOT: Uses '•' for all characters");
    println!("• STAR: Uses '*' for all characters");
    println!("• DASHED: Uses '┄' for horizontal lines, '┆' for vertical lines");
    println!("• ASCII: Uses '+' for corners/verticals, '-' for horizontals");
}
