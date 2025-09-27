// Simple test to verify barmode behavior
use boxy::api::layout::{BoxBuilder, HeaderBuilder};
use boxy::visual::{HEAVY, NORMAL, ROUNDED};

fn main() {
    println!("=== Barmode Style Testing ===\n");

    // Test 1: Normal style barmode
    println!("Normal Style Barmode:");
    let normal_bar = BoxBuilder::new("Content goes here")
        .with_header(HeaderBuilder::new("Header"))
        .with_style(NORMAL)
        .with_barmode()
        .with_fixed_width(30)
        .build();

    println!("{}\n", normal_bar.render());

    // Test 2: Heavy style barmode
    println!("Heavy Style Barmode:");
    let heavy_bar = BoxBuilder::new("Content goes here")
        .with_header(HeaderBuilder::new("Header"))
        .with_style(HEAVY)
        .with_barmode()
        .with_fixed_width(30)
        .build();

    println!("{}\n", heavy_bar.render());

    // Test 3: Rounded style barmode
    println!("Rounded Style Barmode:");
    let rounded_bar = BoxBuilder::new("Content goes here")
        .with_header(HeaderBuilder::new("Header"))
        .with_style(ROUNDED)
        .with_barmode()
        .with_fixed_width(30)
        .build();

    println!("{}\n", rounded_bar.render());

    // Compare with regular box
    println!("Regular Box for comparison:");
    let regular_box = BoxBuilder::new("Content goes here")
        .with_header(HeaderBuilder::new("Header"))
        .with_style(HEAVY)
        .with_fixed_width(30)
        .build();

    println!("{}\n", regular_box.render());
}
