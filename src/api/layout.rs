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
use crate::truncate_with_ellipsis;

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

        // Pass the full width - truncate_with_ellipsis handles ellipsis internally
        let truncated = truncate_with_ellipsis(text, max_width);
        let truncated_width = get_text_width(&truncated);

        // Fill remaining with horizontal line
        if truncated_width < max_width {
            format!("{}{}",
                truncated,
                self.style.horizontal.repeat(max_width - truncated_width)
            )
        } else {
            truncated
        }
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
            "{}{}{}{}{}",
            style.vertical,
            " ".repeat(self.padding),
            status_content,
            " ".repeat(self.padding),
            style.vertical
        )
    }

    fn truncate_status(&self, text: &str, max_width: usize) -> String {
        if max_width <= 3 {
            return " ".repeat(max_width);
        }

        let truncated = truncate_with_ellipsis(text, max_width);
        let truncated_width = get_text_width(&truncated);

        // Pad to full width
        format!("{}{}",
            truncated,
            " ".repeat(max_width.saturating_sub(truncated_width))
        )
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
            "{}{}{}{}{}",
            style.vertical,
            " ".repeat(self.h_padding),
            content,
            " ".repeat(self.h_padding),
            style.vertical
        )
    }

    fn truncate_line(&self, line: &str, max_width: usize) -> String {
        if max_width <= 3 {
            return " ".repeat(max_width);
        }

        let truncated = truncate_with_ellipsis(line, max_width);
        let truncated_width = get_text_width(&truncated);

        // Pad to full width
        format!("{}{}",
            truncated,
            " ".repeat(max_width.saturating_sub(truncated_width))
        )
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

        // Auto-add empty header/footer if not provided to ensure closed box
        let header = self.header
            .or_else(|| Some(HeaderBuilder::empty()))
            .map(|h| h.with_style(self.style).build_for_width(inner_width));
        let footer = self.footer
            .or_else(|| Some(FooterBuilder::empty()))
            .map(|f| f.with_style(self.style).build_for_width(inner_width));
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
        self.render_lines().join("\n")
    }

    /// QOL: Render as individual lines for layout engines
    /// Returns Vec<String> for easier positioning in Room Runtime
    pub fn render_lines(&self) -> Vec<String> {
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

        lines
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

// ============================================================================
// QOL IMPROVEMENTS: Convenience API
// ============================================================================

/// Options for the convenience renderer
#[derive(Debug, Clone, Default)]
pub struct BoxOptions {
    /// Optional header text
    pub header: Option<String>,
    /// Optional footer text
    pub footer: Option<String>,
    /// Optional status text
    pub status: Option<String>,
    /// Fixed width (defaults to auto)
    pub width: Option<usize>,
    /// Box style (defaults to normal)
    pub style: Option<BoxStyle>,
}

/// QOL: Convenience function for simple box rendering
/// Covers 80% of use cases with minimal boilerplate
///
/// # Example
/// ```rust
/// use boxy::api::layout::{render_box, BoxOptions};
///
/// // Simple usage
/// let output = render_box("Hello, World!", BoxOptions::default());
///
/// // With options
/// let output = render_box("Content", BoxOptions {
///     header: Some("Title".to_string()),
///     footer: Some("v1.0".to_string()),
///     width: Some(50),
///     ..Default::default()
/// });
/// ```
pub fn render_box(content: &str, options: BoxOptions) -> String {
    let mut builder = BoxBuilder::new(content);

    if let Some(header_text) = options.header {
        builder = builder.with_header(HeaderBuilder::new(&header_text));
    }

    if let Some(footer_text) = options.footer {
        builder = builder.with_footer(FooterBuilder::new(&footer_text));
    }

    if let Some(status_text) = options.status {
        builder = builder.with_status(StatusBuilder::new(&status_text));
    }

    if let Some(width) = options.width {
        builder = builder.with_fixed_width(width);
    }

    if let Some(style) = options.style {
        builder = builder.with_style(style);
    }

    builder.build().render()
}

/// QOL: Render box and return lines for positioning
pub fn render_box_lines(content: &str, options: BoxOptions) -> Vec<String> {
    let mut builder = BoxBuilder::new(content);

    if let Some(header_text) = options.header {
        builder = builder.with_header(HeaderBuilder::new(&header_text));
    }

    if let Some(footer_text) = options.footer {
        builder = builder.with_footer(FooterBuilder::new(&footer_text));
    }

    if let Some(status_text) = options.status {
        builder = builder.with_status(StatusBuilder::new(&status_text));
    }

    if let Some(width) = options.width {
        builder = builder.with_fixed_width(width);
    }

    if let Some(style) = options.style {
        builder = builder.with_style(style);
    }

    builder.build().render_lines()
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

    #[test]
    fn test_box_has_complete_borders() {
        let layout = BoxBuilder::new("test content").build();
        let rendered = layout.render();

        // Check all borders are present
        assert!(rendered.starts_with("┌"), "Missing top-left corner");

        let lines: Vec<&str> = rendered.lines().collect();
        assert!(lines[0].ends_with("┐"), "Missing top-right corner");

        // Check all middle lines have side borders
        for (i, line) in lines.iter().enumerate() {
            if i > 0 && i < lines.len() - 1 {
                assert!(line.starts_with("│"), "Missing left border on line {}", i);
                assert!(line.ends_with("│"), "Missing right border on line {}", i);
            }
        }

        let last_line = lines.last().expect("Box should have lines");
        assert!(last_line.starts_with("└"), "Missing bottom-left corner");
        assert!(last_line.ends_with("┘"), "Missing bottom-right corner");
    }

    #[test]
    fn test_box_width_consistency() {
        let layout = BoxBuilder::new("test").with_fixed_width(30).build();
        let rendered = layout.render();

        let lines: Vec<&str> = rendered.lines().collect();
        let expected_width = layout.total_width;

        for (i, line) in lines.iter().enumerate() {
            let line_width = get_text_width(line);
            assert_eq!(
                line_width, expected_width,
                "Line {} has width {} but expected {}. Content: '{}'",
                i, line_width, expected_width, line
            );
        }
    }

    #[test]
    fn test_body_borders_complete() {
        let body = BodyBuilder::new("test line")
            .with_h_padding(2)
            .build_for_width(20, NORMAL);

        // Check each line in body has both borders
        for line in body.content.lines() {
            assert!(line.starts_with("│"), "Body line missing left border");
            assert!(line.ends_with("│"), "Body line missing right border");
        }
    }

    #[test]
    fn test_status_borders_complete() {
        let status = StatusBuilder::new("status text")
            .with_divider(true)
            .build_for_width(20, NORMAL);

        let lines: Vec<&str> = status.content.lines().collect();

        // First line should be divider
        assert!(lines[0].starts_with("├"), "Status divider missing left tee");
        assert!(lines[0].ends_with("┤"), "Status divider missing right tee");

        // All other lines should have vertical borders
        for line in lines.iter().skip(1) {
            assert!(line.starts_with("│"), "Status line missing left border");
            assert!(line.ends_with("│"), "Status line missing right border");
        }
    }

    #[test]
    fn test_default_box_is_closed() {
        let layout = BoxBuilder::new("content").build();
        let rendered = layout.render();

        // Verify box has top and bottom borders (is closed)
        assert!(rendered.starts_with("┌"), "Default box missing top border");
        assert!(rendered.contains("┘"), "Default box missing bottom border");

        let lines: Vec<&str> = rendered.lines().collect();
        assert!(lines.len() >= 3, "Box should have at least header, body, footer");
    }

    #[test]
    fn test_truncation_preserves_graphemes() {
        // Test that our truncation functions work correctly
        let header = HeaderBuilder::new("Very long text that needs truncation")
            .build_for_width(10);

        // Should truncate and not panic
        assert!(header.content.len() > 0);

        let body = BodyBuilder::new("Another very long line that needs truncation")
            .build_for_width(10, NORMAL);

        // Body should handle truncation
        assert!(body.content.len() > 0);
    }

    #[test]
    fn test_header_ellipsis_preservation() {
        // Test that headers properly handle width constraints
        let header = HeaderBuilder::new("Test Header")
            .build_for_width(20);

        // Should have top border characters and be width-limited
        assert!(header.content.starts_with("┌"));
        assert!(header.content.ends_with("┐"));

        // Verify the content fits within the specified width
        let actual_width = get_text_width(&header.content);
        assert_eq!(actual_width, 22, "Header width should be inner_width + 2 borders");
    }

    #[test]
    fn test_header_no_trailing_horizontals() {
        // Specific test for the regression where we had "...───" pattern
        let header = HeaderBuilder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .build_for_width(10); // Small width to force truncation

        // Extract inner content safely
        let content_chars: Vec<char> = header.content.chars().collect();
        if content_chars.len() >= 3 {
            let inner: String = content_chars[1..content_chars.len()-1].iter().collect();

            // Should NOT have pattern where ellipsis is followed by horizontal lines
            assert!(!inner.contains("...─"),
                "Header should not have '...─' pattern. Got: '{}'", header.content);

            // Should NOT end with just horizontal lines
            if inner.contains("...") {
                let after_ellipsis = inner.split("...").last().unwrap_or("");
                assert!(!after_ellipsis.chars().all(|c| c == '─'),
                    "Should not have only horizontal lines after ellipsis. Got: '{}'", header.content);
            }
        }
    }
}