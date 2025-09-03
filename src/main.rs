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
    
    let text_width = get_display_width(text);
    let available_width = total_width.saturating_sub(2); // Space for " text "
    
    let final_text = if text_width > available_width {
        truncate_with_ellipsis(text, available_width)
    } else {
        text.to_string()
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

fn draw_box(text: &str, h_padding: usize, v_padding: usize, style: &BoxStyle, color: &str, text_color: &str, title: Option<&str>, footer: Option<&str>, icon: Option<&str>, fixed_width: Option<usize>, status_bar: Option<&str>) {
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

/// Handle theme subcommands: list, show, etc.
fn handle_theme_command(args: &[String]) {
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
                    
                    println!("{} {} - Available Themes", NAME, VERSION);
                    println!();
                    
                    for (name, description) in themes {
                        // Format similar to jynx: theme_name - description
                        println!("  {} - {}", name, description);
                    }
                    
                    println!();
                    println!("Usage: {} --theme <theme_name>", NAME);
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
            handle_theme_create(&args[1]);
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

/// Enhanced theme validation for interactive creation
fn validate_interactive_input(prompt: &str, value: &str, validator: fn(&str) -> Result<(), String>) -> Result<String, String> {
    if value.is_empty() {
        return Ok(value.to_string()); // Allow empty for optional fields
    }
    
    validator(value)?;
    Ok(value.to_string())
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
fn handle_theme_create(name: &str) {
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
            println!("✅ Theme '{}' created successfully!", name);
            println!("   Saved to: {}", theme_file_path.display());
            println!();
            println!("Test your theme:");
            println!("   echo \"Hello World\" | boxy --theme {}", name);
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

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Handle theme subcommands first
    if args.len() >= 2 && args[1] == "theme" {
        handle_theme_command(&args[2..]);
        return;
    }
    
    let mut style = &NORMAL;
    let mut color = "none";
    let mut text_color = "none";
    let mut title: Option<String> = None;
    let mut footer: Option<String> = None;
    let mut icon: Option<String> = None;
    let mut no_boxy = false;
    let mut strict_mode = false;
    let mut fixed_width: Option<usize> = None;
    let mut theme_name: Option<String> = None;
    let mut status_bar: Option<String> = None;
    let mut skip_next = false;
    
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{} {} - {}", NAME, VERSION, DESCRIPTION);
                println!("\nUSAGE:");
                println!("    echo \"text\" | {} [OPTIONS]", NAME);
                println!("    command | {} --no-boxy          Strip box decoration", NAME);
                println!("\nOPTIONS:");
                println!("    -s, --style <STYLE>      Box style [normal, rounded, double, heavy, ascii]");
                println!("    -c, --color <COLOR>      Box color - 90+ colors available including:");
                println!("                             Legacy: red, green, blue, orange, purple, etc.");
                println!("                             Semantic: error, success, warning, info, critical");
                println!("                             Rich: crimson, emerald, azure, amber, violet");
                println!("    --text <COLOR>           Text color [same colors as above] or 'auto' to match box color");
                println!("    -w, --width <WIDTH>      Fixed box width (content truncated with … if needed)");
                println!("    --theme <THEME>          Apply predefined theme (sets icon, color, width)");
                println!("    --title <TEXT>           Add title to top border (supports $VAR expansion)");
                println!("    --footer <TEXT>          Add footer to bottom border");
                println!("    --icon <ICON>            Add icon/emoji to first content line");
                println!("    --status <TEXT>          Add status bar below box (auto-truncated to terminal width)");
                println!("    --no-boxy[=strict]       Strip box decoration (strict: remove all formatting)");
                println!("    -h, --help               Show this help message");
                println!("    --colors                 Show all available colors with preview");
                println!("    -v, -V, --version        Show version");
                println!("\nEXAMPLES:");
                println!("    echo \"Hello\" | {}", NAME);
                println!("    echo \"Hello\" | {} --style rounded --color blue", NAME);
                println!("    echo \"Hello\" | {} --title \"🚀 My App\" --footer \"v1.0\"", NAME);
                println!("    echo \"Long text here\" | {} --width 20", NAME);
                println!("    echo \"Something went wrong\" | {} --theme error", NAME);
                println!("    echo \"Build successful\" | {} --theme success", NAME);
                println!("    echo \"Test\" | {} | {} --no-boxy", NAME, NAME);
                return;
            }
            "--version" | "-v" | "-V" => {
                println!("{} {}", NAME, VERSION);
                return;
            }
            "--colors" => {
                println!("{} {} - Color Palette Preview", NAME, VERSION);
                println!("{}", generate_color_help());
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
                    status_bar = Some(args[i + 1].clone());
                    skip_next = true;
                }
            }
            arg if arg.starts_with("--no-boxy") => {
                no_boxy = true;
                if arg == "--no-boxy=strict" {
                    strict_mode = true;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!("Try '{} --help' for more information", NAME);
                std::process::exit(1);
            }
        }
    }
    
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
        draw_box(&text, 1, 1, style, color, text_color, title.as_deref(), footer.as_deref(), icon.as_deref(), fixed_width, status_bar.as_deref());
    }
}
