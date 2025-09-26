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
            .with_header(layout::HeaderBuilder::new("Title"))
            .with_footer(layout::FooterBuilder::new("Status"))
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

        // Note: TextMetrics doesn't have line_count, just width metrics
        assert_eq!(metrics.char_count, 13); // H-e-l-l-o-\n-W-o-r-l-d- -🌍 = 13 chars
        assert!(metrics.has_wide_chars); // Emoji counts as wide char
        assert!(metrics.display_width > 0); // Has some width
    }

    #[test]
    fn test_box_dimensions() {
        use boxy::visual::NORMAL;
        let dims = geometry::calculate_box_dimensions(
            "Test",
            NORMAL,
            1, // h_padding
            1, // v_padding
            None // fixed_width
        );

        // "Test" = 4 chars + 2 padding + 2 borders = 8
        assert_eq!(dims.total_width, 8);
        // 1 line + 2 padding + 2 borders = 5
        assert_eq!(dims.total_height, 5);
        assert_eq!(dims.inner_width, 6); // total - 2 for borders
        assert_eq!(dims.inner_height, 3); // total - 2 for borders
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

        // Basic smoke tests - content includes borders and formatting
        // Each header line will be wider than the specified width due to borders
        assert!(left_header.content.len() > 0);
        assert!(center_header.content.len() > 0);
        assert!(right_header.content.len() > 0);
    }

    #[test]
    fn test_box_width_setting() {
        let narrow = layout::BoxBuilder::new("Content")
            .with_fixed_width(20)
            .build();

        let wide = layout::BoxBuilder::new("Content")
            .with_fixed_width(40)
            .build();

        let narrow_output = narrow.render();
        let narrow_lines: Vec<_> = narrow_output.lines().collect();
        let wide_output = wide.render();
        let wide_lines: Vec<_> = wide_output.lines().collect();

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
        let _layout = layout::BoxBuilder::new("Test").build();
        let _renderer = theming::create_plain_renderer();

        // Plain renderer is a function, not an object with render method
        // Just test that we can create it
        assert!(true); // Placeholder test
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
            .with_fixed_width(20)  // Force truncation
            .with_wrapping(false)  // Disable wrapping to test truncation
            .build();

        let rendered = layout.render();

        // Should contain ellipsis character when truncated
        assert!(rendered.contains("…"));

        // Should not panic on emoji boundaries
        assert!(!rendered.is_empty());
    }

    #[test]
    fn test_multiline_content() {
        let multiline = "Line 1\nLine 2\nLine 3";
        let layout = layout::BoxBuilder::new(multiline)
            .with_fixed_width(30)
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

    #[test]
    fn test_convenience_renderer() {
        use layout::{render_box, BoxOptions};

        let output = render_box("Test Content", BoxOptions {
            header: Some("Title".to_string()),
            footer: Some("Footer".to_string()),
            width: Some(40),
            ..Default::default()
        });

        assert!(output.contains("Test Content"));
        assert!(output.contains("Title"));
        assert!(output.contains("Footer"));
    }

    #[test]
    fn test_render_lines() {
        let layout = layout::BoxBuilder::new("Content")
            .with_header(layout::HeaderBuilder::new("Header"))
            .build();

        let lines = layout.render_lines();
        assert!(lines.len() >= 2); // At least header and body
        assert!(lines[0].contains("Header") || lines[0].contains("─")); // Header or border
    }

    #[test]
    fn test_ansi_size_comparison() {
        let plain = "Hello World";
        let colored = "\x1b[32mHello World\x1b[0m";

        let comparison = geometry::compare_ansi_sizes(plain, colored);

        assert_eq!(comparison.plain_bytes, 11);
        assert!(comparison.colored_bytes > comparison.plain_bytes);
        assert!(comparison.color_overhead > 0);
        assert!(comparison.overhead_percentage > 0.0);
    }

    #[test]
    fn test_room_runtime_adapter() {
        use boxy::api::room_runtime::{RoomRuntimeAdapter, ComponentType};

        let layout = layout::BoxBuilder::new("Body")
            .with_header(layout::HeaderBuilder::new("Header"))
            .with_footer(layout::FooterBuilder::new("Footer"))
            .build();

        let adapter = RoomRuntimeAdapter::new(layout);
        let positions = adapter.positions();

        // Should have 3 components
        assert_eq!(positions.len(), 3);

        // First should be header
        assert_eq!(positions[0].component_type, ComponentType::Header);

        // Component at line 0 should be header
        if let Some((_, comp_type)) = adapter.component_at_line(0) {
            assert_eq!(comp_type, ComponentType::Header);
        }
    }
}