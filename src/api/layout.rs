//! Layout and component building - Dynamic box assembly
//!
//! This module provides component builders that create box layouts without
//! color coupling. Perfect for Room Runtime and other layout engines that
//! need to position components and apply their own styling.
//!
//! # Component System
//! - HeaderBuilder: Creates header components
//! - FooterBuilder: Creates footer components
//! - StatusBuilder: Creates status bar components
//! - BodyBuilder: Creates content body components
//! - BoxBuilder: Orchestrates complete box layout
//!
//! # RSB MODULE_SPEC Compliance
//! - No cross-module color dependencies
//! - Builder pattern for flexibility
//! - Pure component structure output

use crate::api::geometry::get_text_width;
use crate::visual::{BoxStyle, NORMAL};

/// Layout information for a positioned component
#[derive(Debug, Clone)]
pub struct ComponentLayout {
    /// Component content (no ANSI codes)
    pub content: String,
    /// Display width in terminal columns
    pub width: usize,
    /// Display height in terminal rows
    pub height: usize,
    /// Horizontal alignment
    pub h_align: HorizontalAlign,
    /// Vertical alignment
    pub v_align: VerticalAlign,
}

/// Horizontal alignment options
#[derive(Debug, Clone, PartialEq)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

/// Vertical alignment options
#[derive(Debug, Clone, PartialEq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

/// Builder for header components
#[derive(Debug)]
pub struct HeaderBuilder {
    content: Option<String>,
    align: HorizontalAlign,
    style: BoxStyle,
}

impl HeaderBuilder {
    pub fn new(content: &str) -> Self {
        Self {
            content: Some(content.to_string()),
            align: HorizontalAlign::Center,
            style: NORMAL,
        }
    }

    pub fn empty() -> Self {
        Self {
            content: None,
            align: HorizontalAlign::Center,
            style: NORMAL,
        }
    }

    pub fn align_left(mut self) -> Self {
        self.align = HorizontalAlign::Left;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.align = HorizontalAlign::Center;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.align = HorizontalAlign::Right;
        self
    }

    pub fn with_style(mut self, style: BoxStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build_for_width(self, inner_width: usize) -> ComponentLayout {
        let content = match &self.content {
            Some(text) => format!(
                "{}{}{}{}",
                self.style.top_left,
                self.render_header_line(text, inner_width),
                self.style.top_right,
                ""  // No color codes in pure layout
            ),
            None => format!(
                "{}{}{}",
                self.style.top_left,
                self.style.horizontal.repeat(inner_width),
                self.style.top_right
            ),
        };

        ComponentLayout {
            width: get_text_width(&content),
            height: 1,
            content,
            h_align: HorizontalAlign::Left, // Headers always align left in box context
            v_align: VerticalAlign::Top,
        }
    }

    fn render_header_line(&self, text: &str, inner_width: usize) -> String {
        let text_width = get_text_width(text);

        if text_width >= inner_width {
            // Truncate if too long
            return self.truncate_to_width(text, inner_width);
        }

        let remaining = inner_width - text_width;

        match self.align {
            HorizontalAlign::Left => {
                format!("{}{}", text, self.style.horizontal.repeat(remaining))
            }
            HorizontalAlign::Center => {
                let left_fill = remaining / 2;
                let right_fill = remaining - left_fill;
                format!(
                    "{}{}{}",
                    self.style.horizontal.repeat(left_fill),
                    text,
                    self.style.horizontal.repeat(right_fill)
                )
            }
            HorizontalAlign::Right => {
                format!("{}{}", self.style.horizontal.repeat(remaining), text)
            }
        }
    }

    fn truncate_to_width(&self, text: &str, max_width: usize) -> String {
        if max_width <= 3 {
            return self.style.horizontal.repeat(max_width);
        }

        let mut result = String::new();
        let mut width = 0;

        for ch in text.chars() {
            let char_width = get_text_width(&ch.to_string());
            if width + char_width > max_width - 3 {
                result.push_str("...");
                width += 3;
                break;
            }
            result.push(ch);
            width += char_width;
        }

        // Fill remaining with horizontal line
        if width < max_width {
            result.push_str(&self.style.horizontal.repeat(max_width - width));
        }

        result
    }
}

/// Builder for footer components
#[derive(Debug)]
pub struct FooterBuilder {
    content: Option<String>,
    align: HorizontalAlign,
    style: BoxStyle,
}

impl FooterBuilder {
    pub fn new(content: &str) -> Self {
        Self {
            content: Some(content.to_string()),
            align: HorizontalAlign::Center,
            style: NORMAL,
        }
    }

    pub fn empty() -> Self {
        Self {
            content: None,
            align: HorizontalAlign::Center,
            style: NORMAL,
        }
    }

    pub fn align_left(mut self) -> Self {
        self.align = HorizontalAlign::Left;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.align = HorizontalAlign::Center;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.align = HorizontalAlign::Right;
        self
    }

    pub fn with_style(mut self, style: BoxStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build_for_width(self, inner_width: usize) -> ComponentLayout {
        let content = match &self.content {
            Some(text) => format!(
                "{}{}{}",
                self.style.bottom_left,
                self.render_footer_line(text, inner_width),
                self.style.bottom_right
            ),
            None => format!(
                "{}{}{}",
                self.style.bottom_left,
                self.style.horizontal.repeat(inner_width),
                self.style.bottom_right
            ),
        };

        ComponentLayout {
            width: get_text_width(&content),
            height: 1,
            content,
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Bottom,
        }
    }

    fn render_footer_line(&self, text: &str, inner_width: usize) -> String {
        // Reuse header logic - same alignment behavior
        let header_builder = HeaderBuilder {
            content: Some(text.to_string()),
            align: self.align.clone(),
            style: self.style,
        };
        header_builder.render_header_line(text, inner_width)
    }
}

/// Builder for status bar components
#[derive(Debug)]
pub struct StatusBuilder {
    content: String,
    align: HorizontalAlign,
    show_divider: bool,
    padding: usize,
}

impl StatusBuilder {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            align: HorizontalAlign::Left,
            show_divider: true,
            padding: 1,
        }
    }

    pub fn align_left(mut self) -> Self {
        self.align = HorizontalAlign::Left;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.align = HorizontalAlign::Center;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.align = HorizontalAlign::Right;
        self
    }

    pub fn with_divider(mut self, show: bool) -> Self {
        self.show_divider = show;
        self
    }

    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    pub fn build_for_width(self, inner_width: usize, style: BoxStyle) -> ComponentLayout {
        let mut lines = Vec::new();

        // Add divider if requested
        if self.show_divider {
            let divider = format!(
                "{}{}{}",
                style.tee_left,
                style.horizontal.repeat(inner_width),
                style.tee_right
            );
            lines.push(divider);
        }

        // Add padding lines
        for _ in 0..self.padding {
            let padding_line = format!(
                "{}{}{}",
                style.vertical,
                " ".repeat(inner_width),
                style.vertical
            );
            lines.push(padding_line);
        }

        // Add status content line
        let status_line = self.render_status_line(&self.content, inner_width, &style);
        lines.push(status_line);

        // Add bottom padding lines
        for _ in 0..self.padding {
            let padding_line = format!(
                "{}{}{}",
                style.vertical,
                " ".repeat(inner_width),
                style.vertical
            );
            lines.push(padding_line);
        }

        let content = lines.join("\n");

        ComponentLayout {
            width: inner_width + 2, // Include borders
            height: lines.len(),
            content,
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Bottom,
        }
    }

    fn render_status_line(&self, text: &str, inner_width: usize, style: &BoxStyle) -> String {
        let available_width = inner_width.saturating_sub(2 * self.padding);
        let text_width = get_text_width(text);

        let (left_pad, right_pad) = if text_width >= available_width {
            (0, 0) // Will truncate
        } else {
            let space = available_width - text_width;
            match self.align {
                HorizontalAlign::Left => (0, space),
                HorizontalAlign::Center => {
                    let left = space / 2;
                    (left, space - left)
                }
                HorizontalAlign::Right => (space, 0),
            }
        };

        let status_content = if text_width > available_width {
            self.truncate_status(text, available_width)
        } else {
            format!(
                "{}{}{}",
                " ".repeat(left_pad),
                text,
                " ".repeat(right_pad)
            )
        };

        format!(
            "{}{}{}{}",
            style.vertical,
            " ".repeat(self.padding),
            status_content,
            " ".repeat(self.padding),
        )
    }

    fn truncate_status(&self, text: &str, max_width: usize) -> String {
        if max_width <= 3 {
            return " ".repeat(max_width);
        }

        let mut result = String::new();
        let mut width = 0;

        for ch in text.chars() {
            let char_width = get_text_width(&ch.to_string());
            if width + char_width > max_width - 3 {
                result.push_str("...");
                width += 3;
                break;
            }
            result.push(ch);
            width += char_width;
        }

        // Pad to full width
        if width < max_width {
            result.push_str(&" ".repeat(max_width - width));
        }

        result
    }
}

/// Builder for body content components
#[derive(Debug)]
pub struct BodyBuilder {
    lines: Vec<String>,
    h_padding: usize,
    v_padding: usize,
}

impl BodyBuilder {
    pub fn new(content: &str) -> Self {
        Self {
            lines: content.lines().map(|s| s.to_string()).collect(),
            h_padding: 2,
            v_padding: 0,
        }
    }

    pub fn from_lines(lines: Vec<String>) -> Self {
        Self {
            lines,
            h_padding: 2,
            v_padding: 0,
        }
    }

    pub fn with_h_padding(mut self, padding: usize) -> Self {
        self.h_padding = padding;
        self
    }

    pub fn with_v_padding(mut self, padding: usize) -> Self {
        self.v_padding = padding;
        self
    }

    pub fn build_for_width(self, inner_width: usize, style: BoxStyle) -> ComponentLayout {
        let mut result_lines = Vec::new();

        // Add top padding
        for _ in 0..self.v_padding {
            result_lines.push(self.create_padding_line(inner_width, &style));
        }

        // Add content lines
        for line in &self.lines {
            result_lines.push(self.create_content_line(line, inner_width, &style));
        }

        // Add bottom padding
        for _ in 0..self.v_padding {
            result_lines.push(self.create_padding_line(inner_width, &style));
        }

        let content = result_lines.join("\n");

        ComponentLayout {
            width: inner_width + 2,
            height: result_lines.len(),
            content,
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
        }
    }

    fn create_padding_line(&self, inner_width: usize, style: &BoxStyle) -> String {
        format!(
            "{}{}{}",
            style.vertical,
            " ".repeat(inner_width),
            style.vertical
        )
    }

    fn create_content_line(&self, line: &str, inner_width: usize, style: &BoxStyle) -> String {
        let line_width = get_text_width(line);
        let available_width = inner_width.saturating_sub(2 * self.h_padding);

        let content = if line_width > available_width {
            self.truncate_line(line, available_width)
        } else {
            format!(
                "{}{}",
                line,
                " ".repeat(available_width - line_width)
            )
        };

        format!(
            "{}{}{}{}",
            style.vertical,
            " ".repeat(self.h_padding),
            content,
            " ".repeat(self.h_padding),
        )
    }

    fn truncate_line(&self, line: &str, max_width: usize) -> String {
        if max_width <= 3 {
            return " ".repeat(max_width);
        }

        let mut result = String::new();
        let mut width = 0;

        for ch in line.chars() {
            let char_width = get_text_width(&ch.to_string());
            if width + char_width > max_width - 3 {
                result.push_str("...");
                width += 3;
                break;
            }
            result.push(ch);
            width += char_width;
        }

        // Pad to full width
        if width < max_width {
            result.push_str(&" ".repeat(max_width - width));
        }

        result
    }
}

/// Complete box builder orchestrating all components
#[derive(Debug)]
pub struct BoxBuilder {
    header: Option<HeaderBuilder>,
    footer: Option<FooterBuilder>,
    status: Option<StatusBuilder>,
    body: BodyBuilder,
    style: BoxStyle,
    fixed_width: Option<usize>,
}

impl BoxBuilder {
    pub fn new(content: &str) -> Self {
        Self {
            header: None,
            footer: None,
            status: None,
            body: BodyBuilder::new(content),
            style: NORMAL,
            fixed_width: None,
        }
    }

    pub fn with_header(mut self, header: HeaderBuilder) -> Self {
        self.header = Some(header);
        self
    }

    pub fn with_footer(mut self, footer: FooterBuilder) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn with_status(mut self, status: StatusBuilder) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_style(mut self, style: BoxStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_fixed_width(mut self, width: usize) -> Self {
        self.fixed_width = Some(width);
        self
    }

    pub fn build(self) -> BoxLayout {
        let inner_width = self.calculate_inner_width();

        let header = self.header.map(|h| h.with_style(self.style).build_for_width(inner_width));
        let footer = self.footer.map(|f| f.with_style(self.style).build_for_width(inner_width));
        let status = self.status.map(|s| s.build_for_width(inner_width, self.style));
        let body = self.body.build_for_width(inner_width, self.style);

        BoxLayout {
            header,
            footer,
            status,
            body,
            total_width: inner_width + 2,
            style: self.style,
        }
    }

    fn calculate_inner_width(&self) -> usize {
        match self.fixed_width {
            Some(w) => w.saturating_sub(2),
            None => {
                let body_width = self.body.lines.iter()
                    .map(|line| get_text_width(line))
                    .max()
                    .unwrap_or(0);
                body_width + (2 * self.body.h_padding) // Account for body padding
            }
        }
    }
}

/// Complete box layout result
#[derive(Debug)]
pub struct BoxLayout {
    pub header: Option<ComponentLayout>,
    pub footer: Option<ComponentLayout>,
    pub status: Option<ComponentLayout>,
    pub body: ComponentLayout,
    pub total_width: usize,
    pub style: BoxStyle,
}

impl BoxLayout {
    /// Render the complete box as a string (no colors)
    pub fn render(&self) -> String {
        let mut lines = Vec::new();

        if let Some(header) = &self.header {
            lines.push(header.content.clone());
        }

        lines.push(self.body.content.clone());

        if let Some(status) = &self.status {
            lines.push(status.content.clone());
        }

        if let Some(footer) = &self.footer {
            lines.push(footer.content.clone());
        }

        lines.join("\n")
    }

    /// Get individual component layouts for Room Runtime positioning
    pub fn components(&self) -> Vec<&ComponentLayout> {
        let mut components = Vec::new();

        if let Some(header) = &self.header {
            components.push(header);
        }

        components.push(&self.body);

        if let Some(status) = &self.status {
            components.push(status);
        }

        if let Some(footer) = &self.footer {
            components.push(footer);
        }

        components
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_builder() {
        let header = HeaderBuilder::new("Test Header")
            .align_center()
            .build_for_width(20);

        assert_eq!(header.width, 22); // 20 + 2 borders
        assert_eq!(header.height, 1);
        assert!(header.content.contains("Test Header"));
    }

    #[test]
    fn test_box_builder_complete() {
        let layout = BoxBuilder::new("Hello\nWorld")
            .with_header(HeaderBuilder::new("Title"))
            .with_footer(FooterBuilder::new("Footer"))
            .with_status(StatusBuilder::new("Status"))
            .build();

        assert!(layout.header.is_some());
        assert!(layout.footer.is_some());
        assert!(layout.status.is_some());
        assert_eq!(layout.body.height, 2); // 2 lines of content + padding

        let components = layout.components();
        assert_eq!(components.len(), 4); // header, body, status, footer
    }

    #[test]
    fn test_emoji_in_layout() {
        let layout = BoxBuilder::new("Hello 🌟 World").build();
        let rendered = layout.render();

        // Should handle emoji width correctly
        assert!(rendered.contains("Hello 🌟 World"));
    }
}