fn main() {
    let text = "ℹ️";
    let mut width = 0;
    for ch in text.chars() {
        let char_width = match ch {
            '\u{FE0F}' => 0,  // Variation selector should be 0
            '\u{2139}' => 1,  // ℹ defaults to 1 (not in emoji ranges)
            _ => 1,
        };
        width += char_width;
        println!("Char U+{:04X}: width {}, cumulative: {}", ch as u32, char_width, width);
    }
    println!("Total width for '{}': {}", text, width);
}
