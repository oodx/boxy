use crate::{expand_variables, render_title_or_footer, RESET};
use crate::config::BoxyConfig;

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

    /// Render the status bar content with alignment and padding
    pub fn render(
        &self,
        inner_width: usize,
        color_code: &str,
        text_color_code: &str,
        status_color_code: &str,
    ) -> Vec<String> {
        let mut lines = Vec::new();

        if let Some(status_text) = &self.config.status_bar {
            // Add padding before status if configured
            if self.config.padding.pad_before_status {
                lines.push(self.render_padding_line(inner_width, color_code));
            }

            // Add divider before status if configured
            if self.config.dividers.divider_before_status {
                if self.config.dividers.pad_before_status_divider {
                    lines.push(self.render_padding_line(inner_width, color_code));
                }
                lines.push(self.render_divider_line(inner_width, color_code));
            }

            // Render the actual status line
            lines.push(self.render_status_content(
                status_text,
                inner_width,
                color_code,
                text_color_code,
                status_color_code,
            ));

            // Add padding after status if configured
            if self.config.padding.pad_after_status {
                lines.push(self.render_padding_line(inner_width, color_code));
            }
        }

        lines
    }

    fn render_padding_line(&self, inner_width: usize, color_code: &str) -> String {
        let available_content_width = inner_width.saturating_sub(2 * self.config.width.h_padding);
        let pad = " ".repeat(self.config.width.h_padding);
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

    fn render_divider_line(&self, inner_width: usize, color_code: &str) -> String {
        format!(
            "{}{}{}{}{}",
            color_code,
            self.config.style.tee_left,
            self.config.style.horizontal.repeat(inner_width),
            self.config.style.tee_right,
            RESET
        )
    }

    fn render_status_content(
        &self,
        status_text: &str,
        inner_width: usize,
        color_code: &str,
        text_color_code: &str,
        status_color_code: &str,
    ) -> String {
        use crate::{get_display_width, truncate_with_ellipsis};

        let expanded_status = expand_variables(status_text);
        let (alignment, clean_status) = self.parse_status_alignment(&expanded_status);

        let available_content_width = inner_width.saturating_sub(2 * self.config.width.h_padding);
        let status_display = if self.config.width.fixed_width.is_some() && get_display_width(&clean_status) > available_content_width {
            truncate_with_ellipsis(&clean_status, available_content_width)
        } else {
            clean_status
        };

        let final_width = get_display_width(&status_display);

        // DEBUG: Show width info for status
        // let debug_prefix = format!("[w:{:2} t:{:2}] ", final_width, available_content_width);
        let debug_prefix = "";

        let (left_pad_inner, right_pad_inner) = match alignment.as_str() {
            "center" => {
                let space = available_content_width.saturating_sub(final_width + debug_prefix.len());
                let lp = space / 2;
                (lp, space.saturating_sub(lp))
            }
            "right" => {
                let space = available_content_width.saturating_sub(final_width + debug_prefix.len());
                (space, 0)
            }
            _ => (0, available_content_width.saturating_sub(final_width + debug_prefix.len())),
        };

        let status_line = format!(
            "{}{}{}{}",
            debug_prefix,
            " ".repeat(left_pad_inner),
            status_display,
            " ".repeat(right_pad_inner)
        );

        let status_code = if !status_color_code.is_empty() {
            status_color_code
        } else {
            text_color_code
        };

        let colored_status = if status_code.is_empty() {
            status_line
        } else {
            format!("{}{}{}", status_code, status_line, RESET)
        };

        let pad = " ".repeat(self.config.width.h_padding);
        format!(
            "{}{}{}{}{}{}{}",
            color_code,
            self.config.style.vertical,
            RESET,
            pad,
            colored_status,
            pad,
            format!("{}{}{}", color_code, self.config.style.vertical, RESET)
        )
    }

    fn parse_status_alignment(&self, expanded_status: &str) -> (String, String) {
        if let Some(ov) = &self.config.alignment.status_align_override {
            (ov.clone(), expanded_status.to_string())
        } else if expanded_status.starts_with("sl:") {
            ("left".to_string(), expanded_status.strip_prefix("sl:").unwrap_or(expanded_status).to_string())
        } else if expanded_status.starts_with("sc:") {
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
        use crate::{get_display_width, truncate_with_ellipsis};

        let mut lines = Vec::new();
        let composed_lines = self.compose_content_lines();
        let pad = " ".repeat(self.config.width.h_padding);

        // PROTECTED: Calculate the actual max content width - DO NOT MODIFY
        // This macro preserves the exact working logic for content width calculation
        macro_rules! max_width {
            ($lines:expr) => {{
                $lines.iter()
                    .map(|line| get_display_width(line))
                    .max()
                    .unwrap_or(0)
            }}
        }

        macro_rules! inner_target_width {
            ($inner_width:expr, $h_padding:expr) => {{
                $inner_width.saturating_sub(2 * $h_padding)
            }}
        }

        let content_max_width = max_width!(composed_lines);

        // PARALLEL SOLUTION: Calculate proper inner content width including title/status
        let inner_content_target_width = inner_target_width!(inner_width, self.config.width.h_padding);

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
            use crate::parser::wrap_text_at_word_boundaries;
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
            use crate::parser::wrap_text_at_word_boundaries;

            let mut composed_lines: Vec<String> = Vec::new();

            if let Some(title_text) = &self.config.title {
                composed_lines.push(expand_variables(title_text));
            }

            // Calculate max content width available for wrapping
            // Use the same width calculation as the main box
            use crate::draw::calculate_box_width;
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

/// PARALLEL SOLUTION: Calculate proper inner content target width
/// This helper function determines the correct width for padding content lines
/// without breaking the existing width calculation logic
fn calculate_inner_content_target_width(
    inner_width: usize,
    _composed_lines: &[String],
    _is_fixed_width: bool,
    h_padding: usize
) -> usize {
    // The target width for content should be the inner width minus padding on both sides
    inner_width.saturating_sub(2 * h_padding)
}