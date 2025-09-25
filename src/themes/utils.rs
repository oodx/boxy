//! Public theme utilities
//!
//! This module contains the public API for theme operations that users may
//! explicitly use. Functions here are the main interface for theme functionality.

use crate::theme_engine::{BoxyTheme, ThemeEngine, ThemeFile, ThemeMetadata, ThemeSettings};
use crate::{HashMap, Write, core::*, visual::*};
use crate::{JynxPlugin, jynx_println};
use crate::{fs, io};
use crate::{validate_color, validate_width};
use std::path::PathBuf;

use super::helpers::*;

/// Enhanced validation for theme files before import (ENGINE-014)
pub fn validate_theme_file(path: &PathBuf) -> Result<(), String> {
    validate_theme_file_with_duplicate_check(path, true)
}

/// Comprehensive theme file validation with optional duplicate checking
pub fn validate_theme_file_with_duplicate_check(
    path: &PathBuf,
    check_duplicates: bool,
) -> Result<(), String> {
    // Enhanced YAML structure validation
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read theme file: {}", e))?;

    // Pre-validate YAML structure
    if content.trim().is_empty() {
        return Err("Theme file is empty".to_string());
    }

    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&content).map_err(|e| format!("Invalid YAML format: {}", e))?;

    // Validate top-level structure
    if !yaml_value.is_mapping() {
        return Err("Theme file must be a YAML mapping (key-value structure)".to_string());
    }

    let theme_file: ThemeFile = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse theme file structure: {}", e))?;

    let temp_engine =
        ThemeEngine::new().map_err(|e| format!("Failed to initialize validator: {}", e))?;

    let mut validation_errors = Vec::new();
    let mut warnings = Vec::new();

    // Enhanced metadata validation
    if theme_file.metadata.name.is_empty() {
        validation_errors.push("Missing or empty metadata.name".to_string());
    }

    if theme_file.metadata.version.is_empty() {
        validation_errors.push("Missing or empty metadata.version".to_string());
    }

    // Validate metadata.version format (basic semantic versioning check)
    if !theme_file.metadata.version.is_empty() {
        let version_regex = regex::Regex::new(r"^\d+\.\d+\.\d+").unwrap();
        if !version_regex.is_match(&theme_file.metadata.version) {
            warnings.push(format!(
                "Version '{}' doesn't follow semantic versioning (e.g., '1.0.0')",
                theme_file.metadata.version
            ));
        }
    }

    // Check for empty themes section
    if theme_file.themes.is_empty() {
        validation_errors.push("No themes defined in file - 'themes' section is empty".to_string());
    }

    // Enhanced theme validation with required properties checking
    for (theme_name, theme) in &theme_file.themes {
        // Validate theme name
        if theme_name.is_empty() {
            validation_errors.push("Empty theme name found".to_string());
            continue;
        }

        // Check for reserved theme names
        let reserved_names = vec!["none", "auto", "default", "base", "template"];
        if reserved_names.contains(&theme_name.as_str()) {
            warnings.push(format!(
                "Theme '{}' uses a reserved name - may conflict with built-in themes",
                theme_name
            ));
        }

        // Required properties validation
        let mut missing_required = Vec::new();

        // Color is required (unless it's a template/base theme)
        if theme.color.is_empty()
            && !theme_name.contains("template")
            && !theme_name.contains("base")
        {
            missing_required.push("color");
        }

        // Style is required
        if theme.style.is_empty() {
            missing_required.push("style");
        }

        if !missing_required.is_empty() {
            validation_errors.push(format!(
                "Theme '{}': Missing required properties: {}",
                theme_name,
                missing_required.join(", ")
            ));
        }

        // Validate theme using engine validator
        if let Err(e) = temp_engine.validate_theme(theme) {
            validation_errors.push(format!("Theme '{}': {}", theme_name, e));
        }

        // Additional validation checks
        // Note: Description validation would go here if BoxyTheme had a description field
        // Currently BoxyTheme stores description in metadata at the file level
    }

    // Duplicate theme names detection across existing configs
    if check_duplicates {
        if let Ok(engine) = ThemeEngine::new() {
            for theme_name in theme_file.themes.keys() {
                if engine.get_theme(theme_name).is_some() {
                    warnings.push(format!(
                        "Theme '{}' already exists in loaded configurations - will be overridden",
                        theme_name
                    ));
                }
            }
        }
    }

    // Report errors and warnings
    if !validation_errors.is_empty() {
        return Err(format!(
            "❌ Validation errors:\n  • {}",
            validation_errors.join("\n  • ")
        ));
    }

    if !warnings.is_empty() {
        eprintln!("⚠️  Validation warnings:");
        for warning in &warnings {
            eprintln!("  • {}", warning);
        }
        eprintln!();
    }

    Ok(())
}

/// Handle theme subcommands: list, show, etc.
pub fn handle_theme_command(args: &[String], jynx: &JynxPlugin, opt_dev_level: Option<u8>) {
    if args.is_empty() {
        eprintln!(
            "Theme command requires an action. Usage: {} theme <action>",
            NAME
        );
        eprintln!("Available actions: list, browse, preview <theme>, show <theme>, hierarchy, dryrun <theme>, init, help");
        std::process::exit(1);
    }

    match args[0].as_str() {
        "list" => {
            handle_enhanced_theme_list(opt_dev_level, jynx);
        }
        "create" => {
            if args.len() < 2 {
                eprintln!(
                    "Error: Theme create requires a name. Usage: {} theme create <name>",
                    NAME
                );
                std::process::exit(1);
            }
            handle_theme_create(&args[1], jynx);
        }
        "import" => {
            if args.len() < 2 {
                eprintln!(
                    "Error: Theme import requires a path. Usage: {} theme import <path>",
                    NAME
                );
                std::process::exit(1);
            }
            handle_theme_import(&args[1]);
        }
        "export" => {
            if args.len() < 2 {
                eprintln!(
                    "Error: Theme export requires a name. Usage: {} theme export <name>",
                    NAME
                );
                std::process::exit(1);
            }
            handle_theme_export(&args[1]);
        }
        "edit" => {
            if args.len() < 2 {
                eprintln!(
                    "Error: Theme edit requires a name. Usage: {} theme edit <name>",
                    NAME
                );
                std::process::exit(1);
            }
            handle_theme_edit(&args[1]);
        }
        "hierarchy" => match ThemeEngine::new_with_override(opt_dev_level) {
            Ok(theme_engine) => {
                theme_engine.print_theme_hierarchy();
            }
            Err(e) => {
                eprintln!("Error: Failed to load theme engine: {}", e);
                std::process::exit(1);
            }
        },
        "init" => {
            handle_theme_init();
        }
        "browse" => {
            handle_theme_browse(opt_dev_level, jynx);
        }
        "preview" => {
            if args.len() < 2 {
                eprintln!(
                    "Error: Theme preview requires a theme name. Usage: {} theme preview <theme>",
                    NAME
                );
                std::process::exit(1);
            }
            handle_theme_preview(&args[1]);
        }
        "dryrun" | "test" => {
            if args.len() < 2 {
                eprintln!(
                    "Error: Theme dryrun requires a theme name. Usage: {} theme dryrun <theme>",
                    NAME
                );
                std::process::exit(1);
            }
            handle_theme_dryrun(&args[1]);
        }
        "help" | "--help" => {
            print_theme_help();
        }
        action => {
            eprintln!("Unknown theme action: {}", action);
            eprintln!(
                "Available actions: list, browse, preview, show, hierarchy, dryrun, init, create, import, export, edit, help"
            );
            eprintln!("Use '{} theme help' for more information", NAME);
            std::process::exit(1);
        }
    }
}

/// Handle engine subcommands: init, import, export, list, debug, etc.
pub fn handle_engine_command(args: &[String], _jynx: &JynxPlugin, opt_dev_level: Option<u8>) {
    if args.is_empty() {
        eprintln!("❌ Engine command requires an action");
        eprintln!();
        eprintln!("📖 Usage: {} engine <ACTION>", NAME);
        eprintln!();
        eprintln!("🔧 Available actions:");
        eprintln!("   init              Initialize theme system and create default themes");
        eprintln!("   import <name>     Import boxy_<name>.yml from current directory");
        eprintln!("   export <name>     Export theme to boxy_<name>.yml file");
        eprintln!("   list              Visual catalog of all available themes");
        eprintln!("   debug             Show theme loading hierarchy and diagnostics");
        eprintln!("   status            Quick engine health check");
        eprintln!("   validate <file>   Comprehensive theme file validation");
        eprintln!("   edit <name>       Edit a theme configuration file");
        eprintln!("   help              Show detailed help information");
        eprintln!();
        eprintln!("💡 Examples:");
        eprintln!("   {} engine status         # Check system health", NAME);
        eprintln!(
            "   {} engine list           # Browse available themes",
            NAME
        );
        eprintln!("   {} engine init           # Set up theme system", NAME);
        std::process::exit(1);
    }

    match args[0].as_str() {
        "init" => {
            handle_engine_init();
        }
        "list" => match ThemeEngine::new_with_override(opt_dev_level) {
            Ok(theme_engine) => {
                handle_engine_list_enhanced(&theme_engine);
            }
            Err(e) => {
                eprintln!("Error: Failed to load theme engine: {}", e);
                std::process::exit(1);
            }
        },
        "import" => {
            if args.len() < 2 {
                eprintln!("❌ Engine import requires a theme name");
                eprintln!();
                eprintln!(
                    "📖 Usage: {} engine import <NAME> [--overwrite] [--dry-run]",
                    NAME
                );
                eprintln!();
                eprintln!("🔍 What this does:");
                eprintln!("   • Looks for 'boxy_<NAME>.yml' in current directory");
                eprintln!("   • Copies to global themes directory");
                eprintln!("   • Validates YAML structure before importing");
                eprintln!();
                eprintln!("💡 Examples:");
                eprintln!(
                    "   {} engine import my_theme        # Import boxy_my_theme.yml",
                    NAME
                );
                eprintln!(
                    "   {} engine import work --overwrite # Force overwrite existing",
                    NAME
                );
                eprintln!(
                    "   {} engine import test --dry-run   # Preview import without changes",
                    NAME
                );
                eprintln!();
                eprintln!("🔧 Need help? {} engine help", NAME);
                std::process::exit(1);
            }
            let force_overwrite =
                args.contains(&"--overwrite".to_string()) || args.contains(&"--force".to_string());
            let dry_run = args.contains(&"--dry-run".to_string());
            handle_engine_import(&args[1], force_overwrite, dry_run);
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("❌ Engine export requires a theme name");
                eprintln!();
                eprintln!(
                    "📖 Usage: {} engine export <NAME> [--overwrite] [--dry-run]",
                    NAME
                );
                eprintln!();
                eprintln!("🔍 What this does:");
                eprintln!("   • Finds 'boxy_<NAME>.yml' in global themes directory");
                eprintln!("   • Copies to current directory as 'boxy_<NAME>.yml'");
                eprintln!("   • Creates backup (.bak) if overwriting existing file");
                eprintln!();
                eprintln!("💡 Examples:");
                eprintln!(
                    "   {} engine export default           # Export boxy_default.yml",
                    NAME
                );
                eprintln!(
                    "   {} engine export custom --overwrite # Force overwrite",
                    NAME
                );
                eprintln!(
                    "   {} engine export theme --dry-run    # Preview export without changes",
                    NAME
                );
                eprintln!();
                eprintln!("🔧 Available themes: {} engine list", NAME);
                std::process::exit(1);
            }
            let force_overwrite =
                args.contains(&"--overwrite".to_string()) || args.contains(&"--force".to_string());
            let dry_run = args.contains(&"--dry-run".to_string());
            handle_engine_export(&args[1], force_overwrite, dry_run);
        }
        "edit" => {
            if args.len() < 2 {
                eprintln!(
                    "Error: Engine edit requires a name. Usage: {} engine edit <name>",
                    NAME
                );
                std::process::exit(1);
            }
            handle_engine_edit(&args[1]);
        }
        "debug" => match ThemeEngine::new() {
            Ok(theme_engine) => {
                theme_engine.print_theme_hierarchy();
            }
            Err(e) => {
                eprintln!("Error: Failed to load theme engine: {}", e);
                std::process::exit(1);
            }
        },
        "status" => {
            handle_engine_status();
        }
        "validate" => {
            if args.len() < 2 {
                eprintln!("❌ Engine validate requires a theme file path");
                eprintln!();
                eprintln!("📖 Usage: {} engine validate <FILE>", NAME);
                eprintln!();
                eprintln!("🔍 What this does:");
                eprintln!("   • Validates YAML structure and theme definitions");
                eprintln!("   • Checks for required properties and valid values");
                eprintln!("   • Detects duplicate theme names across configs");
                eprintln!("   • Reports warnings for potential issues");
                eprintln!();
                eprintln!("💡 Examples:");
                eprintln!("   {} engine validate boxy_custom.yml", NAME);
                eprintln!("   {} engine validate themes/my_theme.yml", NAME);
                std::process::exit(1);
            }
            handle_engine_validate(&args[1]);
        }
        "help" | "--help" => {
            print_engine_help();
        }
        _ => {
            let action = &args[0];
            eprintln!("❌ Unknown engine action: '{}'", action);
            eprintln!();
            eprintln!("🔧 Available actions:");
            eprintln!("   init      Initialize theme system");
            eprintln!("   import    Import theme configuration");
            eprintln!("   export    Export theme configuration");
            eprintln!("   list      Show visual theme catalog");
            eprintln!("   debug     Show detailed diagnostics");
            eprintln!("   status    Quick health check");
            eprintln!("   validate  Comprehensive theme file validation");
            eprintln!("   edit      Edit theme configuration");
            eprintln!("   help      Show detailed help");
            eprintln!();
            eprintln!("💡 Get help: {} engine help", NAME);
            eprintln!("🔍 Check status: {} engine status", NAME);
            std::process::exit(1);
        }
    }
}

/// Validate theme name
pub fn validate_theme_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Theme name cannot be empty".to_string());
    }

    if name.len() > 50 {
        return Err("Theme name too long (max 50 characters)".to_string());
    }

    // Allow alphanumeric, underscore, hyphen, and dot
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return Err(
            "Theme name can only contain letters, numbers, underscore, hyphen, and dot".to_string(),
        );
    }

    // Don't allow names that start with reserved prefixes
    let reserved_prefixes = vec!["builtin_", "system_", "default_"];
    for prefix in reserved_prefixes {
        if name.starts_with(prefix) {
            return Err(format!(
                "Theme name cannot start with reserved prefix '{}'",
                prefix
            ));
        }
    }

    Ok(())
}

/// Interactive theme creation utility
pub fn create_theme_interactively(name: &str) -> BoxyTheme {
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
    let text_color = if text_color.is_empty() {
        "auto"
    } else {
        text_color
    };

    // Style selection with validation loop
    let style = loop {
        print!("Border style [normal/rounded/double/heavy/ascii]: ");
        io::stdout().flush().unwrap();
        let mut style_input = String::new();
        io::stdin().read_line(&mut style_input).unwrap();
        let style = style_input.trim();
        let style = if style.is_empty() { "normal" } else { style };

        if let Err(e) = validate_box_style(style) {
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
    let icon = if icon.is_empty() {
        None
    } else {
        Some(icon.to_string())
    };

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
pub fn edit_theme_interactively(name: &str, existing: &BoxyTheme) -> BoxyTheme {
    println!("Edit theme '{}' (press Enter to keep current value):", name);
    println!();

    // Color
    print!("Box color [current: {}]: ", existing.color);
    io::stdout().flush().unwrap();
    let mut color_input = String::new();
    io::stdin().read_line(&mut color_input).unwrap();
    let color = color_input.trim();
    let color = if color.is_empty() {
        &existing.color
    } else {
        color
    };

    // Text color
    print!("Text color [current: {}]: ", existing.text_color);
    io::stdout().flush().unwrap();
    let mut text_color_input = String::new();
    io::stdin().read_line(&mut text_color_input).unwrap();
    let text_color = text_color_input.trim();
    let text_color = if text_color.is_empty() {
        &existing.text_color
    } else {
        text_color
    };

    // Style
    print!("Border style [current: {}]: ", existing.style);
    io::stdout().flush().unwrap();
    let mut style_input = String::new();
    io::stdin().read_line(&mut style_input).unwrap();
    let style = style_input.trim();
    let style = if style.is_empty() {
        &existing.style
    } else {
        style
    };

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
    let current_width = existing
        .width
        .map(|w| w.to_string())
        .unwrap_or_else(|| "auto".to_string());
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
pub fn save_theme_to_file(path: &PathBuf, name: &str, theme: &BoxyTheme) -> Result<(), String> {
    let theme_file = ThemeFile {
        metadata: ThemeMetadata {
            name: format!("{} Theme File", name),
            version: "1.0.0".to_string(),
            description: format!("Custom theme: {}", name),
            author: {
                let user = std::env::var("USER").unwrap_or_else(|_| "boxy".to_string());
                user
            },
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

    fs::write(path, yaml_content).map_err(|e| format!("Failed to write theme file: {}", e))?;

    Ok(())
}

/// Export theme to YAML format
pub fn export_theme_to_yaml(name: &str, theme: &BoxyTheme) -> String {
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

/// Legacy theme structure for v0.5.0 compatibility
#[derive(Debug, Clone)]
pub struct Theme {
    #[allow(dead_code)]
    pub icon: &'static str,
    #[allow(dead_code)]
    pub color: &'static str,
    #[allow(dead_code)]
    pub width: Option<usize>,
}

/// Get legacy themes (v0.5.0 style) - converts from new YAML system when available
#[allow(dead_code)]
pub fn get_themes() -> HashMap<&'static str, Theme> {
    let mut themes = HashMap::new();

    // Try to load from new theme engine first
    if let Ok(engine) = ThemeEngine::new() {
        // Convert new BoxyTheme format to legacy Theme format
        for (name, _description) in engine.list_themes() {
            if let Some(boxy_theme) = engine.get_theme(&name) {
                let legacy_theme = convert_boxy_theme_to_legacy(boxy_theme);
                let static_name: &'static str = Box::leak(name.into_boxed_str());
                themes.insert(static_name, legacy_theme);
            }
        }

        // If we got themes from the engine, return them
        if !themes.is_empty() {
            return themes;
        }
    }

    // Fallback to hardcoded legacy themes if YAML system fails
    get_fallback_legacy_themes()
}

/// Convert new BoxyTheme to legacy Theme format
#[allow(dead_code)]
pub fn convert_boxy_theme_to_legacy(boxy_theme: BoxyTheme) -> Theme {
    let icon = boxy_theme.icon.as_deref().unwrap_or("📦");
    Theme {
        icon: Box::leak(icon.to_string().into_boxed_str()),
        color: Box::leak(boxy_theme.color.into_boxed_str()),
        width: boxy_theme.width,
    }
}

/// Hardcoded legacy themes as fallback (preserves v0.5.0 exact compatibility)
#[allow(dead_code)]
pub fn get_fallback_legacy_themes() -> HashMap<&'static str, Theme> {
    let mut themes = HashMap::new();

    themes.insert(
        "fatal",
        Theme {
            icon: "💀",
            color: "red2",
            width: None,
        },
    );
    themes.insert(
        "error",
        Theme {
            icon: "❌",
            color: "red",
            width: None,
        },
    );
    themes.insert(
        "warn",
        Theme {
            icon: "⚠️",
            color: "orange",
            width: None,
        },
    );
    themes.insert(
        "success",
        Theme {
            icon: "✅",
            color: "green",
            width: None,
        },
    );
    themes.insert(
        "info",
        Theme {
            icon: "ℹ️",
            color: "blue2",
            width: None,
        },
    );
    themes.insert(
        "debug",
        Theme {
            icon: "🐛",
            color: "grey",
            width: None,
        },
    );
    themes.insert(
        "trace",
        Theme {
            icon: "👣",
            color: "grey2",
            width: None,
        },
    );
    themes.insert(
        "dev",
        Theme {
            icon: "🪛",
            color: "cyan",
            width: None,
        },
    );
    themes.insert(
        "new",
        Theme {
            icon: "✨",
            color: "green2",
            width: None,
        },
    );
    themes.insert(
        "silly",
        Theme {
            icon: "🪀",
            color: "purple",
            width: None,
        },
    );
    themes.insert(
        "magic",
        Theme {
            icon: "🌈",
            color: "purple2",
            width: None,
        },
    );
    themes.insert(
        "think",
        Theme {
            icon: "💭",
            color: "cyan",
            width: None,
        },
    );
    themes.insert(
        "notif",
        Theme {
            icon: "📣",
            color: "green",
            width: None,
        },
    );
    themes.insert(
        "lore",
        Theme {
            icon: "🪬",
            color: "grey",
            width: None,
        },
    );
    themes.insert(
        "blocked",
        Theme {
            icon: "🚧",
            color: "orange",
            width: None,
        },
    );
    themes.insert(
        "help",
        Theme {
            icon: "💡",
            color: "blue2",
            width: None,
        },
    );
    themes.insert(
        "oops",
        Theme {
            icon: "👻",
            color: "purple",
            width: None,
        },
    );
    themes.insert(
        "lab",
        Theme {
            icon: "🧪",
            color: "cyan",
            width: None,
        },
    );
    themes.insert(
        "lock",
        Theme {
            icon: "🔒",
            color: "grey2",
            width: None,
        },
    );
    themes.insert(
        "unlock",
        Theme {
            icon: "🔓",
            color: "green",
            width: None,
        },
    );
    themes.insert(
        "key",
        Theme {
            icon: "🔑",
            color: "orange",
            width: None,
        },
    );

    themes
}

/// Print theme help (ENGINE-002: Separate help menus)
pub fn print_theme_help() {
    println!("{} {} - Theme Usage Commands", NAME, VERSION);
    println!();
    println!("USAGE:");
    println!("    {} theme <COMMAND> [OPTIONS]", NAME);
    println!();
    println!("COMMANDS:");
    println!("    list              Enhanced categorized theme list with visual previews");
    println!("    browse            Interactive theme browser with live previews");
    println!("    preview <name>    Comprehensive theme preview with multiple samples");
    println!("    show <name>       Show properties of a specific theme");
    println!("    dryrun <name>     Test theme with sample content");
    println!("    create <name>     Create new theme within a config file");
    println!("    hierarchy         Show theme loading hierarchy (legacy)");
    println!("    init              Initialize local .themes/ directory (legacy)");
    println!("    import <path>     Import theme from file (legacy)");
    println!("    export <name>     Export theme to file (legacy)");
    println!("    edit <name>       Edit existing theme (legacy)");
    println!("    help              Show this help message");
    println!();
    println!("MODERN WORKFLOW:");
    println!(
        "    For theme file management, use `{} engine` commands:",
        NAME
    );
    println!("    • {} engine list      # Visual theme catalog", NAME);
    println!(
        "    • {} engine debug     # Theme loading diagnostics",
        NAME
    );
    println!(
        "    • {} engine import    # Import theme config files",
        NAME
    );
    println!(
        "    • {} engine export    # Export theme config files",
        NAME
    );
    println!();
    println!("EXAMPLES:");
    println!(
        "    {} theme list                 # Discover themes by category",
        NAME
    );
    println!(
        "    {} theme browse               # Interactive theme exploration",
        NAME
    );
    println!(
        "    {} theme preview error        # Full preview of error theme",
        NAME
    );
    println!(
        "    {} theme show success         # Technical details of success theme",
        NAME
    );
    println!(
        "    {} theme dryrun warning       # Test warning theme samples",
        NAME
    );
    println!("    {} theme create my_style      # Create new custom theme", NAME);
    println!();
    println!(
        "💡 TIP: Use `{} engine --help` for config file management",
        NAME
    );
}

/// Print engine help (ENGINE-002: Separate help menus)
pub fn print_engine_help() {
    println!("{} {} - Engine Commands Help", NAME, VERSION);
    println!();
    println!("USAGE:");
    println!("    {} engine <COMMAND>", NAME);
    println!();
    println!("COMMANDS:");
    println!("    init              Initialize global theme directory and defaults");
    println!("    import <name>     Import boxy_<name>.yml to global location");
    println!("    export <name>     Export boxy_<name>.yml from global to local");
    println!("    list              List all available themes from all configs");
    println!("    debug             Show loading hierarchy and engine diagnostics");
    println!("    status            Show engine health and statistics");
    println!("    validate <file>   Comprehensive theme file validation");
    println!("    edit <name>       Edit a theme config file");
    println!("    help              Show this help message");
    println!();
    println!("OPTIONS:");
    println!("    --overwrite       Force overwrite existing files");
    println!("    --dry-run         Preview operations without making changes");
    println!();
    println!("DESCRIPTION:");
    println!("    Engine commands manage theme configuration files (boxy_*.yml).");
    println!("    These are separate from individual theme usage commands.");
    println!();
    println!("EXAMPLES:");
    println!(
        "    {} engine init                   # Set up global theme system",
        NAME
    );
    println!(
        "    {} engine list                   # Show all available themes",
        NAME
    );
    println!(
        "    {} engine debug                  # Debug theme loading",
        NAME
    );
    println!(
        "    {} engine validate theme.yml     # Validate theme file",
        NAME
    );
    println!(
        "    {} engine import myproject       # Import boxy_myproject.yml",
        NAME
    );
    println!(
        "    {} engine import test --dry-run  # Preview import without changes",
        NAME
    );
    println!(
        "    {} engine export default         # Export boxy_default.yml",
        NAME
    );
    println!(
        "    {} engine export theme --dry-run # Preview export without changes",
        NAME
    );
}

/// Enhanced theme list with better categorization and visual previews
pub fn handle_enhanced_theme_list(opt_dev_level: Option<u8>, jynx: &JynxPlugin) {
    match ThemeEngine::new_with_override(opt_dev_level) {
        Ok(theme_engine) => {
            let themes = theme_engine.list_themes();
            if themes.is_empty() {
                println!("No themes available.");
                return;
            }

            // Group themes by category
            let mut builtin_themes = Vec::new();
            let mut custom_themes = Vec::new();
            let mut semantic_themes = Vec::new();
            let mut utility_themes = Vec::new();

            // Categorize themes based on patterns and metadata
            for (name, description) in themes {
                let theme_data = theme_engine.get_theme(&name);

                // Check if it's a semantic theme (error, success, warning, info)
                if matches!(name.as_str(), "error" | "success" | "warning" | "info" | "critical" | "fatal" | "debug") {
                    semantic_themes.push((name, description, theme_data));
                }
                // Check if it's a utility theme (base variants, blueprint, etc.)
                else if name.starts_with("base") || matches!(name.as_str(), "blueprint" | "default" | "trace") {
                    utility_themes.push((name, description, theme_data));
                }
                // Check if it's a built-in theme by description
                else if description.contains("Compiled default") {
                    builtin_themes.push((name, description, theme_data));
                }
                // Everything else is custom
                else {
                    custom_themes.push((name, description, theme_data));
                }
            }

            // Display organized theme catalog
            println!("🎨 {} {} - Theme Discovery Catalog", NAME, VERSION);
            println!("═══════════════════════════════════════════════════");
            println!();

            // Semantic themes (most commonly used)
            if !semantic_themes.is_empty() {
                println!("📊 Semantic Themes (Status & Logging)");
                println!("──────────────────────────────────────");
                for (name, description, theme_opt) in semantic_themes {
                    print_theme_preview_line(&name, &description, theme_opt.as_ref());
                }
                println!();
            }

            // Custom themes (user-defined)
            if !custom_themes.is_empty() {
                println!("🎭 Custom Themes");
                println!("────────────────");
                for (name, description, theme_opt) in custom_themes {
                    print_theme_preview_line(&name, &description, theme_opt.as_ref());
                }
                println!();
            }

            // Utility themes (base styles, etc.)
            if !utility_themes.is_empty() {
                println!("🔧 Utility & Base Themes");
                println!("─────────────────────────");
                for (name, description, theme_opt) in utility_themes {
                    print_theme_preview_line(&name, &description, theme_opt.as_ref());
                }
                println!();
            }

            // Built-in themes (remaining)
            if !builtin_themes.is_empty() {
                println!("📦 Built-in Themes");
                println!("───────────────────");
                for (name, description, theme_opt) in builtin_themes {
                    print_theme_preview_line(&name, &description, theme_opt.as_ref());
                }
                println!();
            }

            println!("💡 Usage:");
            println!("  {} --theme <name> \"Your text\"     # Apply theme", NAME);
            println!("  {} theme preview <name>         # Preview theme with samples", NAME);
            println!("  {} theme browse                 # Interactive theme browser", NAME);
            println!("  {} engine list                  # Detailed visual catalog", NAME);

            // Use jynx for enhanced display if active
            if jynx.is_active() {
                println!();
                println!("✨ Enhanced display available via jynx integration");
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to load theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Print a formatted theme preview line with visual demo
fn print_theme_preview_line(name: &str, description: &str, theme_opt: Option<&BoxyTheme>) {
    if let Some(theme) = theme_opt {
        let color_preview = create_color_preview(&theme.color);
        let style_preview = create_style_preview(&theme.style);
        let icon = theme.icon.as_deref().unwrap_or("📦");

        println!(
            "  {:15} │ {} {} {} │ {}",
            name,
            icon,
            color_preview,
            style_preview,
            description.chars().take(40).collect::<String>()
        );
    } else {
        println!("  {:15} │ 📦 [unavailable] │ {}", name, description);
    }
}

/// Create a colorized preview of the theme color
fn create_color_preview(color: &str) -> String {
    if let Ok(ansi_color) = validate_color(color) {
        format!("{}Sample{}", ansi_color, "\x1B[0m")
    } else {
        format!("Sample")
    }
}

/// Create a visual preview of the border style
fn create_style_preview(style: &str) -> String {
    match style {
        "normal" => "┌─┐│└─┘".to_string(),
        "rounded" => "╭─╮│╰─╯".to_string(),
        "double" => "╔═╗║╚═╝".to_string(),
        "heavy" => "┏━┓┃┗━┛".to_string(),
        "ascii" => "+-+|+-+".to_string(),
        _ => "┌─┐│└─┘".to_string(),
    }
}

/// Interactive theme browser with visual previews and real-time rendering
pub fn handle_theme_browse(opt_dev_level: Option<u8>, jynx: &JynxPlugin) {
    match ThemeEngine::new_with_override(opt_dev_level) {
        Ok(theme_engine) => {
            let themes = theme_engine.list_themes();
            if themes.is_empty() {
                println!("No themes available to browse.");
                return;
            }

            println!("🎨 Interactive Theme Browser");
            println!("════════════════════════════");
            println!("Browse themes with live previews. Press Enter to see full demo.");
            println!();

            // Interactive browsing with live previews
            for (i, (name, description)) in themes.iter().enumerate() {
                println!("┌─ Theme #{}: {} ─", i + 1, name);

                if let Some(theme) = theme_engine.get_theme(name) {
                    // Show compact theme info
                    println!("│ Style: {} │ Color: {} │ Text: {}",
                           theme.style, theme.color, theme.text_color);

                    // Show live preview with sample text
                    println!("│ Preview:");
                    render_theme_sample(name, "Sample preview text", &theme);

                    if let Some(title) = &theme.title {
                        println!("│ Title: {}", title);
                    }

                    println!("│ Description: {}", description);
                } else {
                    println!("│ [Theme unavailable for preview]");
                }

                println!("└─────────────────────────────────");
                println!();
            }

            println!("🔍 Detailed Commands:");
            println!("  {} theme preview <name>     # Full preview with multiple samples", NAME);
            println!("  {} --theme <name> \"text\"     # Apply theme to your text", NAME);

            if jynx.is_active() {
                println!("  Interactive selection available via jynx integration");
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to load theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Enhanced theme preview with multiple sample texts and detailed information
pub fn handle_theme_preview(theme_name: &str) {
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(theme) = theme_engine.get_theme(theme_name) {
                println!("🔍 Theme Preview: {}", theme_name);
                println!("═════════════════════════════════");
                println!();

                // Theme details
                println!("📋 Theme Properties:");
                println!("├─ Color: {}", theme.color);
                println!("├─ Text Color: {}", theme.text_color);
                println!("├─ Style: {} {}", theme.style, create_style_preview(&theme.style));
                println!("├─ Text Style: {}", theme.text_style);

                if let Some(title) = &theme.title {
                    println!("├─ Title: {}", title);
                }
                if let Some(icon) = &theme.icon {
                    println!("├─ Icon: {}", icon);
                }
                if let Some(width) = theme.width {
                    println!("├─ Width: {} characters", width);
                }
                println!("└─ Padding: {} spaces", theme.padding);
                println!();

                // Visual samples
                println!("🎨 Visual Samples:");
                println!("──────────────────");

                let sample_texts = vec![
                    "Short message",
                    "This is a medium-length sample message to show text wrapping",
                    "📋 Lists and bullets:\n• Item one\n• Item two\n• Item three",
                    "⚠️  Status messages with icons and emojis",
                    "Code snippets and technical content: main() { return 0; }",
                    "Multi-line content demonstrates\nhow themes handle line breaks\nand maintain consistent styling",
                ];

                for (i, sample) in sample_texts.iter().enumerate() {
                    println!("\n{}. Sample {}:", i + 1, match i {
                        0 => "Short Text",
                        1 => "Medium Text",
                        2 => "List Content",
                        3 => "Status Message",
                        4 => "Code Content",
                        5 => "Multi-line Text",
                        _ => "Other",
                    });
                    render_theme_sample(theme_name, sample, &theme);
                }

                println!();
                println!("💡 Usage: echo \"Your text\" | {} --theme {}", NAME, theme_name);
                println!("🔧 Edit: {} theme edit {} (if in custom theme file)", NAME, theme_name);
            } else {
                eprintln!("Error: Theme '{}' not found", theme_name);
                eprintln!("Use '{} theme list' to see available themes", NAME);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to load theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Render a sample of text using a theme (direct rendering without subprocess)
fn render_theme_sample(_theme_name: &str, text: &str, theme: &BoxyTheme) {
    // This is a simplified inline renderer for preview purposes
    // For full rendering, we'd need to duplicate the main rendering logic
    // For now, show a basic preview using the theme properties

    let color_code = validate_color(&theme.color).unwrap_or("\x1B[37m");
    let text_color_code = if theme.text_color == "auto" {
        ""
    } else if theme.text_color == "none" {
        ""
    } else {
        validate_color(&theme.text_color).unwrap_or("")
    };

    // Simple box preview
    let lines: Vec<&str> = text.lines().collect();
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0).min(50);
    let box_width = (max_width + 2).max(10);

    // Top border
    match theme.style.as_str() {
        "rounded" => println!("{}╭{}╮{}", color_code, "─".repeat(box_width), "\x1B[0m"),
        "double" => println!("{}╔{}╗{}", color_code, "═".repeat(box_width), "\x1B[0m"),
        "heavy" => println!("{}┏{}┓{}", color_code, "━".repeat(box_width), "\x1B[0m"),
        "ascii" => println!("{}+{}+{}", color_code, "-".repeat(box_width), "\x1B[0m"),
        _ => println!("{}┌{}┐{}", color_code, "─".repeat(box_width), "\x1B[0m"),
    }

    // Content lines
    for line in lines {
        let _padding = " ".repeat(((box_width.saturating_sub(line.len())) / 2).max(1));
        let border_char = match theme.style.as_str() {
            "double" => "║",
            "heavy" => "┃",
            "ascii" => "|",
            _ => "│",
        };

        println!("{}{}{} {}{}{} {}{}{}",
                color_code, border_char, "\x1B[0m",
                text_color_code, line, "\x1B[0m",
                color_code, border_char, "\x1B[0m");
    }

    // Bottom border
    match theme.style.as_str() {
        "rounded" => println!("{}╰{}╯{}", color_code, "─".repeat(box_width), "\x1B[0m"),
        "double" => println!("{}╚{}╝{}", color_code, "═".repeat(box_width), "\x1B[0m"),
        "heavy" => println!("{}┗{}┛{}", color_code, "━".repeat(box_width), "\x1B[0m"),
        "ascii" => println!("{}+{}+{}", color_code, "-".repeat(box_width), "\x1B[0m"),
        _ => println!("{}└{}┘{}", color_code, "─".repeat(box_width), "\x1B[0m"),
    }
}
