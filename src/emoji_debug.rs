/// Emoji width calculation and debugging utilities for boxy
///
/// This module provides tools to debug emoji width issues and compare
/// different width calculation methods for terminal applications.

// use unicode_width::UnicodeWidthStr;  // No longer needed - using custom implementation
use strip_ansi_escapes;

/// Calculate display width using our custom implementation
pub fn get_unicode_width(text: &str) -> usize {
    crate::width_plugin::get_display_width_custom(text)
}

/// Calculate "naive" width (just character count)
pub fn get_char_count(text: &str) -> usize {
    text.chars().count()
}

/// Calculate byte length
pub fn get_byte_count(text: &str) -> usize {
    text.len()
}

/// Estimate terminal width based on emoji patterns
/// This is our custom heuristic for better emoji width calculation
pub fn get_estimated_terminal_width(text: &str) -> usize {
    // For compound emojis, treat the whole grapheme cluster as one unit
    // This is a more accurate approach than char-by-char calculation

    // Simple approach: use unicode-width but with special handling for common compound emojis
    let unicode_width = get_unicode_width(text);

    // Override for known problematic compound emojis
    match text {
        "ℹ️" => 2, // Info emoji should be 2 width, not 3
        _ => unicode_width,
    }
}

/// Heuristic to detect if a character is likely an emoji
/// Currently unused but kept for future use
#[allow(dead_code)]
fn is_likely_emoji(ch: char) -> bool {
    let code = ch as u32;
    match code {
        // Common emoji ranges
        0x1F600..=0x1F64F | // Emoticons
        0x1F300..=0x1F5FF | // Misc Symbols and Pictographs
        0x1F680..=0x1F6FF | // Transport and Map Symbols
        0x1F1E6..=0x1F1FF | // Regional indicator symbols
        0x2600..=0x26FF   | // Misc symbols
        0x2700..=0x27BF   | // Dingbats
        0xFE00..=0xFE0F   | // Variation Selectors
        0x1F900..=0x1F9FF   // Supplemental Symbols and Pictographs
        => true,
        _ => false,
    }
}

/// Comprehensive debug information for a character or string
#[derive(Debug)]
pub struct EmojiDebugInfo {
    pub text: String,
    pub unicode_width: usize,
    pub char_count: usize,
    pub byte_count: usize,
    pub estimated_terminal_width: usize,
    pub codepoints: Vec<u32>,
    pub unicode_names: Vec<String>,
    pub hex_dump: String,
}

impl EmojiDebugInfo {
    pub fn new(text: &str) -> Self {
        let codepoints: Vec<u32> = text.chars().map(|c| c as u32).collect();
        let unicode_names: Vec<String> = text.chars()
            .map(|c| unicode_name(c).unwrap_or_else(|| format!("U+{:04X}", c as u32)))
            .collect();

        let hex_dump = text.bytes()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");

        EmojiDebugInfo {
            text: text.to_string(),
            unicode_width: get_unicode_width(text),
            char_count: get_char_count(text),
            byte_count: get_byte_count(text),
            estimated_terminal_width: get_estimated_terminal_width(text),
            codepoints,
            unicode_names,
            hex_dump,
        }
    }

    pub fn print_debug(&self) {
        println!("🔍 EMOJI DEBUG: '{}'", self.text);
        println!("  Unicode Width: {}", self.unicode_width);
        println!("  Char Count: {}", self.char_count);
        println!("  Byte Count: {}", self.byte_count);
        println!("  Est Terminal Width: {}", self.estimated_terminal_width);
        println!("  Codepoints: {:?}", self.codepoints.iter().map(|c| format!("U+{:04X}", c)).collect::<Vec<_>>());
        println!("  Hex: {}", self.hex_dump);
        println!("  Names: {:?}", self.unicode_names);
    }

    pub fn compare_widths(&self) -> String {
        format!(
            "unicode:{} chars:{} bytes:{} est_term:{}",
            self.unicode_width, self.char_count, self.byte_count, self.estimated_terminal_width
        )
    }
}

/// Simple approximation of Unicode name lookup
fn unicode_name(ch: char) -> Option<String> {
    let code = ch as u32;
    match code {
        0x2705 => Some("WHITE HEAVY CHECK MARK".to_string()),
        0x2139 => Some("INFORMATION SOURCE".to_string()),
        0xFE0F => Some("VARIATION SELECTOR-16".to_string()),
        0x1F680 => Some("ROCKET".to_string()),
        0x1F600 => Some("GRINNING FACE".to_string()),
        0x26A0 => Some("WARNING SIGN".to_string()),
        0x274C => Some("CROSS MARK".to_string()),
        0x1F4A5 => Some("COLLISION".to_string()),
        // Add more as needed
        _ => None,
    }
}

/// Compare multiple characters/strings side by side
pub fn compare_chars(chars: &[&str]) {
    println!("📊 CHARACTER WIDTH COMPARISON");

    // Pre-calculate all widths to determine proper column sizing
    let infos: Vec<EmojiDebugInfo> = chars.iter().map(|&text| EmojiDebugInfo::new(text)).collect();

    // Calculate actual display widths for each text sample for proper alignment
    let text_display_widths: Vec<usize> = chars.iter()
        .map(|&text| get_estimated_terminal_width(text) + 2) // +2 for quotes
        .collect();

    // Find the maximum width needed for the text column (minimum 6 for "Text" header)
    let text_col_width = *text_display_widths.iter().max().unwrap_or(&6).max(&6);

    // Calculate other column widths based on content
    let unicode_col_width = infos.iter()
        .map(|info| info.unicode_width.to_string().len())
        .max().unwrap_or(0).max(7); // minimum for "Unicode" header

    let chars_col_width = infos.iter()
        .map(|info| info.char_count.to_string().len())
        .max().unwrap_or(0).max(5); // minimum for "Chars" header

    let bytes_col_width = infos.iter()
        .map(|info| info.byte_count.to_string().len())
        .max().unwrap_or(0).max(5); // minimum for "Bytes" header

    let est_col_width = infos.iter()
        .map(|info| info.estimated_terminal_width.to_string().len())
        .max().unwrap_or(0).max(8); // minimum for "Est Term" header

    // Print headers with calculated widths
    println!("{:<width_text$} {:<width_uni$} {:<width_chars$} {:<width_bytes$} {:<width_est$}",
             "Text", "Unicode", "Chars", "Bytes", "Est Term",
             width_text = text_col_width,
             width_uni = unicode_col_width,
             width_chars = chars_col_width,
             width_bytes = bytes_col_width,
             width_est = est_col_width);

    // Print separator line
    let total_width = text_col_width + unicode_col_width + chars_col_width + bytes_col_width + est_col_width + 4; // +4 for spaces
    println!("{}", "─".repeat(total_width));

    // Print data rows with proper alignment
    for (i, &text) in chars.iter().enumerate() {
        let info = &infos[i];
        let text_display = format!("'{}'", text);

        // Calculate padding needed for this specific text due to emoji width differences
        let display_width = text_display_widths[i];
        let padding_needed = if display_width < text_col_width {
            text_col_width - display_width
        } else {
            0
        };

        print!("{}{}", text_display, " ".repeat(padding_needed));
        print!(" {:<width_uni$}", info.unicode_width, width_uni = unicode_col_width);
        print!(" {:<width_chars$}", info.char_count, width_chars = chars_col_width);
        print!(" {:<width_bytes$}", info.byte_count, width_bytes = bytes_col_width);
        println!(" {:<width_est$}", info.estimated_terminal_width, width_est = est_col_width);
    }
}

/// Macro for quick emoji debugging
#[macro_export]
macro_rules! debug_emoji {
    ($text:expr) => {
        {
            let info = $crate::emoji_debug::EmojiDebugInfo::new($text);
            info.print_debug();
            info
        }
    };
}

/// Macro for comparing multiple emojis
#[macro_export]
macro_rules! compare_emojis {
    ($($text:expr),+ $(,)?) => {
        {
            let chars = vec![$($text),+];
            $crate::emoji_debug::compare_chars(&chars);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width_calculations() {
        // Simple ASCII
        assert_eq!(get_unicode_width("X"), 1);
        assert_eq!(get_estimated_terminal_width("X"), 1);

        // Simple emoji
        let checkmark_info = EmojiDebugInfo::new("✅");
        println!("Checkmark debug: {:?}", checkmark_info);

        // Compound emoji
        let info_info = EmojiDebugInfo::new("ℹ️");
        println!("Info debug: {:?}", info_info);

        // Compare different emojis
        compare_chars(&["X", "✅", "ℹ️", "🚀"]);
    }
}