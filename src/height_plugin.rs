
use crate::{File, Command, Stdio};

/// Validate height input
pub fn validate_height(height_str: &str) -> Result<(), String> {
    match height_str.parse::<usize>() {
        Ok(h) if h >= 5 && h <= 50 => Ok(()),
        Ok(h) => Err(format!("Height {} out of range (5-50)", h)),
        Err(_) => Err("Height must be a number".to_string()),
    }
}

/// Height diagnostics subcommand
pub fn handle_height_command() {
    // Helper to run command with /dev/tty as stdin when available
    fn run_with_tty(mut cmd: Command) -> Option<String> {
        if let Ok(tty) = File::open("/dev/tty") {
            let _ = cmd.stdin(Stdio::from(tty));
        }
        cmd.output().ok().and_then(|o| String::from_utf8(o.stdout).ok())
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
            if parts.len() == 2 { parts[0].parse::<usize>().ok() } else { None }
        })
    };

    let effective = get_terminal_height();

    println!("Height diagnostics:");
    println!("  effective (get_terminal_height): {}", effective);
    println!("  tput lines (tty): {}", tput_lines_tty.map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()));
    println!("  stty size rows (tty): {}", stty_rows_tty.map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()));
}

/// Get terminal height with fallback to 24 lines
pub fn get_terminal_height() -> usize {
    // Helper to run with /dev/tty
    fn run_with_tty(mut cmd: Command) -> Option<String> {
        if let Ok(tty) = File::open("/dev/tty") {
            let _ = cmd.stdin(Stdio::from(tty));
        }
        cmd.output().ok().and_then(|o| String::from_utf8(o.stdout).ok())
    }

    // Try tput lines with tty
    {
        let mut c = Command::new("tput");
        c.arg("lines");
        if let Some(out) = run_with_tty(c) {
            if let Ok(height) = out.trim().parse::<usize>() {
                if height >= 5 { return height; }
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
                    if height >= 5 { return height; }
                }
            }
        }
    }

    // Try environment variables
    if let Ok(env_lines) = std::env::var("LINES") {
        if let Ok(height) = env_lines.parse::<usize>() {
            if height >= 5 { return height; }
        }
    }

    // Fallback to 24 lines (standard terminal height)
    24
}

/// Calculate height of content lines (helper for layout engines)
pub fn calculate_content_height(lines: &[String]) -> usize {
    lines.len()
}

/// Get maximum safe height for current terminal
pub fn get_max_safe_height() -> usize {
    let terminal_height = get_terminal_height();
    // Leave some space for shell prompt and other terminal UI
    if terminal_height >= 10 {
        terminal_height - 2
    } else {
        terminal_height
    }
}