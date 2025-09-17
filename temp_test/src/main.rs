use unicode_width::UnicodeWidthStr;

fn main() {
    let tests = vec![
        "ℹ️", "ℹ", "⚠️", "⚠", "✅", "©️", "©", "™️", "™"
    ];
    
    for text in tests {
        println!("'{}': {}", text, UnicodeWidthStr::width(text).unwrap_or(0));
    }
}
