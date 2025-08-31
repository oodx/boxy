use unicode_width::UnicodeWidthStr;
use std::io::{self, Read};
use std::env;
use regex::Regex;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

struct BoxStyle {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
}

const NORMAL: BoxStyle = BoxStyle {
    top_left: "┌", top_right: "┐",
    bottom_left: "└", bottom_right: "┘",
    horizontal: "─", vertical: "│",
};

const ROUNDED: BoxStyle = BoxStyle {
    top_left: "╭", top_right: "╮",
    bottom_left: "╰", bottom_right: "╯",
    horizontal: "─", vertical: "│",
};

const DOUBLE: BoxStyle = BoxStyle {
    top_left: "╔", top_right: "╗",
    bottom_left: "╚", bottom_right: "╝",
    horizontal: "═", vertical: "║",
};

const HEAVY: BoxStyle = BoxStyle {
    top_left: "┏", top_right: "┓",
    bottom_left: "┗", bottom_right: "┛",
    horizontal: "━", vertical: "┃",
};

const ASCII: BoxStyle = BoxStyle {
    top_left: "+", top_right: "+",
    bottom_left: "+", bottom_right: "+",
    horizontal: "-", vertical: "|",
};

fn get_color_code(color: &str) -> &'static str {
    match color {
        "red" => "\x1B[38;5;9m",
        "red2" => "\x1B[38;5;197m",
        "deep" => "\x1B[38;5;61m",
        "deep_green" => "\x1B[38;5;60m",
        "orange" => "\x1B[38;5;214m",
        "yellow" => "\x1B[33m",
        "green" => "\x1B[38;5;10m",
        "green2" => "\x1B[32m",
        "blue" => "\x1B[36m",
        "blue2" => "\x1B[38;5;39m",
        "cyan" => "\x1B[38;5;14m",
        "magenta" => "\x1B[35m",
        "purple" => "\x1B[38;5;213m",
        "purple2" => "\x1B[38;5;141m",
        "white" => "\x1B[38;5;247m",
        "white2" => "\x1B[38;5;15m",
        "grey" => "\x1B[38;5;242m",
        "grey2" => "\x1B[38;5;240m",
        "grey3" => "\x1B[38;5;237m",
        _ => "",
    }
}

const RESET: &str = "\x1B[0m";

fn get_display_width(text: &str) -> usize {
    let clean = strip_ansi_escapes::strip(text);
    let clean_str = String::from_utf8_lossy(&clean);
    UnicodeWidthStr::width(clean_str.as_ref())
}

fn render_title_or_footer(text: &str, total_width: usize, style_char: &str) -> String {
    let text_width = get_display_width(text);
    let available_width = total_width.saturating_sub(2); // Space for " text "
    
    let final_text = if text_width > available_width {
        // Truncate with ellipsis
        let mut truncated = String::new();
        let mut current_width = 0;
        for ch in text.chars() {
            let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());
            if current_width + ch_width + 3 > available_width {
                truncated.push_str("...");
                break;
            }
            truncated.push(ch);
            current_width += ch_width;
        }
        truncated
    } else {
        text.to_string()
    };
    
    let final_text_width = get_display_width(&final_text);
    let remaining_width = total_width - final_text_width - 2; // -2 for spaces around text
    let left_pad = remaining_width / 2;
    let right_pad = remaining_width - left_pad;
    
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

fn draw_box(text: &str, padding: usize, style: &BoxStyle, color: &str, title: Option<&str>, footer: Option<&str>, icon: Option<&str>) {
    let lines: Vec<&str> = text.lines().collect();
    
    let max_width = lines.iter()
        .map(|line| get_display_width(line))
        .max()
        .unwrap_or(0);
    
    let inner_width = max_width + 2 * padding;
    let color_code = get_color_code(color);
    let pad = " ".repeat(padding);
    
    // Top border with optional title
    if let Some(title_text) = title {
        let expanded_title = expand_variables(title_text);
        let title_line = render_title_or_footer(&expanded_title, inner_width, style.horizontal);
        println!("{}{}{}{}{}", 
            color_code, style.top_left, title_line, style.top_right, RESET);
    } else {
        let border = style.horizontal.repeat(inner_width);
        println!("{}{}{}{}{}", 
            color_code, style.top_left, border, style.top_right, RESET);
    }
    
    // Content lines with optional icon on first line
    for (i, line) in lines.iter().enumerate() {
        let width = get_display_width(line);
        let spaces = " ".repeat(max_width - width);
        
        if i == 0 && icon.is_some() {
            // First line with icon
            let icon_str = icon.unwrap();
            let icon_expanded = expand_variables(icon_str);
            println!("{}{} {}{}{}{}{}{}{}",
                color_code, style.vertical, RESET,
                icon_expanded, " ",
                line, spaces, pad,
                format!("{}{}{}", color_code, style.vertical, RESET));
        } else {
            println!("{}{}{}{}{}{}{}{}",
                color_code, style.vertical, RESET,
                pad, line, spaces, pad,
                format!("{}{}{}", color_code, style.vertical, RESET));
        }
    }
    
    // Bottom border with optional footer
    if let Some(footer_text) = footer {
        let expanded_footer = expand_variables(footer_text);
        let footer_line = render_title_or_footer(&expanded_footer, inner_width, style.horizontal);
        println!("{}{}{}{}{}", 
            color_code, style.bottom_left, footer_line, style.bottom_right, RESET);
    } else {
        let border = style.horizontal.repeat(inner_width);
        println!("{}{}{}{}{}", 
            color_code, style.bottom_left, border, style.bottom_right, RESET);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut style = &NORMAL;
    let mut color = "none";
    let mut title: Option<String> = None;
    let mut footer: Option<String> = None;
    let mut icon: Option<String> = None;
    let mut no_boxy = false;
    let mut strict_mode = false;
    let mut skip_next = false;
    
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{} {} - {}", NAME, VERSION, DESCRIPTION);
                println!("\nUSAGE:");
                println!("    echo \"text\" | {} [OPTIONS]", NAME);
                println!("    command | {} --no-boxy          Strip box decoration", NAME);
                println!("\nOPTIONS:");
                println!("    -s, --style <STYLE>      Box style [normal, rounded, double, heavy, ascii]");
                println!("    -c, --color <COLOR>      Box color [red, red2, green, green2, blue, blue2,");
                println!("                             cyan, orange, yellow, purple, purple2, magenta,");
                println!("                             deep, deep_green, white, white2, grey, grey2, grey3]");
                println!("    --title <TEXT>           Add title to top border (supports $VAR expansion)");
                println!("    --footer <TEXT>          Add footer to bottom border");
                println!("    --icon <ICON>            Add icon/emoji to first content line");
                println!("    --no-boxy[=strict]       Strip box decoration (strict: remove all formatting)");
                println!("    -h, --help               Show this help message");
                println!("    -V, --version            Show version");
                println!("\nEXAMPLES:");
                println!("    echo \"Hello\" | {}", NAME);
                println!("    echo \"Hello\" | {} --style rounded --color blue", NAME);
                println!("    echo \"Hello\" | {} --title \"🚀 My App\" --footer \"v1.0\"", NAME);
                println!("    echo \"Test\" | {} | {} --no-boxy", NAME, NAME);
                return;
            }
            "--version" | "-V" => {
                println!("{} {}", NAME, VERSION);
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
            "--color" | "-c" => {
                if i + 1 < args.len() {
                    color = &args[i + 1];
                    skip_next = true;
                }
            }
            "--title" => {
                if i + 1 < args.len() {
                    title = Some(args[i + 1].clone());
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
            arg if arg.starts_with("--no-boxy") => {
                no_boxy = true;
                if arg == "--no-boxy=strict" {
                    strict_mode = true;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!("Try '{} --help' for more information", NAME);
                std::process::exit(1);
            }
        }
    }
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let text = input.trim_end_matches('\n');
    
    if no_boxy {
        let stripped = strip_box(&text, strict_mode);
        println!("{}", stripped);
    } else {
        draw_box(&text, 1, style, color, title.as_deref(), footer.as_deref(), icon.as_deref());
    }
}
