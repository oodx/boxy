

use crate::{HashMap, UnicodeWidthStr, Regex};
use crate::width_plugin::*;

// RSB framework imports
use rsb::param;



#[derive(Default, Debug)]
pub struct ParsedContent {
  pub header: Option<String>,
  pub footer: Option<String>,
  pub status: Option<String>,
  pub title: Option<String>,
  pub body: Option<String>,
  pub icon: Option<String>,
  pub layout: Option<String>,
  pub title_color: Option<String>,
  pub status_color: Option<String>,
  pub header_color: Option<String>,
  pub footer_color: Option<String>
}

pub fn expand_variables(text: &str) -> String {
    let mut result = text.to_string();
    let var_regex = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for cap in var_regex.captures_iter(text) {
        if let Some(var_name) = cap.get(1) {
            let value = param!(var_name.as_str());
            if !value.is_empty() {
                result = result.replace(&cap[0], &value);
            }
        }
    }
    result
}

pub fn unescape_stream_value(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some(other) => { out.push(other); },
                None => break,
            }
        } else if c == '/' {
            if let Some('n') = chars.peek().copied() { chars.next(); out.push('\n'); } else { out.push(c); }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn parse_content_stream(input: &str) -> Option<ParsedContent> {
    // Matches k='v' with single quotes; non-greedy across newlines; optional trailing semicolon
    let re = Regex::new(r"(?s)([A-Za-z]{2})\s*=\s*'(.+?)'\s*;?").ok()?;
    let mut map: HashMap<String, String> = HashMap::new();
    for cap in re.captures_iter(input) {
        let k = cap.get(1).map(|m| m.as_str().to_lowercase()).unwrap_or_default();
        let v_raw = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let v = unescape_stream_value(v_raw);
        map.insert(k, v);
    }
    if map.is_empty() {
        return None;
    }
    let mut pc = ParsedContent::default();
    if let Some(v) = map.remove("hd") { pc.header = Some(v); }
    if let Some(v) = map.remove("ft") { pc.footer = Some(v); }
    if let Some(v) = map.remove("st") { pc.status = Some(v); }
    if let Some(v) = map.remove("tl") { pc.title = Some(v); }
    // Body (bd) intentionally ignored; body should come from piped stdin
    if let Some(v) = map.remove("ic") { pc.icon = Some(v); }
    if let Some(v) = map.remove("tc") { pc.title_color = Some(v); }
    if let Some(v) = map.remove("sc") { pc.status_color = Some(v); }
    // If nothing recognized, return None to avoid hijacking arbitrary input
    if pc.header.is_none() && pc.footer.is_none() && pc.status.is_none() && pc.title.is_none() && pc.body.is_none() && pc.icon.is_none() {
        None
    } else {
        Some(pc)
    }
}

pub fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    
    let text_width = get_display_width(text);
    if text_width <= max_width {
        return text.to_string();
    }
    
    // Unicode ellipsis character
    const ELLIPSIS: &str = "…";
    const ELLIPSIS_WIDTH: usize = 1;
    
    if max_width <= ELLIPSIS_WIDTH {
        return ELLIPSIS.to_string();
    }
    
    let target_width = max_width - ELLIPSIS_WIDTH;
    let mut result = String::new();
    let mut current_width = 0;
    
    for ch in text.chars() {
        let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());
        if current_width + ch_width > target_width {
            break;
        }
        result.push(ch);
        current_width += ch_width;
    }
    
    result.push_str(ELLIPSIS);
    result
}

pub fn render_title_or_footer(text: &str, total_width: usize, style_char: &str, align: &str) -> String {
    if total_width < 4 {
        // Minimum viable box: just return style chars
        return style_char.repeat(total_width);
    }
    
    // Enhanced title processing: auto-detect and format icons in titles
    let processed_text = if text.contains(" ") {
        // If title contains spaces, check for icon patterns
        let parts: Vec<&str> = text.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let potential_icon = parts[0];
            let title_text = parts[1];
            
            // Check if first part looks like an icon/emoji (non-ASCII characters)
            if potential_icon.chars().any(|c| !c.is_ascii()) {
                format!("{} {}", potential_icon, title_text)
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        }
    } else {
        text.to_string()
    };
    
    let text_width = get_display_width(&processed_text);
    let available_width = total_width.saturating_sub(2); // Space for " text "
    
    let final_text = if text_width > available_width {
        truncate_with_ellipsis(&processed_text, available_width)
    } else {
        processed_text
    };
    
    let final_text_width = get_display_width(&final_text);
    // CRITICAL FIX: Use saturating_sub to prevent underflow
    let remaining_width = total_width.saturating_sub(final_text_width + 2); // -2 for spaces around text
    let (left_pad, right_pad) = match align {
        "left" => (0, remaining_width),
        "right" => (remaining_width, 0),
        _ => {
            let lp = remaining_width / 2;
            (lp, remaining_width.saturating_sub(lp))
        }
    };
    
    format!("{} {} {}", 
        style_char.repeat(left_pad), 
        final_text, 
        style_char.repeat(right_pad))
}

