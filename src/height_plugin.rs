/// Height plugin for terminal height detection and management
///
/// This module provides terminal height detection using multiple methods,
/// similar to the width_plugin.rs patterns. It supports terminal multiplexers,
/// TUI frameworks, and layout engines requiring predictable vertical spacing.
use crate::{Command, File, Stdio};

/// Validate height input string
///
/// # Arguments
/// * `height_str` - String representation of desired height
///
/// # Returns
/// * `Ok(())` if height is valid (5-50 lines range)
/// * `Err(String)` with descriptive error message if invalid
///
/// # Examples
/// ```
/// use boxy::height_plugin::validate_height;
/// assert!(validate_height("20").is_ok());
/// assert!(validate_height("3").is_err());  // Too small
/// assert!(validate_height("100").is_err()); // Too large
/// ```
pub fn validate_height(height_str: &str) -> Result<(), String> {
    match height_str.parse::<usize>() {
        Ok(h) if h >= 5 && h <= 50 => Ok(()),
        Ok(h) => Err(format!("Height {} out of range (5-50)", h)),
        Err(_) => Err("Height must be a number".to_string()),
    }
}

/// Handle the height diagnostics subcommand
///
/// Displays comprehensive terminal height detection information using
/// multiple detection methods (tput, stty, environment variables).
///
/// # Output
/// Prints diagnostic information showing:
/// - Effective height from get_terminal_height()
/// - tput lines result (if available)
/// - stty size rows result (if available)
///
/// # Usage
/// ```bash
/// boxy height
/// ```
pub fn handle_height_command() {
    // Helper to run command with /dev/tty as stdin when available
    fn run_with_tty(mut cmd: Command) -> Option<String> {
        if let Ok(tty) = File::open("/dev/tty") {
            let _ = cmd.stdin(Stdio::from(tty));
        }
        cmd.output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
    }

    // Gather tput lines (tty)
    let tput_lines_tty = {
        let mut c = Command::new("tput");
        c.arg("lines");
        run_with_tty(c).and_then(|s| s.trim().parse::<usize>().ok())
    };

    // Gather stty size (rows cols) via tty - extract rows (first value)
    let stty_rows_tty = {
        let mut c = Command::new("stty");
        c.arg("size");
        run_with_tty(c).and_then(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() == 2 {
                parts[0].parse::<usize>().ok()
            } else {
                None
            }
        })
    };

    let effective = get_terminal_height();

    println!("Height diagnostics:");
    println!("  effective (get_terminal_height): {}", effective);
    println!(
        "  tput lines (tty): {}",
        tput_lines_tty
            .map(|v| v.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "  stty size rows (tty): {}",
        stty_rows_tty
            .map(|v| v.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    );
}

/// Get terminal height with automatic fallback
///
/// Attempts multiple methods to detect terminal height:
/// 1. `tput lines` command via /dev/tty
/// 2. `stty size` command (extracts rows from "rows cols" output)
/// 3. `LINES` environment variable
/// 4. Fallback to 24 lines (standard terminal height)
///
/// # Returns
/// Terminal height in lines, guaranteed to be at least 5
///
/// # Implementation Note
/// Follows the same pattern as get_terminal_width() in width_plugin.rs
/// for consistency across the codebase.
pub fn get_terminal_height() -> usize {
    // Helper to run with /dev/tty
    fn run_with_tty(mut cmd: Command) -> Option<String> {
        if let Ok(tty) = File::open("/dev/tty") {
            let _ = cmd.stdin(Stdio::from(tty));
        }
        cmd.output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
    }

    // Try tput lines with tty
    {
        let mut c = Command::new("tput");
        c.arg("lines");
        if let Some(out) = run_with_tty(c) {
            if let Ok(height) = out.trim().parse::<usize>() {
                if height >= 5 {
                    return height;
                }
            }
        }
    }

    // Try stty size with tty (rows cols - extract rows)
    {
        let mut c = Command::new("stty");
        c.arg("size");
        if let Some(out) = run_with_tty(c) {
            let parts: Vec<&str> = out.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(height) = parts[0].parse::<usize>() {
                    if height >= 5 {
                        return height;
                    }
                }
            }
        }
    }

    // Try environment variables
    if let Ok(env_lines) = std::env::var("LINES") {
        if let Ok(height) = env_lines.parse::<usize>() {
            if height >= 5 {
                return height;
            }
        }
    }

    // Fallback to 24 lines (standard terminal height)
    24
}

/// Calculate the total height of content lines
///
/// Simple helper function for layout engines to determine
/// the number of lines in content.
///
/// # Arguments
/// * `lines` - Slice of strings representing content lines
///
/// # Returns
/// Number of lines in the content
///
/// # Example
/// ```
/// let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
/// use boxy::height_plugin::calculate_content_height;
/// let lines = vec![String::from("line"), String::new()];
/// assert_eq!(calculate_content_height(&lines), 2);
/// ```
#[allow(dead_code)] // Height pipeline integration still in progress; see docs/HEIGHT_FEATURE.md.
pub fn calculate_content_height(lines: &[String]) -> usize {
    lines.len()
}

/// Get maximum safe height for current terminal
///
/// Calculates the usable terminal height, leaving space for
/// shell prompt and other terminal UI elements.
///
/// # Returns
/// Safe height value (terminal_height - 2) or full height if terminal is very small
///
/// # Usage Note
/// Use this when you want to maximize box height while ensuring
/// the shell prompt remains visible after box rendering.
pub fn get_max_safe_height() -> usize {
    let terminal_height = get_terminal_height();
    // Leave some space for shell prompt and other terminal UI
    if terminal_height >= 10 {
        terminal_height - 2
    } else {
        terminal_height
    }
}
