use boxy::api::layout::BoxBuilder;

fn main() {
    let short_content = "Line 1\nLine 2\nLine 3";

    println!("Testing height padding behavior...\n");

    // Create a box with fixed height that should pad to 15 lines
    let box_layout = BoxBuilder::new(short_content)
        .with_fixed_height(15)
        .with_fixed_width(40)
        .build();

    let rendered = box_layout.render();
    let lines: Vec<&str> = rendered.lines().collect();

    println!("Content lines: {}", short_content.lines().count());
    println!("Rendered lines: {}", lines.len());
    println!("Expected: 15 (fixed height)");
    println!("\nRendered box:\n{}", rendered);

    if lines.len() == 15 {
        println!("\n✅ SUCCESS: Box correctly padded to fixed height of 15 lines");
    } else {
        println!("\n❌ FAILED: Expected 15 lines, got {}", lines.len());
    }
}
