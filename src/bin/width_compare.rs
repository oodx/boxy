/// Width calculation comparison tool
///
/// Usage:
///   cargo run --bin width_compare "✅"
///   cargo run --bin width_compare compare "X" "✅" "ℹ️" "🚀"

use std::env;
use boxy::width_plugin::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "compare" => {
            if args.len() < 4 {
                println!("Usage: width_compare compare <char1> <char2> [char3...]");
                return;
            }
            let chars: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
            compare_width_implementations(&chars);
        }
        text => {
            // Single character/string comparison
            compare_single(text);
        }
    }
}

fn compare_single(text: &str) {
    println!("🔍 WIDTH COMPARISON: '{}'", text);
    println!("  Unicode Crate: {}", get_display_width_unicode_crate(text));
    println!("  Custom Impl:   {}", get_display_width_custom(text));
    println!("  Current (env): {}", get_display_width(text));

    println!("\n📊 VISUAL TEST:");
    println!("[{}]<-- actual display", text);
}

fn compare_width_implementations(chars: &[&str]) {
    println!("📊 WIDTH IMPLEMENTATION COMPARISON");
    println!("{:<10} {:<12} {:<12} {:<12} {:<10}", "Text", "Unicode", "Custom", "Current", "Match?");
    println!("{}", "─".repeat(60));

    for &text in chars {
        let unicode_width = get_display_width_unicode_crate(text);
        let custom_width = get_display_width_custom(text);
        let current_width = get_display_width(text);
        let matches = unicode_width == custom_width;

        println!("{:<10} {:<12} {:<12} {:<12} {:<10}",
                 format!("'{}'", text),
                 unicode_width,
                 custom_width,
                 current_width,
                 if matches { "✅" } else { "❌" });
    }
}

fn print_usage() {
    println!("🔍 Boxy Width Calculation Comparison Tool");
    println!();
    println!("Usage:");
    println!("  width_compare <text>                    - Compare single text");
    println!("  width_compare compare <char1> <char2>   - Compare multiple characters");
    println!();
    println!("Environment Variables:");
    println!("  BOXY_USE_CUSTOM_WIDTH=1   - Use custom width calculation");
    println!("  (unset)                   - Use unicode-width crate (default)");
    println!();
    println!("Examples:");
    println!("  width_compare '✅'");
    println!("  width_compare compare '✅' 'ℹ️' '🚀' 'X'");
    println!("  BOXY_USE_CUSTOM_WIDTH=1 width_compare '✅'");
}