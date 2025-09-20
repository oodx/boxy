/// Standalone emoji debugging utility for boxy development
///
/// Usage:
///   cargo run --bin emoji_debug "✅"
///   cargo run --bin emoji_debug compare "✅" "ℹ️" "🚀" "🟢" "⚠" "✗"
///   echo -en 'á' | iconv -f utf-8 -t UNICODEBIG | xxd -g 2
use std::env;

// Import from the main boxy crate
use boxy::emoji_debug::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "compare" => {
            if args.len() < 4 {
                println!("Usage: emoji_debug compare <char1> <char2> [char3...]");
                return;
            }
            let chars: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
            compare_chars(&chars);
        }
        text => {
            // Single character/string debug
            let info = EmojiDebugInfo::new(text);
            info.print_debug();

            println!("\n📊 VISUAL TEST:");
            println!("[{}]<-- should align", text);
            println!("[{}{}]<-- double width", text, text);
        }
    }
}

fn print_usage() {
    println!("🔍 Boxy Emoji Debug Utility");
    println!();
    println!("Usage:");
    println!("  emoji_debug <text>                    - Debug single emoji/text");
    println!("  emoji_debug compare <char1> <char2>   - Compare multiple characters");
    println!();
    println!("Examples:");
    println!("  emoji_debug '✅'");
    println!("  emoji_debug 'ℹ️'");
    println!("  emoji_debug compare '✅' 'ℹ️' '🚀' 'X'");
}
