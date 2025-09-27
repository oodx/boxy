// Complete feature demonstration for Boxy v0.21.0 API
// Shows all builder features: headers, footers, status, padding, wrapping, height

use boxy::api::layout::{BoxBuilder, FooterBuilder, HeaderBuilder, StatusBuilder};

fn main() {
    println!("Boxy v0.21.0 Complete Feature Showcase");
    println!("=======================================\n");

    // Example 1: Header with alignment options
    println!("1. Header Alignment");
    println!("-------------------\n");

    let left_header = BoxBuilder::new("Content with left-aligned header")
        .with_header(HeaderBuilder::new("Left Header").align_left())
        .with_fixed_width(50)
        .build();
    println!("{}", left_header.render());
    println!();

    let center_header = BoxBuilder::new("Content with center-aligned header")
        .with_header(HeaderBuilder::new("Center Header").align_center())
        .with_fixed_width(50)
        .build();
    println!("{}", center_header.render());
    println!();

    let right_header = BoxBuilder::new("Content with right-aligned header")
        .with_header(HeaderBuilder::new("Right Header").align_right())
        .with_fixed_width(50)
        .build();
    println!("{}", right_header.render());
    println!();

    // Example 2: Footer alignment
    println!("2. Footer Alignment");
    println!("-------------------\n");

    let footer_box = BoxBuilder::new("Main content area")
        .with_footer(FooterBuilder::new("Left Footer").align_left())
        .with_fixed_width(50)
        .build();
    println!("{}", footer_box.render());
    println!();

    let center_footer = BoxBuilder::new("Main content area")
        .with_footer(FooterBuilder::new("Center Footer").align_center())
        .with_fixed_width(50)
        .build();
    println!("{}", center_footer.render());
    println!();

    let right_footer = BoxBuilder::new("Main content area")
        .with_footer(FooterBuilder::new("Right Footer").align_right())
        .with_fixed_width(50)
        .build();
    println!("{}", right_footer.render());
    println!();

    // Example 3: Status line with alignment
    println!("3. Status Line Alignment");
    println!("------------------------\n");

    let status_left = BoxBuilder::new("Content with left status")
        .with_status(StatusBuilder::new("Status: Active").align_left())
        .with_fixed_width(50)
        .build();
    println!("{}", status_left.render());
    println!();

    let status_center = BoxBuilder::new("Content with center status")
        .with_status(StatusBuilder::new("⚡ Processing").align_center())
        .with_fixed_width(50)
        .build();
    println!("{}", status_center.render());
    println!();

    let status_right = BoxBuilder::new("Content with right status")
        .with_status(StatusBuilder::new("100% Complete").align_right())
        .with_fixed_width(50)
        .build();
    println!("{}", status_right.render());
    println!();

    // Example 4: Padding variations
    println!("4. Padding Control");
    println!("------------------\n");

    println!("Default padding (2):");
    let default_padding = BoxBuilder::new("Default horizontal padding")
        .with_fixed_width(40)
        .build();
    println!("{}", default_padding.render());
    println!();

    println!("No horizontal padding:");
    let no_padding = BoxBuilder::new("No horizontal padding")
        .with_h_padding(0)
        .with_fixed_width(40)
        .build();
    println!("{}", no_padding.render());
    println!();

    println!("Extra horizontal padding (5):");
    let extra_padding = BoxBuilder::new("Extra horizontal padding")
        .with_h_padding(5)
        .with_fixed_width(40)
        .build();
    println!("{}", extra_padding.render());
    println!();

    println!("Vertical padding (2 lines above/below):");
    let vertical_padding = BoxBuilder::new("Content with vertical padding")
        .with_v_padding(2)
        .with_fixed_width(40)
        .build();
    println!("{}", vertical_padding.render());
    println!();

    // Example 5: Text wrapping
    println!("5. Text Wrapping");
    println!("----------------\n");

    let long_text = "This is a very long line of text that will demonstrate the wrapping feature at word boundaries for better readability.";

    println!("Without wrapping (truncated):");
    let no_wrap = BoxBuilder::new(long_text).with_fixed_width(40).build();
    println!("{}", no_wrap.render());
    println!();

    println!("With wrapping enabled:");
    let with_wrap = BoxBuilder::new(long_text)
        .with_fixed_width(40)
        .with_wrapping(true)
        .build();
    println!("{}", with_wrap.render());
    println!();

    // Example 6: Height constraints
    println!("6. Height Constraints");
    println!("---------------------\n");

    let many_lines =
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";

    println!("Without height constraint:");
    let no_height = BoxBuilder::new(many_lines).with_fixed_width(30).build();
    println!("{}", no_height.render());
    println!();

    println!("With height constraint (6 lines):");
    let with_height = BoxBuilder::new(many_lines)
        .with_fixed_width(30)
        .with_fixed_height(6)
        .build();
    println!("{}", with_height.render());
    println!();

    // Example 7: Everything combined
    println!("7. All Features Combined");
    println!("------------------------\n");

    let rich_content = "This comprehensive example demonstrates all features working together: text wrapping, height constraints, headers, footers, status lines, and custom padding. The content will wrap at word boundaries and be truncated if it exceeds the height limit.";

    let complete_box = BoxBuilder::new(rich_content)
        .with_header(HeaderBuilder::new("📊 Dashboard").align_center())
        .with_footer(FooterBuilder::new("v0.21.0").align_right())
        .with_status(StatusBuilder::new("⚡ All Systems Online").align_center())
        .with_fixed_width(50)
        .with_fixed_height(12)
        .with_h_padding(3)
        .with_v_padding(1)
        .with_wrapping(true)
        .build();

    println!("{}", complete_box.render());
    println!();

    // Example 8: Status line with divider control
    println!("8. Status Line Divider Control");
    println!("-------------------------------\n");

    let status_with_divider = BoxBuilder::new("Content with status divider")
        .with_status(StatusBuilder::new("Status with divider").with_divider(true))
        .with_fixed_width(40)
        .build();
    println!("{}", status_with_divider.render());
    println!();

    let status_no_divider = BoxBuilder::new("Content without status divider")
        .with_status(StatusBuilder::new("Status without divider").with_divider(false))
        .with_fixed_width(40)
        .build();
    println!("{}", status_no_divider.render());
    println!();

    // Example 9: Status line with custom padding
    println!("9. Status Line Padding");
    println!("----------------------\n");

    let status_default_padding = BoxBuilder::new("Status with default padding")
        .with_status(StatusBuilder::new("Default (1)"))
        .with_fixed_width(40)
        .build();
    println!("{}", status_default_padding.render());
    println!();

    let status_extra_padding = BoxBuilder::new("Status with extra padding")
        .with_status(StatusBuilder::new("Extra padding").with_padding(3))
        .with_fixed_width(40)
        .build();
    println!("{}", status_extra_padding.render());
    println!();

    // Example 10: Real-world use case
    println!("10. Real-World Example: System Monitor");
    println!("---------------------------------------\n");

    let system_info = "CPU Usage: 45%\nMemory: 8.2 GB / 16 GB\nDisk: 234 GB / 500 GB\nNetwork: ↓ 12.3 MB/s ↑ 3.4 MB/s\nUptime: 5 days, 12 hours";

    let monitor = BoxBuilder::new(system_info)
        .with_header(HeaderBuilder::new("🖥️  System Monitor").align_center())
        .with_footer(FooterBuilder::new("Updated: 2s ago").align_right())
        .with_status(StatusBuilder::new("✓ All systems operational").align_center())
        .with_fixed_width(45)
        .with_h_padding(2)
        .build();

    println!("{}", monitor.render());
    println!();

    println!("✅ All features demonstrated successfully!");
}
