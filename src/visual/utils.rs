//! Visual utilities - RSB MODULE_SPEC compliant visual system
//!
//! This module provides comprehensive visual rendering functionality for boxy including:
//! - Box style definitions and validation
//! - Drawing and rendering functions
//! - Component-based rendering architecture
//! - Width calculation utilities with PROTECTED macros
//!
//! Consolidated from boxes.rs, draw.rs, and components.rs following RSB MODULE_SPEC.
//!
//! Version: boxy v0.16.0+ (RSB MODULE_SPEC reorganization)

use crate::{expand_variables, render_title_or_footer, RESET, get_display_width, get_terminal_width, get_color_code, truncate_with_ellipsis};
use crate::core::BoxyConfig;

// ============================================================================
// BOX STYLE SYSTEM (from boxes.rs)
// ============================================================================

//WARN: this must contain all box characters!
pub const BOX_CHARS: &str = "┌┐└┘─│├┤┼╭╮╰╯═║╠╣╬╔╗╚╝━┃┣┫╋┏┓┗┛+-|";

/// Validate style input
pub fn validate_box_style(style: &str) -> Result<(), String> {
    let valid_styles = vec!["normal", "rounded", "double", "heavy", "ascii"];
    if !valid_styles.contains(&style) {
        return Err(format!("Invalid style '{}'. Valid styles: {}", style, valid_styles.join(", ")));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct BoxStyle {
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub horizontal: &'static str,
    pub vertical: &'static str,
    pub tee_left: &'static str,
    pub tee_right: &'static str,
    #[allow(dead_code)]
    pub cross: &'static str,
}

pub const NORMAL: BoxStyle = BoxStyle {
    top_left: "┌", top_right: "┐",
    bottom_left: "└", bottom_right: "┘",
    horizontal: "─", vertical: "│",
    tee_left: "├", tee_right: "┤", cross: "┼",
};

pub const ROUNDED: BoxStyle = BoxStyle {
    top_left: "╭", top_right: "╮",
    bottom_left: "╰", bottom_right: "╯",
    horizontal: "─", vertical: "│",
    tee_left: "├", tee_right: "┤", cross: "┼",
};

pub const DOUBLE: BoxStyle = BoxStyle {
    top_left: "╔", top_right: "╗",
    bottom_left: "╚", bottom_right: "╝",
    horizontal: "═", vertical: "║",
    tee_left: "╠", tee_right: "╣", cross: "╬",
};

pub const HEAVY: BoxStyle = BoxStyle {
    top_left: "┏", top_right: "┓",
    bottom_left: "┗", bottom_right: "┛",
    horizontal: "━", vertical: "┃",
    tee_left: "┣", tee_right: "┫", cross: "╋",
};

pub const ASCII: BoxStyle = BoxStyle {
    top_left: "+", top_right: "+",
    bottom_left: "+", bottom_right: "+",
    horizontal: "-", vertical: "|",
    tee_left: "+", tee_right: "+", cross: "+",
};

impl Default for BoxStyle {
    fn default() -> Self {
        NORMAL
    }
}

// ============================================================================
// DRAWING FUNCTIONS (from draw.rs)
// ============================================================================

/// PROTECTED: Original working width calculation logic - DO NOT MODIFY
/// This macro preserves the exact SIMPLE logic that was working before showcase changes
macro_rules! box_width {
    ($text:expr, $h_padding:expr, $fixed_width:expr) => {{
        let terminal_width = get_terminal_width();

        let box_width = match $fixed_width {
            Some(w) => {
                if w > terminal_width {
                    // Fixed width exceeds terminal, constrain it
                    terminal_width
                } else {
                    w
                }
            },
            None => {
                // Auto-size but constrain to terminal width - SIMPLE VERSION
                let lines: Vec<&str> = $text.lines().collect();
                let content_max_width = lines.iter()
                    .map(|line| get_display_width(line))
                    .max()
                    .unwrap_or(0);
                let ideal_width = content_max_width + 2 * $h_padding + 2; // +2 for borders

                if ideal_width > terminal_width {
                    // Content too wide for terminal
                    terminal_width
                } else {
                    ideal_width
                }
            }
        };

        // Get minimum width from environment variable
        let min_width_str = std::env::var("BOXY_MIN_WIDTH").unwrap_or_else(|_| "5".to_string());
        let min_width: usize = min_width_str.parse().unwrap_or(5);

        // Ensure minimum viable box size (user configurable via BOXY_MIN_WIDTH)
        if box_width < min_width { min_width } else { box_width }
    }}
}

/// Calculate optimal box width based on content and terminal constraints
/// Uses protected macro to preserve working logic
pub fn calculate_box_width(text: &str, h_padding: usize, fixed_width: Option<usize>, _enable_wrapping: bool) -> usize {
    box_width!(text, h_padding, fixed_width)
}

pub fn draw_box(config: BoxyConfig) {
    // Calculate width considering ALL content (text, title, status)
    let mut all_content = config.text.clone();
    if let Some(title) = &config.title {
        all_content.push('\n');
        all_content.push_str(title);
    }
    if let Some(status_bar) = &config.status_bar {
        all_content.push('\n');
        all_content.push_str(status_bar);
    }

    let final_width = calculate_box_width(&all_content, config.width.h_padding, config.width.fixed_width, config.width.enable_wrapping);
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
    for line in &body_lines {
        println!("{}", line);
    }

    // Use Status component for status bar rendering
    let status = Status::new(&config);
    let mut status_lines = Vec::new();
    if status.should_render() {
        status_lines = status.render(inner_width, &color_code, &text_color_code, &status_color_code);
        for line in &status_lines {
            println!("{}", line);
        }
    }

    // Height padding: add blank lines if fixed_height is set and needs more lines
    if let Some(target_height) = config.fixed_height {
        // Calculate current total lines: header(1) + body + status + footer(1)
        let current_total = 1 + body_lines.len() + status_lines.len() + 1;

        if target_height > current_total {
            let filler_needed = target_height - current_total;
            let pad = " ".repeat(config.width.h_padding);

            // Add blank padding lines before footer
            for _ in 0..filler_needed {
                let available_content_width = inner_width.saturating_sub(2 * config.width.h_padding);
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
                println!("{}", blank_line);
            }
        }
    }

    // Use Footer component for bottom border rendering
    let footer = Footer::new(&config);
    println!("{}", footer.render(inner_width, &color_code));

    // Note: header_color and footer_color are stored in config but not yet implemented
    // body_align, body_pad_emoji, pad_body_above, pad_body_below are stored in config but not yet implemented
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

// ============================================================================
// COMPONENT SYSTEM (from components.rs)
// ============================================================================

/// Header component that renders the top section of the box
pub struct Header<'a> {
    config: &'a BoxyConfig,
}

impl<'a> Header<'a> {
    pub fn new(config: &'a BoxyConfig) -> Self {
        Self { config }
    }

    /// Render the header (top border) with optional header text
    pub fn render(&self, inner_width: usize, color_code: &str) -> String {
        if let Some(header_text) = &self.config.header {
            let expanded_header = expand_variables(header_text);
            let header_line = render_title_or_footer(
                &expanded_header,
                inner_width,
                self.config.style.horizontal,
                &self.config.alignment.header_align
            );
            format!(
                "{}{}{}{}{}",
                color_code,
                self.config.style.top_left,
                header_line,
                self.config.style.top_right,
                RESET
            )
        } else {
            let border = self.config.style.horizontal.repeat(inner_width);
            format!(
                "{}{}{}{}{}",
                color_code,
                self.config.style.top_left,
                border,
                self.config.style.top_right,
                RESET
            )
        }
    }
}

/// Footer component that renders the bottom section of the box
pub struct Footer<'a> {
    config: &'a BoxyConfig,
}

impl<'a> Footer<'a> {
    pub fn new(config: &'a BoxyConfig) -> Self {
        Self { config }
    }

    /// Render the footer (bottom border) with optional footer text
    pub fn render(&self, inner_width: usize, color_code: &str) -> String {
        if let Some(footer_text) = &self.config.footer {
            let expanded_footer = expand_variables(footer_text);
            let footer_line = render_title_or_footer(
                &expanded_footer,
                inner_width,
                self.config.style.horizontal,
                &self.config.alignment.footer_align
            );
            format!(
                "{}{}{}{}{}",
                color_code,
                self.config.style.bottom_left,
                footer_line,
                self.config.style.bottom_right,
                RESET
            )
        } else {
            let border = self.config.style.horizontal.repeat(inner_width);
            format!(
                "{}{}{}{}{}",
                color_code,
                self.config.style.bottom_left,
                border,
                self.config.style.bottom_right,
                RESET
            )
        }
    }
}

/// Status component that renders the status bar inside the box
pub struct Status<'a> {
    config: &'a BoxyConfig,
}

impl<'a> Status<'a> {
    pub fn new(config: &'a BoxyConfig) -> Self {
        Self { config }
    }

    /// Check if status bar should be rendered
    pub fn should_render(&self) -> bool {
        self.config.status_bar.is_some()
    }

    /// Render status bar lines
    pub fn render(
        &self,
        inner_width: usize,
        color_code: &str,
        text_color_code: &str,
        status_color_code: &str,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        let pad = " ".repeat(self.config.width.h_padding);

        if let Some(status_bar) = &self.config.status_bar {
            let expanded_status = expand_variables(status_bar);
            let (align, content) = self.parse_status_alignment(&expanded_status);

            // Line before status (conditional)
            if self.config.padding.pad_before_status {
                let padding_line = format!("{}{}{}{}{}",
                    color_code,
                    self.config.style.vertical,
                    pad,
                    " ".repeat(inner_width.saturating_sub(2 * self.config.width.h_padding)),
                    self.config.style.vertical
                );
                lines.push(format!("{}{}", padding_line, RESET));
            }

            // Status line
            let available_width = inner_width.saturating_sub(2 * self.config.width.h_padding);
            let status_width = get_display_width(&content);
            let padding_needed = available_width.saturating_sub(status_width);

            let (left_pad, right_pad) = match align.as_str() {
                "center" => {
                    let left = padding_needed / 2;
                    let right = padding_needed - left;
                    (left, right)
                },
                "right" => (padding_needed, 0),
                _ => (0, padding_needed), // left alignment (default)
            };

            let status_line = format!(
                "{}{}{}{}{}{}{}{}{}{}",
                color_code,
                self.config.style.vertical,
                pad,
                " ".repeat(left_pad),
                status_color_code,
                content,
                text_color_code,
                " ".repeat(right_pad),
                pad,
                self.config.style.vertical
            );
            lines.push(format!("{}{}", status_line, RESET));

            // Line after status (conditional)
            if self.config.padding.pad_after_status {
                let padding_line = format!("{}{}{}{}{}",
                    color_code,
                    self.config.style.vertical,
                    pad,
                    " ".repeat(inner_width.saturating_sub(2 * self.config.width.h_padding)),
                    self.config.style.vertical
                );
                lines.push(format!("{}{}", padding_line, RESET));
            }
        }

        lines
    }

    /// Parse status alignment from status text
    fn parse_status_alignment(&self, expanded_status: &str) -> (String, String) {
        if expanded_status.starts_with("sc:") {
            ("center".to_string(), expanded_status.strip_prefix("sc:").unwrap_or(expanded_status).to_string())
        } else if expanded_status.starts_with("sr:") {
            ("right".to_string(), expanded_status.strip_prefix("sr:").unwrap_or(expanded_status).to_string())
        } else {
            ("left".to_string(), expanded_status.to_string())
        }
    }
}

/// Body component that renders the main content with preserved emoji/width calculations
pub struct Body<'a> {
    config: &'a BoxyConfig,
}

impl<'a> Body<'a> {
    pub fn new(config: &'a BoxyConfig) -> Self {
        Self { config }
    }

    /// Render the body content preserving existing emoji and width calculations
    pub fn render(
        &self,
        inner_width: usize,
        color_code: &str,
        text_color_code: &str,
        title_color_code: &str,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        let composed_lines = self.compose_content_lines();
        let pad = " ".repeat(self.config.width.h_padding);

        // PROTECTED: Calculate the actual max content width - DO NOT MODIFY
        // This macro preserves the exact working logic for content width calculation
        let _content_max_width = composed_lines.iter()
            .map(|line| get_display_width(line))
            .max()
            .unwrap_or(0);

        // PARALLEL SOLUTION: Calculate proper inner content width including title/status
        let inner_content_target_width = inner_width.saturating_sub(2 * self.config.width.h_padding);

        // Available space for content within the box
        let available_content_width = inner_width.saturating_sub(2 * self.config.width.h_padding);

        // Debug: uncomment to see width calculations
        // eprintln!("DEBUG: content_max_width={}, available_content_width={}", content_max_width, available_content_width);

        // Optional padding blank line before title
        if self.config.padding.pad_before_title && self.config.title.is_some() {
            lines.push(self.render_padding_line(inner_width, color_code, &pad));
        }

        for (i, line) in composed_lines.iter().enumerate() {
            // Only truncate if there are explicit width constraints (fixed_width) AND wrapping is disabled
            let line_width = get_display_width(&line);
            let should_truncate = self.config.width.fixed_width.is_some()
                && !self.config.width.enable_wrapping
                && line_width > available_content_width;
            let display_line = if should_truncate {
                truncate_with_ellipsis(&line, available_content_width)
            } else {
                line.to_string()
            };

            let width = get_display_width(&display_line);
            // IMPROVED: Use parallel solution for better inner content width calculation
            let target_width = inner_content_target_width;
            let spaces = " ".repeat(target_width.saturating_sub(width));

            // DEBUG: Show line width info
            // let debug_prefix = format!("[w:{:2} t:{:2}] ", width, target_width);
            let debug_prefix = "";

            // CRITICAL: Preserve existing icon handling logic (lines 128-170 from original draw.rs)
            if i == 0 && self.config.icon.is_some() {
                lines.push(self.render_first_line_with_icon(
                    line,
                    &display_line,
                    available_content_width,
                    color_code,
                    &pad,
                    text_color_code,
                    title_color_code,
                ));
            } else {
                // DEBUG: Prepend width info to the display line
                let debug_display = format!("{}{}", debug_prefix, display_line);

                lines.push(self.render_regular_line(
                    i,
                    &debug_display,
                    &spaces,
                    color_code,
                    &pad,
                    text_color_code,
                    title_color_code,
                ));
            }

            // Handle dividers and padding after title
            if self.config.dividers.divider_after_title && i == 0 {
                lines.push(self.render_title_divider(inner_width, color_code));
                if self.config.dividers.pad_after_title_divider {
                    lines.push(self.render_padding_line(inner_width, color_code, &pad));
                }
            } else if self.config.padding.pad_after_title && i == 0 {
                lines.push(self.render_padding_line(inner_width, color_code, &pad));
            }
        }


        lines
    }

    /// Compose content lines with optional title as first line (preserves existing logic)
    fn compose_content_lines(&self) -> Vec<String> {
        use crate::expand_variables;

        if self.config.width.fixed_width.is_none() {
            // AUTO WIDTH: Default wrapping at terminal boundaries, remove hints
            use crate::core::wrap_text_at_word_boundaries;
            use crate::width_plugin::get_terminal_width;

            let mut composed_lines: Vec<String> = Vec::new();

            if let Some(title_text) = &self.config.title {
                composed_lines.push(expand_variables(title_text));
            }

            // For auto width, wrap at terminal width minus padding and borders
            let terminal_width = get_terminal_width();
            let available_width = terminal_width.saturating_sub(2 * self.config.width.h_padding + 2);

            // Clean hints from text for auto width mode (but preserve #NL# as newlines)
            // Both #W# and #T# should normalize to spaces for consistent width calculation
            let cleaned_text = self.config.text.replace("#W#", " ").replace("#T#", " ");
            // Normalize whitespace within lines but preserve #NL# markers
            let lines: Vec<&str> = cleaned_text.lines().collect();
            let normalized_lines: Vec<String> = lines.iter()
                .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
                .collect();
            let cleaned_text = normalized_lines.join("\n").replace("#NL#", "\n");

            let wrapped_lines = wrap_text_at_word_boundaries(&cleaned_text, available_width);
            composed_lines.extend(wrapped_lines);

            composed_lines
        } else if self.config.width.enable_wrapping {
            // FIXED WIDTH + WRAPPING: Use hint-aware wrapping within fixed width
            use crate::core::wrap_text_at_word_boundaries;

            let mut composed_lines: Vec<String> = Vec::new();

            if let Some(title_text) = &self.config.title {
                composed_lines.push(expand_variables(title_text));
            }

            // Calculate max content width available for wrapping
            // Use the same width calculation as the main box
            let final_width = calculate_box_width(&self.config.text, self.config.width.h_padding, self.config.width.fixed_width, true);
            let available_width = final_width.saturating_sub(2); // Account for borders

            let wrapped_lines = wrap_text_at_word_boundaries(&self.config.text, available_width);
            composed_lines.extend(wrapped_lines);

            composed_lines
        } else {
            // FIXED WIDTH WITHOUT WRAPPING: Original truncation mode
            let lines: Vec<&str> = self.config.text.lines().collect();
            let mut composed_lines: Vec<String> = Vec::new();

            if let Some(title_text) = &self.config.title {
                composed_lines.push(expand_variables(title_text));
            }
            composed_lines.extend(lines.iter().map(|l| (*l).to_string()));

            composed_lines
        }
    }

    fn render_padding_line(&self, inner_width: usize, color_code: &str, pad: &str) -> String {
        let available_content_width = inner_width.saturating_sub(2 * self.config.width.h_padding);
        format!(
            "{}{}{}{}{}{}{}",
            color_code,
            self.config.style.vertical,
            RESET,
            pad,
            " ".repeat(available_content_width),
            pad,
            format!("{}{}{}", color_code, self.config.style.vertical, RESET)
        )
    }

    fn render_title_divider(&self, inner_width: usize, color_code: &str) -> String {
        format!(
            "{}{}{}{}{}",
            color_code,
            self.config.style.tee_left,
            self.config.style.horizontal.repeat(inner_width),
            self.config.style.tee_right,
            RESET
        )
    }

    /// CRITICAL: Preserve exact icon handling logic from original lines 128-170
    fn render_first_line_with_icon(
        &self,
        line: &str,
        display_line: &str,
        available_content_width: usize,
        color_code: &str,
        pad: &str,
        text_color_code: &str,
        title_color_code: &str,
    ) -> String {
        use crate::{get_display_width, truncate_with_ellipsis, expand_variables};

        // Avoid duplicate icon if the title line already starts with an emoji/non-ASCII
        let starts_with_emoji = line.chars().next().map(|c| !c.is_ascii()).unwrap_or(false);
        if starts_with_emoji {
            // Fall through to normal rendering without icon injection
            let line_code = if !title_color_code.is_empty() { title_color_code } else { text_color_code };
            let colored_display_line = if line_code.is_empty() { display_line.to_string() } else { format!("{}{}{}", line_code, display_line, RESET) };
            let width = get_display_width(&display_line);
            let spaces = " ".repeat(available_content_width.saturating_sub(width));
            return format!("{}{}{}{}{}{}{}{}",
                color_code, self.config.style.vertical, RESET,
                pad, colored_display_line, spaces, pad,
                format!("{}{}{}", color_code, self.config.style.vertical, RESET));
        }

        // First line with icon - LIPSIFIED
        let icon_str = self.config.icon.as_ref().unwrap();
        let icon_expanded = expand_variables(icon_str);

        // Only truncate icon content if there are explicit width constraints
        let icon_width = get_display_width(&icon_expanded) + 1; // +1 for space
        let line_width = get_display_width(line);
        let final_line = if self.config.width.fixed_width.is_some() && line_width > available_content_width.saturating_sub(icon_width) {
            truncate_with_ellipsis(line, available_content_width.saturating_sub(icon_width))
        } else {
            display_line.to_string()
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

        format!("{}{} {}{}{}{}{}{}{}",
            color_code, self.config.style.vertical, RESET,
            icon_expanded, " ",
            colored_final_line, final_spaces, pad,
            format!("{}{}{}", color_code, self.config.style.vertical, RESET))
    }

    fn render_regular_line(
        &self,
        line_index: usize,
        display_line: &str,
        spaces: &str,
        color_code: &str,
        pad: &str,
        text_color_code: &str,
        title_color_code: &str,
    ) -> String {
        // Apply text/status/title color to the display line
        let line_code = if line_index == 0 && !title_color_code.is_empty() { title_color_code } else { text_color_code };
        let colored_display_line = if line_code.is_empty() {
            display_line.to_string()
        } else {
            format!("{}{}{}", line_code, display_line, RESET)
        };

        format!("{}{}{}{}{}{}{}{}",
            color_code, self.config.style.vertical, RESET,
            pad, colored_display_line, spaces, pad,
            format!("{}{}{}", color_code, self.config.style.vertical, RESET))
    }
}

// ============================================================================
// CRITICAL PROTECTED MACROS
// ============================================================================
// MUST be accessible from outside this module

/// PROTECTED: Calculate the actual max content width - DO NOT MODIFY
/// This macro preserves the exact working logic for content width calculation
#[macro_export]
macro_rules! max_width {
    ($lines:expr) => {{
        $lines.iter()
            .map(|line| crate::get_display_width(line))
            .max()
            .unwrap_or(0)
    }}
}

#[macro_export]
macro_rules! inner_target_width {
    ($inner_width:expr, $h_padding:expr) => {{
        ($inner_width as usize).saturating_sub(2 * ($h_padding as usize))
    }}
}

// Note: macros are exported via #[macro_export] and available globally

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

    #[test]
    fn test_validate_box_style() {
        // Valid styles
        assert!(validate_box_style("normal").is_ok());
        assert!(validate_box_style("rounded").is_ok());
        assert!(validate_box_style("double").is_ok());
        assert!(validate_box_style("heavy").is_ok());
        assert!(validate_box_style("ascii").is_ok());

        // Invalid style
        assert!(validate_box_style("invalid").is_err());
        assert!(validate_box_style("").is_err());
    }

    #[test]
    fn test_box_style_constants() {
        // Test that constants have expected values
        assert_eq!(NORMAL.top_left, "┌");
        assert_eq!(ROUNDED.top_left, "╭");
        assert_eq!(DOUBLE.top_left, "╔");
        assert_eq!(HEAVY.top_left, "┏");
        assert_eq!(ASCII.top_left, "+");

        // Test default
        let default_style: BoxStyle = Default::default();
        assert_eq!(default_style.top_left, NORMAL.top_left);
    }
}