//repair

/// 🔒 PROTECTED ICON POSITIONING MACRO 🔒
///
/// ⚠️  CRITICAL: This macro encapsulates the ONLY working icon positioning pattern!
///
/// The icon positioning was a NIGHTMARE to get right. This macro enforces:
/// 1. Prepend icon to content string early: "✅ Success!"
/// 2. Clear icon variable so draw_box() uses normal (non-icon) path
/// 3. Everything flows through consistent spacing calculations
///
/// 🔥 NEVER MODIFY THIS MACRO OR USE ANY OTHER PATTERN! 🔥
/// 🔥 ANY DEVIATION WILL BREAK ICON SPACING IN WINDOWS TERMINAL! 🔥
macro_rules! apply_icon_to_text {
    ($text:expr, $icon:expr) => {
        if let Some(manual_icon) = &$icon {
            let icon_expanded = expand_variables(manual_icon);
            if $text.trim().is_empty() {
                $text = icon_expanded;
            } else {
                $text = format!("{} {}", icon_expanded, $text);
            }
            // Clear icon so it doesn't get used in positioning system
            $icon = None;
        }
    };
}

/// 🔒 PROTECTED THEME ICON APPLICATION 🔒
/// Uses the same safe pattern as apply_icon_to_text! macro
fn apply_theme_icon_to_text(text: &mut String, theme_icon: &str) {
    let icon_expanded = expand_variables(theme_icon);
    if text.trim().is_empty() {
        *text = icon_expanded;
    } else {
        *text = format!("{} {}", icon_expanded, text);
    }
}

mod boxes;
mod parser;
mod colors;
mod emoji_debug;
mod jynx_plugin;
mod width_plugin;
mod theme_engine;
mod themes;
mod help;
mod draw;
mod config;
mod components;

use std::io::{self, Read, Write};
use std::env;
use std::process::{Command, Stdio};
use std::fs::{self, File};

use regex::Regex;
use std::collections::HashMap;
// use unicode_width::UnicodeWidthStr;  // No longer needed - using custom implementation

use boxes::*;
use parser::*;
use colors::*;
use width_plugin::*;
use jynx_plugin::*;
use themes::*; //technically a plugin
use theme_engine::*;
use help::*;
use draw::*;
use config::*;

// RSB (Rebel String-Based) framework imports
// Note: RSB param! macro removed in favor of std::env::var for BOXY_THEME environment variable

// Simple error type for RSB integration
type AppError = String;





fn main() {
    // RSB bootstrap pattern - delegate to application logic
    match run_boxy_application() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_boxy_application() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();

    let mut style = &NORMAL;
    let mut style_from_cli = false;
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
    let mut theme_from_env = false;
    let mut enable_wrapping = false;
    
    // Check for BOXY_THEME environment variable as default (overridden by --theme)
    if let Ok(env_theme) = env::var("BOXY_THEME") {
        if !env_theme.is_empty() {
            theme_name = Some(env_theme);
            theme_from_env = true;
        }
    }
    let mut status_bar: Option<String> = None;
    let mut title_color: Option<String> = None;
    let mut status_color: Option<String> = None;
    let mut header_color: Option<String> = None;
    let mut footer_color: Option<String> = None;
    let mut header_align: &str = "center";
    let mut footer_align: &str = "center";
    let mut status_align_override: Option<String> = None;
    let mut body_align: &str = "left"; //todo: missing implementation?
    let mut body_pad_emoji = false; //todo: missing implementation?
    let mut pad_body_above = false;
    let mut pad_body_below = false;
    let mut divider_after_title = false;
    let mut divider_before_status = false;
    let mut pad_after_title_divider = false;
    let mut pad_before_status_divider = false;
    let mut pad_before_title = false;
    let mut pad_after_status = false;
    let mut pad_after_title = false;
    let mut pad_before_status = false;
    let mut skip_next = false;
    let mut params_flag: Option<String> = None;
    // Deprecated suggestions removed in v0.6.x -> simplified migration help view only
    let mut no_color_requested = false;
    

    // PRIORITY 1: Handle subcommands first - these take absolute precedence over stdin
    // Subcommands should always execute regardless of piped input
    if args.len() >= 2 && args[1] == "width" {
        handle_width_command();
        return Ok(());
    }
    
    if args.len() >= 2 && args[1] == "theme" {
        // Initialize jynx for theme commands
        let no_color = args.contains(&"--no-color".to_string()) || args.contains(&"--no-colour".to_string());
        let theme_jynx = JynxPlugin::new(no_color);
        handle_theme_command(&args[2..], &theme_jynx);
        return Ok(());
    }
    
    // Handle migrate-commands subcommand
    // if args.len() >= 2 && args[1] == "migrate-commands" {
    //     // Initialize jynx for migration commands
    //     let no_color = args.contains(&"--no-color".to_string()) || args.contains(&"--no-colour".to_string());
    //     let migrate_jynx = JynxPlugin::new(no_color);
    //     handle_migrate_command(&args[2..], &migrate_jynx);
    //     return;
    // }    
    // PRIORITY 2: Check for other subcommands that should prevent stdin reading
    // This explicit check ensures no ambiguity about input precedence
    let has_subcommand = args.len() >= 2 && matches!(args[1].as_str(), "width" | "theme" );
    if has_subcommand {
        // This should never be reached due to early returns above, but serves as a safety net
        return Ok(());
    }
    

    // Pre-scan for --no-color to initialize jynx properly
    for arg in args.iter().skip(1) {
        if arg == "--no-color" || arg == "--no-colour" {
            no_color_requested = true;
            break;
        }
    }
    
    // Initialize jynx integration early
    let jynx = JynxPlugin::new(no_color_requested);
    
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match arg.as_str() {
            "--help" | "-h" => {
                show_comprehensive_help(&jynx);
                return Ok(());
            }
            "--version" | "-v" | "-V" => {
                println!("{} {} ({})", NAME, VERSION, jynx.get_version_string());
                return Ok(());
            }
            "--colors" => {
                println!("{} {} - Color Palette Preview", NAME, VERSION);
                println!("{}", generate_color_help());
                return Ok(());
            }
            "--examples" => {
                show_usage_examples();
                return Ok(());
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
                    style_from_cli = true;
                    skip_next = true;
                }
            }
            "--params" => {
                if i + 1 < args.len() {
                    params_flag = Some(args[i + 1].clone());
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
                            return Err(format!("Invalid color: {}", error_msg));
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
                            return Err(format!("Invalid text color: {}", error_msg));
                        }
                    }
                }
            }
            "--width" | "-w" => {
                if i + 1 < args.len() {
                    let warg = &args[i + 1];
                    if warg.eq_ignore_ascii_case("max") {
                        fixed_width = Some(get_terminal_width());
                        skip_next = true;
                    } else if warg.eq_ignore_ascii_case("auto") {
                        fixed_width = None; // let auto-sizing decide
                        skip_next = true;
                    } else {
                        match warg.parse::<usize>() {
                            Ok(w) if w >= 4 => {
                                fixed_width = Some(w);
                                skip_next = true;
                            }
                            _ => {
                                eprintln!("Error: Width must be <number>=4, or 'max'/'auto'");
                                return Err("Invalid width specification".to_string());
                            }
                        }
                    }
                }
            }
            "--theme" => {
                if i + 1 < args.len() {
                    theme_name = Some(args[i + 1].clone());
                    theme_from_env = false; // CLI theme overrides environment
                    skip_next = true;
                }
            }
            "--wrap" => {
                enable_wrapping = true;
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
                    // if !status_text.starts_with("sl:") && !status_text.starts_with("sc:") && !status_text.starts_with("sr:") &&
                    //    !status_text.starts_with("hl:") && !status_text.starts_with("hc:") && !status_text.starts_with("hr:") &&
                    //    !status_text.starts_with("fl:") && !status_text.starts_with("fc:") && !status_text.starts_with("fr:") &&
                    //    get_display_width(status_text) > 50 {
                    //     deprecation_warnings.push(format!(
                    //         "Long status text without alignment prefix. Consider using sl:, sc:, or sr: prefixes for better control."
                    //     ));
                    //}
                    
                    skip_next = true;
                }
            }
            "--title-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { title_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--status-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { status_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--header-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { header_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--footer-color" => {
                if i + 1 < args.len() {
                    let c = &args[i + 1];
                    if validate_color(c).is_ok() { footer_color = Some(c.clone()); }
                    skip_next = true;
                }
            }
            "--layout" => {
                if i + 1 < args.len() {
                    let spec = &args[i + 1];
                    for token in spec.split(',') {
                        match token.trim() {
                            "hl" => header_align = "left",
                            "hc" => header_align = "center",
                            "hr" => header_align = "right",
                            "fl" => footer_align = "left",
                            "fc" => footer_align = "center",
                            "fr" => footer_align = "right",
                            "sl" => status_align_override = Some("left".to_string()),
                            "sc" => status_align_override = Some("center".to_string()),
                            "sr" => status_align_override = Some("right".to_string()),
                            "bl" => body_align = "left",
                            "bc" => body_align = "center",
                            "br" => body_align = "right",
                            "bp" => body_pad_emoji = true,
                            "dt" => divider_after_title = true,
                            "ds" => divider_before_status = true,
                            "dtn" => { divider_after_title = true; pad_after_title_divider = true; },
                            "dsn" => { divider_before_status = true; pad_before_status_divider = true; },
                            "stn" => { pad_before_title = true; },
                            "ssn" => { pad_after_status = true; },
                            "ptn" => { pad_after_title = true; },
                            "psn" => { pad_before_status = true; },
                            _ => { /* ignore unknown tokens */ }
                        }
                    }
                    skip_next = true;
                }
            }
            "--pad" => {
                if i + 1 < args.len() {
                    for t in args[i+1].split(',') {
                        match t.trim() {
                            "a"|"above" => pad_body_above = true,
                            "b"|"below" => pad_body_below = true,
                            _ => {}
                        }
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
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }
    


    
    // PRIORITY 3: Read from stdin only if no subcommands were processed
    // At this point, all subcommands and utility flags (--help, --version, etc.) have been handled
    // This ensures clear precedence: subcommands > utility flags > stdin processing
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let mut text = input.trim_end_matches('\n').to_string();

    // Params stream parsing: ONLY via --params flag. Piped stdin remains the body.
    if let Some(ref blob) = params_flag {
        if let Some(pc) = parse_content_stream(blob) {
            if header.is_none() { header = pc.header; }
            if footer.is_none() { footer = pc.footer; }
            if status_bar.is_none() { status_bar = pc.status; }
            if title.is_none() { title = pc.title; }
            if let Some(ic) = pc.icon { icon = Some(ic); }
            if title_color.is_none() { title_color = pc.title_color; }
            if status_color.is_none() { status_color = pc.status_color; } // ? why
            if header_color.is_none() { header_color = pc.header_color; }
            if footer_color.is_none() { footer_color = pc.footer_color; }
            // Map layout tokens if provided via params
            if let Some(spec) = pc.layout.as_deref() {
                for token in spec.split(',') {
                    match token.trim() {
                        "hl" => header_align = "left",
                        "hc" => header_align = "center",
                        "hr" => header_align = "right",
                        "fl" => footer_align = "left",
                        "fc" => footer_align = "center",
                        "fr" => footer_align = "right",
                        "sl" => status_align_override = Some("left".to_string()),
                        "sc" => status_align_override = Some("center".to_string()),
                        "sr" => status_align_override = Some("right".to_string()),
                        "bl" => body_align = "left",
                        "bc" => body_align = "center",
                        "br" => body_align = "right",
                        "bp" => body_pad_emoji = true,
                        "dt" => divider_after_title = true,
                        "ds" => divider_before_status = true,
                        "dtn" => { divider_after_title = true; pad_after_title_divider = true; },
                        "dsn" => { divider_before_status = true; pad_before_status_divider = true; },
                        "stn" => { pad_before_title = true; },
                        "ssn" => { pad_after_status = true; },
                        "ptn" => { pad_after_title = true; },
                        "psn" => { pad_before_status = true; },
                        _ => {}
                    }
                }
            }
            // Body remains the piped stdin text
        }
    }
    
    //TODO: refactor to themes.rs => handle_theme_enigne(&theme_name)
    //      needs to return the right values for icon, fixed_width etc.
    // Apply theme if specified - using new theme engine
    if let Some(theme_name_str) = &theme_name {
        match ThemeEngine::new() {
            Ok(theme_engine) => {
                if let Some(boxy_theme) = theme_engine.get_theme(theme_name_str) {
                    // Theme overrides defaults but explicit flags override theme
                    if color == "none" {
                        color = Box::leak(boxy_theme.color.clone().into_boxed_str());
                    }
                    if text_color == "none" {
                        text_color = Box::leak(boxy_theme.text_color.clone().into_boxed_str());
                    }
                    // Apply theme icon directly to text using safe pattern (no icon variable)
                    if icon.is_none() {
                        if let Some(icon_str) = &boxy_theme.icon {
                            apply_theme_icon_to_text(&mut text, icon_str);
                        } else if let Some(title_str) = &boxy_theme.title {
                            let emoji_part: String = title_str.chars().take_while(|c| !c.is_ascii()).collect();
                            if !emoji_part.trim().is_empty() {
                                apply_theme_icon_to_text(&mut text, emoji_part.trim());
                            }
                        }
                    }
                    if fixed_width.is_none() {
                        fixed_width = boxy_theme.width;
                    }
                    // Apply theme style if not overridden by CLI
                    if !style_from_cli {
                        style = match boxy_theme.style.as_str() {
                            "rounded" => &ROUNDED,
                            "double" => &DOUBLE,
                            "heavy" => &HEAVY,
                            "ascii" => &ASCII,
                            "normal" => &NORMAL,
                            _ => &NORMAL,
                        };
                    }
                } else {
                    if theme_from_env {
                        // For environment themes, warn but continue with default
                        eprintln!("Warning: Unknown theme '{}' from BOXY_THEME environment variable.", theme_name_str);
                        eprintln!("Falling back to default theme.");
                        // Reset theme_name to None to use default behavior
                        #[allow(unused_assignments)]
                        {
                            theme_name = None;
                        }
                    } else {
                        // For CLI themes, show error and exit
                        eprintln!("Unknown theme: {}. Available themes:", theme_name_str);
                        let theme_list = theme_engine.list_themes();
                        let theme_names: Vec<String> = theme_list.iter().map(|(name, _)| name.clone()).collect();
                        eprintln!("  {}", theme_names.join(", "));
                        return Err(format!("Unknown theme: {}", theme_name_str));
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to load theme engine: {}", e);
                eprintln!("Continuing without theme...");
            }
        }
    }
    
    let _status_color_str = status_color.as_deref().unwrap_or("");
    // DEBUG: Status color selection (commented for clean output)
    // eprintln!("Status Color: {}", status_color_str);

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
    // 🔒 PROTECTED ICON POSITIONING - DO NOT MODIFY MANUALLY! 🔒
    apply_icon_to_text!(text, icon);


    if no_boxy {
        let stripped = strip_box(&text, strict_mode);
        println!("{}", stripped);
    } else {
        let config = resolve_box_config(
          &text, 
          1, 1, 
          style, color, text_color, 
          title.as_deref(), 
          footer.as_deref(), 
          icon.as_deref(), 
          fixed_width, 
          status_bar.as_deref(), 
          header.as_deref(), 
          header_align, footer_align, 
          status_align_override.as_deref(), 
          divider_after_title, divider_before_status, 
          pad_after_title_divider, pad_before_status_divider, 
          pad_before_title, pad_after_status, 
          pad_after_title, pad_before_status, 
          title_color.as_deref(), 
          status_color.as_deref(), 
          body_align,
          body_pad_emoji, 
          pad_body_above,
          pad_body_below,
          header_color.as_deref(),
          footer_color.as_deref(),
          enable_wrapping
        );
        draw_box(config);
    }
    
    Ok(())
}

// KEEP THE COMMENTS BELOW THIS POINT, THEY ARE HERE FOR REPAIR REFERENCE

// fn draw_box(
//   text: &str, 
//   h_padding: usize, 
//   _v_padding: usize, 
//   style: &BoxStyle, 
//   color: &str, 
//   text_color: &str, 
//   title: Option<&str>, 
//   footer: Option<&str>, 
//   icon: Option<&str>, 
//   fixed_width: Option<usize>, 
//   status_bar: Option<&str>, 
//   header: Option<&str>, 
//   header_align: &str, 
//   footer_align: &str, 
//   status_align_override: Option<&str>, 
//   divider_after_title: bool, 
//   divider_before_status: bool, 
//   pad_after_title_divider: bool, 
//   pad_before_status_divider: bool, 
//   pad_before_title: bool, 
//   pad_after_status: bool, 
//   pad_after_title: bool, 
//   pad_before_status: bool, 
//   title_color_name: Option<&str>, 
//   status_color_name: Option<&str>, 
//   body_align: Option<&str>, 
//   body_pad_emoji: Option<&str>, 
//   pad_body_above: bool, 
//   pad_body_below: bool
// )

