// Dynamic layout example
// Shows how to create responsive and dynamic box layouts

use boxy::api::{geometry, layout};

fn main() {
    println!("Dynamic Layout Examples");
    println!("======================\n");

    // Example 1: Responsive width based on content
    println!("1. Auto-sizing Based on Content");
    println!("-------------------------------\n");

    let short_text = "Short";
    let medium_text = "This is medium length content";
    let long_text = "This is much longer content that demonstrates how boxes can automatically adjust their width based on the content they contain";

    for (label, text) in &[
        ("Short", short_text),
        ("Medium", medium_text),
        ("Long", long_text),
    ] {
        let metrics = geometry::get_text_metrics(text);
        let optimal_width = metrics.display_width + 4; // Add padding

        let layout = layout::BoxBuilder::new(text)
            .with_header(layout::HeaderBuilder::new(label))
            .with_fixed_width(optimal_width.min(80)) // Cap at 80 chars
            .build();

        println!("{}", layout.render());
        println!();
    }

    // Example 2: Multi-column layout simulation
    println!("2. Multi-Column Layout");
    println!("----------------------\n");

    let column1 = layout::BoxBuilder::new("Column 1\nContent\nGoes\nHere")
        .with_header(layout::HeaderBuilder::new("Left"))
        .with_fixed_width(25)
        .build();

    let column2 = layout::BoxBuilder::new("Column 2\nMore\nContent\nHere")
        .with_header(layout::HeaderBuilder::new("Right"))
        .with_fixed_width(25)
        .build();

    // Render side by side (simulated)
    let render1 = column1.render();
    let render2 = column2.render();
    let lines1: Vec<_> = render1.lines().collect();
    let lines2: Vec<_> = render2.lines().collect();

    let max_lines = lines1.len().max(lines2.len());
    for i in 0..max_lines {
        let line1 = lines1.get(i).unwrap_or(&"");
        let line2 = lines2.get(i).unwrap_or(&"");
        println!("{}  {}", line1, line2);
    }
    println!();

    // Example 3: Dynamic status updates
    println!("3. Dynamic Status Components");
    println!("----------------------------\n");

    let statuses = vec![
        ("Initializing", "0%"),
        ("Processing", "25%"),
        ("Computing", "50%"),
        ("Finalizing", "75%"),
        ("Complete", "100%"),
    ];

    for (status, progress) in statuses {
        let layout = layout::BoxBuilder::new("Task in progress...")
            .with_header(layout::HeaderBuilder::new("Process Monitor"))
            .with_footer(
                layout::FooterBuilder::new(&format!("{} - {}", status, progress))
                    .align_center()
            )
            .with_fixed_width(40)
            .build();

        // In a real app, you'd clear and redraw
        println!("{}", layout.render());
        println!();
    }

    // Example 4: Nested boxes (conceptual)
    println!("4. Nested Box Layouts");
    println!("--------------------\n");

    let inner_box = layout::BoxBuilder::new("Inner content")
        .with_fixed_width(20)
        .build();

    let outer_box = layout::BoxBuilder::new(&format!("Outer box containing:\n{}", inner_box.render()))
        .with_header(layout::HeaderBuilder::new("Container"))
        .with_fixed_width(40)
        .build();

    println!("{}", outer_box.render());
    println!();

    // Example 5: Unicode-aware truncation
    println!("5. Smart Truncation with Unicode");
    println!("--------------------------------\n");

    let unicode_text = "This text contains emojis 🚀🎨 and 中文 characters and काठमाडौं text that might need truncation";

    for width in &[30, 50, 70] {
        let layout = layout::BoxBuilder::new(unicode_text)
            .with_header(layout::HeaderBuilder::new(&format!("Width: {}", width)))
            .with_fixed_width(*width)
            .build();

        println!("{}", layout.render());
        println!();
    }
}