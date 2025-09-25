// Test width comparison between box and barmode
use boxy::api::layout::{BoxBuilder, HeaderBuilder};

fn main() {
    println!("=== Width Comparison Test ===\n");

    let width = 40;

    println!("Regular box mode:");
    let box_mode = BoxBuilder::new("Content")
        .with_header(HeaderBuilder::new("Test Header"))
        .with_fixed_width(width)
        .build();

    let box_rendered = box_mode.render();
    let box_lines: Vec<&str> = box_rendered.lines().collect();
    println!("Box width: {}", box_lines[0].chars().count());
    println!("{}\n", box_rendered);

    println!("Barmode:");
    let bar_mode = BoxBuilder::new("Content")
        .with_header(HeaderBuilder::new("Test Header"))
        .with_fixed_width(width)
        .with_barmode()
        .build();

    let bar_rendered = bar_mode.render();
    let bar_lines: Vec<&str> = bar_rendered.lines().collect();
    println!("Bar width: {}", bar_lines[0].chars().count());
    println!("{}", bar_rendered);

    println!("\nLayout total_width: {}", bar_mode.total_width);
}