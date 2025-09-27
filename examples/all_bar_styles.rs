// Test all available bar styles to show differences
use boxy::api::layout::{BoxBuilder, HeaderBuilder};
use boxy::visual::{ASCII, COLON, DASHED, DOT, DOUBLE, HEAVY, NORMAL, ROUNDED, STAR, THICKSII};

fn main() {
    println!("=== All Bar Styles Demonstration ===\n");

    let width = 40;
    let content = "Sample content";
    let header = "Style Demo";

    // Test all available styles
    let styles = [
        ("NORMAL", NORMAL),
        ("ROUNDED", ROUNDED),
        ("DOUBLE", DOUBLE),
        ("HEAVY", HEAVY),
        ("ASCII", ASCII),
        ("THICKSII", THICKSII),
        ("COLON", COLON),
        ("DOT", DOT),
        ("STAR", STAR),
        ("DASHED", DASHED),
    ];

    for (name, style) in styles {
        println!("{}:", name);

        // Regular box for reference
        let regular = BoxBuilder::new(content)
            .with_header(HeaderBuilder::new(header))
            .with_style(style)
            .with_fixed_width(width)
            .build();

        println!("Box mode:");
        println!("{}", regular.render());

        // Barmode version
        let barmode = BoxBuilder::new(content)
            .with_header(HeaderBuilder::new(header))
            .with_style(style)
            .with_fixed_width(width)
            .with_barmode()
            .build();

        println!("Bar mode:");
        println!("{}", barmode.render());
        println!("{}", "─".repeat(50));
        println!();
    }

    println!("Note: NORMAL and ROUNDED use the same horizontal character ('─')");
    println!("DOUBLE uses '═', HEAVY uses '━', ASCII uses '-'");
    println!(
        "THICKSII uses '=', COLON uses ':', DOT uses '•' for all characters, STAR uses '*', DASHED uses '┄' horizontally and '┆' vertically"
    );
}
