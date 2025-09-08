// Integration tests for boxy theme system and CLI functionality
// These tests verify end-to-end functionality rather than individual units

use std::process::Command;
use std::io::Write;

// Helper function to run boxy command and capture output
fn run_boxy(args: &[&str], input: Option<&str>) -> Result<std::process::Output, std::io::Error> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").args(args);
    
    if let Some(input_text) = input {
        cmd.stdin(std::process::Stdio::piped());
        let mut child = cmd.spawn()?;
        
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(input_text.as_bytes())?;
        }
        
        child.wait_with_output()
    } else {
        cmd.output()
    }
}

// Helper to check if output contains ANSI color codes
fn contains_ansi_colors(output: &str) -> bool {
    output.contains("\x1B[")
}

#[test]
fn test_basic_box_rendering() {
    let output = run_boxy(&[], Some("Hello World")).expect("Failed to run boxy");
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should contain box drawing characters
    assert!(stdout.contains("┌") || stdout.contains("┐") || stdout.contains("└") || stdout.contains("┘"));
    assert!(stdout.contains("Hello World"));
}

#[test]
fn test_theme_application() {
    let themes = ["error", "success", "warning", "info"];
    
    for theme in themes {
        let output = run_boxy(&["--theme", theme], Some("Test content"))
            .expect(&format!("Failed to run boxy with theme {}", theme));
        
        assert!(output.status.success(), "Theme {} should work", theme);
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(contains_ansi_colors(&stdout), "Theme {} should produce colored output", theme);
        assert!(stdout.contains("Test content"));
    }
}

#[test]
fn test_color_validation() {
    // Test valid color
    let output = run_boxy(&["--color", "azure"], Some("Test"))
        .expect("Failed to run boxy with valid color");
    assert!(output.status.success());
    
    // Test invalid color should fail
    let output = run_boxy(&["--color", "invalid_color"], Some("Test"))
        .expect("Failed to run boxy with invalid color");
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Color Error") || stderr.contains("Unknown color"));
}

#[test]
fn test_style_options() {
    let styles = ["normal", "rounded", "double", "heavy", "ascii"];
    
    for style in styles {
        let output = run_boxy(&["--style", style], Some("Test content"))
            .expect(&format!("Failed to run boxy with style {}", style));
        
        assert!(output.status.success(), "Style {} should work", style);
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Test content"));
    }
}

#[test]
fn test_header_and_title_distinction() {
    let output = run_boxy(&["--header", "My App", "--title", "✅ Status"], Some("Content"))
        .expect("Failed to run boxy with header and title");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("My App"));    // Header should appear
    assert!(stdout.contains("✅ Status"));  // Title should appear
    assert!(stdout.contains("Content"));   // Content should appear
}

#[test]
fn test_status_bar_alignment() {
    let alignments = [
        ("sl:Left status", "Left status"),
        ("sc:Center status", "Center status"),
        ("sr:Right status", "Right status"),
    ];
    
    for (status_arg, expected_text) in alignments {
        let output = run_boxy(&["--status", status_arg], Some("Content"))
            .expect(&format!("Failed to run boxy with status {}", status_arg));
        
        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(expected_text));
    }
}

#[test]
fn test_width_constraint() {
    let output = run_boxy(&["--width", "30"], Some("This is a very long line that should be truncated"))
        .expect("Failed to run boxy with width constraint");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should contain truncation indicator or have limited width
    assert!(stdout.contains("…") || stdout.lines().any(|line| line.len() <= 35));
}

#[test]
fn test_no_boxy_stripping() {
    // First create a box
    let output = run_boxy(&["--theme", "info"], Some("Test content"))
        .expect("Failed to create box");
    assert!(output.status.success());
    
    let boxed_content = String::from_utf8_lossy(&output.stdout);
    
    // Now strip the box
    let output = run_boxy(&["--no-boxy"], Some(&boxed_content))
        .expect("Failed to strip box");
    assert!(output.status.success());
    
    let stripped_content = String::from_utf8_lossy(&output.stdout);
    
    // Should not contain box drawing characters
    assert!(!stripped_content.contains("┌"));
    assert!(!stripped_content.contains("│"));
    assert!(stripped_content.contains("Test content"));
}

#[test]
fn test_theme_list_command() {
    let output = run_boxy(&["theme", "list"], None)
        .expect("Failed to run theme list");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("error"));
    assert!(stdout.contains("success"));
    assert!(stdout.contains("warning"));
    assert!(stdout.contains("info"));
}


#[test]
fn test_help_system() {
    let help_commands = [
        "--help",
        "--examples",
        "--colors",
        "--version",
        "theme help",
    ];
    
    for cmd in help_commands {
        let args: Vec<&str> = cmd.split_whitespace().collect();
        let output = run_boxy(&args, None)
            .expect(&format!("Failed to run help command: {}", cmd));
        
        assert!(output.status.success(), "Help command should succeed: {}", cmd);
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty(), "Help command should produce output: {}", cmd);
    }
}

#[test]
fn test_variable_expansion() {
    // Test environment variable expansion
    unsafe {
        std::env::set_var("TEST_VAR", "test_value");
    }
    
    let output = run_boxy(&["--header", "App: $TEST_VAR"], Some("Content"))
        .expect("Failed to run boxy with variable expansion");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test_value"));
}

#[test]
fn test_error_handling() {
    // Test various error conditions
    
    // Invalid width
    let output = run_boxy(&["--width", "invalid"], Some("Test"))
        .expect("Command should run even with invalid width");
    assert!(!output.status.success());
    
    // Missing theme argument
    let output = run_boxy(&["--theme"], Some("Test"))
        .expect("Command should run even with missing theme arg");
    assert!(!output.status.success());
    
    // Unknown theme
    let output = run_boxy(&["--theme", "nonexistent_theme"], Some("Test"))
        .expect("Command should run even with unknown theme");
    assert!(!output.status.success());
}

#[test]
fn test_complex_combination() {
    // Test combining multiple features
    let output = run_boxy(&[
        "--theme", "success",
        "--header", "Build Pipeline",
        "--title", "✅ Complete", 
        "--status", "sc:All tests passed",
        "--width", "60"
    ], Some("Build finished successfully\nAll 127 tests passed\nArtifacts uploaded"))
        .expect("Failed to run complex boxy command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Build Pipeline"));
    assert!(stdout.contains("✅ Complete"));
    assert!(stdout.contains("All tests passed"));
    assert!(stdout.contains("Build finished successfully"));
    assert!(contains_ansi_colors(&stdout));
}

#[test]
fn test_icon_integration() {
    // Test that icon integration works properly
    let output = run_boxy(&["--title", "📦 Package"], Some("Package contents"))
        .expect("Failed to run boxy with icon in title");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("📦 Package"));
    assert!(stdout.contains("Package contents"));
}

#[test]
fn test_text_color_options() {
    let text_colors = ["auto", "none", "white", "crimson"];
    
    for text_color in text_colors {
        let output = run_boxy(&["--text", text_color, "--color", "azure"], Some("Test"))
            .expect(&format!("Failed to run boxy with text color {}", text_color));
        
        assert!(output.status.success(), "Text color {} should work", text_color);
    }
}

#[test]
fn test_footer_functionality() {
    let output = run_boxy(&["--footer", "© 2024 Company"], Some("Content"))
        .expect("Failed to run boxy with footer");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("© 2024 Company"));
    assert!(stdout.contains("Content"));
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_theme_performance() {
        let start = Instant::now();
        
        // Run multiple theme operations
        for _ in 0..10 {
            let _output = run_boxy(&["--theme", "info"], Some("Performance test content"))
                .expect("Failed to run performance test");
        }
        
        let duration = start.elapsed();
        
        // Should complete reasonably quickly (adjust threshold as needed)
        assert!(duration.as_millis() < 5000, "Theme operations took too long: {:?}", duration);
    }
    
    #[test]
    fn test_color_system_performance() {
        let start = Instant::now();
        
        let colors = ["crimson", "emerald", "azure", "amber", "violet"];
        for color in colors {
            let _output = run_boxy(&["--color", color], Some("Color test"))
                .expect("Failed to run color performance test");
        }
        
        let duration = start.elapsed();
        
        // Color operations should be fast
        assert!(duration.as_millis() < 3000, "Color operations took too long: {:?}", duration);
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;
    
    #[test]
    fn test_icon_spacing_regression() {
        // Ensure icon spacing doesn't regress
        let output = run_boxy(&["--title", "✅ Success"], Some("Operation complete"))
            .expect("Failed to test icon spacing");
        
        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Icon and text should be properly spaced
        assert!(stdout.contains("✅ Success"));
        // Should not have double spaces or layout issues
        assert!(!stdout.contains("✅  Success")); // Double space would be a regression
    }
    
    #[test]
    fn test_width_truncation_regression() {
        // Ensure width truncation works correctly
        let long_content = "This is an extremely long line of content that should definitely be truncated when a width limit is applied to ensure consistent formatting";
        
        let output = run_boxy(&["--width", "50"], Some(long_content))
            .expect("Failed to test width truncation");
        
        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should contain ellipsis or be truncated
        assert!(stdout.contains("…") || !stdout.contains("consistent formatting"));
    }
    
    #[test]
    fn test_theme_compatibility_regression() {
        // Ensure all built-in themes remain compatible
        let themes = ["error", "success", "warning", "info"];
        
        for theme in themes {
            let output = run_boxy(&["--theme", theme], Some("Compatibility test"))
                .expect(&format!("Failed to test theme compatibility for {}", theme));
            
            assert!(output.status.success(), "Theme {} compatibility broken", theme);
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(contains_ansi_colors(&stdout), "Theme {} should produce colors", theme);
            assert!(stdout.contains("Compatibility test"));
        }
    }
}
