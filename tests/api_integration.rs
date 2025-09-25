// Integration tests for the Boxy library API
// Tests the public API surface to ensure it works as documented

#[cfg(test)]
mod api_tests {
    use boxy::api::{geometry, layout, theming};

    #[test]
    fn test_basic_box_creation() {
        let box_layout = layout::BoxBuilder::new("Test content")
            .build();

        let rendered = box_layout.render();
        assert!(rendered.contains("Test content"));
        assert!(rendered.contains("┌")); // Should have box borders
        assert!(rendered.contains("┐"));
        assert!(rendered.contains("└"));
        assert!(rendered.contains("┘"));
    }

    #[test]
    fn test_box_with_header_footer() {
        let box_layout = layout::BoxBuilder::new("Main")
            .with_header("Title")
            .with_footer("Status")
            .build();

        let rendered = box_layout.render();
        assert!(rendered.contains("Title"));
        assert!(rendered.contains("Main"));
        assert!(rendered.contains("Status"));
    }

    #[test]
    fn test_geometry_text_width() {
        // ASCII text
        assert_eq!(geometry::get_text_width("Hello"), 5);

        // Emoji (should be width 2)
        assert_eq!(geometry::get_text_width("🚀"), 2);

        // CJK characters (should be width 2 each)
        assert_eq!(geometry::get_text_width("中文"), 4);

        // Mixed content
        assert_eq!(geometry::get_text_width("Hi 🌟"), 5); // "Hi " = 3, "🌟" = 2
    }

    #[test]
    fn test_text_metrics() {
        let metrics = geometry::get_text_metrics("Hello\nWorld 🌍");

        assert_eq!(metrics.line_count, 2);
        assert_eq!(metrics.char_count, 12); // Including newline and emoji
        assert!(metrics.has_emoji);
        assert_eq!(metrics.display_width, 8); // "World 🌍" is longer
    }

    #[test]
    fn test_box_dimensions() {
        let dims = geometry::calculate_box_dimensions(
            "Test",
            "normal",
            1, // h_padding
            1  // v_padding
        );

        // "Test" = 4 chars + 2 padding + 2 borders = 8
        assert_eq!(dims.total_width, 8);
        // 1 line + 2 padding + 2 borders = 5
        assert_eq!(dims.total_height, 5);
        assert_eq!(dims.content_width, 4);
        assert_eq!(dims.content_height, 1);
    }

    #[test]
    fn test_header_alignment() {
        let left_header = layout::HeaderBuilder::new("Left")
            .align_left()
            .build_for_width(20);

        let center_header = layout::HeaderBuilder::new("Center")
            .align_center()
            .build_for_width(20);

        let right_header = layout::HeaderBuilder::new("Right")
            .align_right()
            .build_for_width(20);

        // Basic smoke tests
        assert_eq!(left_header.content.len(), 20);
        assert_eq!(center_header.content.len(), 20);
        assert_eq!(right_header.content.len(), 20);
    }

    #[test]
    fn test_box_width_setting() {
        let narrow = layout::BoxBuilder::new("Content")
            .with_width(20)
            .build();

        let wide = layout::BoxBuilder::new("Content")
            .with_width(40)
            .build();

        let narrow_lines: Vec<_> = narrow.render().lines().collect();
        let wide_lines: Vec<_> = wide.render().lines().collect();

        // Check that first line (top border) has correct width
        assert!(narrow_lines[0].contains("─"));
        assert!(wide_lines[0].contains("─"));

        // Wide box should have longer lines
        let narrow_width = narrow_lines[0].chars().filter(|&c| c == '─').count();
        let wide_width = wide_lines[0].chars().filter(|&c| c == '─').count();
        assert!(wide_width > narrow_width);
    }

    #[test]
    fn test_plain_renderer() {
        let layout = layout::BoxBuilder::new("Test").build();
        let renderer = theming::create_plain_renderer();
        let output = renderer.render(&layout);

        // Should not contain ANSI escape codes
        assert!(!output.contains("\x1b["));
        assert!(output.contains("Test"));
    }

    #[test]
    fn test_background_color_variants() {
        // Test that all background color types can be created
        let _ansi = theming::BackgroundColor::Ansi(42);
        let _rgb = theming::BackgroundColor::Rgb(255, 0, 0);
        let _named = theming::BackgroundColor::Named("blue".to_string());
        let _hex = theming::BackgroundColor::Hex("#ff0000".to_string());
        let _none = theming::BackgroundColor::None;

        // Basic smoke test for color scheme
        let _scheme = theming::ColorScheme::default();
    }

    #[test]
    fn test_unicode_truncation() {
        let text = "This is a long text with emoji 🚀 that needs truncation";
        let layout = layout::BoxBuilder::new(text)
            .with_width(20)  // Force truncation
            .build();

        let rendered = layout.render();

        // Should contain ellipsis when truncated
        assert!(rendered.contains("..."));

        // Should not panic on emoji boundaries
        assert!(!rendered.is_empty());
    }

    #[test]
    fn test_multiline_content() {
        let multiline = "Line 1\nLine 2\nLine 3";
        let layout = layout::BoxBuilder::new(multiline)
            .with_width(30)
            .build();

        let rendered = layout.render();
        let lines: Vec<_> = rendered.lines().collect();

        // Should have at least 3 content lines plus borders
        assert!(lines.len() >= 5); // top border + 3 lines + bottom border

        // Check all lines are present
        assert!(rendered.contains("Line 1"));
        assert!(rendered.contains("Line 2"));
        assert!(rendered.contains("Line 3"));
    }
}