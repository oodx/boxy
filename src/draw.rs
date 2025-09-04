

use crate::{ BoxStyle, BOX_CHARS, RESET };
use crate::{ get_display_width,  expand_variables, render_title_or_footer,
             get_terminal_width, get_color_code, truncate_with_ellipsis };


pub fn draw_box(text: &str, h_padding: usize, _v_padding: usize, style: &BoxStyle, color: &str, text_color: &str, title: Option<&str>, footer: Option<&str>, icon: Option<&str>, fixed_width: Option<usize>, status_bar: Option<&str>, header: Option<&str>, header_align: &str, footer_align: &str, status_align_override: Option<&str>, divider_after_title: bool, divider_before_status: bool, pad_after_title_divider: bool, pad_before_status_divider: bool, pad_before_title: bool, pad_after_status: bool, pad_after_title: bool, pad_before_status: bool, title_color_name: Option<&str>, status_color_name: Option<&str>, body_align: &str, body_pad_emoji: bool, pad_body_above: bool, pad_body_below: bool, header_color:Option<&str>, footer_color:Option<&str>){
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

    let _ = header_color; //missig impl
    let _ = footer_color; //missig impl
    let _ = pad_body_above;
    let _ = pad_body_below;
    let _ = body_align;
    let _ = body_pad_emoji;
}

pub fn strip_box(text: &str, strict: bool) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut content_lines = Vec::new();
    
    // Box drawing characters to detect
    let box_chars = BOX_CHARS;
    
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
