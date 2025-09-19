//! Internal color helpers
//!
//! This module contains internal implementations consumed by utils.rs.
//! Functions here are not part of the public API.

/// Provide fallback suggestions for invalid color names
///
/// Internal helper used by validate_color() to generate helpful error messages.
pub fn get_color_suggestion(color: &str) -> Result<&'static str, String> {
    let suggestion = match color {
        c if c.contains("red") => Some("crimson"),
        c if c.contains("green") => Some("emerald"),
        c if c.contains("blue") => Some("azure"),
        c if c.contains("yellow") => Some("amber"),
        c if c.contains("purple") => Some("violet"),
        c if c.contains("orange") => Some("tangerine"),
        c if c.contains("grey") || c.contains("gray") => Some("slate"),
        _ => None,
    };

    if let Some(fallback) = suggestion {
        Err(format!("Unknown color '{}'. Did you mean '{}'?", color, fallback))
    } else {
        Err(format!("Unknown color '{}'. Use --help to see available colors.", color))
    }
}

/// Pad a string to a specific width, accounting for ANSI escape sequences
///
/// Internal helper for formatting color help output.
pub fn pad_cell(s: &str, width: usize) -> String {
    // Strip ANSI escape sequences to get actual visible width
    let visible_text = strip_ansi_codes(s);
    let visible_len = visible_text.chars().count();

    if visible_len >= width {
        return s.to_string();
    }

    let pad = " ".repeat(width - visible_len);
    format!("{}{}", s, pad)
}

/// Strip ANSI escape sequences from text for accurate width calculation
///
/// Internal implementation for removing color codes when calculating text width.
pub fn strip_ansi_codes(text: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        if ch == '\x1B' {
            in_escape = true;
            continue;
        }

        if in_escape {
            if ch.is_alphabetic() {
                in_escape = false;
            }
            continue;
        }

        result.push(ch);
    }

    result
}