// Debug barmode component processing
use boxy::api::layout::{BoxBuilder, HeaderBuilder};

fn main() {
    println!("=== Debug Barmode Component Processing ===\n");

    let header = HeaderBuilder::new("Test Header").build_for_width(38);

    println!("Header component content:");
    for (i, line) in header.content.lines().enumerate() {
        println!("Line {}: '{}' (len: {})", i, line, line.chars().count());
    }

    let box_layout = BoxBuilder::new("Content")
        .with_header(HeaderBuilder::new("Test Header"))
        .with_fixed_width(40)
        .with_barmode()
        .build();

    println!("\nBarmode output:");
    for (i, line) in box_layout.render().lines().enumerate() {
        println!("Line {}: '{}' (len: {})", i, line, line.chars().count());
    }

    println!("\nTotal width: {}", box_layout.total_width);
}
