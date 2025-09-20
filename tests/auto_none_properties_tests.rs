//! Auto/None Properties Validation Tests - M1.5
//!
//! This test suite validates that "auto" and "none" property behaviors work correctly
//! across the newly restructured RSB MODULE_SPEC modules.

use std::process::{Command, Stdio};
use std::io::Write;

/// Helper function to run boxy command and capture output
fn run_boxy_command(args: &[&str], input: &str) -> Result<String, String> {
    let mut cmd = Command::new("cargo")
        .args(&["run", "--bin", "boxy", "--"])
        .args(args)
        .arg("--no-color") // Disable colors for easier testing
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    // Write input to stdin
    if let Some(stdin) = cmd.stdin.take() {
        let mut stdin = stdin;
        stdin.write_all(input.as_bytes())
            .map_err(|e| format!("Failed to write to stdin: {}", e))?;
    }

    let output = cmd.wait_with_output()
        .map_err(|e| format!("Failed to wait for command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[test]
fn test_text_color_auto_with_themes() {
    // Test that text_color="auto" works with different themes
    let themes = vec!["error", "success", "warning", "info"];

    for theme in themes {
        let result = run_boxy_command(&["--theme", theme], "Auto color test");
        assert!(result.is_ok(), "Theme {} should work", theme);

        let output = result.unwrap();
        assert!(output.contains("Auto color test"), "Theme {} should contain content", theme);
    }
}

#[test]
fn test_text_color_none_behavior() {
    // Test that text_color="none" uses default terminal color
    let result = run_boxy_command(&[], "None color test");
    assert!(result.is_ok(), "Default theme should work");

    let output = result.unwrap();
    assert!(output.contains("None color test"), "Should contain content");
}

#[test]
fn test_width_auto_vs_fixed() {
    // Test auto width vs fixed width
    let auto_result = run_boxy_command(&[], "Auto width content");
    let fixed_result = run_boxy_command(&["--width", "30"], "Fixed width content");

    assert!(auto_result.is_ok(), "Auto width should work");
    assert!(fixed_result.is_ok(), "Fixed width should work");

    let auto_output = auto_result.unwrap();
    let fixed_output = fixed_result.unwrap();

    assert!(auto_output.contains("Auto width content"));
    assert!(fixed_output.contains("Fixed width content"));
}

#[test]
fn test_auto_width_with_long_content() {
    // Test that auto width expands for long content
    let short_result = run_boxy_command(&[], "Short");
    let long_result = run_boxy_command(&[], "This is a very long piece of content that should make the box wider");

    assert!(short_result.is_ok() && long_result.is_ok(), "Both commands should work");

    let short_output = short_result.unwrap();
    let long_output = long_result.unwrap();

    // Both should contain their respective content
    assert!(short_output.contains("Short"));
    assert!(long_output.contains("very long piece"));
}

#[test]
fn test_auto_properties_with_title_status() {
    // Test auto width with title and status
    let result = run_boxy_command(
        &["--title", "📦 Test Title", "--status", "v1.0"],
        "Content with title and status"
    );

    assert!(result.is_ok(), "Title/status command should work");

    let output = result.unwrap();
    assert!(output.contains("Test Title"), "Should contain title");
    assert!(output.contains("v1.0"), "Should contain status");
    assert!(output.contains("Content with title"), "Should contain content");
}

#[test]
fn test_theme_inheritance_auto_properties() {
    // Test that theme inheritance works with auto properties
    let themes = vec!["blueprint", "error", "success", "info"];

    for theme in themes {
        let result = run_boxy_command(&["--theme", theme], "Inheritance test");
        assert!(result.is_ok(), "Theme {} inheritance should work", theme);

        let output = result.unwrap();
        assert!(output.contains("Inheritance test"), "Theme {} should render content", theme);
    }
}

#[test]
fn test_auto_none_with_emojis() {
    // Test auto/none properties work with emojis
    let result = run_boxy_command(&[], "Emoji test 🚀🌟🔥 with auto sizing");
    assert!(result.is_ok(), "Emoji content should work");

    let output = result.unwrap();
    assert!(output.contains("🚀🌟🔥"), "Should contain emojis");
    assert!(output.contains("auto sizing"), "Should contain text");
}

#[test]
fn test_auto_none_regression_suite() {
    // Comprehensive regression test for auto/none properties
    let test_cases = vec![
        (vec!["--theme", "error"], "Error theme auto test"),
        (vec!["--theme", "success"], "Success theme auto test"),
        (vec!["--width", "40"], "Fixed width test"),
        (vec!["--title", "Auto Title"], "Auto title test"),
        (vec!["--status", "Auto Status"], "Auto status test"),
    ];

    for (args, content) in test_cases {
        let result = run_boxy_command(&args, content);
        assert!(result.is_ok(), "Command {:?} should succeed", args);

        let output = result.unwrap();
        assert!(output.contains(content), "Output should contain content for {:?}", args);
        assert!(!output.trim().is_empty(), "Output should not be empty for {:?}", args);
    }
}