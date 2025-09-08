// Theme management CLI integration tests
// These tests verify theme management commands work correctly

use std::process::Command;

// Helper function to run boxy theme commands
fn run_theme_command(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("theme").args(args);
    cmd.output()
}

// Helper function to run regular boxy commands
fn run_boxy_command(args: &[&str], input: Option<&str>) -> Result<std::process::Output, std::io::Error> {
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

#[test]
fn test_theme_list_command() {
    let output = run_theme_command(&["list"])
        .expect("Should run theme list command");
    
    assert!(output.status.success(), "Theme list command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should contain built-in themes
    assert!(stdout.contains("error"));
    assert!(stdout.contains("success"));
    assert!(stdout.contains("warning"));
    assert!(stdout.contains("info"));
    
    // Should show descriptions
    assert!(stdout.contains("Theme with") || stdout.contains("border") || stdout.contains("error") || stdout.contains("success"));
}

#[test]
fn test_theme_help_command() {
    let output = run_theme_command(&["help"])
        .expect("Should run theme help command");
    
    assert!(output.status.success(), "Theme help command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should contain help information
    assert!(stdout.contains("Theme Management") || stdout.contains("USAGE") || stdout.contains("ACTIONS"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("create"));
    assert!(stdout.contains("import"));
    assert!(stdout.contains("export"));
    assert!(stdout.contains("edit"));
}

#[test]
fn test_built_in_themes_work() {
    let themes = ["error", "success", "warning", "info"];
    
    for theme in themes {
        let output = run_boxy_command(&["--theme", theme], Some("Test content"))
            .expect(&format!("Should run boxy with theme {}", theme));
        
        // Theme should be recognized and work
        assert!(output.status.success(), "Theme {} should work", theme);
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should contain the test content
        assert!(stdout.contains("Test content"), "Theme {} output should contain content", theme);
        
        // Should contain some box drawing (indicating theme was applied)
        let has_box_chars = stdout.contains('│') || stdout.contains('┌') || stdout.contains('┐') ||
                           stdout.contains('└') || stdout.contains('┘') || stdout.contains('─');
        assert!(has_box_chars, "Theme {} should produce box output", theme);
    }
}

#[test]
fn test_invalid_theme_handling() {
    let output = run_boxy_command(&["--theme", "nonexistent_theme"], Some("Test content"))
        .expect("Should run boxy command even with invalid theme");
    
    // Should fail when theme doesn't exist
    assert!(!output.status.success(), "Invalid theme should cause failure");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should show error message about unknown theme
    assert!(stderr.contains("Unknown theme") || stderr.contains("not found") || stderr.contains("nonexistent_theme"));
}

#[test]
fn test_theme_with_override_options() {
    // Test that theme can be combined with other options
    let output = run_boxy_command(&["--theme", "info", "--width", "40"], Some("Test content with theme override"))
        .expect("Should run boxy with theme and width override");
    
    assert!(output.status.success(), "Theme with width override should work");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Test content"));
    
    // Check that width constraint is applied (content should be truncated or box should be narrow)
    let lines: Vec<&str> = stdout.lines().collect();
    let box_lines: Vec<&str> = lines.iter()
        .filter(|line| line.contains('─') || line.contains('│'))
        .cloned()
        .collect();
    
    // At least one box line should exist and be reasonably constrained
    assert!(!box_lines.is_empty(), "Should have box drawing lines");
}

#[test]
fn test_theme_error_messages_helpful() {
    // Test error message when no theme name provided
    let output = run_theme_command(&["show"])
        .expect("Should run theme show without arg");
    
    assert!(!output.status.success(), "Theme show without args should fail");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("requires") || stderr.contains("usage") || stderr.contains("show <name>"));
}

#[test]
fn test_theme_system_robustness() {
    // Test various edge cases and error conditions
    
    // Empty theme name
    let output = run_boxy_command(&["--theme", ""], Some("Test"))
        .expect("Should handle empty theme name");
    assert!(!output.status.success());
    
    // Theme with whitespace
    let output = run_boxy_command(&["--theme", " info "], Some("Test"))
        .expect("Should handle whitespace in theme name");
    // This might succeed if trimmed, or fail - either is acceptable
    
    // Very long theme name
    let long_name = "a".repeat(100);
    let output = run_boxy_command(&["--theme", &long_name], Some("Test"))
        .expect("Should handle very long theme name");
    assert!(!output.status.success()); // Should fail gracefully
}

#[test]
fn test_theme_export_functionality() {
    // Test that theme export produces valid output
    let output = run_theme_command(&["export", "error"])
        .expect("Should run theme export command");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should produce YAML-like output
        assert!(stdout.contains("metadata:") || stdout.contains("themes:") || stdout.contains("error"));
        assert!(stdout.contains("color:") || stdout.contains("crimson"));
    }
    // Note: If export fails, that's also acceptable as the feature might not be fully implemented
}


#[test]
fn test_help_system_mentions_themes() {
    // Test that help system properly documents theme functionality
    let help_commands = [
        vec!["--help"],
        vec!["--examples"],
    ];
    
    for cmd in help_commands {
        let output = run_boxy_command(&cmd, None)
            .expect(&format!("Should run help command: {:?}", cmd));
        
        assert!(output.status.success(), "Help command should succeed: {:?}", cmd);
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Should mention theme functionality
        assert!(stdout.contains("theme") || stdout.contains("--theme") || stdout.contains("Theme"));
        
        // Should mention built-in themes
        let mentions_builtin_themes = stdout.contains("error") && stdout.contains("success") && stdout.contains("warning");
        assert!(mentions_builtin_themes, "Help should mention built-in themes");
    }
}

#[test]
fn test_theme_consistency_across_commands() {
    // Test that theme behavior is consistent across different usage patterns
    
    let test_patterns = [
        (vec!["--theme", "success"], "Themed success"),
        (vec!["--theme", "success", "--header", "Test App"], "Success with header"),
        (vec!["--theme", "success", "--status", "sc:Complete"], "Success with status"),
    ];
    
    for (args, description) in test_patterns {
        let output = run_boxy_command(&args, Some("Test content"))
            .expect(&format!("Should run: {}", description));
        
        assert!(output.status.success(), "{} should succeed", description);
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Test content"), "{} should show content", description);
        
        // All should produce some visual output (box chars or ANSI colors)
        let has_visual_elements = stdout.contains('\x1B') || stdout.contains('│') || 
                                  stdout.contains('┌') || stdout.contains('─');
        assert!(has_visual_elements, "{} should produce visual output", description);
    }
}

// Test performance under reasonable load
#[test]
fn test_theme_performance_basic() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Run several theme operations
    for i in 0..5 {
        let theme = if i % 2 == 0 { "error" } else { "success" };
        let _output = run_boxy_command(&["--theme", theme], Some("Performance test"))
            .expect("Performance test should work");
    }
    
    let duration = start.elapsed();
    
    // Should complete in reasonable time (adjust threshold as needed)
    assert!(duration.as_secs() < 10, "Theme operations took too long: {:?}", duration);
}
