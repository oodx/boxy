// Text wrapping example - demonstrates responsive content handling
// Shows how text wrapping enables dynamic resizing for different screen widths

use boxy::api::layout::{BoxBuilder, HeaderBuilder};
use boxy::width_plugin::get_terminal_width;

fn main() {
    println!("Text Wrapping Examples");
    println!("======================\n");

    let long_text = "This is a very long piece of text that demonstrates the text wrapping feature. When wrapping is enabled, text that exceeds the box width will automatically wrap to the next line at word boundaries, making it perfect for responsive layouts and dynamic content that needs to adapt to different screen sizes.";

    // Example 1: Without wrapping (truncation)
    println!("1. Without Wrapping (Truncation)");
    println!("---------------------------------\n");

    let no_wrap = BoxBuilder::new(long_text)
        .with_header(HeaderBuilder::new("Truncated"))
        .with_fixed_width(50)
        .with_wrapping(false) // Explicitly disable (default)
        .build();

    println!("{}", no_wrap.render());
    println!();

    // Example 2: With wrapping enabled
    println!("2. With Wrapping Enabled");
    println!("------------------------\n");

    let with_wrap = BoxBuilder::new(long_text)
        .with_header(HeaderBuilder::new("Wrapped"))
        .with_fixed_width(50)
        .with_wrapping(true) // Enable wrapping!
        .build();

    println!("{}", with_wrap.render());
    println!();

    // Example 3: Responsive wrapping at different widths
    println!("3. Responsive Wrapping at Different Widths");
    println!("-------------------------------------------\n");

    for width in &[30, 40, 60] {
        let responsive = BoxBuilder::new(long_text)
            .with_header(HeaderBuilder::new(&format!("Width: {}", width)))
            .with_fixed_width(*width)
            .with_wrapping(true)
            .build();

        println!("{}", responsive.render());
        println!();
    }

    // Example 4: Terminal-width responsive box
    println!("4. Terminal-Width Responsive");
    println!("----------------------------\n");

    let term_width = get_terminal_width();
    let box_width = (term_width - 10).max(40); // Leave margin, minimum 40

    let responsive_box = BoxBuilder::new(long_text)
        .with_header(HeaderBuilder::new(&format!(
            "Terminal Width: {} → Box: {}",
            term_width, box_width
        )))
        .with_fixed_width(box_width)
        .with_wrapping(true)
        .build();

    println!("{}", responsive_box.render());
    println!();

    // Example 5: Multi-paragraph wrapping
    println!("5. Multi-Paragraph Wrapping");
    println!("---------------------------\n");

    let multi_paragraph = "First paragraph with some important information that needs to be displayed.\n\nSecond paragraph that continues the discussion with additional details and context.\n\nThird paragraph wrapping up the content with final thoughts.";

    let paragraphs = BoxBuilder::new(multi_paragraph)
        .with_header(HeaderBuilder::new("Document"))
        .with_fixed_width(45)
        .with_wrapping(true)
        .build();

    println!("{}", paragraphs.render());
    println!();

    // Example 6: Comparing wrapped vs truncated side-by-side
    println!("6. Side-by-Side Comparison");
    println!("--------------------------\n");

    let sample_text =
        "The quick brown fox jumps over the lazy dog. This text demonstrates wrapping behavior.";

    let truncated = BoxBuilder::new(sample_text)
        .with_header(HeaderBuilder::new("Truncated"))
        .with_fixed_width(25)
        .with_wrapping(false)
        .build();

    let wrapped = BoxBuilder::new(sample_text)
        .with_header(HeaderBuilder::new("Wrapped"))
        .with_fixed_width(25)
        .with_wrapping(true)
        .build();

    // Render side-by-side
    let truncated_render = truncated.render();
    let wrapped_render = wrapped.render();
    let truncated_lines: Vec<_> = truncated_render.lines().collect();
    let wrapped_lines: Vec<_> = wrapped_render.lines().collect();

    let max_lines = truncated_lines.len().max(wrapped_lines.len());
    for i in 0..max_lines {
        let line1 = truncated_lines.get(i).unwrap_or(&"");
        let line2 = wrapped_lines.get(i).unwrap_or(&"");
        println!("{}  {}", line1, line2);
    }
    println!();

    println!("💡 Tip: Resize your terminal and run this again to see");
    println!("   how Example 4 adapts to different screen sizes!");
}
