// Example demonstrating barmode layout system
// Shows horizontal-only bars for document integration

use boxy::api::layout::{BoxBuilder, HeaderBuilder, FooterBuilder, StatusBuilder, render_box, BoxOptions, LayoutMode};
use boxy::visual::ROUNDED;

fn main() {
    println!("=== Barmode Layout System Example ===\n");

    // Example 1: Standard box vs barmode comparison
    println!("Example 1: Standard Box vs Barmode Comparison");
    println!("Standard box layout:");
    let standard_box = BoxBuilder::new("This is standard box content with full borders")
        .with_header(HeaderBuilder::new("Standard Header"))
        .with_footer(FooterBuilder::new("Standard Footer"))
        .with_fixed_width(50)
        .build();

    println!("{}\n", standard_box.render());

    println!("Barmode layout (horizontal lines only):");
    let barmode_box = BoxBuilder::new("This is barmode content with horizontal bars only")
        .with_header(HeaderBuilder::new("Barmode Header"))
        .with_footer(FooterBuilder::new("Barmode Footer"))
        .with_fixed_width(50)
        .with_barmode()
        .build();

    println!("{}\n", barmode_box.render());

    // Example 2: Document integration use case
    println!("Example 2: Document Integration Use Case");
    println!("Perfect for separating sections in documents:");

    let doc_section1 = BoxBuilder::new("Chapter 1: Introduction\n\nThis chapter covers the fundamental concepts and provides an overview of the topic at hand.")
        .with_header(HeaderBuilder::new("📚 Document Section"))
        .with_barmode()
        .with_fixed_width(60)
        .build();

    println!("{}", doc_section1.render());

    let doc_section2 = BoxBuilder::new("Chapter 2: Implementation\n\nHere we dive into the practical aspects and show how to implement the concepts discussed.")
        .with_header(HeaderBuilder::new("🛠️ Implementation Guide"))
        .with_barmode()
        .with_fixed_width(60)
        .build();

    println!("{}\n", doc_section2.render());

    // Example 3: Status reporting with barmode
    println!("Example 3: Status Reporting with Barmode");

    let status_report = BoxBuilder::new("System Status: All services are operational\nLatency: 45ms\nUptime: 99.9%")
        .with_header(HeaderBuilder::new("⚡ System Monitor"))
        .with_status(StatusBuilder::new("Last updated: 2024-01-15 10:30:45"))
        .with_barmode()
        .with_fixed_width(45)
        .build();

    println!("{}\n", status_report.render());

    // Example 4: Convenience API with barmode
    println!("Example 4: Convenience API with Barmode");

    let convenient_bar = render_box("Quick barmode using convenience API", BoxOptions {
        header: Some("🚀 Quick Setup".to_string()),
        footer: Some("v2.0".to_string()),
        width: Some(40),
        layout_mode: Some(LayoutMode::Bar),
        ..Default::default()
    });

    println!("{}\n", convenient_bar);

    // Example 5: Different styles with barmode
    println!("Example 5: Different Styles with Barmode");

    let rounded_bar = BoxBuilder::new("Rounded style bars")
        .with_header(HeaderBuilder::new("🎨 Styled Bars"))
        .with_style(ROUNDED)
        .with_barmode()
        .with_fixed_width(30)
        .build();

    println!("{}\n", rounded_bar.render());

    // Example 6: Multi-line content handling
    println!("Example 6: Multi-line Content Handling");

    let multiline_content = "Line 1: First line of content
Line 2: Second line with more text
Line 3: Third line with even more text
Line 4: Fourth line to show wrapping";

    let multiline_bar = BoxBuilder::new(multiline_content)
        .with_header(HeaderBuilder::new("📝 Multi-line Content"))
        .with_footer(FooterBuilder::new("End of Content"))
        .with_barmode()
        .with_fixed_width(40)
        .build();

    println!("{}\n", multiline_bar.render());

    // Example 7: Comparison of both modes
    println!("Example 7: Side-by-side Functionality Comparison");

    println!("Regular Box Mode:");
    println!("- Full borders (top, bottom, left, right)");
    println!("- Enclosed content");
    println!("- Traditional box appearance");
    println!("- Good for standalone content\n");

    println!("Barmode Layout:");
    println!("- Horizontal bars only (top and bottom)");
    println!("- Open sides for document flow");
    println!("- Clean separation without enclosure");
    println!("- Perfect for document integration");
    println!("- Better for text flow and readability");

    println!("\n=== API Usage Summary ===");
    println!("Builder Pattern:");
    println!("  BoxBuilder::new(content).with_barmode().build()");
    println!("");
    println!("Convenience API:");
    println!("  render_box(content, BoxOptions {{");
    println!("      layout_mode: Some(LayoutMode::Bar),");
    println!("      ..Default::default()");
    println!("  }})");
    println!("");
    println!("Use barmode for:");
    println!("• Document section separators");
    println!("• Status reports and summaries");
    println!("• Content that needs to flow with surrounding text");
    println!("• Clean horizontal divisions");
    println!("• Integration with other layout systems");
}