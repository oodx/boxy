

use rsb::prelude::*;
use crate::{ BOX_CHARS };
use crate::{ get_display_width, get_terminal_width, get_color_code };
use crate::config::BoxyConfig;
use crate::RESET;
use std::env;
use crate::components::{Header, Footer, Status, Body};

// RSB-compliant helper functions for draw_box decomposition

/// Calculate optimal box width based on content and terminal constraints
fn calculate_box_width(text: &str, h_padding: usize, fixed_width: Option<usize>) -> usize {
    let terminal_width = get_terminal_width();

    let box_width = match fixed_width {
        Some(w) => {
            if w > terminal_width {
                // Fixed width exceeds terminal, constrain it
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
                // Content too wide for terminal
                terminal_width
            } else {
                ideal_width
            }
        }
    };
    
    // Get minimum width from environment variable (RSB-compliant)
    let min_width_str = param!("BOXY_MIN_WIDTH", default: "5");
    let min_width: usize = min_width_str.parse().unwrap_or(5);
    
    // Ensure minimum viable box size (user configurable via BOXY_MIN_WIDTH)
    if box_width < min_width { min_width } else { box_width }
}

// Helper functions have been replaced by component architecture


pub fn draw_box(config: BoxyConfig) {
    // Use RSB-compliant helper for width calculation
    let final_width = calculate_box_width(&config.text, config.width.h_padding, config.width.fixed_width);
    let inner_width = final_width.saturating_sub(2); // Account for borders
    let color_code = get_color_code(&config.colors.box_color);
    
    // Determine text color: "auto" means match box color, "none" means default
    let text_color_code = match config.colors.text_color.as_str() {
        "auto" => get_color_code(&config.colors.box_color), // Use same color as box
        "none" => "",                    // Default terminal color
        _ => get_color_code(&config.colors.text_color), // Explicit color
    };
    
    // pad removed - now handled by components
    let title_color_code = config.colors.title_color.as_ref().map(|n| get_color_code(n)).unwrap_or("");
    let status_color_code = config.colors.status_color.as_ref().map(|n| get_color_code(n)).unwrap_or("");
    
    // Use Header component for top border rendering
    let header = Header::new(&config);
    println!("{}", header.render(inner_width, &color_code));
    
    // Use Body component for content rendering with preserved emoji/width calculations
    let body = Body::new(&config);
    let body_lines = body.render(inner_width, &color_code, &text_color_code, &title_color_code);

    // Prepare status lines early for height calculation
    let status = Status::new(&config);
    let status_lines: Vec<String> = if status.should_render() {
        status.render(inner_width, &color_code, &text_color_code, &status_color_code)
    } else {
        Vec::new()
    };

    // Multiplex mode: pad interior to fixed height if requested
    let mut multiplex_mode = rsb::prelude::param!("BOXY_MULTIPLEX_MODE", default: "");
    if multiplex_mode.is_empty() {
        multiplex_mode = env::var("BOXY_MULTIPLEX_MODE").unwrap_or_default();
    }
    let multiplex_on = !multiplex_mode.is_empty()
        && multiplex_mode != "0"
        && multiplex_mode.to_lowercase() != "false";
    let mut padded_body_lines = body_lines.clone();
    if multiplex_on {
        if let Some(target_height) = config.fixed_height {
            // Total lines are: header(1) + body + status + footer(1)
            let current_total = 1 + padded_body_lines.len() + status_lines.len() + 1;
            if target_height > current_total {
                let filler_needed = target_height - current_total;
                // Build a blank interior line template
                let available_content_width = inner_width.saturating_sub(2 * config.width.h_padding);
                let pad = " ".repeat(config.width.h_padding);
                let blank_line = format!(
                    "{}{}{}{}{}{}{}",
                    &color_code,
                    config.style.vertical,
                    RESET,
                    &pad,
                    " ".repeat(available_content_width),
                    &pad,
                    format!("{}{}{}", &color_code, config.style.vertical, RESET)
                );
                for _ in 0..filler_needed {
                    padded_body_lines.push(blank_line.clone());
                }
            }
        }
    }

    // Print body (padded if needed)
    for line in padded_body_lines {
        println!("{}", line);
    }

    // Print status lines after padding so they stay at bottom
    for line in status_lines {
        println!("{}", line);
    }

    // Use Footer component for bottom border rendering
    let footer = Footer::new(&config);
    println!("{}", footer.render(inner_width, &color_code));

    // Note: header_color and footer_color are stored in config but not yet implemented
    // body_align, body_pad_emoji, pad_body_above, pad_body_below are stored in config but not yet implemented
}

pub fn render_box_to_string(config: BoxyConfig) -> String {
    let mut output = String::new();

    // Use RSB-compliant helper for width calculation
    let final_width = calculate_box_width(&config.text, config.width.h_padding, config.width.fixed_width);
    let inner_width = final_width.saturating_sub(2); // Account for borders
    let color_code = get_color_code(&config.colors.box_color);

    // Determine text color: "auto" means match box color, "none" means default
    let text_color_code = match config.colors.text_color.as_str() {
        "auto" => get_color_code(&config.colors.box_color), // Use same color as box
        "none" => "",                    // Default terminal color
        _ => get_color_code(&config.colors.text_color), // Explicit color
    };

    let title_color_code = config.colors.title_color.as_ref().map(|n| get_color_code(n)).unwrap_or("");
    let status_color_code = config.colors.status_color.as_ref().map(|n| get_color_code(n)).unwrap_or("");

    // Use Header component for top border rendering
    let header = Header::new(&config);
    output.push_str(&header.render(inner_width, &color_code));
    output.push('\n');

    // Use Body component for content rendering with preserved emoji/width calculations
    let body = Body::new(&config);
    let mut body_lines = body.render(inner_width, &color_code, &text_color_code, &title_color_code);

    // Prepare status lines early for height calculation
    let status = Status::new(&config);
    let status_lines: Vec<String> = if status.should_render() {
        status.render(inner_width, &color_code, &text_color_code, &status_color_code)
    } else {
        Vec::new()
    };

    // Multiplex mode: pad interior to fixed height if requested
    let mut multiplex_mode = rsb::prelude::param!("BOXY_MULTIPLEX_MODE", default: "");
    if multiplex_mode.is_empty() {
        multiplex_mode = env::var("BOXY_MULTIPLEX_MODE").unwrap_or_default();
    }
    let multiplex_on = !multiplex_mode.is_empty()
        && multiplex_mode != "0"
        && multiplex_mode.to_lowercase() != "false";
    if multiplex_on {
        if let Some(target_height) = config.fixed_height {
            // Total lines are: header(1) + body + status + footer(1)
            let current_total = 1 + body_lines.len() + status_lines.len() + 1;
            if target_height > current_total {
                let filler_needed = target_height - current_total;
                // Build a blank interior line template
                let available_content_width = inner_width.saturating_sub(2 * config.width.h_padding);
                let pad = " ".repeat(config.width.h_padding);
                let blank_line = format!(
                    "{}{}{}{}{}{}{}",
                    &color_code,
                    config.style.vertical,
                    RESET,
                    &pad,
                    " ".repeat(available_content_width),
                    &pad,
                    format!("{}{}{}", &color_code, config.style.vertical, RESET)
                );
                for _ in 0..filler_needed {
                    body_lines.push(blank_line.clone());
                }
            }
        }
    }

    // Append (possibly padded) body
    for line in &body_lines {
        output.push_str(line);
        output.push('\n');
    }

    // Append status lines after padding so they stay at bottom
    for line in &status_lines {
        output.push_str(line);
        output.push('\n');
    }

    // Use Footer component for bottom border rendering
    let footer = Footer::new(&config);
    output.push_str(&footer.render(inner_width, &color_code));

    output
}

pub fn strip_box(text: &str, strict: bool) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut content_lines = Vec::new();
    
    // Box drawing characters to detect
    let box_chars = BOX_CHARS;
    
    for (i, line) in lines.iter().enumerate() {
        // Strip ANSI codes from line for processing
        let clean_line = strip_ansi_escapes::strip(line);
        let clean_str = String::from_utf8_lossy(&clean_line);
        let trimmed_clean = clean_str.trim();
        
        // Skip lines that look like box borders (first and last)
        if i == 0 || i == lines.len() - 1 {
            if trimmed_clean.chars().all(|c| box_chars.contains(c) || c.is_whitespace()) {
                continue;
            }
        }
        
        // Process content lines
        let mut content = if strict {
            // For strict mode, use clean version (ANSI already stripped)
            clean_str.to_string()
        } else {
            // For normal mode, preserve original but we'll strip box chars carefully
            line.to_string()
        };
        
        // Remove box sides - handle both strict and normal modes
        if strict {
            // For strict mode, work with clean characters
            if content.len() > 2 {
                let chars: Vec<char> = content.chars().collect();
                
                // Remove leading box character
                if box_chars.contains(chars[0]) {
                    content = chars.iter().skip(1).collect();
                }
                
                // Remove trailing box character
                let chars: Vec<char> = content.chars().collect();
                if !chars.is_empty() && box_chars.contains(chars[chars.len() - 1]) {
                    content = chars.iter().take(chars.len() - 1).collect();
                }
            }
        } else {
            // For normal mode, use the clean version to guide box character removal
            // but preserve ANSI codes in the final result
            let clean_chars: Vec<char> = clean_str.chars().collect();
            
            if clean_chars.len() > 2 {
                // Check if clean version starts/ends with box characters
                let starts_with_box = box_chars.contains(clean_chars[0]);
                let ends_with_box = !clean_chars.is_empty() && box_chars.contains(clean_chars[clean_chars.len() - 1]);
                
                if starts_with_box || ends_with_box {
                    // Use the clean version as the template, but extract from original preserving ANSI
                    let mut clean_content = clean_str.to_string();
                    
                    // Remove box characters from clean version
                    if starts_with_box && clean_content.len() > 0 {
                        let chars: Vec<char> = clean_content.chars().collect();
                        if box_chars.contains(chars[0]) {
                            clean_content = chars.iter().skip(1).collect();
                        }
                    }
                    
                    if ends_with_box && clean_content.len() > 0 {
                        let chars: Vec<char> = clean_content.chars().collect();
                        if !chars.is_empty() && box_chars.contains(chars[chars.len() - 1]) {
                            clean_content = chars.iter().take(chars.len() - 1).collect();
                        }
                    }
                    
                    // Now extract the equivalent content from the original, preserving ANSI codes
                    // For simplicity in this surgical fix, use the clean content if ANSI handling is complex
                    content = clean_content;
                }
            }
        }
        
        // Trim padding
        content = content.trim().to_string();
        
        if strict {
            // Remove emojis and special Unicode (keep basic ASCII)
            content = content.chars()
                .filter(|c| c.is_ascii())
                .collect();
        } else {
            // For normal mode, clean up leading/trailing ANSI codes that might be left over
            // after box character removal
            use regex::Regex;
            let ansi_regex = Regex::new(r"^\x1b\[[0-9;]*m").unwrap();
            content = ansi_regex.replace(&content, "").to_string();
            let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m$").unwrap();
            content = ansi_regex.replace(&content, "").to_string();
        }
        
        if !content.is_empty() || !strict {
            content_lines.push(content);
        }
    }
    
    content_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_box_content_preservation() {
        // Test the exact scenario from ceremony_14 step 6
        let boxed_content = "┌───────────────────────────┐\n│ Test Content for Pipeline │\n└───────────────────────────┘";
        let stripped = strip_box(boxed_content, false);
        assert_eq!(stripped, "Test Content for Pipeline");
    }

    #[test]
    fn test_strip_box_with_ansi() {
        // Test with ANSI codes like in the actual output
        let boxed_content = "┌───────────────────────────┐\u{1b}[0m\n│\u{1b}[0m Test Content for Pipeline │\u{1b}[0m\n└───────────────────────────┘\u{1b}[0m";
        let stripped = strip_box(boxed_content, false);
        assert_eq!(stripped, "Test Content for Pipeline");
    }
}
