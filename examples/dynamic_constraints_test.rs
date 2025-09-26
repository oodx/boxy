use boxy::api::layout::BoxBuilder;

fn main() {
    println!("=== Dynamic Constraint & Visibility Demo ===\n");

    // Test 1: Min width constraint
    println!("1. Min Width (min=40, content only needs 15):");
    let min_width_box = BoxBuilder::new("Short")
        .with_min_width(40)
        .build();
    println!("{}\n", min_width_box.render());

    // Test 2: Max width constraint
    println!("2. Max Width (max=25, content needs more):");
    let max_width_box = BoxBuilder::new("This is a very long line that will be constrained by max width")
        .with_max_width(25)
        .with_wrapping(false)  // Show truncation
        .build();
    println!("{}\n", max_width_box.render());

    // Test 3: Min height constraint
    println!("3. Min Height (min=12, content only needs 5):");
    let min_height_box = BoxBuilder::new("Line 1\nLine 2\nLine 3")
        .with_fixed_width(30)
        .with_min_height(12)
        .build();
    let lines = min_height_box.render().lines().count();
    println!("{}", min_height_box.render());
    println!("Actual lines: {}\n", lines);

    // Test 4: Max height constraint
    println!("4. Max Height (max=8, content has 20 lines):");
    let many_lines = "Line\n".repeat(20);
    let max_height_box = BoxBuilder::new(&many_lines)
        .with_fixed_width(25)
        .with_max_height(8)
        .build();
    println!("{}\n", max_height_box.render());

    // Test 5: Combined min/max (dynamic range)
    println!("5. Dynamic Range (min=30, max=50, content adaptive):");
    let dynamic_box = BoxBuilder::new("Adaptive content")
        .with_min_width(30)
        .with_max_width(50)
        .with_min_height(8)
        .with_max_height(15)
        .build();
    println!("{}\n", dynamic_box.render());

    // Test 6: Visibility control
    println!("6. Visibility Control:");

    let visible_box = BoxBuilder::new("This is visible")
        .with_fixed_width(30)
        .with_visibility(true)
        .build();
    println!("Visible box:\n{}\n", visible_box.render());

    let hidden_box = BoxBuilder::new("This is hidden")
        .with_fixed_width(30)
        .hide()
        .build();
    let hidden_output = hidden_box.render();
    println!("Hidden box output: '{}' (empty string)", hidden_output);
    println!("Hidden box is_empty: {}\n", hidden_output.is_empty());

    // Test 7: Grid cell simulation
    println!("7. Grid Cell Simulation (50x12 cell):");
    let grid_cell = BoxBuilder::new("Grid cell content\nthat adapts to\nthe allocated space")
        .with_fixed_width(50)
        .with_fixed_height(12)
        .build();
    println!("{}\n", grid_cell.render());

    println!("=== All Tests Complete ===");
}