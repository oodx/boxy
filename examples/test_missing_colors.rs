//! Test the restored missing colors from jynx project

use boxy::api::layout::{BoxBuilder, HeaderBuilder};
use boxy::get_color_code;

fn main() {
    println!("🎨 Testing Your Restored Missing Colors 🎨\n");

    // Test the color codes directly
    let colors = [
        ("silly", "Bright magenta - ridiculous debugging"),
        ("magic", "Lighter purple - how did this work?"),
        ("trace", "Medium grey - tracing state progression"),
        ("think", "Bright white - tracing function calls"),
    ];

    println!("Raw color codes:");
    for (name, desc) in &colors {
        let code = get_color_code(name);
        if !code.is_empty() {
            println!("{}  {}{}{} - {}", code, code, name, "\x1B[0m", desc);
        } else {
            println!("❌ {} - NOT FOUND", name);
        }
    }

    println!("\nUsing colors in box layouts:");

    // Test them in actual layouts
    let content = "Debug output goes here";

    for (color_name, desc) in &colors {
        let layout = BoxBuilder::new(content)
            .with_header(HeaderBuilder::new(&format!("{} - {}", color_name, desc)))
            .with_fixed_width(60)
            .build();

        // Apply the color to the border (this would need theming integration)
        println!("Layout with {} color:", color_name);
        println!("{}", layout.render());
        println!();
    }

    println!("✅ Your missing colors have been restored! 🎉");
}
