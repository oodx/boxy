use unicode_width::UnicodeWidthStr;

fn get_display_width(text: &str) -> usize {
    let clean = strip_ansi_escapes::strip(text.as_bytes());
    let clean_str = String::from_utf8_lossy(&clean);
    UnicodeWidthStr::width(clean_str.as_ref()).unwrap_or(0)
}

fn main() {
    let test_chars = vec!["X", "XX", "✅", "✅✅", "ℹ️"];

    for c in test_chars {
        let width = get_display_width(c);
        println!("{}: unicode-width={}, rust_len={}", c, width, c.len());
    }
}