// Basic box drawing example using the Boxy library API
// This example shows how to create simple boxes with the library

use boxy::api::layout::{BoxBuilder, HeaderBuilder, FooterBuilder};

fn main() {
    // Example 1: Simple box with default settings
    println!("Example 1: Simple box");
    println!("=====================\n");

    let simple_box = BoxBuilder::new("Hello, Boxy!")
        .build();

    println!("{}", simple_box.render());
    println!();

    // Example 2: Box with custom width
    println!("Example 2: Box with custom width");
    println!("=================================\n");

    let wide_box = BoxBuilder::new("This is a wider box with more content")
        .with_fixed_width(60)
        .build();

    println!("{}", wide_box.render());
    println!();

    // Example 3: Box with header and footer
    println!("Example 3: Box with header and footer");
    println!("======================================\n");

    let full_box = BoxBuilder::new("Main content goes here\nMultiple lines are supported")
        .with_header(HeaderBuilder::new("Title"))
        .with_footer(FooterBuilder::new("Status: Ready"))
        .with_fixed_width(50)
        .build();

    println!("{}", full_box.render());
    println!();

    // Example 4: Box with padding
    println!("Example 4: Box with custom padding");
    println!("===================================\n");

    let padded_box = BoxBuilder::new("Extra padding for emphasis")
        .with_padding(3)
        .with_fixed_width(40)
        .build();

    println!("{}", padded_box.render());
}