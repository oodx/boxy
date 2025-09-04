//
//v7
use unicode_width::UnicodeWidthStr;
use std::io::{self, Read, Write};
use std::env;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use regex::Regex;

mod colors;
mod theme_engine;
use colors::*;
use theme_engine::{ThemeEngine, BoxyTheme, ThemeFile, ThemeMetadata, ThemeSettings};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

struct BoxStyle {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
}

const NORMAL: BoxStyle = BoxStyle {
    top_left: "┌", top_right: "┐",
    bottom_left: "└", bottom_right: "┘",
    horizontal: "─", vertical: "│",
};

const ROUNDED: BoxStyle = BoxStyle {
    top_left: "╭", top_right: "╮",
    bottom_left: "╰", bottom_right: "╯",
    horizontal: "─", vertical: "│",
};

const DOUBLE: BoxStyle = BoxStyle {
    top_left: "╔", top_right: "╗",
    bottom_left: "╚", bottom_right: "╝",
    horizontal: "═", vertical: "║",
};

const HEAVY: BoxStyle = BoxStyle {
    top_left: "┏", top_right: "┓",
    bottom_left: "┗", bottom_right: "┛",
    horizontal: "━", vertical: "┃",
};

const ASCII: BoxStyle = BoxStyle {
    top_left: "+", top_right: "+",
    bottom_left: "+", bottom_right: "+",
    horizontal: "-", vertical: "|",
};


// Color function moved to colors.rs module - using shared implementation

const RESET: &str = "\x1B[0m";

// ================ JYNX INTEGRATION SYSTEM ================
// Enhanced output formatting with jynx integration for beautiful CLI experience
// Falls back gracefully when jynx is not available

/// Jynx availability detection with version checking
struct JynxIntegration {
    available: bool,
    version: Option<String>,
    supports_templates: bool,
    no_color_requested: bool,
}

impl JynxIntegration {
    /// Get formatted jynx version for display
    fn get_version_string(&self) -> String {
        match &self.version {
            Some(version) => format!("with jynx {}", version),
            None => "jynx not detected".to_string(),
        }
    }
}

impl JynxIntegration {
    /// Initialize jynx integration with comprehensive detection
    fn new(no_color: bool) -> Self {
        let mut integration = JynxIntegration {
            available: false,
            version: None,
            supports_templates: false,
            no_color_requested: no_color,
        };
        
        // Skip jynx detection if --no-color is explicitly requested
        if no_color {
            return integration;
        }
        
        // Check if jynx is available in PATH
        if let Ok(output) = Command::new("jynx").arg("--version").output() {
            if output.status.success() {
                if let Ok(version_output) = String::from_utf8(output.stdout) {
                    integration.available = true;
                    integration.version = Some(version_output.trim().to_string());
                    
                    // Check for template support (jynx 0.3.0+)
                    integration.supports_templates = version_output.contains("0.3") || 
                        version_output.contains("0.4") || 
                        version_output.contains("0.5") ||
                        version_output.contains("1.") ||
                        version_output.contains("2.");
                }
            }
        }
        
        integration
    }
    
    /// Check if jynx is available and color is enabled
    fn is_active(&self) -> bool {
        self.available && !self.no_color_requested
    }
}

/// Enhanced output with jynx integration
fn pipe_to_jynx(content: &str, template: &str, jynx: &JynxIntegration) -> String {
    // Return original content if jynx integration is not active
    if !jynx.is_active() {
        return content.to_string();
    }
    
    // Use jynx for enhanced formatting
    let mut cmd = Command::new("jynx");
    
    // Configure jynx command based on template type
    match template {
        "help" => {
            cmd.args(&["--template", "help", "--style", "enhanced"]);
        }
        "list" => {
            cmd.args(&["--template", "list", "--bullets", "→"]);
        }
        "success" => {
            cmd.args(&["--template", "success", "--icon", "✅"]);
        }
        "error" => {
            cmd.args(&["--template", "error", "--icon", "❌"]);
        }
        "warning" => {
            cmd.args(&["--template", "warning", "--icon", "⚠️"]);
        }
        "info" => {
            cmd.args(&["--template", "info", "--icon", "ℹ️"]);
        }
        "migration" => {
            cmd.args(&["--template", "guide", "--style", "migration"]);
        }
        "theme_list" => {
            cmd.args(&["--template", "themes", "--format", "compact"]);
        }
        _ => {
            // Generic enhancement
            cmd.args(&["--enhance"]);
        }
    }
    
    // Execute jynx with content as stdin
    match cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            // Send content to jynx stdin
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(content.as_bytes());
            }
            
            // Get jynx output
            match child.wait_with_output() {
                Ok(output) => {
                    if output.status.success() {
                        String::from_utf8_lossy(&output.stdout).to_string()
                    } else {
                        // Fallback to original content if jynx fails
                        content.to_string()
                    }
                }
                Err(_) => content.to_string(),
            }
        }
        Err(_) => content.to_string(),
    }
}

/// Enhanced print function with jynx integration
fn jynx_println(content: &str, template: &str, jynx: &JynxIntegration) {
    let enhanced_content = pipe_to_jynx(content, template, jynx);
    print!("{}", enhanced_content);
}

/// Get terminal width with fallback to 80 columns
fn get_terminal_width() -> usize {
    // Try tput cols first
    if let Ok(output) = Command::new("tput").arg("cols").output() {
        if let Ok(width_str) = String::from_utf8(output.stdout) {
            if let Ok(width) = width_str.trim().parse::<usize>() {
                return width;
            }
        }
    }
    
    // Try COLUMNS environment variable
    if let Ok(cols) = env::var("COLUMNS") {
        if let Ok(width) = cols.parse::<usize>() {
            return width;
        }
    }
    
    // Fallback to 80 columns
    80
}

fn get_display_width(text: &str) -> usize {
    let clean = strip_ansi_escapes::strip(text);
    let clean_str = String::from_utf8_lossy(&clean);
    UnicodeWidthStr::width(clean_str.as_ref())
}

fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    
    let text_width = get_display_width(text);
    if text_width <= max_width {
        return text.to_string();
    }
    
    // Unicode ellipsis character
    const ELLIPSIS: &str = "…";
    const ELLIPSIS_WIDTH: usize = 1;
    
    if max_width <= ELLIPSIS_WIDTH {
        return ELLIPSIS.to_string();
    }
    
    let target_width = max_width - ELLIPSIS_WIDTH;
    let mut result = String::new();
    let mut current_width = 0;
    
    for ch in text.chars() {
        let ch_width = UnicodeWidthStr::width(ch.to_string().as_str());
        if current_width + ch_width > target_width {
            break;
        }
        result.push(ch);
        current_width += ch_width;
    }
    
    result.push_str(ELLIPSIS);
    result
}

fn render_title_or_footer(text: &str, total_width: usize, style_char: &str) -> String {
    if total_width < 4 {
        // Minimum viable box: just return style chars
        return style_char.repeat(total_width);
    }
    
    // Enhanced title processing: auto-detect and format icons in titles
    let processed_text = if text.contains(" ") {
        // If title contains spaces, check for icon patterns
        let parts: Vec<&str> = text.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let potential_icon = parts[0];
            let title_text = parts[1];
            
            // Check if first part looks like an icon/emoji (non-ASCII characters)
            if potential_icon.chars().any(|c| !c.is_ascii()) {
                format!("{} {}", potential_icon, title_text)
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        }
    } else {
        text.to_string()
    };
    
    let text_width = get_display_width(&processed_text);
    let available_width = total_width.saturating_sub(2); // Space for " text "
    
    let final_text = if text_width > available_width {
        truncate_with_ellipsis(&processed_text, available_width)
    } else {
        processed_text
    };
    
    let final_text_width = get_display_width(&final_text);
    // CRITICAL FIX: Use saturating_sub to prevent underflow
    let remaining_width = total_width.saturating_sub(final_text_width + 2); // -2 for spaces around text
    let left_pad = remaining_width / 2;
    let right_pad = remaining_width.saturating_sub(left_pad);
    
    format!("{} {} {}", 
        style_char.repeat(left_pad), 
        final_text, 
        style_char.repeat(right_pad))
}

fn expand_variables(text: &str) -> String {
    let mut result = text.to_string();
    let var_regex = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    
    for cap in var_regex.captures_iter(text) {
        if let Some(var_name) = cap.get(1) {
            if let Ok(value) = env::var(var_name.as_str()) {
                result = result.replace(&cap[0], &value);
            }
        }
    }
    result
}

fn strip_box(text: &str, strict: bool) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut content_lines = Vec::new();
    
    // Box drawing characters to detect
    let box_chars = "┌┐└┘─│╭╮╰╯═║╔╗╚╝━┃┏┓┗┛+-|";
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Skip lines that look like box borders
        if i == 0 || i == lines.len() - 1 {
            if trimmed.chars().all(|c| box_chars.contains(c) || c.is_whitespace()) {
                continue;
            }
        }
        
        // Process content lines
        let mut content = line.to_string();
        
        // Remove box sides
        if content.len() > 2 {
            let chars: Vec<char> = content.chars().collect();
            if box_chars.contains(chars[0]) {
                content = content.chars().skip(1).collect();
            }
            let chars: Vec<char> = content.chars().collect();
            if !chars.is_empty() && box_chars.contains(chars[chars.len() - 1]) {
                content = content.chars().take(content.len() - 1).collect();
            }
        }
        
        // Trim padding
        content = content.trim().to_string();
        
        if strict {
            // Strip all ANSI codes
            let clean = strip_ansi_escapes::strip(&content);
            content = String::from_utf8_lossy(&clean).to_string();
            
            // Remove emojis and special Unicode (keep basic ASCII)
            content = content.chars()
                .filter(|c| c.is_ascii())
                .collect();
        }
        
        if !content.is_empty() || !strict {
            content_lines.push(content);
        }
    }
    
    content_lines.join("\n")
}

fn draw_box(text: &str, h_padding: usize, _v_padding: usize, style: &BoxStyle, color: &str, text_color: &str, title: Option<&str>, footer: Option<&str>, icon: Option<&str>, fixed_width: Option<usize>, status_bar: Option<&str>, header: Option<&str>) {
    let terminal_width = get_terminal_width();
    
    // Calculate effective box width - respect terminal constraints
    let box_width = match fixed_width {
        Some(w) => {
            if w > terminal_width {
                // LIPSIFY: Fixed width exceeds terminal, constrain it
                terminal_width
            } else {
                w
            }
        },
        None => {
            // Auto-size but constrain to terminal width
            let lines: Vec<&str> = text.lines().collect();
            let content_max_width = lines.iter()
                .map(|line| get_display_width(line))
                .max()
                .unwrap_or(0);
            let ideal_width = content_max_width + 2 * h_padding + 2; // +2 for borders
            
            if ideal_width > terminal_width {
                // LIPSIFY: Content too wide for terminal
                terminal_width
            } else {
                ideal_width
            }
        }
    };
    
    // Ensure minimum viable box size
    let final_width = if box_width < 4 { 4 } else { box_width };
    let lines: Vec<&str> = text.lines().collect();
    let inner_width = final_width.saturating_sub(2); // Account for borders
    let color_code = get_color_code(color);
    
    // Determine text color: "auto" means match box color, "none" means default
    let text_color_code = match text_color {
        "auto" => get_color_code(color), // Use same color as box
        "none" => "",                    // Default terminal color
        _ => get_color_code(text_color), // Explicit color
    };
    
    let pad = " ".repeat(h_padding);
    
    // External header (appears above the box)
    if let Some(header_text) = header {
        let expanded_header = expand_variables(header_text);
        let header_width = get_display_width(&expanded_header);
        
        // LIPSIFY header more aggressively - use 90% of terminal width as threshold
        let header_threshold = (terminal_width * 9) / 10; // 90% of terminal width
        let final_header = if header_width > header_threshold {
            truncate_with_ellipsis(&expanded_header, header_threshold)
        } else {
            expanded_header
        };
        
        // Print external header with subtle styling
        println!("{}{}{}", get_color_code("grey7"), final_header, RESET);
    }
    
    // Top border with optional title - LIPSIFIED for terminal width
    if let Some(title_text) = title {
        let expanded_title = expand_variables(title_text);
        let title_line = render_title_or_footer(&expanded_title, inner_width, style.horizontal);
        println!("{}{}{}{}{}", 
            color_code, style.top_left, title_line, style.top_right, RESET);
    } else {
        let border = style.horizontal.repeat(inner_width);
        println!("{}{}{}{}{}", 
            color_code, style.top_left, border, style.top_right, RESET);
    }
    
    // Content lines - LIPSIFIED for all cases
    for (i, line) in lines.iter().enumerate() {
        let available_content_width = inner_width.saturating_sub(2 * h_padding);
        
        // LIPSIFY: Always truncate if line exceeds available width
        let line_width = get_display_width(line);
        let display_line = if line_width > available_content_width {
            truncate_with_ellipsis(line, available_content_width)
        } else {
            line.to_string()
        };
        
        let width = get_display_width(&display_line);
        let spaces = " ".repeat(available_content_width.saturating_sub(width));
        
        if i == 0 && icon.is_some() {
            // First line with icon - LIPSIFIED
            let icon_str = icon.unwrap();
            let icon_expanded = expand_variables(icon_str);
            
            // LIPSIFY: Account for icon when truncating
            let icon_width = get_display_width(&icon_expanded) + 1; // +1 for space
            let line_width = get_display_width(line);
            let final_line = if line_width > available_content_width.saturating_sub(icon_width) {
                truncate_with_ellipsis(line, available_content_width.saturating_sub(icon_width))
            } else {
                display_line
            };
            
            // Apply text color to the text part only (not icon)
            let colored_final_line = if text_color_code.is_empty() {
                final_line.to_string()
            } else {
                format!("{}{}{}", text_color_code, final_line, RESET)
            };
            
            let final_width = get_display_width(&final_line);
            let final_spaces = " ".repeat(available_content_width.saturating_sub(final_width + icon_width));
            
            println!("{}{} {}{}{}{}{}{}{}",
                color_code, style.vertical, RESET,
                icon_expanded, " ",
                colored_final_line, final_spaces, pad,
                format!("{}{}{}", color_code, style.vertical, RESET));
        } else {
            // Apply text color to the display line
            let colored_display_line = if text_color_code.is_empty() {
                display_line.to_string()
            } else {
                format!("{}{}{}", text_color_code, display_line, RESET)
            };
            
            println!("{}{}{}{}{}{}{}{}",
                color_code, style.vertical, RESET,
                pad, colored_display_line, spaces, pad,
                format!("{}{}{}", color_code, style.vertical, RESET));
        }
    }
    
    // Bottom border with optional footer - LIPSIFIED for terminal width
    if let Some(footer_text) = footer {
        let expanded_footer = expand_variables(footer_text);
        let footer_line = render_title_or_footer(&expanded_footer, inner_width, style.horizontal);
        println!("{}{}{}{}{}", 
            color_code, style.bottom_left, footer_line, style.bottom_right, RESET);
    } else {
        let border = style.horizontal.repeat(inner_width);
        println!("{}{}{}{}{}", 
            color_code, style.bottom_left, border, style.bottom_right, RESET);
    }
    
    // Optional status bar with alignment support - LIPSIFIED for terminal width
    if let Some(status_text) = status_bar {
        let expanded_status = expand_variables(status_text);
        
        // Check for alignment codes: sl, sc, sr (status), hl, hr, hc (header), fl, fr, fc (footer)
        let (alignment, clean_status) = if expanded_status.starts_with("sl:") {
            ("left", expanded_status.strip_prefix("sl:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("sc:") {
            ("center", expanded_status.strip_prefix("sc:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("sr:") {
            ("right", expanded_status.strip_prefix("sr:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("hl:") {
            ("left", expanded_status.strip_prefix("hl:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("hc:") {
            ("center", expanded_status.strip_prefix("hc:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("hr:") {
            ("right", expanded_status.strip_prefix("hr:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("fl:") {
            ("left", expanded_status.strip_prefix("fl:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("fc:") {
            ("center", expanded_status.strip_prefix("fc:").unwrap_or(&expanded_status).to_string())
        } else if expanded_status.starts_with("fr:") {
            ("right", expanded_status.strip_prefix("fr:").unwrap_or(&expanded_status).to_string())
        } else {
            ("left", expanded_status) // default alignment
        };
        
        let status_width = get_display_width(&clean_status);
        
        // LIPSIFY status bar more aggressively - use 70% of terminal width as threshold  
        let status_threshold = (terminal_width * 7) / 10; // 70% of terminal width
        let final_status = if status_width > status_threshold {
            truncate_with_ellipsis(&clean_status, status_threshold)
        } else {
            clean_status
        };
        
        // Apply alignment
        let aligned_status = match alignment {
            "center" => {
                let final_width = get_display_width(&final_status);
                let padding_total = terminal_width.saturating_sub(final_width);
                let left_padding = padding_total / 2;
                format!("{}{}", " ".repeat(left_padding), final_status)
            }
            "right" => {
                let final_width = get_display_width(&final_status);
                let padding_total = terminal_width.saturating_sub(final_width);
                format!("{}{}", " ".repeat(padding_total), final_status)
            }
            _ => final_status, // left alignment (default)
        };
        
        // Status bar with subtle styling
        println!("{}{}{}", get_color_code("grey3"), aligned_status, RESET);
    }
}

/// Handle migrate-commands subcommand for helping users transition
fn handle_migrate_command(args: &[String], jynx: &JynxIntegration) {
    if args.is_empty() {
        println!("{} {} - Migration Assistant", NAME, VERSION);
        println!();
        println!("USAGE:");
        println!("    {} migrate-commands [OPTIONS]", NAME);
        println!();
        println!("OPTIONS:");
        println!("    --check <command>    Check a command for migration suggestions");
        println!("    --interactive        Interactive migration guide");
        println!("    --examples           Show migration examples");
        println!("    --guide              Comprehensive migration guide");
        println!("    --v6-changes         Show v0.6.0 breaking changes");
        println!("    --help               Show this help message");
        println!();
        println!("EXAMPLES:");
        println!("    {} migrate-commands --check 'echo test | boxy --icon \u{1f4e6} --title Status'", NAME);
        println!("    {} migrate-commands --interactive", NAME);
        println!("    {} migrate-commands --examples", NAME);
        println!("    {} migrate-commands --guide", NAME);
        return;
    }
    
    match args[0].as_str() {
        "--check" => {
            if args.len() < 2 {
                eprintln!("Error: --check requires a command. Usage: {} migrate-commands --check <command>", NAME);
                std::process::exit(1);
            }
            analyze_command_for_migration(&args[1]);
        }
        "--interactive" => {
            run_interactive_migration_guide();
        }
        "--examples" => {
            show_migration_examples(jynx);
        }
        "--guide" => {
            show_comprehensive_migration_guide();
        }
        "--v6-changes" => {
            show_v6_breaking_changes();
        }
        "--help" => {
            println!("{} {} - Migration Assistant", NAME, VERSION);
            println!();
            println!("The migration assistant helps you update commands to use new boxy features:");
            println!();
            println!("  • --header vs --title distinction");
            println!("  • Enhanced --title with icon support");
            println!("  • Status alignment prefixes");
            println!("  • Improved theme integration");
            println!();
            println!("Run with --examples to see before/after examples.");
            println!("Run with --guide for the comprehensive migration guide.");
        }
        _ => {
            eprintln!("Unknown migrate-commands option: {}", args[0]);
            eprintln!("Use '{} migrate-commands --help' for available options", NAME);
            eprintln!("Available options: --check, --interactive, --examples, --guide, --v6-changes, --help");
            std::process::exit(1);
        }
    }
}

/// Analyze a command string for migration opportunities
fn analyze_command_for_migration(command: &str) {
    println!("Migration Analysis for: {}", command);
    println!("{}=========================={}", get_color_code("azure"), RESET);
    println!();
    
    let mut suggestions = Vec::new();
    
    // Check for --icon + --title pattern
    if command.contains("--icon") && command.contains("--title") {
        suggestions.push((
            "🔄 Icon + Title Combination".to_string(),
            "Consider using --title with embedded icon instead of separate --icon and --title flags.".to_string(),
            extract_migration_suggestion_for_icon_title(command)
        ));
    }
    
    // Check for long --status without alignment
    if let Some(status_part) = extract_status_from_command(command) {
        if status_part.len() > 50 && !status_part.starts_with("sl:") && !status_part.starts_with("sc:") && !status_part.starts_with("sr:") {
            suggestions.push((
                "📍 Status Alignment".to_string(),
                "Long status text should use alignment prefixes for better control.".to_string(),
                format!("  Old: --status \"{}\"", status_part),
            ));
        }
    }
    
    if suggestions.is_empty() {
        println!("✅ No migration suggestions found. Your command follows current best practices!");
    } else {
        println!("Found {} migration suggestions:", suggestions.len());
        println!();
        
        for (i, (title, description, example)) in suggestions.iter().enumerate() {
            println!("{}. {}", i + 1, title);
            println!("   {}", description);
            if !example.is_empty() {
                println!("{}", example);
            }
            println!();
        }
    }
}

/// Extract migration suggestion for icon+title pattern
fn extract_migration_suggestion_for_icon_title(command: &str) -> String {
    // Simple extraction - in a real implementation would be more sophisticated
    let icon_part = if let Some(start) = command.find("--icon ") {
        let after_icon = &command[start + 7..];
        if let Some(end) = after_icon.find(" --") {
            after_icon[..end].trim().trim_matches('"').trim_matches('\'').to_string()
        } else {
            after_icon.split_whitespace().next().unwrap_or("").trim_matches('"').trim_matches('\'').to_string()
        }
    } else {
        "📦".to_string()
    };
    
    let title_part = if let Some(start) = command.find("--title ") {
        let after_title = &command[start + 8..];
        if let Some(end) = after_title.find(" --") {
            after_title[..end].trim().trim_matches('"').trim_matches('\'').to_string()
        } else {
            after_title.split_whitespace().next().unwrap_or("Title").trim_matches('"').trim_matches('\'').to_string()
        }
    } else {
        "Title".to_string()
    };
    
    format!(
        "  Old: --icon \"{}\" --title \"{}\"\n  New: --title \"{} {}\"",
        icon_part, title_part, icon_part, title_part
    )
}

/// Extract status text from command
fn extract_status_from_command(command: &str) -> Option<String> {
    if let Some(start) = command.find("--status ") {
        let after_status = &command[start + 9..];
        if let Some(end) = after_status.find(" --") {
            Some(after_status[..end].trim().trim_matches('"').trim_matches('\'').to_string())
        } else {
            Some(after_status.trim().trim_matches('"').trim_matches('\'').to_string())
        }
    } else {
        None
    }
}

/// Show migration examples
fn show_migration_examples(jynx: &JynxIntegration) {
    let header = format!("{} {} - Migration Examples", NAME, VERSION);
    
    if jynx.is_active() {
        jynx_println(&header, "migration", jynx);
        println!();
    } else {
        println!("{}", header);
        println!();
    }
    
    let examples = vec![
        (
            "🔄 Icon + Title Combination",
            "echo 'Success' | boxy --icon ✅ --title 'Status'",
            "echo 'Success' | boxy --title '✅ Status'"
        ),
        (
            "📍 Status Alignment", 
            "echo 'Done' | boxy --status 'This is a very long status message'",
            "echo 'Done' | boxy --status 'sc:This is a very long status message'"
        ),
        (
            "🏷️ Header vs Title",
            "echo 'Output' | boxy --title 'Application Name'",
            "echo 'Output' | boxy --header 'Application Name' --title '✅ Status'"
        ),
        (
            "🎨 Theme Integration",
            "echo 'Error' | boxy --icon ❌ --color red",
            "echo 'Error' | boxy --theme error"
        ),
    ];
    
    for (category, old_command, new_command) in examples {
        println!("{}", category);
        println!("  {}OLD:{} {}", get_color_code("red"), RESET, old_command);
        println!("  {}NEW:{} {}", get_color_code("green"), RESET, new_command);
        println!();
    }
    
    println!("💡 {}TIP:{} Use 'boxy migrate-commands --check <command>' to analyze specific commands", get_color_code("azure"), RESET);
}

/// Interactive migration guide
fn run_interactive_migration_guide() {
    println!("{} {} - Interactive Migration Guide", NAME, VERSION);
    println!();
    println!("This guide will help you understand the key changes in boxy's new version:");
    println!();
    
    // Step 1: Header vs Title
    println!("{}📋 Step 1: Understanding --header vs --title{}", get_color_code("azure"), RESET);
    println!();
    println!("  {}--header{}  Appears ABOVE the box (external)", get_color_code("green"), RESET);
    println!("  {}--title{}   Appears IN the top border (internal)", get_color_code("blue"), RESET);
    println!();
    println!("Example:");
    println!("  echo 'Content' | boxy --header 'My App' --title '✅ Success'");
    println!();
    
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // Step 2: Enhanced Title
    println!("{}🎨 Step 2: Enhanced --title with Icon Support{}", get_color_code("azure"), RESET);
    println!();
    println!("  Old way: --icon 📦 --title 'Status'");
    println!("  New way: --title '📦 Status'");
    println!();
    println!("The new --title automatically detects and formats icons!");
    println!();
    
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // Step 3: Status Alignment
    println!("{}📍 Step 3: Status Bar Alignment{}", get_color_code("azure"), RESET);
    println!();
    println!("  {}sl:{}text  Left aligned", get_color_code("green"), RESET);
    println!("  {}sc:{}text  Center aligned", get_color_code("green"), RESET);
    println!("  {}sr:{}text  Right aligned", get_color_code("green"), RESET);
    println!();
    println!("Example: --status 'sc:Centered status message'");
    println!();
    
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // Step 4: Themes
    println!("{}🎭 Step 4: Theme Integration{}", get_color_code("azure"), RESET);
    println!();
    println!("  Instead of: --icon ❌ --color red");
    println!("  Use:        --theme error");
    println!();
    println!("Available themes: error, success, warning, info, critical");
    println!();
    
    println!("{}✅ Migration guide complete!{}", get_color_code("green"), RESET);
    println!();
    println!("Next steps:");
    println!("  • Use 'boxy migrate-commands --examples' to see more examples");
    println!("  • Use 'boxy migrate-commands --check <command>' to analyze specific commands");
    println!("  • Check 'boxy --help' for the complete updated syntax");
}

/// Comprehensive migration guide
fn show_comprehensive_migration_guide() {
    println!("{} {} - Comprehensive Migration Guide", NAME, VERSION);
    println!("{}===================================={}", get_color_code("azure"), RESET);
    println!();
    
    // Overview
    println!("{}📝 OVERVIEW{}", get_color_code("green"), RESET);
    println!("This guide covers all breaking changes and migration paths from boxy v0.5.x to v0.6.0+.");
    println!();
    
    // Breaking Changes Summary
    println!("{}⚡ BREAKING CHANGES SUMMARY{}", get_color_code("red"), RESET);
    println!("1. 🏷️ New --header flag for external headers (above box)");
    println!("2. 🎨 Enhanced --title flag with automatic icon support");
    println!("3. 📍 Improved status alignment with prefix system");
    println!("4. 🎭 Better theme integration and icon handling");
    println!();
    
    // Detailed Migration Sections
    println!("{}📦 1. HEADER vs TITLE DISTINCTION{}", get_color_code("blue"), RESET);
    println!("OLD BEHAVIOR: --title was used for both external labels and internal titles");
    println!("NEW BEHAVIOR: Clear separation between external and internal titles");
    println!();
    println!("  {}--header{}  External header (appears above the box)", get_color_code("green"), RESET);
    println!("  {}--title{}   Internal title (embedded in the box border)", get_color_code("blue"), RESET);
    println!();
    println!("{}MIGRATION EXAMPLES:{}", get_color_code("orange"), RESET);
    println!("  OLD: echo 'data' | boxy --title 'MyApp v1.0'");
    println!("  NEW: echo 'data' | boxy --header 'MyApp v1.0' --title '✅ Ready'");
    println!();
    println!("  OLD: echo 'output' | boxy --title 'Processing'");
    println!("  NEW: echo 'output' | boxy --title '⚙️ Processing' (if internal status)");
    println!("       echo 'output' | boxy --header 'Processing' (if application label)");
    println!();
    
    // Icon Integration
    println!("{}🎨 2. ENHANCED TITLE WITH ICON SUPPORT{}", get_color_code("blue"), RESET);
    println!("OLD BEHAVIOR: Separate --icon and --title flags with complex positioning");
    println!("NEW BEHAVIOR: Unified --title with automatic icon detection and formatting");
    println!();
    println!("{}MIGRATION EXAMPLES:{}", get_color_code("orange"), RESET);
    println!("  OLD: echo 'Success' | boxy --icon ✅ --title 'Operation'");
    println!("  NEW: echo 'Success' | boxy --title '✅ Operation'");
    println!();
    println!("  OLD: echo 'Error' | boxy --icon ❌ --title 'Failed'");
    println!("  NEW: echo 'Error' | boxy --title '❌ Failed'");
    println!();
    println!("{}BENEFITS:{}", get_color_code("cyan"), RESET);
    println!("  • Consistent spacing and alignment");
    println!("  • Better text color support");
    println!("  • Simpler command syntax");
    println!("  • Automatic icon detection");
    println!();
    
    // Status Alignment
    println!("{}📍 3. STATUS BAR ALIGNMENT{}", get_color_code("blue"), RESET);
    println!("NEW FEATURE: Status bars now support alignment prefixes for better control");
    println!();
    println!("{}ALIGNMENT PREFIXES:{}", get_color_code("green"), RESET);
    println!("  {}sl:{} Left aligned", get_color_code("green"), RESET);
    println!("  {}sc:{} Center aligned", get_color_code("green"), RESET);
    println!("  {}sr:{} Right aligned", get_color_code("green"), RESET);
    println!();
    println!("{}MIGRATION EXAMPLES:{}", get_color_code("orange"), RESET);
    println!("  OLD: echo 'Done' | boxy --status 'Build completed successfully'");
    println!("  NEW: echo 'Done' | boxy --status 'sc:Build completed successfully'");
    println!();
    
    // Theme Integration
    println!("{}🎭 4. IMPROVED THEME INTEGRATION{}", get_color_code("blue"), RESET);
    println!("ENHANCED: Themes now work seamlessly with the new header/title system");
    println!();
    println!("{}MIGRATION EXAMPLES:{}", get_color_code("orange"), RESET);
    println!("  OLD: echo 'Error' | boxy --icon ❌ --color red");
    println!("  NEW: echo 'Error' | boxy --theme error");
    println!();
    println!("  OLD: echo 'Success' | boxy --icon ✅ --color green");
    println!("  NEW: echo 'Success' | boxy --theme success");
    println!();
    
    // Backward Compatibility
    println!("{}🔄 5. BACKWARD COMPATIBILITY{}", get_color_code("purple"), RESET);
    println!("DEPRECATION PERIOD: Old syntax still works but shows warnings");
    println!("MIGRATION TOOLS: Use 'boxy migrate-commands' for assistance");
    println!();
    println!("{}TIMELINE:{}", get_color_code("orange"), RESET);
    println!("  v0.6.0: New features added, old syntax shows warnings");
    println!("  v0.7.0: Old syntax will be removed (planned)");
    println!();
    
    // Quick Reference
    println!("{}📝 6. QUICK REFERENCE{}", get_color_code("cyan"), RESET);
    println!("{}COMMON PATTERNS:{}", get_color_code("green"), RESET);
    println!("  Application output:  echo 'data' | boxy --header 'MyApp' --title '✅ Ready'");
    println!("  Error message:       echo 'Failed' | boxy --theme error");
    println!("  Success message:     echo 'Done' | boxy --theme success");
    println!("  Status with align:   echo 'OK' | boxy --status 'sc:Centered status'");
    println!();
    
    // Tools and Resources
    println!("{}🔧 MIGRATION TOOLS{}", get_color_code("azure"), RESET);
    println!("  {}boxy migrate-commands --check <command>{}     Analyze specific commands", get_color_code("green"), RESET);
    println!("  {}boxy migrate-commands --interactive{}        Interactive guide", get_color_code("green"), RESET);
    println!("  {}boxy migrate-commands --examples{}           See before/after examples", get_color_code("green"), RESET);
    println!("  {}boxy --help{}                               Updated syntax reference", get_color_code("green"), RESET);
    println!();
    
    println!("{}✅ Need help? Run 'boxy migrate-commands --interactive' for step-by-step guidance{}", get_color_code("green"), RESET);
}

/// Show v0.6.0 breaking changes summary
fn show_v6_breaking_changes() {
    println!("{} {} - v0.6.0 Breaking Changes", NAME, VERSION);
    println!("{}==========================={}", get_color_code("red"), RESET);
    println!();
    
    println!("{}⚡ BREAKING CHANGES IN v0.6.0{}", get_color_code("red"), RESET);
    println!();
    
    println!("{}1. NEW --header FLAG{}", get_color_code("orange"), RESET);
    println!("   • {}--header{} now creates external headers (above the box)", get_color_code("green"), RESET);
    println!("   • {}--title{} is now for internal titles (in the border)", get_color_code("blue"), RESET);
    println!("   • Migration: Long app names should use --header, status should use --title");
    println!();
    
    println!("{}2. ENHANCED --title WITH ICONS{}", get_color_code("orange"), RESET);
    println!("   • --title now auto-detects and formats icons");
    println!("   • Pattern: --title 'icon text' instead of --icon + --title");
    println!("   • Migration: Combine --icon 📦 --title Status → --title '📦 Status'");
    println!();
    
    println!("{}3. STATUS ALIGNMENT PREFIXES{}", get_color_code("orange"), RESET);
    println!("   • New prefixes: sl: (left), sc: (center), sr: (right)");
    println!("   • Long status text without prefixes shows deprecation warning");
    println!("   • Migration: --status 'text' → --status 'sc:text'");
    println!();
    
    println!("{}4. IMPROVED THEME SYSTEM{}", get_color_code("orange"), RESET);
    println!("   • Themes now integrate better with new header/title system");
    println!("   • Theme icons are automatically formatted");
    println!("   • Migration: --icon + --color → --theme");
    println!();
    
    println!("{}🛠️ MIGRATION TIMELINE{}", get_color_code("blue"), RESET);
    println!("   • {}v0.6.0:{} New features available, old syntax shows warnings", get_color_code("green"), RESET);
    println!("   • {}v0.6.x:{} Deprecation period - both syntaxes work", get_color_code("orange"), RESET);
    println!("   • {}v0.7.0:{} Old deprecated syntax will be removed", get_color_code("red"), RESET);
    println!();
    
    println!("{}📝 COMPATIBILITY{}", get_color_code("cyan"), RESET);
    println!("   • All existing commands still work");
    println!("   • New features are opt-in");
    println!("   • Deprecation warnings help identify migration opportunities");
    println!("   • Use migration tools for smooth transition");
    println!();
    
    println!("{}🚀 NEXT STEPS{}", get_color_code("green"), RESET);
    println!("   1. Run: boxy migrate-commands --interactive");
    println!("   2. Test your scripts with new syntax");
    println!("   3. Update documentation and scripts gradually");
    println!("   4. Use 'boxy migrate-commands --check <command>' for specific help");
    println!();
    
    println!("{}ℹ️ For detailed migration guide: boxy migrate-commands --guide{}", get_color_code("azure"), RESET);
}

/// Handle theme subcommands: list, show, etc.
fn handle_theme_command(args: &[String], jynx: &JynxIntegration) {
    if args.is_empty() {
        eprintln!("Theme command requires an action. Usage: {} theme <action>", NAME);
        eprintln!("Available actions: list, show <theme>, help");
        std::process::exit(1);
    }
    
    match args[0].as_str() {
        "list" => {
            match ThemeEngine::new() {
                Ok(theme_engine) => {
                    let themes = theme_engine.list_themes();
                    if themes.is_empty() {
                        println!("No themes available.");
                        return;
                    }
                    
                    // Build theme list content
                    let mut theme_content = String::new();
                    theme_content.push_str(&format!("{} {} - Available Themes\n", NAME, VERSION));
                    theme_content.push('\n');
                    
                    for (name, description) in themes {
                        theme_content.push_str(&format!("  {} - {}\n", name, description));
                    }
                    
                    theme_content.push('\n');
                    theme_content.push_str(&format!("Usage: {} --theme <theme_name>\n", NAME));
                    
                    // Use jynx for enhanced theme list display
                    if jynx.is_active() {
                        jynx_println(&theme_content, "theme_list", jynx);
                    } else {
                        print!("{}", theme_content);
                    }
                }
                Err(e) => {
                    eprintln!("Error: Failed to load theme engine: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "create" => {
            if args.len() < 2 {
                eprintln!("Error: Theme create requires a name. Usage: {} theme create <name>", NAME);
                std::process::exit(1);
            }
            handle_theme_create(&args[1], jynx);
        }
        "import" => {
            if args.len() < 2 {
                eprintln!("Error: Theme import requires a path. Usage: {} theme import <path>", NAME);
                std::process::exit(1);
            }
            handle_theme_import(&args[1]);
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("Error: Theme export requires a name. Usage: {} theme export <name>", NAME);
                std::process::exit(1);
            }
            handle_theme_export(&args[1]);
        }
        "edit" => {
            if args.len() < 2 {
                eprintln!("Error: Theme edit requires a name. Usage: {} theme edit <name>", NAME);
                std::process::exit(1);
            }
            handle_theme_edit(&args[1]);
        }
        "help" => {
            println!("{} {} - Theme Management", NAME, VERSION);
            println!();
            println!("USAGE:");
            println!("    {} theme <action> [options]", NAME);
            println!();
            println!("ACTIONS:");
            println!("    list              List all available themes");
            println!("    show <theme>      Show detailed theme information");
            println!("    create <name>     Create a new theme interactively");
            println!("    import <path>     Import theme from file");
            println!("    export <name>     Export theme to file");
            println!("    edit <name>       Edit existing theme");
            println!("    help              Show this help message");
            println!();
            println!("EXAMPLES:");
            println!("    {} theme list", NAME);
            println!("    {} theme show error", NAME);
            println!("    {} theme create my_theme", NAME);
            println!("    {} theme export error > error.yml", NAME);
        }
        action => {
            eprintln!("Unknown theme action: {}", action);
            eprintln!("Available actions: list, show, create, import, export, edit, help");
            eprintln!("Use '{} theme help' for more information", NAME);
            std::process::exit(1);
        }
    }
}

/// Validate theme file before import
fn validate_theme_file(path: &PathBuf) -> Result<(), String> {
    // Read and parse the theme file
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read theme file: {}", e))?;
    
    let theme_file: ThemeFile = serde_yaml::from_str(&content)
        .map_err(|e| format!("Invalid YAML format: {}", e))?;
    
    // Validate each theme in the file
    let temp_engine = ThemeEngine::new()
        .map_err(|e| format!("Failed to initialize validator: {}", e))?;
    
    let mut validation_errors = Vec::new();
    
    for (theme_name, theme) in &theme_file.themes {
        if let Err(e) = temp_engine.validate_theme(theme) {
            validation_errors.push(format!("Theme '{}': {}", theme_name, e));
        }
    }
    
    // Validate metadata
    if theme_file.metadata.name.is_empty() {
        validation_errors.push("Missing or empty metadata.name".to_string());
    }
    
    if theme_file.metadata.version.is_empty() {
        validation_errors.push("Missing or empty metadata.version".to_string());
    }
    
    if !validation_errors.is_empty() {
        return Err(format!("Validation errors:\n  {}", validation_errors.join("\n  ")));
    }
    
    Ok(())
}


/// Validate style input
fn validate_style(style: &str) -> Result<(), String> {
    let valid_styles = vec!["normal", "rounded", "double", "heavy", "ascii"];
    if !valid_styles.contains(&style) {
        return Err(format!("Invalid style '{}'. Valid styles: {}", style, valid_styles.join(", ")));
    }
    Ok(())
}

/// Validate width input
fn validate_width(width_str: &str) -> Result<(), String> {
    match width_str.parse::<usize>() {
        Ok(w) if w >= 10 && w <= 200 => Ok(()),
        Ok(w) => Err(format!("Width {} out of range (10-200)", w)),
        Err(_) => Err("Width must be a number".to_string()),
    }
}

/// Validate theme name
fn validate_theme_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Theme name cannot be empty".to_string());
    }
    
    if name.len() > 50 {
        return Err("Theme name too long (max 50 characters)".to_string());
    }
    
    // Allow alphanumeric, underscore, hyphen, and dot
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return Err("Theme name can only contain letters, numbers, underscore, hyphen, and dot".to_string());
    }
    
    // Don't allow names that start with reserved prefixes
    let reserved_prefixes = vec!["builtin_", "system_", "default_"];
    for prefix in reserved_prefixes {
        if name.starts_with(prefix) {
            return Err(format!("Theme name cannot start with reserved prefix '{}'", prefix));
        }
    }
    
    Ok(())
}

/// Interactive theme creation utility
fn create_theme_interactively(name: &str) -> BoxyTheme {
    println!("Configure theme '{}' (press Enter for default):", name);
    println!();
    
    // Color selection
    print!("Box color [azure]: ");
    io::stdout().flush().unwrap();
    let mut color_input = String::new();
    io::stdin().read_line(&mut color_input).unwrap();
    let color = color_input.trim();
    let mut color = if color.is_empty() { "azure" } else { color };
    
    // Validate color
    if let Err(e) = validate_color(color) {
        println!("Warning: {}", e);
        println!("Using default color 'azure'");
        color = "azure";
    }
    
    // Text color selection  
    print!("Text color [auto/none/color]: ");
    io::stdout().flush().unwrap();
    let mut text_color_input = String::new();
    io::stdin().read_line(&mut text_color_input).unwrap();
    let text_color = text_color_input.trim();
    let text_color = if text_color.is_empty() { "auto" } else { text_color };
    
    // Style selection with validation loop
    let style = loop {
        print!("Border style [normal/rounded/double/heavy/ascii]: ");
        io::stdout().flush().unwrap();
        let mut style_input = String::new();
        io::stdin().read_line(&mut style_input).unwrap();
        let style = style_input.trim();
        let style = if style.is_empty() { "normal" } else { style };
        
        if let Err(e) = validate_style(style) {
            println!("Error: {}", e);
            continue;
        }
        break style.to_string();
    };
    
    // Icon/emoji
    print!("Icon/emoji [optional]: ");
    io::stdout().flush().unwrap();
    let mut icon_input = String::new();
    io::stdin().read_line(&mut icon_input).unwrap();
    let icon = icon_input.trim();
    let icon = if icon.is_empty() { None } else { Some(icon.to_string()) };
    
    // Width with validation loop
    let width = loop {
        print!("Fixed width [10-200, or press Enter for auto]: ");
        io::stdout().flush().unwrap();
        let mut width_input = String::new();
        io::stdin().read_line(&mut width_input).unwrap();
        let width = width_input.trim();
        
        if width.is_empty() {
            break None;
        }
        
        match validate_width(width) {
            Ok(()) => break Some(width.parse().unwrap()),
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    };
    
    BoxyTheme {
        color: color.to_string(),
        text_color: text_color.to_string(),
        style: style.to_string(),
        icon,
        width,
        ..Default::default()
    }
}

/// Interactive theme editing utility
fn edit_theme_interactively(name: &str, existing: &BoxyTheme) -> BoxyTheme {
    println!("Edit theme '{}' (press Enter to keep current value):", name);
    println!();
    
    // Color
    print!("Box color [current: {}]: ", existing.color);
    io::stdout().flush().unwrap();
    let mut color_input = String::new();
    io::stdin().read_line(&mut color_input).unwrap();
    let color = color_input.trim();
    let color = if color.is_empty() { &existing.color } else { color };
    
    // Text color
    print!("Text color [current: {}]: ", existing.text_color);
    io::stdout().flush().unwrap();
    let mut text_color_input = String::new();
    io::stdin().read_line(&mut text_color_input).unwrap();
    let text_color = text_color_input.trim();
    let text_color = if text_color.is_empty() { &existing.text_color } else { text_color };
    
    // Style
    print!("Border style [current: {}]: ", existing.style);
    io::stdout().flush().unwrap();
    let mut style_input = String::new();
    io::stdin().read_line(&mut style_input).unwrap();
    let style = style_input.trim();
    let style = if style.is_empty() { &existing.style } else { style };
    
    // Icon
    let current_icon = existing.icon.as_deref().unwrap_or("none");
    print!("Icon/emoji [current: {}]: ", current_icon);
    io::stdout().flush().unwrap();
    let mut icon_input = String::new();
    io::stdin().read_line(&mut icon_input).unwrap();
    let icon = icon_input.trim();
    let icon = if icon.is_empty() { 
        existing.icon.clone() 
    } else if icon == "none" { 
        None 
    } else { 
        Some(icon.to_string()) 
    };
    
    // Width
    let current_width = existing.width.map(|w| w.to_string()).unwrap_or_else(|| "auto".to_string());
    print!("Fixed width [current: {}]: ", current_width);
    io::stdout().flush().unwrap();
    let mut width_input = String::new();
    io::stdin().read_line(&mut width_input).unwrap();
    let width = width_input.trim();
    let width = if width.is_empty() { 
        existing.width 
    } else if width == "auto" { 
        None 
    } else { 
        match width.parse::<usize>() {
            Ok(w) if w >= 10 => Some(w),
            _ => {
                println!("Warning: Invalid width, keeping current");
                existing.width
            }
        }
    };
    
    BoxyTheme {
        color: color.to_string(),
        text_color: text_color.to_string(),
        style: style.to_string(),
        icon,
        width,
        ..existing.clone()
    }
}

/// Save theme to YAML file
fn save_theme_to_file(path: &PathBuf, name: &str, theme: &BoxyTheme) -> Result<(), String> {
    let theme_file = ThemeFile {
        metadata: ThemeMetadata {
            name: format!("{} Theme File", name),
            version: "1.0.0".to_string(),
            description: format!("Custom theme: {}", name),
            author: env::var("USER").unwrap_or_else(|_| "boxy".to_string()),
            created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            updated: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            compatibility: "boxy v0.6+".to_string(),
        },
        colors: std::collections::HashMap::new(),
        themes: {
            let mut themes = std::collections::HashMap::new();
            themes.insert(name.to_string(), theme.clone());
            themes
        },
        presets: std::collections::HashMap::new(),
        text_styles: std::collections::HashMap::new(),
        settings: ThemeSettings::default(),
    };
    
    let yaml_content = serde_yaml::to_string(&theme_file)
        .map_err(|e| format!("Failed to serialize theme: {}", e))?;
    
    fs::write(path, yaml_content)
        .map_err(|e| format!("Failed to write theme file: {}", e))?;
    
    Ok(())
}

/// Export theme to YAML format
fn export_theme_to_yaml(name: &str, theme: &BoxyTheme) -> String {
    let theme_file = ThemeFile {
        metadata: ThemeMetadata {
            name: format!("{} Theme Export", name),
            version: "1.0.0".to_string(),
            description: format!("Exported theme: {}", name),
            author: "boxy".to_string(),
            created: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            updated: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            compatibility: "boxy v0.6+".to_string(),
        },
        colors: std::collections::HashMap::new(),
        themes: {
            let mut themes = std::collections::HashMap::new();
            themes.insert(name.to_string(), theme.clone());
            themes
        },
        presets: std::collections::HashMap::new(),
        text_styles: std::collections::HashMap::new(),
        settings: ThemeSettings::default(),
    };
    
    serde_yaml::to_string(&theme_file)
        .unwrap_or_else(|e| format!("# Error serializing theme: {}", e))
}

/// Handle `boxy theme create <name>` command
fn handle_theme_create(name: &str, jynx: &JynxIntegration) {
    // Validate theme name first
    if let Err(e) = validate_theme_name(name) {
        eprintln!("Error: Invalid theme name: {}", e);
        std::process::exit(1);
    }
    
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            // Check if theme already exists
            if theme_engine.get_theme(name).is_some() {
                eprintln!("Error: Theme '{}' already exists", name);
                eprintln!("Use 'boxy theme edit {}' to modify it", name);
                std::process::exit(1);
            }
            
            println!("{} {} - Create New Theme: {}", NAME, VERSION, name);
            println!();
            println!("Creating theme interactively. Press Ctrl+C to cancel.");
            println!();
            
            // Interactive theme creation
            let theme = create_theme_interactively(name);
            
            // Save theme to XDG+ directory
            let themes_dir = theme_engine.get_themes_directory();
            if let Err(e) = fs::create_dir_all(&themes_dir) {
                eprintln!("Error: Failed to create themes directory: {}", e);
                std::process::exit(1);
            }
            
            let theme_file_path = themes_dir.join(format!("{}.yml", name));
            if let Err(e) = save_theme_to_file(&theme_file_path, name, &theme) {
                eprintln!("Error: Failed to save theme: {}", e);
                std::process::exit(1);
            }
            
            println!();
            let success_msg = format!("✅ Theme '{}' created successfully!\n   Saved to: {}\n\nTest your theme:\n   echo \"Hello World\" | boxy --theme {}", name, theme_file_path.display(), name);
            
            if jynx.is_active() {
                jynx_println(&success_msg, "success", jynx);
            } else {
                println!("{}", success_msg);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy theme import <path>` command
fn handle_theme_import(path: &str) {
    let import_path = PathBuf::from(path);
    if !import_path.exists() {
        eprintln!("Error: File '{}' does not exist", path);
        std::process::exit(1);
    }
    
    // Validate file extension
    if !import_path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
        eprintln!("Error: Only YAML files (.yml, .yaml) are supported for import");
        std::process::exit(1);
    }
    
    // Pre-validate the theme file before importing
    println!("Validating theme file...");
    if let Err(e) = validate_theme_file(&import_path) {
        eprintln!("Error: Theme file validation failed: {}", e);
        eprintln!("The file contains invalid theme configurations and cannot be imported.");
        std::process::exit(1);
    }
    println!("✅ Theme file validation passed");
    
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            let themes_dir = theme_engine.get_themes_directory();
            if let Err(e) = fs::create_dir_all(&themes_dir) {
                eprintln!("Error: Failed to create themes directory: {}", e);
                std::process::exit(1);
            }
            
            let filename = import_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("imported_theme.yml");
                
            let target_path = themes_dir.join(filename);
            
            if target_path.exists() {
                print!("Theme file '{}' already exists. Overwrite? (y/N): ", filename);
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Import cancelled.");
                    return;
                }
            }
            
            if let Err(e) = fs::copy(&import_path, &target_path) {
                eprintln!("Error: Failed to import theme: {}", e);
                std::process::exit(1);
            }
            
            println!("✅ Theme imported successfully!");
            println!("   From: {}", import_path.display());
            println!("   To: {}", target_path.display());
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy theme export <name>` command
fn handle_theme_export(name: &str) {
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(theme) = theme_engine.get_theme(name) {
                let yaml_content = export_theme_to_yaml(name, &theme);
                print!("{}", yaml_content);
            } else {
                eprintln!("Error: Theme '{}' not found", name);
                let theme_list = theme_engine.list_themes();
                let theme_names: Vec<String> = theme_list.iter().map(|(n, _)| n.clone()).collect();
                eprintln!("Available themes: {}", theme_names.join(", "));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy theme edit <name>` command
fn handle_theme_edit(name: &str) {
    // Validate theme name first
    if let Err(e) = validate_theme_name(name) {
        eprintln!("Error: Invalid theme name: {}", e);
        std::process::exit(1);
    }
    
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(existing_theme) = theme_engine.get_theme(name) {
                println!("{} {} - Edit Theme: {}", NAME, VERSION, name);
                println!();
                
                // Interactive theme editing
                let updated_theme = edit_theme_interactively(name, &existing_theme);
                
                // Save updated theme
                let themes_dir = theme_engine.get_themes_directory();
                if let Err(e) = fs::create_dir_all(&themes_dir) {
                    eprintln!("Error: Failed to create themes directory: {}", e);
                    std::process::exit(1);
                }
                
                let theme_file_path = themes_dir.join(format!("{}.yml", name));
                if let Err(e) = save_theme_to_file(&theme_file_path, name, &updated_theme) {
                    eprintln!("Error: Failed to save theme: {}", e);
                    std::process::exit(1);
                }
                
                println!();
                println!("✅ Theme '{}' updated successfully!", name);
                println!("   Saved to: {}", theme_file_path.display());
            } else {
                eprintln!("Error: Theme '{}' not found", name);
                let theme_list = theme_engine.list_themes();
                let theme_names: Vec<String> = theme_list.iter().map(|(n, _)| n.clone()).collect();
                eprintln!("Available themes: {}", theme_names.join(", "));
                eprintln!("Use 'boxy theme create {}' to create a new theme", name);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Show comprehensive CLI help with examples and usage patterns
fn show_comprehensive_help(jynx: &JynxIntegration) {
    // Generate help content - if jynx is available, we'll enhance it
    let help_header = format!("{} {} - {}", NAME, VERSION, DESCRIPTION);
    
    if jynx.is_active() {
        // Use jynx to enhance help output
        jynx_println(&help_header, "help", jynx);
        println!();
    } else {
        // Standard output without jynx
        println!("{}", help_header);
        println!();
    }
    
    // =============== OVERVIEW ===============
    println!("{}OVERVIEW:{}", get_color_code("azure"), RESET);
    println!("  Boxy v0.6 introduces a comprehensive theme system with semantic formatting,");
    println!("  enhanced layout control, and 90+ colors for professional CLI output.");
    println!();
    
    // =============== BASIC USAGE ===============
    println!("{}BASIC USAGE:{}", get_color_code("emerald"), RESET);
    println!("  echo \"content\" | {} [OPTIONS]", NAME);
    println!("  command | {} --theme <theme>", NAME);
    println!("  {} theme <action> [args]        # Theme management", NAME);
    println!();
    
    // =============== CORE OPTIONS ===============
    println!("{}CORE OPTIONS:{}", get_color_code("amber"), RESET);
    println!("  {}Visual Styling:{}", get_color_code("cyan"), RESET);
    println!("    -s, --style <STYLE>        Border style: normal, rounded, double, heavy, ascii");
    println!("    -c, --color <COLOR>        Border color from 90+ palette (see --colors)");
    println!("    --text <COLOR>             Text color: 'auto' matches border, 'none' default");
    println!("    -w, --width <WIDTH>        Fixed width in characters (auto-truncated)");
    println!();
    
    println!("  {}Content & Layout:{}", get_color_code("cyan"), RESET);
    println!("    --header <TEXT>            External header above box (app names, labels)");
    println!("    --title <TEXT>             Internal title in border with icon support");
    println!("    --footer <TEXT>            Footer text in bottom border");
    println!("    --icon <ICON>              Add icon to content (deprecated - use --title)");
    println!("    --status <TEXT>            Status bar below box with alignment (sl:|sc:|sr:)");
    println!();
    
    println!("  {}Theme System:{}", get_color_code("cyan"), RESET);
    println!("    --theme <THEME>            Apply semantic theme (error, success, warning, info)");
    println!();
    
    println!("  {}Utility:{}", get_color_code("cyan"), RESET);
    println!("    --no-boxy[=strict]         Strip box decoration (strict removes all formatting)");
    println!("    --no-color                 Disable jynx integration and color output");
    println!("    -h, --help                 Show this help message");
    println!("    --colors                   Preview all 90+ available colors");
    println!("    -v, --version              Show version information");
    println!();
    
    // =============== THEME SYSTEM ===============
    println!("{}THEME SYSTEM:{}", get_color_code("violet"), RESET);
    println!("  {}Built-in Themes:{}", get_color_code("cyan"), RESET);
    println!("    error      Crimson borders, error icon (❌), bold text");
    println!("    success    Emerald borders, success icon (✅), rounded style");
    println!("    warning    Amber borders, warning icon (⚠️), italic text"); 
    println!("    info       Azure borders, info icon (ℹ️), normal style");
    println!("    critical   Enhanced error theme with double borders");
    println!();
    
    println!("  {}Theme Management:{}", get_color_code("cyan"), RESET);
    println!("    {} theme list                List available themes", NAME);
    println!("    {} theme show <name>         Show theme details", NAME);
    println!("    {} theme create <name>       Create new theme interactively", NAME);
    println!("    {} theme import <file>       Import theme from YAML", NAME);
    println!("    {} theme export <name>       Export theme to YAML", NAME);
    println!("    {} theme edit <name>         Edit existing theme", NAME);
    println!();
    
    // =============== NEW IN V0.6 ===============
    println!("{}NEW IN v0.6:{}", get_color_code("orchid"), RESET);
    println!("  {}Header vs Title Distinction:{}", get_color_code("cyan"), RESET);
    println!("    --header     External headers (app names, system labels)");
    println!("    --title      Internal titles (status, with icon integration)");
    println!();
    
    println!("  {}Enhanced Icon Integration:{}", get_color_code("cyan"), RESET);
    println!("    --title \"📦 Status\"       Icon automatically spaced and aligned");
    println!("    Instead of: --icon 📦 --title \"Status\"");
    println!();
    
    println!("  {}Status Bar Alignment:{}", get_color_code("cyan"), RESET);
    println!("    --status \"sl:Left\"        Left-aligned status");
    println!("    --status \"sc:Center\"      Center-aligned status");
    println!("    --status \"sr:Right\"       Right-aligned status");
    println!();
    
    println!("  {}Rich Color Palette:{}", get_color_code("cyan"), RESET);
    println!("    90+ colors including: crimson, emerald, azure, amber, violet");
    println!("    Semantic colors: error, success, warning, info, critical");
    println!("    Use --colors to preview all available colors");
    println!();
    
    // =============== EXAMPLES ===============
    println!("{}EXAMPLES:{}", get_color_code("gold"), RESET);
    
    println!("  {}Basic Usage:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Hello World\" | {}                    # Simple box", NAME);
    println!("    echo \"Data\" | {} --style rounded --color azure", NAME);
    println!();
    
    println!("  {}Theme Examples:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Operation failed\" | {} --theme error", NAME);
    println!("    echo \"Backup complete\" | {} --theme success", NAME);
    println!("    echo \"API deprecated\" | {} --theme warning", NAME);
    println!("    echo \"Server status\" | {} --theme info", NAME);
    println!();
    
    println!("  {}Advanced Layout:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Content\" | {} --header \"🚀 MyApp v2.1\" --title \"✅ Online\"", NAME);
    println!("    echo \"Status\" | {} --header \"System\" --status \"sr:Updated $(date)\"", NAME);
    println!("    echo \"Data\" | {} --title \"📊 Analytics\" --footer \"© 2024\"", NAME);
    println!();
    
    println!("  {}CI/CD Integration:{}", get_color_code("cyan"), RESET);
    println!("    # Build status reporting");
    println!("    build_status | {} --theme success --header \"Build Pipeline\"", NAME);
    println!("    test_results | {} --theme error --status \"sc:$(date)\"", NAME);
    println!();
    
    println!("  {}Content Processing:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Raw content\" | {} --width 40        # Fixed width", NAME);
    println!("    cat log.txt | {} --no-boxy                # Strip formatting", NAME);
    println!("    echo \"Content\" | {} | {} --no-boxy=strict  # Remove all ANSI", NAME, NAME);
    println!();
    
    // =============== MIGRATION ===============
    println!("{}MIGRATION (v0.5 → v0.6):{}", get_color_code("rust"), RESET);
    println!("  {} migrate-commands --interactive    Interactive migration guide", NAME);
    println!("  {} migrate-commands --check \"cmd\"     Analyze existing command", NAME);
    println!("  {} migrate-commands --examples       Before/after examples", NAME);
    println!("  {} migrate-commands --guide          Comprehensive migration guide", NAME);
    println!();
    
    println!("  {}Common Migrations:{}", get_color_code("cyan"), RESET);
    println!("    OLD: --icon ✅ --title \"Status\"     → NEW: --title \"✅ Status\"");
    println!("    OLD: --color red --style heavy      → NEW: --theme error");
    println!("    OLD: --title \"MyApp\"               → NEW: --header \"MyApp\" --title \"🟢 Ready\"");
    println!();
    
    // =============== TIPS ===============
    println!("{}TIPS & BEST PRACTICES:{}", get_color_code("sage"), RESET);
    println!("  • Use semantic themes (--theme error) over manual styling for consistency");
    println!("  • Headers for app identity, titles for status/state information");
    println!("  • Status alignment prefixes (sl:, sc:, sr:) for professional layouts");
    println!("  • Variable expansion works in headers, titles, footers: --title \"Status: $USER\"");
    println!("  • Chain with other commands: git status | {} --theme info --header \"Git\"", NAME);
    println!("  • Use --width for consistent formatting in scripts and CI/CD");
    println!();
    
    // =============== MORE INFO ===============
    println!("{}MORE INFORMATION:{}", get_color_code("steel"), RESET);
    println!("  {} --colors                 Preview color palette", NAME);
    println!("  {} theme list               Show available themes", NAME);
    println!("  {} migrate-commands --help   Migration assistance", NAME);
    println!("  GitHub: https://github.com/qodeninja/boxy");
    println!("  Documentation: See THEME_SYSTEM_v0.6.md");
    println!();
    
    println!("{}Transform your CLI output with semantic themes and professional formatting!{}", get_color_code("emerald"), RESET);
}

/// Show practical usage examples for different scenarios
fn show_usage_examples() {
    println!("{} {} - Usage Examples", NAME, VERSION);
    println!();
    
    // =============== QUICK START ===============
    println!("{}QUICK START EXAMPLES:{}", get_color_code("emerald"), RESET);
    println!("  # Basic usage - simple box around content");
    println!("  echo \"Hello World\" | {}", NAME);
    println!();
    println!("  # Apply semantic theme - automatic colors and icons"); 
    println!("  echo \"Error occurred\" | {} --theme error", NAME);
    println!("  echo \"Task complete\" | {} --theme success", NAME);
    println!("  echo \"Warning: deprecated\" | {} --theme warning", NAME);
    println!();
    
    // =============== LAYOUT EXAMPLES ===============
    println!("{}LAYOUT & STYLING:{}", get_color_code("azure"), RESET);
    println!("  # Header (external) vs Title (internal) distinction");
    println!("  echo \"Ready\" | {} --header \"MyApp v2.1\" --title \"🟢 Online\"", NAME);
    println!();
    println!("  # Different border styles and colors");
    println!("  echo \"Data\" | {} --style rounded --color azure", NAME);
    println!("  echo \"Alert\" | {} --style heavy --color crimson", NAME);
    println!("  echo \"Code\" | {} --style ascii --color steel", NAME);
    println!();
    println!("  # Fixed width for consistent formatting");
    println!("  echo \"Long content here\" | {} --width 30", NAME);
    println!();
    
    // =============== STATUS BARS ===============
    println!("{}STATUS BAR ALIGNMENT:{}", get_color_code("amber"), RESET);
    println!("  # Left, center, right aligned status bars");
    println!("  echo \"Data\" | {} --status \"sl:Processing...\"", NAME);
    println!("  echo \"Data\" | {} --status \"sc:50% Complete\"", NAME);
    println!("  echo \"Data\" | {} --status \"sr:Updated $(date '+%%H:%%M')\"", NAME);
    println!();
    
    // =============== DEVELOPMENT WORKFLOW ===============
    println!("{}DEVELOPMENT WORKFLOW:{}", get_color_code("violet"), RESET);
    println!("  # Git status with themes");
    println!("  git status --short | {} --theme warning --header \"Git Status\"", NAME);
    println!();
    println!("  # Build results");
    println!("  if make build; then");
    println!("    echo \"Build successful\" | {} --theme success", NAME);
    println!("  else");
    println!("    echo \"Build failed\" | {} --theme error", NAME);
    println!("  fi");
    println!();
    println!("  # Test results with status");
    println!("  pytest --tb=short | {} --theme info --header \"Test Suite\" --status \"sc:$(date)\"", NAME);
    println!();
    
    // =============== SYSTEM ADMINISTRATION ===============
    println!("{}SYSTEM ADMINISTRATION:{}", get_color_code("steel"), RESET);
    println!("  # Service status monitoring");
    println!("  systemctl status nginx | {} --header \"Nginx Status\" --theme info", NAME);
    println!();
    println!("  # Log analysis with fixed width");
    println!("  tail -10 /var/log/syslog | {} --width 80 --header \"System Log\"", NAME);
    println!();
    println!("  # Resource usage alerts");
    println!("  echo \"CPU: 85%, Memory: 92%\" | {} --theme warning --title \"⚠️ High Usage\"", NAME);
    println!();
    
    // =============== CI/CD INTEGRATION ===============
    println!("{}CI/CD INTEGRATION:{}", get_color_code("orchid"), RESET);
    println!("  # Pipeline status reporting");
    println!("  echo \"All tests passed\" | {} --theme success --header \"CI Pipeline\" --footer \"Build #42\"", NAME);
    println!();
    println!("  # Deployment notifications");
    println!("  echo \"Deployed to production\" | {} --theme success --header \"🚀 Deployment\" --status \"sr:$(git rev-parse --short HEAD)\"", NAME);
    println!();
    println!("  # Security scan results");
    println!("  echo \"3 vulnerabilities found\" | {} --theme warning --title \"🔒 Security Scan\"", NAME);
    println!();
    
    // =============== DATA PROCESSING ===============
    println!("{}DATA PROCESSING:{}", get_color_code("sage"), RESET);
    println!("  # Processing status with progress");
    println!("  echo \"Processed 1,247 records\" | {} --theme info --title \"📊 Data Processing\" --status \"sc:85% complete\"", NAME);
    println!();
    println!("  # Database operations");
    println!("  echo \"Backup completed\" | {} --theme success --header \"Database Backup\" --footer \"Size: 2.4GB\"", NAME);
    println!();
    println!("  # API responses");
    println!("  curl -s api/health | {} --theme info --header \"API Health Check\"", NAME);
    println!();
    
    // =============== ADVANCED USAGE ===============
    println!("{}ADVANCED USAGE:{}", get_color_code("rust"), RESET);
    println!("  # Variable expansion in text");
    println!("  echo \"Welcome\" | {} --header \"System: $HOSTNAME\" --title \"User: $USER\" --status \"sr:$(date)\"", NAME);
    println!();
    println!("  # Chain with other commands");
    println!("  ps aux | grep nginx | {} --header \"Nginx Processes\" --theme info", NAME);
    println!();
    println!("  # Remove box formatting (useful for parsing)");
    println!("  echo \"Content with ANSI\" | {} --theme success | {} --no-boxy=strict", NAME, NAME);
    println!();
    
    // =============== THEME MANAGEMENT ===============
    println!("{}THEME MANAGEMENT:{}", get_color_code("coral"), RESET);
    println!("  # List available themes");
    println!("  {} theme list", NAME);
    println!();
    println!("  # Create custom theme");
    println!("  {} theme create my_project_theme", NAME);
    println!();
    println!("  # Import/export themes");
    println!("  {} theme export error > error_theme.yml", NAME);
    println!("  {} theme import ~/my_theme.yml", NAME);
    println!();
    
    // =============== MIGRATION ===============
    println!("{}MIGRATION FROM v0.5:{}", get_color_code("amber"), RESET);
    println!("  # Get migration help for existing commands");
    println!("  {} migrate-commands --check \"echo 'test' | boxy --icon ✅ --color green\"", NAME);
    println!();
    println!("  # Interactive migration assistant");
    println!("  {} migrate-commands --interactive", NAME);
    println!();
    
    // =============== TIPS ===============
    println!("{}PRO TIPS:{}", get_color_code("emerald"), RESET);
    println!("  • Combine themes with specific overrides: --theme info --width 60");
    println!("  • Use headers for app identity, titles for status");
    println!("  • Status prefixes (sl:, sc:, sr:) provide professional alignment");
    println!("  • Fixed widths ensure consistent formatting in logs and reports");
    println!("  • Themes are faster than manual color/style combinations");
    println!();
    
    println!("{}More help: {} --help | {} --colors | {} theme help{}", 
        get_color_code("steel"), NAME, NAME, NAME, RESET);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // PRIORITY 1: Handle subcommands first - these take absolute precedence over stdin
    // Subcommands should always execute regardless of piped input
    if args.len() >= 2 && args[1] == "theme" {
        // Initialize jynx for theme commands
        let no_color = args.contains(&"--no-color".to_string()) || args.contains(&"--no-colour".to_string());
        let theme_jynx = JynxIntegration::new(no_color);
        handle_theme_command(&args[2..], &theme_jynx);
        return;
    }
    
    // Handle migrate-commands subcommand
    if args.len() >= 2 && args[1] == "migrate-commands" {
        // Initialize jynx for migration commands
        let no_color = args.contains(&"--no-color".to_string()) || args.contains(&"--no-colour".to_string());
        let migrate_jynx = JynxIntegration::new(no_color);
        handle_migrate_command(&args[2..], &migrate_jynx);
        return;
    }
    
    // PRIORITY 2: Check for other subcommands that should prevent stdin reading
    // This explicit check ensures no ambiguity about input precedence
    let has_subcommand = args.len() >= 2 && matches!(args[1].as_str(), "theme" | "migrate-commands");
    if has_subcommand {
        // This should never be reached due to early returns above, but serves as a safety net
        return;
    }
    
    let mut style = &NORMAL;
    let mut color = "none";
    let mut text_color = "none";
    let mut title: Option<String> = None;
    let mut footer: Option<String> = None;
    let mut header: Option<String> = None;
    let mut icon: Option<String> = None;
    let mut no_boxy = false;
    let mut strict_mode = false;
    let mut fixed_width: Option<usize> = None;
    let mut theme_name: Option<String> = None;
    let mut status_bar: Option<String> = None;
    let mut skip_next = false;
    let mut deprecation_warnings: Vec<String> = Vec::new();
    let mut no_color_requested = false;
    
    // Pre-scan for --no-color to initialize jynx properly
    for arg in args.iter().skip(1) {
        if arg == "--no-color" || arg == "--no-colour" {
            no_color_requested = true;
            break;
        }
    }
    
    // Initialize jynx integration early
    let jynx = JynxIntegration::new(no_color_requested);
    
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match arg.as_str() {
            "--help" | "-h" => {
                show_comprehensive_help(&jynx);
                return;
            }
            "--version" | "-v" | "-V" => {
                println!("{} {} ({})", NAME, VERSION, jynx.get_version_string());
                return;
            }
            "--colors" => {
                println!("{} {} - Color Palette Preview", NAME, VERSION);
                println!("{}", generate_color_help());
                return;
            }
            "--examples" => {
                show_usage_examples();
                return;
            }
            "--style" | "-s" => {
                if i + 1 < args.len() {
                    style = match args[i + 1].as_str() {
                        "rounded" => &ROUNDED,
                        "double" => &DOUBLE,
                        "heavy" => &HEAVY,
                        "ascii" => &ASCII,
                        "normal" => &NORMAL,
                        _ => {
                            eprintln!("Unknown style: {}. Using normal.", args[i + 1]);
                            &NORMAL
                        }
                    };
                    skip_next = true;
                }
            }
            "--color" | "-c" => {
                if i + 1 < args.len() {
                    let requested_color = &args[i + 1];
                    // Validate color and provide helpful error messages
                    match validate_color(requested_color) {
                        Ok(_) => {
                            color = requested_color;
                            skip_next = true;
                        }
                        Err(error_msg) => {
                            eprintln!("Color Error: {}", error_msg);
                            eprintln!("Use '{} --colors' to see all available colors", NAME);
                            std::process::exit(1);
                        }
                    }
                }
            }
            "--text" => {
                if i + 1 < args.len() {
                    let requested_text_color = &args[i + 1];
                    // Validate text color (same validation as box color)
                    match validate_color(requested_text_color) {
                        Ok(_) => {
                            text_color = requested_text_color;
                            skip_next = true;
                        }
                        Err(error_msg) => {
                            eprintln!("Text Color Error: {}", error_msg);
                            eprintln!("Use '{} --colors' to see all available colors", NAME);
                            std::process::exit(1);
                        }
                    }
                }
            }
            "--width" | "-w" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(w) if w >= 4 => {
                            fixed_width = Some(w);
                            skip_next = true;
                        }
                        _ => {
                            eprintln!("Error: Width must be a number >= 4 (minimum for box borders)");
                            std::process::exit(1);
                        }
                    }
                }
            }
            "--theme" => {
                if i + 1 < args.len() {
                    theme_name = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--title" => {
                if i + 1 < args.len() {
                    title = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--header" => {
                if i + 1 < args.len() {
                    header = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--footer" => {
                if i + 1 < args.len() {
                    footer = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--icon" => {
                if i + 1 < args.len() {
                    icon = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            "--status" => {
                if i + 1 < args.len() {
                    let status_text = &args[i + 1];
                    status_bar = Some(status_text.clone());
                    
                    // Check for deprecation patterns
                    if !status_text.starts_with("sl:") && !status_text.starts_with("sc:") && !status_text.starts_with("sr:") &&
                       !status_text.starts_with("hl:") && !status_text.starts_with("hc:") && !status_text.starts_with("hr:") &&
                       !status_text.starts_with("fl:") && !status_text.starts_with("fc:") && !status_text.starts_with("fr:") &&
                       get_display_width(status_text) > 50 {
                        deprecation_warnings.push(format!(
                            "Long status text without alignment prefix. Consider using sl:, sc:, or sr: prefixes for better control."
                        ));
                    }
                    
                    skip_next = true;
                }
            }
            arg if arg.starts_with("--no-boxy") => {
                no_boxy = true;
                if arg == "--no-boxy=strict" {
                    strict_mode = true;
                }
            }
            "--no-color" | "--no-colour" => {
                // Color handling already processed in pre-scan
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!("Try '{} --help' for more information", NAME);
                std::process::exit(1);
            }
        }
    }
    
    // Check for deprecated pattern combinations with enhanced suggestions
    if icon.is_some() && title.is_some() {
        let icon_str = icon.as_deref().unwrap_or("📦");
        let title_str = title.as_deref().unwrap_or("Title");
        deprecation_warnings.push(format!(
            "Using --icon with --title may cause layout conflicts.\n       → Try: --title \"{} {}\"",
            icon_str, title_str
        ));
    }
    
    // Auto-detect potential migration opportunities
    let mut auto_suggestions = Vec::new();
    
    // Suggest header/title distinction for external-looking titles
    if title.is_some() && !icon.is_some() {
        let title_text = title.as_deref().unwrap_or("");
        if title_text.len() > 20 || title_text.to_lowercase().contains("app") || title_text.to_lowercase().contains("system") {
            auto_suggestions.push(format!(
                "Consider using --header for application names: --header \"{}\"", title_text
            ));
        }
    }
    
    // Display auto-suggestions
    if !auto_suggestions.is_empty() {
        eprintln!("{}🤖 AUTO-SUGGESTION:{}", get_color_code("cyan"), RESET);
        for suggestion in &auto_suggestions {
            eprintln!("{}{}{}", get_color_code("cyan"), suggestion, RESET);
        }
        eprintln!();
    }
    
    // Display deprecation warnings with migration suggestions
    if !deprecation_warnings.is_empty() {
        for warning in &deprecation_warnings {
            eprintln!("{}⚠️  DEPRECATION WARNING:{} {}", get_color_code("orange"), RESET, warning);
        }
        eprintln!();
        eprintln!("{}💡 MIGRATION TIP:{} Use 'boxy migrate-commands --help' for migration assistance", get_color_code("azure"), RESET);
        eprintln!();
    }
    
    // PRIORITY 3: Read from stdin only if no subcommands were processed
    // At this point, all subcommands and utility flags (--help, --version, etc.) have been handled
    // This ensures clear precedence: subcommands > utility flags > stdin processing
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let mut text = input.trim_end_matches('\n').to_string();
    
    // Apply theme if specified - using new theme engine
    if let Some(theme_name_str) = &theme_name {
        match ThemeEngine::new() {
            Ok(theme_engine) => {
                if let Some(boxy_theme) = theme_engine.get_theme(theme_name_str) {
                    // Theme overrides defaults but explicit flags override theme
                    if color == "none" {
                        color = Box::leak(boxy_theme.color.clone().into_boxed_str());
                    }
                    // When using themes, theme icons get their own line
                    let theme_emoji = if let Some(override_icon) = &icon {
                        // If explicit icon provided, use that instead of theme's emoji
                        override_icon.clone()
                    } else {
                        // Extract icon from either icon field or title field (YAML themes use title)
                        if let Some(icon_str) = &boxy_theme.icon {
                            icon_str.clone()
                        } else if let Some(title_str) = &boxy_theme.title {
                            // Extract just the emoji from title like "❌ Error" -> "❌"
                            let emoji_part: String = title_str.chars().take_while(|c| !c.is_ascii()).collect();
                            emoji_part.trim().to_string()
                        } else {
                            "📦".to_string()
                        }
                    };
                    text = format!("{}\n{}", theme_emoji, text);
                    // Clear icon so it doesn't get used in positioning system
                    icon = None;
                    if fixed_width.is_none() {
                        fixed_width = boxy_theme.width;
                    }
                } else {
                    eprintln!("Unknown theme: {}. Available themes:", theme_name_str);
                    let theme_list = theme_engine.list_themes();
                    let theme_names: Vec<String> = theme_list.iter().map(|(name, _)| name.clone()).collect();
                    eprintln!("  {}", theme_names.join(", "));
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to load theme engine: {}", e);
                eprintln!("Continuing without theme...");
            }
        }
    }
    
    // ⚠️  CRITICAL: DO NOT CHANGE THIS ICON LOGIC! ⚠️
    //
    // 🚨 WARNING: The icon positioning was a NIGHTMARE to get right! 🚨
    //
    // HISTORY: Originally had complex icon positioning logic in draw_box() that
    // calculated spacing, handled truncation, managed padding, etc. It was buggy,
    // spacing was inconsistent, and adding text colors broke everything.
    //
    // SOLUTION: Use the SAME unified approach as themes:
    // 1. Prepend icon to content string early: "✅ Success!"
    // 2. Clear icon variable so draw_box() uses normal (non-icon) path
    // 3. Everything flows through consistent spacing calculations
    //
    // RESULT: Perfect spacing, consistent with themes, text colors work flawlessly
    //
    // 🔥 IF YOU TOUCH THIS, YOU WILL BREAK SPACING AND HATE YOURSELF 🔥
    // 🔥 MANUAL ICONS MUST USE SAME PATTERN AS THEMES - NO EXCEPTIONS! 🔥
    //
    // Apply manual icon using the same unified approach as themes
    if let Some(manual_icon) = &icon {
        let icon_expanded = expand_variables(manual_icon);
        text = format!("{} {}", icon_expanded, text);
        // Clear icon so it doesn't get used in positioning system
        icon = None;
    }
    
    if no_boxy {
        let stripped = strip_box(&text, strict_mode);
        println!("{}", stripped);
    } else {
        draw_box(&text, 1, 1, style, color, text_color, title.as_deref(), footer.as_deref(), icon.as_deref(), fixed_width, status_bar.as_deref(), header.as_deref());
    }
}
