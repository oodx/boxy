//repair
use unicode_width::UnicodeWidthStr;
use std::io::{self, Read, Write};
use std::env;
use std::process::{Command, Stdio};
use std::fs::{self, File};
use std::path::PathBuf;
use regex::Regex;
use std::collections::HashMap;

mod colors;
mod jynx_plugin;
mod theme_engine;
mod help;
use colors::*;
use jynx_plugin::*;
use theme_engine::{ThemeEngine, BoxyTheme, ThemeFile, ThemeMetadata, ThemeSettings};
use help::*;



struct BoxStyle {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
    tee_left: &'static str,
    tee_right: &'static str,
    #[allow(dead_code)]
    cross: &'static str,
}

const NORMAL: BoxStyle = BoxStyle {
    top_left: "┌", top_right: "┐",
    bottom_left: "└", bottom_right: "┘",
    horizontal: "─", vertical: "│",
    tee_left: "├", tee_right: "┤", cross: "┼",
};

const ROUNDED: BoxStyle = BoxStyle {
    top_left: "╭", top_right: "╮",
    bottom_left: "╰", bottom_right: "╯",
    horizontal: "─", vertical: "│",
    tee_left: "├", tee_right: "┤", cross: "┼",
};

const DOUBLE: BoxStyle = BoxStyle {
    top_left: "╔", top_right: "╗",
    bottom_left: "╚", bottom_right: "╝",
    horizontal: "═", vertical: "║",
    tee_left: "╠", tee_right: "╣", cross: "╬",
};

const HEAVY: BoxStyle = BoxStyle {
    top_left: "┏", top_right: "┓",
    bottom_left: "┗", bottom_right: "┛",
    horizontal: "━", vertical: "┃",
    tee_left: "┣", tee_right: "┫", cross: "╋",
};

const ASCII: BoxStyle = BoxStyle {
    top_left: "+", top_right: "+",
    bottom_left: "+", bottom_right: "+",
    horizontal: "-", vertical: "|",
    tee_left: "+", tee_right: "+", cross: "+",
};





#[derive(Default, Debug)]
struct ParsedContent {
    header: Option<String>,
    footer: Option<String>,
    status: Option<String>,
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    layout: Option<String>,
    title_color: Option<String>,
    status_color: Option<String>,
    header_color: Option<String>,
    footer_color: Option<String>
}

fn unescape_stream_value(s: &str) -> String {
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

fn parse_content_stream(input: &str) -> Option<ParsedContent> {
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

/// Width diagnostics subcommand
fn handle_width_command() {
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
fn get_terminal_width() -> usize {
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

fn get_display_width(text: &str) -> usize {
    let clean = strip_ansi_escapes::strip(text);
    let clean_str = String::from_utf8_lossy(&clean);
    UnicodeWidthStr::width(clean_str.as_ref())
}

fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
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

fn render_title_or_footer(text: &str, total_width: usize, style_char: &str, align: &str) -> String {
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

fn expand_variables(text: &str) -> String {
    let mut result = text.to_string();
    let var_regex = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    
    for cap in var_regex.captures_iter(text) {
        if let Some(var_name) = cap.get(1) {
            if let Ok(value) = env::var(var_name.as_str()) {
                result = result.replace(&cap[0], &value);
            }
        }
    }
    result
}

fn strip_box(text: &str, strict: bool) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut content_lines = Vec::new();
    
    // Box drawing characters to detect
    let box_chars = "┌┐└┘─│╭╮╰╯═║╔╗╚╝━┃┏┓┗┛+-|";
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip lines that look like box borders
        if i == 0 || i == lines.len() - 1 {
            if trimmed.chars().all(|c| box_chars.contains(c) || c.is_whitespace()) {
                continue;
            }
        }
        
        // Process content lines
        let mut content = line.to_string();
        
        // Remove box sides
        if content.len() > 2 {
            let chars: Vec<char> = content.chars().collect();
            if box_chars.contains(chars[0]) {
                content = content.chars().skip(1).collect();
            }
            let chars: Vec<char> = content.chars().collect();
            if !chars.is_empty() && box_chars.contains(chars[chars.len() - 1]) {
                content = content.chars().take(content.len() - 1).collect();
            }
        }
        
        // Trim padding
        content = content.trim().to_string();
        
        if strict {
            // Strip all ANSI codes
            let clean = strip_ansi_escapes::strip(&content);
            content = String::from_utf8_lossy(&clean).to_string();
            
            // Remove emojis and special Unicode (keep basic ASCII)
            content = content.chars()
                .filter(|c| c.is_ascii())
                .collect();
        }
        
        if !content.is_empty() || !strict {
            content_lines.push(content);
        }
    }
    
    content_lines.join("\n")
}

fn draw_box(text: &str, h_padding: usize, _v_padding: usize, style: &BoxStyle, color: &str, text_color: &str, title: Option<&str>, footer: Option<&str>, icon: Option<&str>, fixed_width: Option<usize>, status_bar: Option<&str>, header: Option<&str>, header_align: &str, footer_align: &str, status_align_override: Option<&str>, divider_after_title: bool, divider_before_status: bool, pad_after_title_divider: bool, pad_before_status_divider: bool, pad_before_title: bool, pad_after_status: bool, pad_after_title: bool, pad_before_status: bool, title_color_name: Option<&str>, status_color_name: Option<&str>, body_align: &str, body_pad_emoji: bool, pad_body_above: bool, pad_body_below: bool){
    let terminal_width = get_terminal_width();
    
    // Calculate effective box width - respect terminal constraints
    let box_width = match fixed_width {
        Some(w) => {
            if w > terminal_width {
                // LIPSIFY: Fixed width exceeds terminal, constrain it
                terminal_width
            } else {
                w
            }
        },
        None => {
            // Auto-size but constrain to terminal width
            let lines: Vec<&str> = text.lines().collect();
            let content_max_width = lines.iter()
                .map(|line| get_display_width(line))
                .max()
                .unwrap_or(0);
            let ideal_width = content_max_width + 2 * h_padding + 2; // +2 for borders
            
            if ideal_width > terminal_width {
                // LIPSIFY: Content too wide for terminal
                terminal_width
            } else {
                ideal_width
            }
        }
    };
    
    // Ensure minimum viable box size
    let final_width = if box_width < 4 { 4 } else { box_width };
    let lines: Vec<&str> = text.lines().collect();
    let inner_width = final_width.saturating_sub(2); // Account for borders
    let color_code = get_color_code(color);
    
    // Determine text color: "auto" means match box color, "none" means default
    let text_color_code = match text_color {
        "auto" => get_color_code(color), // Use same color as box
        "none" => "",                    // Default terminal color
        _ => get_color_code(text_color), // Explicit color
    };
    
    let pad = " ".repeat(h_padding);
    let title_color_code = title_color_name.map(|n| get_color_code(n)).unwrap_or("");
    let status_color_code = status_color_name.map(|n| get_color_code(n)).unwrap_or("");
    
    // Top border with optional HEADER inside the border
    if let Some(header_text) = header {
        let expanded_header = expand_variables(header_text);
        let header_line = render_title_or_footer(&expanded_header, inner_width, style.horizontal, header_align);
        println!("{}{}{}{}{}", color_code, style.top_left, header_line, style.top_right, RESET);
    } else {
        let border = style.horizontal.repeat(inner_width);
        println!("{}{}{}{}{}", color_code, style.top_left, border, style.top_right, RESET);
    }
    
    // Content lines - LIPSIFIED for all cases
    // Compose content lines; if title is provided, insert it as the first line
    let mut composed_lines: Vec<String> = Vec::new();
    if let Some(title_text) = title {
        composed_lines.push(expand_variables(title_text));
    }
    composed_lines.extend(lines.iter().map(|l| (*l).to_string()));

    // Optional padding blank line before title
    if pad_before_title && title.is_some() {
        let available_content_width = inner_width.saturating_sub(2 * h_padding);
        println!(
            "{}{}{}{}{}{}{}{}",
            color_code,
            style.vertical,
            RESET,
            pad,
            " ".repeat(available_content_width),
            pad,
            format!("{}{}{}", color_code, style.vertical, RESET),
            ""
        );
    }

    for (i, line) in composed_lines.iter().enumerate() {
        let available_content_width = inner_width.saturating_sub(2 * h_padding);
        
        // LIPSIFY: Always truncate if line exceeds available width
        let line_width = get_display_width(&line);
        let display_line = if line_width > available_content_width {
            truncate_with_ellipsis(&line, available_content_width)
        } else {
            line.to_string()
        };
        
        let width = get_display_width(&display_line);
        let spaces = " ".repeat(available_content_width.saturating_sub(width));
        
        if i == 0 && icon.is_some() {
            // Avoid duplicate icon if the title line already starts with an emoji/non-ASCII
            let starts_with_emoji = line.chars().next().map(|c| !c.is_ascii()).unwrap_or(false);
            if starts_with_emoji {
                // Fall through to normal rendering without icon injection
                let line_code = if !title_color_code.is_empty() { title_color_code } else { text_color_code };
                let colored_display_line = if line_code.is_empty() { display_line.to_string() } else { format!("{}{}{}", line_code, display_line, RESET) };
                println!("{}{}{}{}{}{}{}{}",
                    color_code, style.vertical, RESET,
                    pad, colored_display_line, spaces, pad,
                    format!("{}{}{}", color_code, style.vertical, RESET));
                // Continue to next line
                continue;
            }
            // First line with icon - LIPSIFIED
            let icon_str = icon.unwrap();
            let icon_expanded = expand_variables(icon_str);
            
            // LIPSIFY: Account for icon when truncating
            let icon_width = get_display_width(&icon_expanded) + 1; // +1 for space
            let line_width = get_display_width(line);
            let final_line = if line_width > available_content_width.saturating_sub(icon_width) {
                truncate_with_ellipsis(line, available_content_width.saturating_sub(icon_width))
            } else {
                display_line
            };
            
            // Apply text color to the text part only (not icon)
            let line_code = if !title_color_code.is_empty() { title_color_code } else { text_color_code };
            let colored_final_line = if line_code.is_empty() {
                final_line.to_string()
            } else {
                format!("{}{}{}", line_code, final_line, RESET)
            };
            
            let final_width = get_display_width(&final_line);
            let final_spaces = " ".repeat(available_content_width.saturating_sub(final_width + icon_width));
            
            println!("{}{} {}{}{}{}{}{}{}",
                color_code, style.vertical, RESET,
                icon_expanded, " ",
                colored_final_line, final_spaces, pad,
                format!("{}{}{}", color_code, style.vertical, RESET));
        } else {
            // Apply text/status/title color to the display line
            let line_code = if i == 0 && !title_color_code.is_empty() { title_color_code } else { text_color_code };
            let colored_display_line = if line_code.is_empty() {
                display_line.to_string()
            } else {
                format!("{}{}{}", line_code, display_line, RESET)
            };
            
            println!("{}{}{}{}{}{}{}{}",
                color_code, style.vertical, RESET,
                pad, colored_display_line, spaces, pad,
                format!("{}{}{}", color_code, style.vertical, RESET));
        }

        if divider_after_title && i == 0 {
            println!(
                "{}{}{}{}{}",
                color_code,
                style.tee_left,
                style.horizontal.repeat(inner_width),
                style.tee_right,
                RESET
            );
            if pad_after_title_divider {
                let available_content_width = inner_width.saturating_sub(2 * h_padding);
                println!(
                    "{}{}{}{}{}{}{}{}",
                    color_code,
                    style.vertical,
                    RESET,
                    pad,
                    " ".repeat(available_content_width),
                    pad,
                    format!("{}{}{}", color_code, style.vertical, RESET),
                    ""
                );
            }
        } else if pad_after_title && i == 0 {
            // Optional padding blank line after title when no divider requested
            let available_content_width = inner_width.saturating_sub(2 * h_padding);
            println!(
                "{}{}{}{}{}{}{}{}",
                color_code,
                style.vertical,
                RESET,
                pad,
                " ".repeat(available_content_width),
                pad,
                format!("{}{}{}", color_code, style.vertical, RESET),
                ""
            );
        }
    }
    
    // Optional STATUS line rendered inside the box (before footer)
    if let Some(status_text) = status_bar {
        if pad_before_status {
            let available_content_width = inner_width.saturating_sub(2 * h_padding);
            println!(
                "{}{}{}{}{}{}{}{}",
                color_code,
                style.vertical,
                RESET,
                pad,
                " ".repeat(available_content_width),
                pad,
                format!("{}{}{}", color_code, style.vertical, RESET),
                ""
            );
        }
        if divider_before_status {
            if pad_before_status_divider {
                let available_content_width = inner_width.saturating_sub(2 * h_padding);
                println!(
                    "{}{}{}{}{}{}{}{}",
                    color_code,
                    style.vertical,
                    RESET,
                    pad,
                    " ".repeat(available_content_width),
                    pad,
                    format!("{}{}{}", color_code, style.vertical, RESET),
                    ""
                );
            }
            println!(
                "{}{}{}{}{}",
                color_code,
                style.tee_left,
                style.horizontal.repeat(inner_width),
                style.tee_right,
                RESET
            );
        }
        let expanded_status = expand_variables(status_text);
        let (alignment, clean_status) = if let Some(ov) = status_align_override { (ov, expanded_status) } else if expanded_status.starts_with("sl:") {
            ("left", expanded_status.strip_prefix("sl:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("sc:") {
            ("center", expanded_status.strip_prefix("sc:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("sr:") {
            ("right", expanded_status.strip_prefix("sr:").unwrap_or(&expanded_status).to_string())
        } else {
            ("left", expanded_status)
        };

        let available_content_width = inner_width.saturating_sub(2 * h_padding);
        let status_display = if get_display_width(&clean_status) > available_content_width {
            truncate_with_ellipsis(&clean_status, available_content_width)
        } else {
            clean_status
        };

        let final_width = get_display_width(&status_display);
        let (left_pad_inner, right_pad_inner) = match alignment {
            "center" => {
                let space = available_content_width.saturating_sub(final_width);
                let lp = space / 2; (lp, space.saturating_sub(lp))
            }
            "right" => {
                let space = available_content_width.saturating_sub(final_width);
                (space, 0)
            }
            _ => (0, available_content_width.saturating_sub(final_width)),
        };

        let status_line = format!("{}{}{}", " ".repeat(left_pad_inner), status_display, " ".repeat(right_pad_inner));
        let status_code = if !status_color_code.is_empty() { status_color_code } else { text_color_code };
        let colored_status = if status_code.is_empty() { status_line } else { format!("{}{}{}", status_code, status_line, RESET) }; //todo: status_code color code? doesnt do anything yet, incomplete feature

        println!("{}{}{}{}{}{}{}{}",
            color_code, style.vertical, RESET,
            pad, colored_status, pad,
            format!("{}{}{}", color_code, style.vertical, RESET),
            "");

        // Optional padding blank line after status
        if pad_after_status {
            let available_content_width = inner_width.saturating_sub(2 * h_padding);
            println!(
                "{}{}{}{}{}{}{}{}",
                color_code,
                style.vertical,
                RESET,
                pad,
                " ".repeat(available_content_width),
                pad,
                format!("{}{}{}", color_code, style.vertical, RESET),
                ""
            );
        }
    }

    // Bottom border with optional FOOTER inside the border
    if let Some(footer_text) = footer {
        let expanded_footer = expand_variables(footer_text);
        let footer_line = render_title_or_footer(&expanded_footer, inner_width, style.horizontal, footer_align);
        println!("{}{}{}{}{}", color_code, style.bottom_left, footer_line, style.bottom_right, RESET);
    } else {
        let border = style.horizontal.repeat(inner_width);
        println!("{}{}{}{}{}", color_code, style.bottom_left, border, style.bottom_right, RESET);
    }
}



/// Handle theme subcommands: list, show, etc.
fn handle_theme_command(args: &[String], jynx: &JynxPlugin) {
    if args.is_empty() {
        eprintln!("Theme command requires an action. Usage: {} theme <action>", NAME);
        eprintln!("Available actions: list, show <theme>, help");
        std::process::exit(1);
    }
    
    match args[0].as_str() {
        "list" => {
            match ThemeEngine::new() {
                Ok(theme_engine) => {
                    let themes = theme_engine.list_themes();
                    if themes.is_empty() {
                        println!("No themes available.");
                        return;
                    }
                    
                    // Build theme list content
                    let mut theme_content = String::new();
                    theme_content.push_str(&format!("{} {} - Available Themes\n", NAME, VERSION));
                    theme_content.push('\n');
                    
                    for (name, description) in themes {
                        theme_content.push_str(&format!("  {} - {}\n", name, description));
                    }
                    
                    theme_content.push('\n');
                    theme_content.push_str(&format!("Usage: {} --theme <theme_name>\n", NAME));
                    
                    // Use jynx for enhanced theme list display
                    if jynx.is_active() {
                        jynx_println(&theme_content, "theme_list", jynx);
                    } else {
                        print!("{}", theme_content);
                    }
                }
                Err(e) => {
                    eprintln!("Error: Failed to load theme engine: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "create" => {
            if args.len() < 2 {
                eprintln!("Error: Theme create requires a name. Usage: {} theme create <name>", NAME);
                std::process::exit(1);
            }
            handle_theme_create(&args[1], jynx);
        }
        "import" => {
            if args.len() < 2 {
                eprintln!("Error: Theme import requires a path. Usage: {} theme import <path>", NAME);
                std::process::exit(1);
            }
            handle_theme_import(&args[1]);
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("Error: Theme export requires a name. Usage: {} theme export <name>", NAME);
                std::process::exit(1);
            }
            handle_theme_export(&args[1]);
        }
        "edit" => {
            if args.len() < 2 {
                eprintln!("Error: Theme edit requires a name. Usage: {} theme edit <name>", NAME);
                std::process::exit(1);
            }
            handle_theme_edit(&args[1]);
        }
        "help" => {
            println!("{} {} - Theme Management", NAME, VERSION);
            println!();
            println!("USAGE:");
            println!("    {} theme <action> [options]", NAME);
            println!();
            println!("ACTIONS:");
            println!("    list              List all available themes");
            println!("    show <theme>      Show detailed theme information");
            println!("    create <name>     Create a new theme interactively");
            println!("    import <path>     Import theme from file");
            println!("    export <name>     Export theme to file");
            println!("    edit <name>       Edit existing theme");
            println!("    help              Show this help message");
            println!();
            println!("EXAMPLES:");
            println!("    {} theme list", NAME);
            println!("    {} theme show error", NAME);
            println!("    {} theme create my_theme", NAME);
            println!("    {} theme export error > error.yml", NAME);
        }
        action => {
            eprintln!("Unknown theme action: {}", action);
            eprintln!("Available actions: list, show, create, import, export, edit, help");
            eprintln!("Use '{} theme help' for more information", NAME);
            std::process::exit(1);
        }
    }
}

/// Validate theme file before import
fn validate_theme_file(path: &PathBuf) -> Result<(), String> {
    // Read and parse the theme file
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read theme file: {}", e))?;
    
    let theme_file: ThemeFile = serde_yaml::from_str(&content)
        .map_err(|e| format!("Invalid YAML format: {}", e))?;
    
    // Validate each theme in the file
    let temp_engine = ThemeEngine::new()
        .map_err(|e| format!("Failed to initialize validator: {}", e))?;
    
    let mut validation_errors = Vec::new();
    
    for (theme_name, theme) in &theme_file.themes {
        if let Err(e) = temp_engine.validate_theme(theme) {
            validation_errors.push(format!("Theme '{}': {}", theme_name, e));
        }
    }
    
    // Validate metadata
    if theme_file.metadata.name.is_empty() {
        validation_errors.push("Missing or empty metadata.name".to_string());
    }
    
    if theme_file.metadata.version.is_empty() {
        validation_errors.push("Missing or empty metadata.version".to_string());
    }
    
    if !validation_errors.is_empty() {
        return Err(format!("Validation errors:\n  {}", validation_errors.join("\n  ")));
    }
    
    Ok(())
}


/// Validate style input
fn validate_style(style: &str) -> Result<(), String> {
    let valid_styles = vec!["normal", "rounded", "double", "heavy", "ascii"];
    if !valid_styles.contains(&style) {
        return Err(format!("Invalid style '{}'. Valid styles: {}", style, valid_styles.join(", ")));
    }
    Ok(())
}

/// Validate width input
fn validate_width(width_str: &str) -> Result<(), String> {
    match width_str.parse::<usize>() {
        Ok(w) if w >= 10 && w <= 200 => Ok(()),
        Ok(w) => Err(format!("Width {} out of range (10-200)", w)),
        Err(_) => Err("Width must be a number".to_string()),
    }
}

/// Validate theme name
fn validate_theme_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Theme name cannot be empty".to_string());
    }
    
    if name.len() > 50 {
        return Err("Theme name too long (max 50 characters)".to_string());
    }
    
    // Allow alphanumeric, underscore, hyphen, and dot
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return Err("Theme name can only contain letters, numbers, underscore, hyphen, and dot".to_string());
    }
    
    // Don't allow names that start with reserved prefixes
    let reserved_prefixes = vec!["builtin_", "system_", "default_"];
    for prefix in reserved_prefixes {
        if name.starts_with(prefix) {
            return Err(format!("Theme name cannot start with reserved prefix '{}'", prefix));
        }
    }
    
    Ok(())
}

/// Interactive theme creation utility
fn create_theme_interactively(name: &str) -> BoxyTheme {
    println!("Configure theme '{}' (press Enter for default):", name);
    println!();
    
    // Color selection
    print!("Box color [azure]: ");
    io::stdout().flush().unwrap();
    let mut color_input = String::new();
    io::stdin().read_line(&mut color_input).unwrap();
    let color = color_input.trim();
    let mut color = if color.is_empty() { "azure" } else { color };
    
    // Validate color
    if let Err(e) = validate_color(color) {
        println!("Warning: {}", e);
        println!("Using default color 'azure'");
        color = "azure";
    }
    
    // Text color selection  
    print!("Text color [auto/none/color]: ");
    io::stdout().flush().unwrap();
    let mut text_color_input = String::new();
    io::stdin().read_line(&mut text_color_input).unwrap();
    let text_color = text_color_input.trim();
    let text_color = if text_color.is_empty() { "auto" } else { text_color };
    
    // Style selection with validation loop
    let style = loop {
        print!("Border style [normal/rounded/double/heavy/ascii]: ");
        io::stdout().flush().unwrap();
        let mut style_input = String::new();
        io::stdin().read_line(&mut style_input).unwrap();
        let style = style_input.trim();
        let style = if style.is_empty() { "normal" } else { style };
        
        if let Err(e) = validate_style(style) {
            println!("Error: {}", e);
            continue;
        }
        break style.to_string();
    };
    
    // Icon/emoji
    print!("Icon/emoji [optional]: ");
    io::stdout().flush().unwrap();
    let mut icon_input = String::new();
    io::stdin().read_line(&mut icon_input).unwrap();
    let icon = icon_input.trim();
    let icon = if icon.is_empty() { None } else { Some(icon.to_string()) };
    
    // Width with validation loop
    let width = loop {
        print!("Fixed width [10-200, or press Enter for auto]: ");
        io::stdout().flush().unwrap();
        let mut width_input = String::new();
        io::stdin().read_line(&mut width_input).unwrap();
        let width = width_input.trim();
        
        if width.is_empty() {
            break None;
        }
        
        match validate_width(width) {
            Ok(()) => break Some(width.parse().unwrap()),
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    };
    
    BoxyTheme {
        color: color.to_string(),
        text_color: text_color.to_string(),
        style: style.to_string(),
        icon,
        width,
        ..Default::default()
    }
}

/// Interactive theme editing utility
fn edit_theme_interactively(name: &str, existing: &BoxyTheme) -> BoxyTheme {
    println!("Edit theme '{}' (press Enter to keep current value):", name);
    println!();
    
    // Color
    print!("Box color [current: {}]: ", existing.color);
    io::stdout().flush().unwrap();
    let mut color_input = String::new();
    io::stdin().read_line(&mut color_input).unwrap();
    let color = color_input.trim();
    let color = if color.is_empty() { &existing.color } else { color };
    
    // Text color
    print!("Text color [current: {}]: ", existing.text_color);
    io::stdout().flush().unwrap();
    let mut text_color_input = String::new();
    io::stdin().read_line(&mut text_color_input).unwrap();
    let text_color = text_color_input.trim();
    let text_color = if text_color.is_empty() { &existing.text_color } else { text_color };
    
    // Style
    print!("Border style [current: {}]: ", existing.style);
    io::stdout().flush().unwrap();
    let mut style_input = String::new();
    io::stdin().read_line(&mut style_input).unwrap();
    let style = style_input.trim();
    let style = if style.is_empty() { &existing.style } else { style };
    
    // Icon
    let current_icon = existing.icon.as_deref().unwrap_or("none");
    print!("Icon/emoji [current: {}]: ", current_icon);
    io::stdout().flush().unwrap();
    let mut icon_input = String::new();
    io::stdin().read_line(&mut icon_input).unwrap();
    let icon = icon_input.trim();
    let icon = if icon.is_empty() { 
        existing.icon.clone() 
    } else if icon == "none" { 
        None 
    } else { 
        Some(icon.to_string()) 
    };
    
    // Width
    let current_width = existing.width.map(|w| w.to_string()).unwrap_or_else(|| "auto".to_string());
    print!("Fixed width [current: {}]: ", current_width);
    io::stdout().flush().unwrap();
    let mut width_input = String::new();
    io::stdin().read_line(&mut width_input).unwrap();
    let width = width_input.trim();
    let width = if width.is_empty() { 
        existing.width 
    } else if width == "auto" { 
        None 
    } else { 
        match width.parse::<usize>() {
            Ok(w) if w >= 10 => Some(w),
            _ => {
                println!("Warning: Invalid width, keeping current");
                existing.width
            }
        }
    };
    
    BoxyTheme {
        color: color.to_string(),
        text_color: text_color.to_string(),
        style: style.to_string(),
        icon,
        width,
        ..existing.clone()
    }
}

/// Save theme to YAML file
fn save_theme_to_file(path: &PathBuf, name: &str, theme: &BoxyTheme) -> Result<(), String> {
    let theme_file = ThemeFile {
        metadata: ThemeMetadata {
            name: format!("{} Theme File", name),
            version: "1.0.0".to_string(),
            description: format!("Custom theme: {}", name),
            author: env::var("USER").unwrap_or_else(|_| "boxy".to_string()),
            created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            updated: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            compatibility: "boxy v0.6+".to_string(),
        },
        colors: std::collections::HashMap::new(),
        themes: {
            let mut themes = std::collections::HashMap::new();
            themes.insert(name.to_string(), theme.clone());
            themes
        },
        presets: std::collections::HashMap::new(),
        text_styles: std::collections::HashMap::new(),
        settings: ThemeSettings::default(),
    };
    
    let yaml_content = serde_yaml::to_string(&theme_file)
        .map_err(|e| format!("Failed to serialize theme: {}", e))?;
    
    fs::write(path, yaml_content)
        .map_err(|e| format!("Failed to write theme file: {}", e))?;
    
    Ok(())
}

/// Export theme to YAML format
fn export_theme_to_yaml(name: &str, theme: &BoxyTheme) -> String {
    let theme_file = ThemeFile {
        metadata: ThemeMetadata {
            name: format!("{} Theme Export", name),
            version: "1.0.0".to_string(),
            description: format!("Exported theme: {}", name),
            author: "boxy".to_string(),
            created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            updated: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            compatibility: "boxy v0.6+".to_string(),
        },
        colors: std::collections::HashMap::new(),
        themes: {
            let mut themes = std::collections::HashMap::new();
            themes.insert(name.to_string(), theme.clone());
            themes
        },
        presets: std::collections::HashMap::new(),
        text_styles: std::collections::HashMap::new(),
        settings: ThemeSettings::default(),
    };
    
    serde_yaml::to_string(&theme_file)
        .unwrap_or_else(|e| format!("# Error serializing theme: {}", e))
}

/// Handle `boxy theme create <name>` command
fn handle_theme_create(name: &str, jynx: &JynxPlugin) {
    // Validate theme name first
    if let Err(e) = validate_theme_name(name) {
        eprintln!("Error: Invalid theme name: {}", e);
        std::process::exit(1);
    }
    
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            // Check if theme already exists
            if theme_engine.get_theme(name).is_some() {
                eprintln!("Error: Theme '{}' already exists", name);
                eprintln!("Use 'boxy theme edit {}' to modify it", name);
                std::process::exit(1);
            }
            
            println!("{} {} - Create New Theme: {}", NAME, VERSION, name);
            println!();
            println!("Creating theme interactively. Press Ctrl+C to cancel.");
            println!();
            
            // Interactive theme creation
            let theme = create_theme_interactively(name);
            
            // Save theme to XDG+ directory
            let themes_dir = theme_engine.get_themes_directory();
            if let Err(e) = fs::create_dir_all(&themes_dir) {
                eprintln!("Error: Failed to create themes directory: {}", e);
                std::process::exit(1);
            }
            
            let theme_file_path = themes_dir.join(format!("{}.yml", name));
            if let Err(e) = save_theme_to_file(&theme_file_path, name, &theme) {
                eprintln!("Error: Failed to save theme: {}", e);
                std::process::exit(1);
            }
            
            println!();
            let success_msg = format!("✅ Theme '{}' created successfully!\n   Saved to: {}\n\nTest your theme:\n   echo \"Hello World\" | boxy --theme {}", name, theme_file_path.display(), name);
            
            if jynx.is_active() {
                jynx_println(&success_msg, "success", jynx);
            } else {
                println!("{}", success_msg);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy theme import <path>` command
fn handle_theme_import(path: &str) {
    let import_path = PathBuf::from(path);
    if !import_path.exists() {
        eprintln!("Error: File '{}' does not exist", path);
        std::process::exit(1);
    }
    
    // Validate file extension
    if !import_path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
        eprintln!("Error: Only YAML files (.yml, .yaml) are supported for import");
        std::process::exit(1);
    }
    
    // Pre-validate the theme file before importing
    println!("Validating theme file...");
    if let Err(e) = validate_theme_file(&import_path) {
        eprintln!("Error: Theme file validation failed: {}", e);
        eprintln!("The file contains invalid theme configurations and cannot be imported.");
        std::process::exit(1);
    }
    println!("✅ Theme file validation passed");
    
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            let themes_dir = theme_engine.get_themes_directory();
            if let Err(e) = fs::create_dir_all(&themes_dir) {
                eprintln!("Error: Failed to create themes directory: {}", e);
                std::process::exit(1);
            }
            
            let filename = import_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("imported_theme.yml");
                
            let target_path = themes_dir.join(filename);
            
            if target_path.exists() {
                print!("Theme file '{}' already exists. Overwrite? (y/N): ", filename);
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Import cancelled.");
                    return;
                }
            }
            
            if let Err(e) = fs::copy(&import_path, &target_path) {
                eprintln!("Error: Failed to import theme: {}", e);
                std::process::exit(1);
            }
            
            println!("✅ Theme imported successfully!");
            println!("   From: {}", import_path.display());
            println!("   To: {}", target_path.display());
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy theme export <name>` command
fn handle_theme_export(name: &str) {
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(theme) = theme_engine.get_theme(name) {
                let yaml_content = export_theme_to_yaml(name, &theme);
                print!("{}", yaml_content);
            } else {
                eprintln!("Error: Theme '{}' not found", name);
                let theme_list = theme_engine.list_themes();
                let theme_names: Vec<String> = theme_list.iter().map(|(n, _)| n.clone()).collect();
                eprintln!("Available themes: {}", theme_names.join(", "));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy theme edit <name>` command
fn handle_theme_edit(name: &str) {
    // Validate theme name first
    if let Err(e) = validate_theme_name(name) {
        eprintln!("Error: Invalid theme name: {}", e);
        std::process::exit(1);
    }
    
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(existing_theme) = theme_engine.get_theme(name) {
                println!("{} {} - Edit Theme: {}", NAME, VERSION, name);
                println!();
                
                // Interactive theme editing
                let updated_theme = edit_theme_interactively(name, &existing_theme);
                
                // Save updated theme
                let themes_dir = theme_engine.get_themes_directory();
                if let Err(e) = fs::create_dir_all(&themes_dir) {
                    eprintln!("Error: Failed to create themes directory: {}", e);
                    std::process::exit(1);
                }
                
                let theme_file_path = themes_dir.join(format!("{}.yml", name));
                if let Err(e) = save_theme_to_file(&theme_file_path, name, &updated_theme) {
                    eprintln!("Error: Failed to save theme: {}", e);
                    std::process::exit(1);
                }
                
                println!();
                println!("✅ Theme '{}' updated successfully!", name);
                println!("   Saved to: {}", theme_file_path.display());
            } else {
                eprintln!("Error: Theme '{}' not found", name);
                let theme_list = theme_engine.list_themes();
                let theme_names: Vec<String> = theme_list.iter().map(|(n, _)| n.clone()).collect();
                eprintln!("Available themes: {}", theme_names.join(", "));
                eprintln!("Use 'boxy theme create {}' to create a new theme", name);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut style = &NORMAL;
    let mut color = "none";
    let mut text_color = "none";
    let mut title: Option<String> = None;
    let mut footer: Option<String> = None;
    let mut header: Option<String> = None;
    let mut icon: Option<String> = None;
    let mut no_boxy = false;
    let mut strict_mode = false;
    let mut fixed_width: Option<usize> = None;
    let mut theme_name: Option<String> = None;
    let mut status_bar: Option<String> = None;
    let mut title_color: Option<String> = None;
    let mut status_color: Option<String> = None;
    let mut header_color: Option<String> = None;
    let mut footer_color: Option<String> = None;
    let mut header_align: &str = "center";
    let mut footer_align: &str = "center";
    let mut status_align_override: Option<String> = None;
    let mut body_align: &str = "left"; //todo: missing implementation?
    let mut body_pad_emoji = false; //todo: missing implementation?
    let mut pad_body_above = false;
    let mut pad_body_below = false;
    let mut divider_after_title = false;
    let mut divider_before_status = false;
    let mut pad_after_title_divider = false;
    let mut pad_before_status_divider = false;
    let mut pad_before_title = false;
    let mut pad_after_status = false;
    let mut pad_after_title = false;
    let mut pad_before_status = false;
    let mut skip_next = false;
    let mut params_flag: Option<String> = None;
    // Deprecated suggestions removed in v0.6.x -> simplified migration help view only
    let mut no_color_requested = false;
    

    // PRIORITY 1: Handle subcommands first - these take absolute precedence over stdin
    // Subcommands should always execute regardless of piped input
    if args.len() >= 2 && args[1] == "width" {
        handle_width_command();
        return;
    }
    
    if args.len() >= 2 && args[1] == "theme" {
        // Initialize jynx for theme commands
        let no_color = args.contains(&"--no-color".to_string()) || args.contains(&"--no-colour".to_string());
        let theme_jynx = JynxPlugin::new(no_color);
        handle_theme_command(&args[2..], &theme_jynx);
        return;
    }
    
    // Handle migrate-commands subcommand
    // if args.len() >= 2 && args[1] == "migrate-commands" {
    //     // Initialize jynx for migration commands
    //     let no_color = args.contains(&"--no-color".to_string()) || args.contains(&"--no-colour".to_string());
    //     let migrate_jynx = JynxPlugin::new(no_color);
    //     handle_migrate_command(&args[2..], &migrate_jynx);
    //     return;
    // }    
    // PRIORITY 2: Check for other subcommands that should prevent stdin reading
    // This explicit check ensures no ambiguity about input precedence
    let has_subcommand = args.len() >= 2 && matches!(args[1].as_str(), "width" | "theme" );
    if has_subcommand {
        // This should never be reached due to early returns above, but serves as a safety net
        return;
    }
    

    // Pre-scan for --no-color to initialize jynx properly
    for arg in args.iter().skip(1) {
        if arg == "--no-color" || arg == "--no-colour" {
            no_color_requested = true;
            break;
        }
    }
    
    // Initialize jynx integration early
    let jynx = JynxPlugin::new(no_color_requested);
    
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match arg.as_str() {
            "--help" | "-h" => {
                show_comprehensive_help(&jynx);
                return;
            }
            "--version" | "-v" | "-V" => {
                println!("{} {} ({})", NAME, VERSION, jynx.get_version_string());
                return;
            }
            "--colors" => {
                println!("{} {} - Color Palette Preview", NAME, VERSION);
                println!("{}", generate_color_help());
                return;
            }
            "--examples" => {
                show_usage_examples();
                return;
            }
            "--style" | "-s" => {
                if i + 1 < args.len() {
                    style = match args[i + 1].as_str() {
                        "rounded" => &ROUNDED,
                        "double" => &DOUBLE,
                        "heavy" => &HEAVY,
                        "ascii" => &ASCII,
                        "normal" => &NORMAL,
                        _ => {
                            eprintln!("Unknown style: {}. Using normal.", args[i + 1]);
                            &NORMAL
                        }
                    };
                    skip_next = true;
                }
            }
            "--params" => {
                if i + 1 < args.len() {
                    params_flag = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--color" | "-c" => {
                if i + 1 < args.len() {
                    let requested_color = &args[i + 1];
                    // Validate color and provide helpful error messages
                    match validate_color(requested_color) {
                        Ok(_) => {
                            color = requested_color;
                            skip_next = true;
                        }
                        Err(error_msg) => {
                            eprintln!("Color Error: {}", error_msg);
                            eprintln!("Use '{} --colors' to see all available colors", NAME);
                            std::process::exit(1);
                        }
                    }
                }
            }
            "--text" => {
                if i + 1 < args.len() {
                    let requested_text_color = &args[i + 1];
                    // Validate text color (same validation as box color)
                    match validate_color(requested_text_color) {
                        Ok(_) => {
                            text_color = requested_text_color;
                            skip_next = true;
                        }
                        Err(error_msg) => {
                            eprintln!("Text Color Error: {}", error_msg);
                            eprintln!("Use '{} --colors' to see all available colors", NAME);
                            std::process::exit(1);
                        }
                    }
                }
            }
            "--width" | "-w" => {
                if i + 1 < args.len() {
                    let warg = &args[i + 1];
                    if warg.eq_ignore_ascii_case("max") {
                        fixed_width = Some(get_terminal_width());
                        skip_next = true;
                    } else if warg.eq_ignore_ascii_case("auto") {
                        fixed_width = None; // let auto-sizing decide
                        skip_next = true;
                    } else {
                        match warg.parse::<usize>() {
                            Ok(w) if w >= 4 => {
                                fixed_width = Some(w);
                                skip_next = true;
                            }
                            _ => {
                                eprintln!("Error: Width must be <number>=4, or 'max'/'auto'");
                                std::process::exit(1);
                            }
                        }
                    }
                }
            }
            "--theme" => {
                if i + 1 < args.len() {
                    theme_name = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }

            "--title" => {
                if i + 1 < args.len() {
                    title = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--header" => {
                if i + 1 < args.len() {
                    header = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--footer" => {
                if i + 1 < args.len() {
                    footer = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--icon" => {
                if i + 1 < args.len() {
                    icon = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--status" => {
                if i + 1 < args.len() {
                    let status_text = &args[i + 1];
                    status_bar = Some(status_text.clone());
                    
                    // Check for deprecation patterns
                    // if !status_text.starts_with("sl:") && !status_text.starts_with("sc:") && !status_text.starts_with("sr:") &&
                    //    !status_text.starts_with("hl:") && !status_text.starts_with("hc:") && !status_text.starts_with("hr:") &&
                    //    !status_text.starts_with("fl:") && !status_text.starts_with("fc:") && !status_text.starts_with("fr:") &&
                    //    get_display_width(status_text) > 50 {
                    //     deprecation_warnings.push(format!(
                    //         "Long status text without alignment prefix. Consider using sl:, sc:, or sr: prefixes for better control."
                    //     ));
                    //}
                    
                    skip_next = true;
                }
            }
            "--title-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { title_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--status-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { status_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--header-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { header_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--footer-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { footer_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--layout" => {
                if i + 1 < args.len() {
                    let spec = &args[i + 1];
                    for token in spec.split(',') {
                        match token.trim() {
                            "hl" => header_align = "left",
                            "hc" => header_align = "center",
                            "hr" => header_align = "right",
                            "fl" => footer_align = "left",
                            "fc" => footer_align = "center",
                            "fr" => footer_align = "right",
                            "sl" => status_align_override = Some("left".to_string()),
                            "sc" => status_align_override = Some("center".to_string()),
                            "sr" => status_align_override = Some("right".to_string()),
                            "bl" => body_align = "left",
                            "bc" => body_align = "center",
                            "br" => body_align = "right",
                            "bp" => body_pad_emoji = true,
                            "dt" => divider_after_title = true,
                            "ds" => divider_before_status = true,
                            "dtn" => { divider_after_title = true; pad_after_title_divider = true; },
                            "dsn" => { divider_before_status = true; pad_before_status_divider = true; },
                            "stn" => { pad_before_title = true; },
                            "ssn" => { pad_after_status = true; },
                            "ptn" => { pad_after_title = true; },
                            "psn" => { pad_before_status = true; },
                            _ => { /* ignore unknown tokens */ }
                        }
                    }
                    skip_next = true;
                }
            }
            "--pad" => {
                if i + 1 < args.len() {
                    for t in args[i+1].split(',') {
                        match t.trim() {
                            "a"|"above" => pad_body_above = true,
                            "b"|"below" => pad_body_below = true,
                            _ => {}
                        }
                    }
                    skip_next = true;
                }
            }
            arg if arg.starts_with("--no-boxy") => {
                no_boxy = true;
                if arg == "--no-boxy=strict" {
                    strict_mode = true;
                }
            }
            "--no-color" | "--no-colour" => {
                // Color handling already processed in pre-scan
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!("Try '{} --help' for more information", NAME);
                std::process::exit(1);
            }
        }
    }
    
    // Check for deprecated pattern combinations with enhanced suggestions
    if icon.is_some() && title.is_some() {
        let icon_str = icon.as_deref().unwrap_or("📦");
        let title_str = title.as_deref().unwrap_or("Title");
        // deprecation_warnings.push(format!(
        //     "Using --icon with --title may cause layout conflicts.\n       → Try: --title \"{} {}\"",
        //     icon_str, title_str
        // ));
    }
    
    // Auto-detect potential migration opportunities
    //let mut auto_suggestions = Vec::new();
    
    // Suggest header/title distinction for external-looking titles
    if title.is_some() && !icon.is_some() {
        let title_text = title.as_deref().unwrap_or("");
        // if title_text.len() > 20 || title_text.to_lowercase().contains("app") || title_text.to_lowercase().contains("system") {
        //     auto_suggestions.push(format!(
        //         "Consider using --header for application names: --header \"{}\"", title_text
        //     ));
        // }
    }
    
    // Display auto-suggestions
    // if !auto_suggestions.is_empty() {
    //     eprintln!("{}🤖 AUTO-SUGGESTION:{}", get_color_code("cyan"), RESET);
    //     for suggestion in &auto_suggestions {
    //         eprintln!("{}{}{}", get_color_code("cyan"), suggestion, RESET);
    //     }
    //     eprintln!();
    // }
    
    // TODO:remove
    // if !deprecation_warnings.is_empty() {
    //     for warning in &deprecation_warnings {
    //         eprintln!("{}⚠️  DEPRECATION WARNING:{} {}", get_color_code("orange"), RESET, warning);
    //     }
    //     eprintln!();
    //     eprintln!("{}💡 MIGRATION TIP:{} Use 'boxy migrate-commands --help' for migration assistance", get_color_code("azure"), RESET);
    //     eprintln!();
    // }
    
    // PRIORITY 3: Read from stdin only if no subcommands were processed
    // At this point, all subcommands and utility flags (--help, --version, etc.) have been handled
    // This ensures clear precedence: subcommands > utility flags > stdin processing
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let text = input.trim_end_matches('\n').to_string();

    // Params stream parsing: ONLY via --params flag. Piped stdin remains the body.
    if let Some(ref blob) = params_flag {
        if let Some(pc) = parse_content_stream(blob) {
            if header.is_none() { header = pc.header; }
            if footer.is_none() { footer = pc.footer; }
            if status_bar.is_none() { status_bar = pc.status; }
            if title.is_none() { title = pc.title; }
            if let Some(ic) = pc.icon { icon = Some(ic); }
            if title_color.is_none() { title_color = pc.title_color; }
            if status_color.is_none() { status_color = pc.status_color; } // ? why
            if header_color.is_none() { header_color = pc.header_color; }
            if footer_color.is_none() { footer_color = pc.footer_color; }
            // Map layout tokens if provided via params
            if let Some(spec) = pc.layout.as_deref() {
                for token in spec.split(',') {
                    match token.trim() {
                        "hl" => header_align = "left",
                        "hc" => header_align = "center",
                        "hr" => header_align = "right",
                        "fl" => footer_align = "left",
                        "fc" => footer_align = "center",
                        "fr" => footer_align = "right",
                        "sl" => status_align_override = Some("left".to_string()),
                        "sc" => status_align_override = Some("center".to_string()),
                        "sr" => status_align_override = Some("right".to_string()),
                        "bl" => body_align = "left",
                        "bc" => body_align = "center",
                        "br" => body_align = "right",
                        "bp" => body_pad_emoji = true,
                        "dt" => divider_after_title = true,
                        "ds" => divider_before_status = true,
                        "dtn" => { divider_after_title = true; pad_after_title_divider = true; },
                        "dsn" => { divider_before_status = true; pad_before_status_divider = true; },
                        "stn" => { pad_before_title = true; },
                        "ssn" => { pad_after_status = true; },
                        "ptn" => { pad_after_title = true; },
                        "psn" => { pad_before_status = true; },
                        _ => {}
                    }
                }
            }
            // Body remains the piped stdin text
        }
    }
    
    // Apply theme if specified - using new theme engine
    if let Some(theme_name_str) = &theme_name {
        match ThemeEngine::new() {
            Ok(theme_engine) => {
                if let Some(boxy_theme) = theme_engine.get_theme(theme_name_str) {
                    // Theme overrides defaults but explicit flags override theme
                    if color == "none" {
                        color = Box::leak(boxy_theme.color.clone().into_boxed_str());
                    }
                    // Prefer to use theme icon as icon decoration (first content line), not a separate line
                    if icon.is_none() {
                        if let Some(icon_str) = &boxy_theme.icon {
                            icon = Some(icon_str.clone());
                        } else if let Some(title_str) = &boxy_theme.title {
                            let emoji_part: String = title_str.chars().take_while(|c| !c.is_ascii()).collect();
                            if !emoji_part.trim().is_empty() { icon = Some(emoji_part.trim().to_string()); }
                        }
                    }
                    if fixed_width.is_none() {
                        fixed_width = boxy_theme.width;
                    }
                } else {
                    eprintln!("Unknown theme: {}. Available themes:", theme_name_str);
                    let theme_list = theme_engine.list_themes();
                    let theme_names: Vec<String> = theme_list.iter().map(|(name, _)| name.clone()).collect();
                    eprintln!("  {}", theme_names.join(", "));
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to load theme engine: {}", e);
                eprintln!("Continuing without theme...");
            }
        }
    }
    
    let status_color_str = status_color.as_deref().unwrap_or("");
    eprintln!("Status Color: {}", status_color_str);

    // ⚠️  CRITICAL: DO NOT CHANGE THIS ICON LOGIC! ⚠️
    //
    // 🚨 WARNING: The icon positioning was a NIGHTMARE to get right! 🚨
    //
    // HISTORY: Originally had complex icon positioning logic in draw_box() that
    // calculated spacing, handled truncation, managed padding, etc. It was buggy,
    // spacing was inconsistent, and adding text colors broke everything.
    //
    // SOLUTION: Use the SAME unified approach as themes:
    // 1. Prepend icon to content string early: "✅ Success!"
    // 2. Clear icon variable so draw_box() uses normal (non-icon) path
    // 3. Everything flows through consistent spacing calculations
    //
    // RESULT: Perfect spacing, consistent with themes, text colors work flawlessly
    //
    // 🔥 IF YOU TOUCH THIS, YOU WILL BREAK SPACING AND HATE YOURSELF 🔥
    // 🔥 MANUAL ICONS MUST USE SAME PATTERN AS THEMES - NO EXCEPTIONS! 🔥
    //
    // No longer prepend icon to the raw text; icon is injected on first line in draw_box
    // todo: check on this ? Apply manual icon using the same unified approach as themes
    // DO NOT REMOVE THIS
    // if let Some(manual_icon) = &icon {
    //     let icon_expanded = expand_variables(manual_icon);
    //     text = format!("{} {}", icon_expanded, text);
    //     // Clear icon so it doesn't get used in positioning system
    //     icon = None;
    // }


    if no_boxy {
        let stripped = strip_box(&text, strict_mode);
        println!("{}", stripped);
    } else {
        draw_box(
          &text, 
          1, 1, 
          style, color, text_color, 
          title.as_deref(), 
          footer.as_deref(), 
          icon.as_deref(), 
          fixed_width, 
          status_bar.as_deref(), 
          header.as_deref(), 
          header_align, footer_align, 
          status_align_override.as_deref(), 
          divider_after_title, divider_before_status, 
          pad_after_title_divider, pad_before_status_divider, 
          pad_before_title, pad_after_status, 
          pad_after_title, pad_before_status, 
          title_color.as_deref(), 
          status_color.as_deref(), 
          body_align,
          body_pad_emoji, 
          pad_body_above, 
          pad_body_below
        );
    }
    
  }

// KEEP THE COMMENTS BELOW THIS POINT, THEY ARE HERE FOR REPAIR REFERENCE

// fn draw_box(
//   text: &str, 
//   h_padding: usize, 
//   _v_padding: usize, 
//   style: &BoxStyle, 
//   color: &str, 
//   text_color: &str, 
//   title: Option<&str>, 
//   footer: Option<&str>, 
//   icon: Option<&str>, 
//   fixed_width: Option<usize>, 
//   status_bar: Option<&str>, 
//   header: Option<&str>, 
//   header_align: &str, 
//   footer_align: &str, 
//   status_align_override: Option<&str>, 
//   divider_after_title: bool, 
//   divider_before_status: bool, 
//   pad_after_title_divider: bool, 
//   pad_before_status_divider: bool, 
//   pad_before_title: bool, 
//   pad_after_status: bool, 
//   pad_after_title: bool, 
//   pad_before_status: bool, 
//   title_color_name: Option<&str>, 
//   status_color_name: Option<&str>, 
//   body_align: Option<&str>, 
//   body_pad_emoji: Option<&str>, 
//   pad_body_above: bool, 
//   pad_body_below: bool
// )


//save reference
    //   // Apply theme if specified - using new theme engine
    // if let Some(theme_name_str) = &theme_name {
    //     match ThemeEngine::new() {
    //         Ok(theme_engine) => {
    //             if let Some(boxy_theme) = theme_engine.get_theme(theme_name_str) {
    //                 // Theme overrides defaults but explicit flags override theme
    //                 if color == "none" {
    //                     color = Box::leak(boxy_theme.color.clone().into_boxed_str());
    //                 }
    //                 // When using themes, theme icons get their own line
    //                 let theme_emoji = if let Some(override_icon) = &icon {
    //                     // If explicit icon provided, use that instead of theme's emoji
    //                     override_icon.clone()
    //                 } else {
    //                     // Extract icon from either icon field or title field (YAML themes use title)
    //                     if let Some(icon_str) = &boxy_theme.icon {
    //                         icon_str.clone()
    //                     } else if let Some(title_str) = &boxy_theme.title {
    //                         // Extract just the emoji from title like "❌ Error" -> "❌"
    //                         let emoji_part: String = title_str.chars().take_while(|c| !c.is_ascii()).collect();
    //                         emoji_part.trim().to_string()
    //                     } else {
    //                         "📦".to_string()
    //                     }
    //                 };
    //                 text = format!("{}\n{}", theme_emoji, text);
    //                 // Clear icon so it doesn't get used in positioning system
    //                 icon = None;
    //                 if fixed_width.is_none() {
    //                     fixed_width = boxy_theme.width;
    //                 }
    //             } else {
    //                 eprintln!("Unknown theme: {}. Available themes:", theme_name_str);
    //                 let theme_list = theme_engine.list_themes();
    //                 let theme_names: Vec<String> = theme_list.iter().map(|(name, _)| name.clone()).collect();
    //                 eprintln!("  {}", theme_names.join(", "));
    //                 std::process::exit(1);
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Warning: Failed to load theme engine: {}", e);
    //             eprintln!("Continuing without theme...");
    //         }
    //     }
    // }
