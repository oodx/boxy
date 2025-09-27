use boxy::api::layout::{BoxBuilder, HeaderBuilder};
use boxy::height_plugin::get_terminal_height;

fn main() {
    println!("Height Constraint Examples");
    println!("==========================\n");

    let many_lines = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12\nLine 13\nLine 14\nLine 15\nLine 16\nLine 17\nLine 18\nLine 19\nLine 20";

    println!("1. Without Height Constraint");
    println!("-----------------------------\n");

    let no_constraint = BoxBuilder::new(many_lines)
        .with_header(HeaderBuilder::new("All Lines Visible"))
        .with_fixed_width(30)
        .build();

    println!("{}", no_constraint.render());
    println!();

    println!("2. With Height Constraint (10 lines)");
    println!("-------------------------------------\n");

    let constrained = BoxBuilder::new(many_lines)
        .with_header(HeaderBuilder::new("Truncated"))
        .with_fixed_width(30)
        .with_fixed_height(10)
        .build();

    println!("{}", constrained.render());
    println!();

    println!("3. Height Constraints at Different Values");
    println!("------------------------------------------\n");

    for height in &[5, 8, 12] {
        let box_layout = BoxBuilder::new(many_lines)
            .with_header(HeaderBuilder::new(&format!("Height: {}", height)))
            .with_fixed_width(30)
            .with_fixed_height(*height)
            .build();

        println!("{}", box_layout.render());
        println!();
    }

    println!("4. Terminal-Height Responsive");
    println!("------------------------------\n");

    let term_height = get_terminal_height();
    let box_height = (term_height - 5).max(10);

    let responsive = BoxBuilder::new(many_lines)
        .with_header(HeaderBuilder::new(&format!(
            "Terminal: {} → Box: {}",
            term_height, box_height
        )))
        .with_fixed_width(40)
        .with_fixed_height(box_height)
        .build();

    println!("{}", responsive.render());
    println!();

    println!("5. 2D Responsive Layout (Width + Height)");
    println!("-----------------------------------------\n");

    let long_text = "This is a very long piece of text that demonstrates both text wrapping for width constraints and truncation for height constraints. When both are enabled, text that exceeds the box width will automatically wrap to the next line at word boundaries, and if the total height exceeds the maximum, the content will be truncated with an ellipsis indicator showing how many lines were hidden.";

    let term_width = boxy::width_plugin::get_terminal_width();
    let box_width = (term_width - 20).max(40);

    let two_d_responsive = BoxBuilder::new(long_text)
        .with_header(HeaderBuilder::new("2D Responsive"))
        .with_fixed_width(box_width)
        .with_fixed_height(15)
        .with_wrapping(true)
        .build();

    println!("{}", two_d_responsive.render());
    println!();

    println!("6. Side-by-Side Comparison");
    println!("--------------------------\n");

    let sample = "A\nB\nC\nD\nE\nF\nG\nH\nI\nJ";

    let unconstrained = BoxBuilder::new(sample)
        .with_header(HeaderBuilder::new("No Limit"))
        .with_fixed_width(15)
        .build();

    let constrained_small = BoxBuilder::new(sample)
        .with_header(HeaderBuilder::new("Height: 5"))
        .with_fixed_width(15)
        .with_fixed_height(5)
        .build();

    let unconstrained_render = unconstrained.render();
    let constrained_render = constrained_small.render();
    let unconstrained_lines: Vec<_> = unconstrained_render.lines().collect();
    let constrained_lines: Vec<_> = constrained_render.lines().collect();

    let max_lines = unconstrained_lines.len().max(constrained_lines.len());
    for i in 0..max_lines {
        let line1 = unconstrained_lines.get(i).unwrap_or(&"");
        let line2 = constrained_lines.get(i).unwrap_or(&"");
        println!("{}  {}", line1, line2);
    }
    println!();

    println!("💡 Tip: Resize your terminal and run this again to see");
    println!("   how Example 4 adapts to different screen heights!");
}
