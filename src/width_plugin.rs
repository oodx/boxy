

use crate::{File, Command, Stdio};



/// Validate width input
pub fn validate_width(width_str: &str) -> Result<(), String> {
  match width_str.parse::<usize>() {
    Ok(w) if w >= 10 && w <= 200 => Ok(()),
    Ok(w) => Err(format!("Width {} out of range (10-200)", w)),
    Err(_) => Err("Width must be a number".to_string()),
  }
}

/// Width diagnostics subcommand
pub fn handle_width_command() {
    // Helper to run command with /dev/tty as stdin when available
    fn run_with_tty(mut cmd: Command) -> Option<String> {
        if let Ok(tty) = File::open("/dev/tty") {
            let _ = cmd.stdin(Stdio::from(tty));
        }
        cmd.output().ok().and_then(|o| String::from_utf8(o.stdout).ok())
    }

    // Gather tput cols (tty)
    let tput_cols_tty = {
        let mut c = Command::new("tput");
        c.arg("cols");
        run_with_tty(c).and_then(|s| s.trim().parse::<usize>().ok())
    };

    // Gather stty size (rows cols) via tty
    let stty_cols_tty = {
        let mut c = Command::new("stty");
        c.arg("size");
        run_with_tty(c).and_then(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() == 2 { parts[1].parse::<usize>().ok() } else { None }
        })
    };

    let effective = get_terminal_width();
    
    println!("Width diagnostics:");
    println!("  effective (get_terminal_width): {}", effective);
    println!("  tput cols (tty): {}", tput_cols_tty.map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()));
    println!("  stty size cols (tty): {}", stty_cols_tty.map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()));
}

/// Get terminal width with fallback to 80 columns
pub fn get_terminal_width() -> usize {
    // Helper to run with /dev/tty
    fn run_with_tty(mut cmd: Command) -> Option<String> {
        if let Ok(tty) = File::open("/dev/tty") {
            let _ = cmd.stdin(Stdio::from(tty));
        }
        cmd.output().ok().and_then(|o| String::from_utf8(o.stdout).ok())
    }

    // Try tput cols with tty (preferred)
    {
        let mut c = Command::new("tput");
        c.arg("cols");
        if let Some(out) = run_with_tty(c) {
        if let Ok(width) = out.trim().parse::<usize>() {
            if width >= 10 { return width; }
        }
        }
    }

    // Try stty size with tty
    {
        let mut c = Command::new("stty");
        c.arg("size");
        if let Some(out) = run_with_tty(c) {
        let parts: Vec<&str> = out.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(width) = parts[1].trim().parse::<usize>() {
                if width >= 10 { return width; }
            }
        }
        }
    }

    80
}

/// Our custom width calculation based on emoji research
pub fn get_display_width_custom(text: &str) -> usize {
    let clean = strip_ansi_escapes::strip(text);
    let clean_str = String::from_utf8_lossy(&clean);

    let mut width = 0;
    let mut chars = clean_str.chars().peekable();

    while let Some(ch) = chars.next() {
        // Check if next char is a variation selector for special handling
        let next_is_emoji_selector = chars.peek() == Some(&'\u{FE0F}');

        width += match ch {
            // ASCII characters are always 1 width
            c if c.is_ascii() => 1,

            // Zero-width characters (but handle emoji variation selectors specially)
            '\u{200B}' | // Zero Width Space
            '\u{200C}' | // Zero Width Non-Joiner
            '\u{200D}' | // Zero Width Joiner
            '\u{FEFF}'   // Zero Width No-Break Space
            => 0,

            // Variation selectors: these modify display but don't add width
            '\u{FE0E}' | // Variation Selector-15 (text style)
            '\u{FE0F}' => { // Variation Selector-16 (emoji style)
                // Variation selectors are zero-width modifiers
                // They change how the previous character is displayed but don't add width
                0
            },

            // Special case: characters that become emoji with variation selector
            '\u{2049}' | // ⁉ exclamation question mark
            '\u{203C}' | // ‼ double exclamation mark
            '\u{00A9}' | // © copyright
            '\u{00AE}'   // ® registered
            if next_is_emoji_selector => {
                // These become width 2 emoji when followed by FE0F
                // Skip the variation selector since we're handling it here
                chars.next();
                2
            },

            // Special case: ℹ️ has inconsistent width across terminals
            // Most terminals render it as width 1, Kitty as width 2
            // We follow the majority behavior for better compatibility
            '\u{2139}' if next_is_emoji_selector => {
                // Skip the variation selector
                chars.next();
                // Despite Unicode spec saying emoji presentation should be width 2,
                // most terminals (Terminal.app, etc.) render ℹ️ as width 1
                1
            },

            // Common double-width emoji ranges
            '\u{1F600}'..='\u{1F64F}' | // Emoticons
            '\u{1F300}'..='\u{1F5FF}' | // Misc Symbols and Pictographs
            '\u{1F680}'..='\u{1F6FF}' | // Transport and Map Symbols
            '\u{1F700}'..='\u{1F77F}' | // Alchemical Symbols
            '\u{1F780}'..='\u{1F7FF}' | // Geometric Shapes Extended
            '\u{1F800}'..='\u{1F8FF}' | // Supplemental Arrows-C
            '\u{1F900}'..='\u{1F9FF}' | // Supplemental Symbols and Pictographs
            '\u{1FA00}'..='\u{1FA6F}' | // Chess Symbols
            '\u{1FA70}'..='\u{1FAFF}' | // Symbols and Pictographs Extended-A
            '\u{2600}'..='\u{26FF}' |   // Miscellaneous Symbols
            '\u{2700}'..='\u{27BF}' |   // Dingbats
            '\u{1F1E6}'..='\u{1F1FF}'   // Regional Indicator Symbols
            => 2,

            // CJK characters are typically double-width
            '\u{4E00}'..='\u{9FFF}' |   // CJK Unified Ideographs
            '\u{3400}'..='\u{4DBF}' |   // CJK Unified Ideographs Extension A
            '\u{AC00}'..='\u{D7AF}' |   // Hangul Syllables
            '\u{3040}'..='\u{309F}' |   // Hiragana
            '\u{30A0}'..='\u{30FF}'     // Katakana
            => 2,

            // Special cases based on our research (moved above general ranges)
            // Note: ✅ (U+2705) is already covered by Miscellaneous Symbols range
            // Note: ℹ (U+2139) would be covered by general case

            // Most other Unicode characters default to 1
            _ => 1,
        };
    }

    width
}

/// Compare our custom width with unicode-width library
pub fn compare_width_methods(text: &str) -> (usize, usize) {
    let custom = get_display_width_custom(text);
    let standard = unicode_width::UnicodeWidthStr::width(text);
    (custom, standard)
}

/// Legacy function - now redirects to custom implementation
/// (kept for compatibility with comparison tools)
pub fn get_display_width_unicode_crate(text: &str) -> usize {
    // Now uses standard unicode-width library for comparison
    unicode_width::UnicodeWidthStr::width(text)
}

/// Main width function - strips ANSI codes then calculates width
pub fn get_display_width(text: &str) -> usize {
    // Strip ANSI escape codes first, then use unicode-width
    let clean = strip_ansi_escapes::strip(text);
    let clean_str = String::from_utf8_lossy(&clean);
    unicode_width::UnicodeWidthStr::width(&*clean_str)
}
