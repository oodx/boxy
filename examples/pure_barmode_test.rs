// Test pure barmode without header/footer
use boxy::api::layout::BoxBuilder;
use boxy::visual::{NORMAL, HEAVY};

fn main() {
    println!("=== Pure Barmode Testing ===\n");

    println!("Pure barmode (no header/footer):");
    let pure_bar = BoxBuilder::new("Just content, no extras")
        .with_barmode()
        .with_fixed_width(30)
        .build();

    println!("'{}'", pure_bar.render());
    println!();

    println!("Pure barmode with heavy style:");
    let heavy_pure = BoxBuilder::new("Heavy style content")
        .with_style(HEAVY)
        .with_barmode()
        .with_fixed_width(25)
        .build();

    println!("'{}'", heavy_pure.render());
    println!();

    println!("Regular box for comparison:");
    let regular = BoxBuilder::new("Regular box content")
        .with_fixed_width(25)
        .build();

    println!("'{}'", regular.render());
}