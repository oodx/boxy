//! Internal theme helpers
//!
//! This module contains internal implementations consumed by utils.rs.
//! Functions here are not part of the public API.

use crate::core::{NAME, VERSION};
use crate::plugins::jynx::*;
use crate::plugins::theme_engine::ThemeEngine;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};

/// Handle `boxy theme create <name>` command
pub fn handle_theme_create(name: &str, jynx: &JynxPlugin) {
    // Validate theme name first
    if let Err(e) = super::utils::validate_theme_name(name) {
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
            let theme = super::utils::create_theme_interactively(name);

            // Save theme to XDG+ directory
            let themes_dir = theme_engine.get_themes_directory();
            if let Err(e) = fs::create_dir_all(&themes_dir) {
                eprintln!("Error: Failed to create themes directory: {}", e);
                std::process::exit(1);
            }

            let theme_file_path = themes_dir.join(format!("{}.yml", name));
            if let Err(e) = super::utils::save_theme_to_file(&theme_file_path, name, &theme) {
                eprintln!("Error: Failed to save theme: {}", e);
                std::process::exit(1);
            }

            println!();
            let success_msg = format!(
                "✅ Theme '{}' created successfully!\n   Saved to: {}\n\nTest your theme:\n   echo \"Hello World\" | boxy --use {}",
                name,
                theme_file_path.display(),
                name
            );

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
pub fn handle_theme_import(path: &str) {
    let import_path = PathBuf::from(path);
    if !import_path.exists() {
        eprintln!("Error: File '{}' does not exist", path);
        std::process::exit(1);
    }

    // Validate file extension
    if !import_path
        .extension()
        .map_or(false, |ext| ext == "yml" || ext == "yaml")
    {
        eprintln!("Error: Only YAML files (.yml, .yaml) are supported for import");
        std::process::exit(1);
    }

    // Pre-validate the theme file before importing
    println!("Validating theme file...");
    if let Err(e) = super::utils::validate_theme_file(&import_path) {
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

            let filename = import_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("imported_theme.yml");

            let target_path = themes_dir.join(filename);

            if target_path.exists() {
                print!(
                    "Theme file '{}' already exists. Overwrite? (y/N): ",
                    filename
                );
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
pub fn handle_theme_export(name: &str) {
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(theme) = theme_engine.get_theme(name) {
                let yaml_content = super::utils::export_theme_to_yaml(name, &theme);
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
pub fn handle_theme_edit(name: &str) {
    // Validate theme name first
    if let Err(e) = super::utils::validate_theme_name(name) {
        eprintln!("Error: Invalid theme name: {}", e);
        std::process::exit(1);
    }

    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(existing_theme) = theme_engine.get_theme(name) {
                println!("{} {} - Edit Theme: {}", NAME, VERSION, name);
                println!();

                // Interactive theme editing
                let updated_theme = super::utils::edit_theme_interactively(name, &existing_theme);

                // Save updated theme
                let themes_dir = theme_engine.get_themes_directory();
                if let Err(e) = fs::create_dir_all(&themes_dir) {
                    eprintln!("Error: Failed to create themes directory: {}", e);
                    std::process::exit(1);
                }

                let theme_file_path = themes_dir.join(format!("{}.yml", name));
                if let Err(e) =
                    super::utils::save_theme_to_file(&theme_file_path, name, &updated_theme)
                {
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

/// Handle `boxy theme dryrun <theme>` command - shows theme application with sample content
pub fn handle_theme_dryrun(theme_name: &str) {
    match ThemeEngine::new() {
        Ok(theme_engine) => {
            if let Some(theme) = theme_engine.get_theme(theme_name) {
                println!("🧪 Theme Dry Run: {} theme", theme_name);
                println!();

                // Show theme loading hierarchy trail with files
                println!("🔍 Theme Loading Trail:");
                let file_trail = theme_engine.get_file_trail();
                for entry in file_trail.iter() {
                    println!("{}", entry);
                }
                println!();

                // Show theme source and properties
                println!("📋 Theme Properties:");
                println!("  Color: {}", theme.color);
                println!("  Text Color: {}", theme.text_color);
                println!("  Style: {}", theme.style);
                println!("  Text Style: {}", theme.text_style);
                if let Some(title) = &theme.title {
                    println!("  Title: {}", title);
                }
                if let Some(icon) = &theme.icon {
                    println!("  Icon: {}", icon);
                }
                if let Some(width) = theme.width {
                    println!("  Width: {}", width);
                }

                println!();
                println!("🎨 Sample Output:");

                // Create sample content with the theme
                let sample_texts = vec![
                    "This is a sample message",
                    "Theme testing with longer content to see how it wraps and displays",
                    "Short text",
                    "🎯 Unicode and emoji test: ℹ️ 📦 ✅ ❌ ⚠️",
                ];

                for sample in sample_texts {
                    // Use the boxy binary to render with the theme
                    let output = std::process::Command::new("./target/release/boxy")
                        .arg("--theme")
                        .arg(theme_name)
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::piped())
                        .spawn();

                    match output {
                        Ok(mut child) => {
                            if let Some(mut stdin) = child.stdin.take() {
                                use std::io::Write;
                                let _ = writeln!(stdin, "{}", sample);
                            }

                            match child.wait_with_output() {
                                Ok(output) => {
                                    let stdout = String::from_utf8_lossy(&output.stdout);
                                    print!("{}", stdout);
                                }
                                Err(_) => {
                                    println!("Failed to execute dry run for: {}", sample);
                                }
                            }
                        }
                        Err(_) => {
                            println!("Failed to start boxy process for dry run");
                            break;
                        }
                    }
                }

                println!("💡 Use: echo \"your text\" | boxy --use {}", theme_name);
            } else {
                eprintln!("Error: Theme '{}' not found", theme_name);
                eprintln!("Use 'boxy theme list' to see available themes");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to load theme engine: {}", e);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy theme init` command - creates local .themes/boxy-custom.yaml template
pub fn handle_theme_init() {
    use std::fs;

    // Create .themes directory if it doesn't exist
    let themes_dir = std::path::PathBuf::from(".themes");
    if let Err(e) = fs::create_dir_all(&themes_dir) {
        eprintln!("Error: Failed to create .themes directory: {}", e);
        std::process::exit(1);
    }

    // Target file path
    let target_file = themes_dir.join("boxy-custom.yaml");

    // Check if file already exists
    if target_file.exists() {
        eprintln!("Error: {} already exists", target_file.display());
        eprintln!("Remove it first or edit it directly");
        std::process::exit(1);
    }

    // Try to copy from template file first (if running from source)
    let template_path = std::path::PathBuf::from("themes/theme_template.yml");

    let template_content = if template_path.exists() {
        // Running from source - use the template file
        match fs::read_to_string(&template_path) {
            Ok(content) => {
                println!("📄 Using template from: {}", template_path.display());
                content
            }
            Err(_) => {
                // Fallback to embedded template
                get_embedded_theme_template()
            }
        }
    } else {
        // Not running from source - use embedded template
        get_embedded_theme_template()
    };

    // Write the template to .themes/boxy-custom.yaml
    if let Err(e) = fs::write(&target_file, template_content) {
        eprintln!("Error: Failed to write theme template: {}", e);
        std::process::exit(1);
    }

    println!("✅ Created local theme template: {}", target_file.display());
    println!();
    println!("📝 Next steps:");
    println!(
        "  1. Edit {} to customize your themes",
        target_file.display()
    );
    println!("  2. Test with: {} theme dryrun <theme_name>", NAME);
    println!("  3. Use with: echo \"text\" | {} --use <theme_name>", NAME);
    println!();
    println!(
        "💡 The .themes/ directory has the highest priority after individual boxy*.yaml files"
    );
    println!(
        "   Use: {} theme hierarchy to see the complete loading order",
        NAME
    );
}

/// Get embedded theme template as fallback when not running from source
fn get_embedded_theme_template() -> String {
    println!("📄 Using embedded blueprint template (comprehensive feature showcase)");

    // Complete blueprint template showcasing ALL boxy features
    r#"# 📘 Boxy Theme Blueprint - Complete Feature Reference
# This comprehensive template was created by `boxy theme init`
# It showcases ALL available theme features and options
# Customize, copy sections, or use as reference documentation

metadata:
  name: "boxy-blueprint-themes"           # (string) Human name for this theme collection
  version: "1.0.0"                       # (string) Semantic version
  description: "Complete Boxy feature showcase and custom themes"  # (string) Description
  author: "you"                          # (optional) Author name
  created: "2025-09-16"                  # (optional) Creation date
  updated: "2025-09-16"                  # (optional) Last updated
  compatibility: "boxy v0.9+"            # (optional) Version compatibility

# (optional) Define custom color names mapping to ANSI sequences
# Use these anywhere a color is accepted (color, text_color, title_color, etc.)
colors:
  blueprint_blue: "\u001B[38;5;33m"      # Custom blue for blueprint theme
  tech_cyan: "\u001B[38;5;51m"           # Bright cyan for technical content
  warning_amber: "\u001B[38;5;214m"      # Custom amber for warnings
  # More examples:
  # company_red: "\u001B[38;5;160m"
  # brand_purple: "\u001B[38;5;129m"

# Theme definitions - each key becomes a theme name for --theme <name>
themes:
  # 📐 BLUEPRINT BASE: Complete feature showcase with ASCII borders
  blueprint:
    # === CORE VISUAL PROPERTIES ===
    color: "blueprint_blue"              # Border color (color name or custom)
    text_color: "blueprint_blue"         # Text color (color|auto|none)
    style: "ascii"                       # Border style (normal|rounded|double|heavy|ascii)
    text_style: "normal"                 # Text styling (normal|bold|italic|underline|dim)

    # === LAYOUT AND SPACING ===
    padding: 2                           # Inner horizontal padding (int)
    width: 80                            # Fixed width override (optional int)

    # === CONTENT ALIGNMENT ===
    title_align: "center"                # Title alignment (left|center|right)
    header_align: "left"                 # Header section alignment
    footer_align: "right"                # Footer section alignment
    status_align: "left"                 # Status line alignment

    # === SECTION-SPECIFIC COLORS ===
    title_color: "tech_cyan"             # Override title color (bright cyan for contrast)
    status_color: "blueprint_blue"       # Status section color
    header_color: "blueprint_blue"       # Header section color
    footer_color: "blueprint_blue"       # Footer section color

    # === CONTENT SECTIONS ===
    title: "📘 Blueprint"                # Custom title with emoji
    icon: "📐"                           # Icon (if no title, or for minimal mode)

    # === ADVANCED LAYOUT CONTROL ===
    # Default section layout tokens (comma-separated):
    # hc=header_content, fr=free_content, sc=status_content, dt=date_time, dsn=description
    layout: "hc,fr,sc,dt"                # Custom section ordering

    # === TEXT PROCESSING ===
    max_line_length: 76                  # (optional) Max chars per line before wrapping
    word_wrap: true                      # (optional) Enable word wrapping

    # === METADATA ===
    description: "Blueprint theme showcasing all boxy features with ASCII technical styling"
    tags: ["technical", "documentation", "reference"]  # (optional) Theme tags

  # 🏗️ INHERITANCE EXAMPLE: Base theme for others to inherit from
  base_blueprint:
    color: "blueprint_blue"
    text_color: "auto"
    style: "ascii"
    text_style: "normal"
    padding: 1
    title_align: "center"
    header_align: "center"
    footer_align: "center"
    status_align: "left"

  # ✅ SUCCESS VARIANT: Inheriting from base with overrides
  blueprint_success:
    inherits: "base_blueprint"           # Inherit all properties from base_blueprint
    color: "emerald"                     # Override border color
    text_style: "bold"                   # Override text style
    title: "✅ Blueprint Success"        # Custom title
    title_color: "green"                # Green title text
    status_color: "emerald"              # Match border color
    description: "Success variant of blueprint theme"

  # ❌ ERROR VARIANT: Heavy borders for emphasis
  blueprint_error:
    inherits: "base_blueprint"
    style: "heavy"                       # Heavy borders for errors
    color: "crimson"
    text_color: "white"
    text_style: "bold"
    title: "❌ Blueprint Error"
    title_color: "red"
    width: 60                            # Narrower for error messages
    description: "Error variant with heavy borders"

  # ⚠️ WARNING VARIANT: Custom color with italics
  blueprint_warning:
    inherits: "base_blueprint"
    color: "warning_amber"
    text_style: "italic"
    title: "⚠️ Blueprint Warning"
    title_color: "yellow"
    description: "Warning variant with italic text"

  # ℹ️ INFO VARIANT: Minimal clean design
  blueprint_info:
    inherits: "base_blueprint"
    color: "azure"
    title: "ℹ️ Blueprint Info"
    padding: 1                           # Minimal padding
    description: "Clean info variant"

  # 🔧 TECHNICAL VARIANT: Monospace feel with specific layout
  blueprint_tech:
    inherits: "base_blueprint"
    color: "tech_cyan"
    text_color: "white"
    text_style: "normal"
    title: "🔧 Technical"
    layout: "hc,dt,fr,sc"                # Custom: header, datetime, content, status
    width: 100                           # Wide for technical content
    max_line_length: 96                  # Leave room for borders
    word_wrap: true
    description: "Technical documentation theme with custom layout"

  # 🎨 SHOWCASE VARIANT: Demonstrating all color overrides
  blueprint_showcase:
    inherits: "base_blueprint"
    style: "double"                      # Double-line borders
    color: "purple"                      # Purple border
    text_color: "white"                  # White text
    title: "🎨 Feature Showcase"
    title_color: "magenta"               # Magenta title
    header_color: "blue"                 # Blue header
    footer_color: "green"                # Green footer
    status_color: "yellow"               # Yellow status
    padding: 3                           # Extra padding
    description: "Showcase theme demonstrating all color customization options"

# (optional) Theme name shortcuts/aliases
presets:
  good: "blueprint_success"              # `--theme good` → blueprint_success
  bad: "blueprint_error"                 # `--theme bad` → blueprint_error
  warn: "blueprint_warning"              # `--theme warn` → blueprint_warning
  tech: "blueprint_tech"                 # `--theme tech` → blueprint_tech

# (optional) Custom text style definitions (name → ANSI sequences)
text_styles:
  loud: "\u001B[1m\u001B[4m"            # Bold + underline combination
  subtle: "\u001B[2m"                    # Dim text
  emphasis: "\u001B[1m\u001B[3m"        # Bold + italic combination

# Global settings for this theme collection
settings:
  default_theme: blueprint               # Theme used when none specified
  fallback_color: slate                  # Fallback border color
  max_width: 120                         # Global maximum box width
  min_width: 10                          # Global minimum box width
  cache_themes: true                     # Cache compiled themes in memory
  validate_colors: true                  # Validate color names at load time

# 📚 USAGE EXAMPLES:
#
# Basic usage:
#   echo "Hello World" | boxy --theme blueprint
#   echo "Success!" | boxy --theme blueprint_success
#   echo "Error occurred" | boxy --theme blueprint_error
#
# Using presets:
#   echo "All good" | boxy --theme good
#   echo "Problem detected" | boxy --theme bad
#
# Testing themes:
#   boxy theme dryrun blueprint
#   boxy theme dryrun blueprint_tech
#
# View hierarchy:
#   boxy theme hierarchy
#
# 🎯 CUSTOMIZATION TIPS:
#
# 1. Copy any theme above and modify colors/styles
# 2. Create inheritance chains: your_base → your_variant1, your_variant2
# 3. Use custom colors for brand consistency
# 4. Adjust padding and width for different content types
# 5. Use layout tokens to reorder content sections
# 6. Test with `boxy theme dryrun <name>` before using
"#
    .to_string()
}

// ==================== ENGINE COMMAND HANDLERS ====================

/// Handle `boxy engine init` command - creates global theme directory and default config
pub fn handle_engine_init() {
    use std::fs;
    use std::path::PathBuf;

    // Determine the global themes directory
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Error: Cannot determine home directory (HOME environment variable not set)");
            std::process::exit(1);
        }
    };

    let global_themes_dir = PathBuf::from(home).join(".local/etc/odx/boxy/themes");

    println!("🏗️ Initializing Boxy engine global directory...");
    println!();

    // Create the directory structure
    if let Err(e) = fs::create_dir_all(&global_themes_dir) {
        eprintln!("Error: Failed to create global themes directory: {}", e);
        eprintln!("Path: {}", global_themes_dir.display());
        std::process::exit(1);
    }

    println!("✅ Created directory: {}", global_themes_dir.display());

    // Create default theme config file
    let default_config_path = global_themes_dir.join("boxy_default.yml");

    if default_config_path.exists() {
        println!(
            "ℹ️  Default config already exists: {}",
            default_config_path.display()
        );
    } else {
        let default_config_content = get_default_engine_config();

        if let Err(e) = fs::write(&default_config_path, default_config_content) {
            eprintln!("Error: Failed to write default config file: {}", e);
            eprintln!("Path: {}", default_config_path.display());
            std::process::exit(1);
        }

        println!(
            "✅ Created default config: {}",
            default_config_path.display()
        );
    }

    println!();
    println!("🎯 Engine initialization complete!");
    println!();
    println!("📋 Next steps:");
    println!("  • Use `{} engine list` to see available themes", NAME);
    println!("  • Use `{} engine debug` to verify theme loading", NAME);
    println!("  • Use `{} --theme <name>` to apply themes", NAME);
    println!(
        "  • Edit {} to customize themes",
        default_config_path.display()
    );
}

/// Handle `boxy engine import <name>` command - imports boxy_<name>.yml to global location
pub fn handle_engine_import(name: &str, force_overwrite: bool, dry_run: bool) {
    use std::fs;
    use std::path::PathBuf;

    // Determine paths
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Error: Cannot determine home directory (HOME environment variable not set)");
            std::process::exit(1);
        }
    };

    let global_themes_dir = PathBuf::from(home).join(".local/etc/odx/boxy/themes");
    let local_file = PathBuf::from(format!("boxy_{}.yml", name));
    let global_file = global_themes_dir.join(format!("boxy_{}.yml", name));

    if dry_run {
        println!("🔍 DRY RUN - Previewing import of theme config: {}", name);
        println!("   (No changes will be made)");
    } else {
        println!("📥 Importing theme config: {}", name);
    }
    println!();

    // Check if local file exists
    if !local_file.exists() {
        eprintln!("❌ Local theme config not found: {}", local_file.display());
        eprintln!();
        eprintln!("🔍 Expected file: boxy_{}.yml in current directory", name);
        eprintln!();
        eprintln!("💡 To fix this:");
        eprintln!("   1. Ensure the file exists: boxy_{}.yml", name);
        eprintln!("   2. Check filename spelling and ensure boxy_ prefix");
        eprintln!("   3. Verify you're in the correct directory");
        eprintln!();
        eprintln!(
            "📂 Current directory: {}",
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        );
        eprintln!();
        eprintln!("🔧 Alternative commands:");
        eprintln!(
            "   • boxy engine export {}    # Export from global themes",
            name
        );
        eprintln!("   • boxy engine list         # See available themes");
        eprintln!("   • boxy engine init         # Create default themes");
        std::process::exit(1);
    }

    // Ensure global directory exists
    if dry_run {
        println!("📁 Would create directory: {}", global_themes_dir.display());
    } else {
        if let Err(e) = fs::create_dir_all(&global_themes_dir) {
            eprintln!("Error: Failed to create global themes directory: {}", e);
            eprintln!("Path: {}", global_themes_dir.display());
            std::process::exit(1);
        }
    }

    // Check if target file exists and handle overwrite
    if global_file.exists() && !force_overwrite && !dry_run {
        eprintln!(
            "Error: Theme config already exists in global directory: {}",
            global_file.display()
        );
        eprintln!("Use --overwrite flag to replace existing config:");
        eprintln!("  {} engine import {} --overwrite", NAME, name);
        std::process::exit(1);
    }

    // Preview file existence for dry run
    if dry_run && global_file.exists() {
        println!("⚠️  Target file already exists: {}", global_file.display());
        if force_overwrite {
            println!("   Would overwrite existing file");
        } else {
            println!("   Would fail without --overwrite flag");
        }
    }

    // Validate the theme file before importing
    println!("🔍 Validating theme config...");
    if let Err(e) = super::utils::validate_theme_file(&local_file) {
        eprintln!("Error: Theme config validation failed: {}", e);
        eprintln!(
            "Please fix the issues in {} before importing",
            local_file.display()
        );
        std::process::exit(1);
    }

    // Create backup if overwriting
    if global_file.exists() {
        let backup_file = global_themes_dir.join(format!("boxy_{}.yml.bak", name));
        if dry_run {
            println!("📋 Would create backup: {}", backup_file.display());
        } else {
            if let Err(e) = fs::copy(&global_file, &backup_file) {
                eprintln!("Warning: Failed to create backup: {}", e);
            } else {
                println!("📋 Created backup: {}", backup_file.display());
            }
        }
    }

    // Copy the file
    if dry_run {
        println!(
            "📄 Would copy: {} → {}",
            local_file.display(),
            global_file.display()
        );
        println!();
        println!("🎯 DRY RUN SUMMARY:");
        println!("   Source:      {}", local_file.display());
        println!("   Target:      {}", global_file.display());
        println!("   Directory:   {}", global_themes_dir.display());
        if global_file.exists() {
            println!("   Action:      Overwrite existing file");
        } else {
            println!("   Action:      Create new file");
        }
        println!();
        println!("💡 To execute: Remove --dry-run flag");
    } else {
        if let Err(e) = fs::copy(&local_file, &global_file) {
            eprintln!("Error: Failed to import theme config: {}", e);
            eprintln!("Source: {}", local_file.display());
            eprintln!("Target: {}", global_file.display());
            std::process::exit(1);
        }
        println!(
            "✅ Successfully imported: {} → {}",
            local_file.display(),
            global_file.display()
        );
    }
    println!();
    println!("🎯 Import complete!");
    println!();
    println!("📋 Next steps:");
    println!("  • Use `{} engine list` to see the imported themes", NAME);
    println!("  • Use `{} engine debug` to verify theme loading", NAME);
    println!(
        "  • Use `{} --theme <theme_name>` to test imported themes",
        NAME
    );
}

/// Handle `boxy engine export <name>` command - exports boxy_<name>.yml from global to local
pub fn handle_engine_export(name: &str, force_overwrite: bool, dry_run: bool) {
    use std::fs;
    use std::path::PathBuf;

    // Determine paths
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Error: Cannot determine home directory (HOME environment variable not set)");
            std::process::exit(1);
        }
    };

    let global_themes_dir = PathBuf::from(home).join(".local/etc/odx/boxy/themes");
    let global_file = global_themes_dir.join(format!("boxy_{}.yml", name));
    let local_file = PathBuf::from(format!("boxy_{}.yml", name));

    if dry_run {
        println!("🔍 DRY RUN - Previewing export of theme config: {}", name);
        println!("   (No changes will be made)");
    } else {
        println!("📤 Exporting theme config: {}", name);
    }
    println!();

    // Check if global file exists
    if !global_file.exists() {
        eprintln!("❌ Global theme config not found: boxy_{}.yml", name);
        eprintln!();
        eprintln!("🔍 Looked in: {}", global_themes_dir.display());
        eprintln!();

        let available_themes: Vec<String> = if let Ok(entries) = fs::read_dir(&global_themes_dir) {
            entries
                .flatten()
                .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
                .filter(|name| name.starts_with("boxy_") && name.ends_with(".yml"))
                .map(|name| {
                    name.strip_prefix("boxy_")
                        .unwrap()
                        .strip_suffix(".yml")
                        .unwrap()
                        .to_string()
                })
                .collect()
        } else {
            vec![]
        };

        if available_themes.is_empty() {
            eprintln!("📝 No theme configs found in global directory.");
            eprintln!();
            eprintln!("💡 To fix this:");
            eprintln!("   • boxy engine init              # Create default themes");
            eprintln!("   • boxy engine import <name>     # Import from local directory");
            eprintln!("   • boxy engine status            # Check system health");
        } else {
            eprintln!("📚 Available themes ({} total):", available_themes.len());
            for theme in &available_themes {
                eprintln!("   • {}", theme);
            }
            eprintln!();
            eprintln!("💡 Use one of these instead:");
            eprintln!("   {} engine export {}", NAME, available_themes[0]);
        }
        eprintln!();
        eprintln!("🔧 Need help? {} engine list", NAME);
        std::process::exit(1);
    }

    // Check if local file exists and handle overwrite
    if local_file.exists() && !force_overwrite && !dry_run {
        eprintln!("⚠️  Theme config already exists: {}", local_file.display());
        eprintln!();
        eprintln!("💡 To replace the existing file:");
        eprintln!("   {} engine export {} --overwrite", NAME, name);
        eprintln!();
        eprintln!("🔒 This will:");
        eprintln!("   • Create backup: boxy_{}.yml.bak", name);
        eprintln!("   • Replace existing file with global version");
        eprintln!("   • Preserve your current file as backup");
        std::process::exit(1);
    }

    // Preview file existence for dry run
    if dry_run && local_file.exists() {
        println!("⚠️  Target file already exists: {}", local_file.display());
        if force_overwrite {
            println!("   Would overwrite existing file");
        } else {
            println!("   Would fail without --overwrite flag");
        }
    }

    // Validate the global theme file before exporting
    println!("🔍 Validating theme config...");
    if let Err(e) = super::utils::validate_theme_file(&global_file) {
        eprintln!("Error: Global theme config validation failed: {}", e);
        eprintln!(
            "The global theme file {} appears to be corrupted",
            global_file.display()
        );
        eprintln!("Consider re-importing a valid theme file");
        std::process::exit(1);
    }

    // Create backup if overwriting
    if local_file.exists() {
        let backup_file = PathBuf::from(format!("boxy_{}.yml.bak", name));
        if dry_run {
            println!("📋 Would create backup: {}", backup_file.display());
        } else {
            if let Err(e) = fs::copy(&local_file, &backup_file) {
                eprintln!("Warning: Failed to create backup: {}", e);
            } else {
                println!("📋 Created backup: {}", backup_file.display());
            }
        }
    }

    // Copy the file
    if dry_run {
        println!(
            "📄 Would copy: {} → {}",
            global_file.display(),
            local_file.display()
        );
        println!();
        println!("🎯 DRY RUN SUMMARY:");
        println!("   Source:      {}", global_file.display());
        println!("   Target:      {}", local_file.display());
        println!(
            "   Directory:   {}",
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        );
        if local_file.exists() {
            println!("   Action:      Overwrite existing file");
        } else {
            println!("   Action:      Create new file");
        }
        println!();
        println!("💡 To execute: Remove --dry-run flag");
    } else {
        if let Err(e) = fs::copy(&global_file, &local_file) {
            eprintln!("Error: Failed to export theme config: {}", e);
            eprintln!("Source: {}", global_file.display());
            eprintln!("Target: {}", local_file.display());
            std::process::exit(1);
        }
        println!(
            "✅ Successfully exported: {} → {}",
            global_file.display(),
            local_file.display()
        );
    }
    println!();
    println!("🎯 Export complete!");
    println!();
    println!("📋 Next steps:");
    println!("  • Edit {} to customize themes", local_file.display());
    println!(
        "  • Use `{} engine import {}` to import changes back to global",
        NAME, name
    );
    println!(
        "  • Use `{} engine debug` to verify theme loading hierarchy",
        NAME
    );
}

/// Handle `boxy engine edit <name>` command - edits a config file
pub fn handle_engine_edit(name: &str) {
    // TODO: Move from theme edit, implement proper file operations
    eprintln!("Error: Engine edit command not yet fully implemented.");
    eprintln!("This will edit the boxy_{}.yml config file.", name);
    eprintln!("Coming in ENGINE-015 implementation.");
    std::process::exit(1);
}

/// Handle `boxy engine validate <file>` command - comprehensive theme file validation
pub fn handle_engine_validate(file_path: &str) {
    use std::path::PathBuf;

    println!("{} {} - Theme File Validation", NAME, VERSION);
    println!();

    let path = PathBuf::from(file_path);

    // Check if file exists
    if !path.exists() {
        eprintln!("❌ File not found: {}", path.display());
        eprintln!();
        eprintln!("💡 Please check:");
        eprintln!("   • File path spelling: {}", file_path);
        eprintln!(
            "   • File exists in current directory: {}",
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        );
        eprintln!("   • File has correct extension (.yml or .yaml)");
        std::process::exit(1);
    }

    println!("🔍 Validating: {}", path.display());
    println!();

    // Perform comprehensive validation
    match super::utils::validate_theme_file_with_duplicate_check(&path, true) {
        Ok(()) => {
            println!("✅ Validation passed!");
            println!();
            println!("🎯 Summary:");
            println!("   • YAML structure: Valid");
            println!("   • Theme definitions: Valid");
            println!("   • Required properties: Present");
            println!("   • Duplicate detection: Complete");
            println!();
            println!("💡 File is ready for import:");
            println!("   {} engine import <name>", NAME);
        }
        Err(e) => {
            eprintln!("{}", e);
            eprintln!();
            eprintln!("🔧 To fix validation issues:");
            eprintln!("   • Check YAML syntax with an online validator");
            eprintln!("   • Ensure all required properties are present");
            eprintln!("   • Verify color and style values are valid");
            eprintln!("   • Review theme names for conflicts");
            eprintln!();
            eprintln!("📚 Examples:");
            eprintln!("   {} engine list          # See available themes", NAME);
            eprintln!("   {} engine debug         # Check system status", NAME);
            std::process::exit(1);
        }
    }
}

/// Handle `boxy engine status` command - shows engine health
pub fn handle_engine_status() {
    use crate::theme_engine::ThemeEngine;

    println!("{} {} - Engine Status", NAME, VERSION);
    println!();

    // Initialize theme engine
    let engine = match ThemeEngine::new() {
        Ok(engine) => engine,
        Err(e) => {
            println!("❌ Engine Status: CRITICAL ERROR");
            println!("   Failed to initialize theme engine: {}", e);
            std::process::exit(1);
        }
    };

    // Get global directory status
    let global_dir = engine.get_themes_directory();
    let global_exists = global_dir.exists();

    // Count config files and themes using engine stats
    let mut config_count = 0;
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check global directory for config files
    if global_exists {
        if let Ok(entries) = std::fs::read_dir(&global_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .map_or(false, |ext| ext == "yml" || ext == "yaml")
                {
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        // ENGINE-006: Only count boxy_ prefixed files
                        if filename.starts_with("boxy_")
                            && !filename.contains("template")
                            && !filename.contains("tmpl")
                        {
                            config_count += 1;
                        }
                    }
                }
            }
        } else {
            errors.push("Cannot read global themes directory".to_string());
        }
    } else {
        warnings.push("Global themes directory does not exist".to_string());
    }

    // Get total theme count from loaded engine
    let total_themes = engine.list_themes().len();

    // Display status
    if errors.is_empty() && warnings.is_empty() {
        println!("✅ Engine Status: HEALTHY");
    } else if !errors.is_empty() {
        println!("❌ Engine Status: ERRORS DETECTED");
    } else {
        println!("⚠️  Engine Status: WARNINGS");
    }

    println!();
    println!("📊 Configuration Summary:");
    println!("   Global Directory: {}", global_dir.display());
    println!(
        "   Directory Exists: {}",
        if global_exists { "✅ Yes" } else { "❌ No" }
    );
    println!("   Config Files:     {} boxy_*.yml files", config_count);
    println!("   Total Themes:     {} themes available", total_themes);

    if !warnings.is_empty() {
        println!();
        println!("⚠️  Warnings:");
        for warning in &warnings {
            println!("   • {}", warning);
        }
        println!();
        println!(
            "💡 To fix: Run `{} engine init` to create default themes",
            NAME
        );
    }

    if !errors.is_empty() {
        println!();
        println!("❌ Errors:");
        for error in &errors {
            println!("   • {}", error);
        }
        println!();
        println!(
            "💡 To fix: Use `{} engine debug` for detailed diagnostics",
            NAME
        );
    }

    if errors.is_empty() && warnings.is_empty() {
        println!();
        println!("🎯 Quick Actions:");
        println!("   {} engine list     # Visual theme catalog", NAME);
        println!("   {} engine debug    # Detailed diagnostics", NAME);
        println!("   {} --use <theme>   # Apply a theme", NAME);
    }
}

/// Enhanced list output for ENGINE-011: Show themes with visual properties
pub fn handle_engine_list_enhanced(theme_engine: &ThemeEngine) {
    println!("🎨 BOXY ENGINE THEME CATALOG");
    println!("============================");
    println!();

    let themes = theme_engine.list_themes();
    if themes.is_empty() {
        println!(
            "⚠️  No themes available. Run `{} engine init` to set up themes.",
            NAME
        );
        return;
    }

    // Group themes by source for organized display
    let mut by_source: std::collections::HashMap<String, Vec<(String, String)>> =
        std::collections::HashMap::new();

    for (name, description) in themes {
        // Determine source based on description patterns
        let source = if description.contains("Compiled default") {
            "Built-in Themes".to_string()
        } else if description.contains("Test theme") {
            "Global Themes (XDG)".to_string()
        } else {
            "Loaded Themes".to_string()
        };

        by_source
            .entry(source)
            .or_insert_with(Vec::new)
            .push((name, description));
    }

    // Display each source group
    for (source, mut theme_list) in by_source {
        theme_list.sort_by(|a, b| a.0.cmp(&b.0));

        println!("📂 {}", source);
        println!("{}", "─".repeat(source.len() + 4));

        for (name, _description) in theme_list {
            // Get theme details for visual display
            if let Some(theme) = theme_engine.get_theme(&name) {
                display_theme_with_visual_properties(&name, &theme);
            } else {
                println!(
                    "  {} ❓ {} (theme not accessible)",
                    get_icon_for_theme(&name),
                    name
                );
            }
        }
        println!();
    }

    println!("💡 Usage Examples:");
    println!(
        "  {} --use error \"Error occurred\"      # Apply error theme",
        NAME
    );
    println!(
        "  {} --use success \"Task complete\"     # Apply success theme",
        NAME
    );
    println!(
        "  {} engine debug                      # Debug theme loading",
        NAME
    );
}

/// Display a single theme with visual properties (ENGINE-011 requirement)
fn display_theme_with_visual_properties(name: &str, theme: &crate::theme_engine::BoxyTheme) {
    use crate::colors::get_color_code;

    // Get visual elements
    let icon = get_icon_for_theme(name);
    let color_code = get_color_code(&theme.color);
    let text_color_code = if theme.text_color == "auto" {
        color_code
    } else {
        get_color_code(&theme.text_color)
    };

    // Get box drawing characters for the style
    let (top_left, horizontal, top_right, vertical, bottom_left, bottom_right) =
        get_box_chars_for_style_full(&theme.style);

    // Get layout tokens if present
    let layout_str = if let Some(ref layout) = theme.layout {
        format!(" [{}]", layout)
    } else {
        String::new()
    };

    // Build improved visual preview format:
    // theme_name | icon text(colored) border_type border_demo(colored) layouts
    let visual_preview = format!(
        "  {:14} │ {} {}Text{} {:8} {}{}{}{}{}{}{}{}{}{}",
        name,
        icon,
        if theme.text_color != "none" {
            text_color_code
        } else {
            ""
        },
        crate::colors::RESET,
        theme.style,
        color_code,
        top_left,
        horizontal,
        top_right,
        vertical,
        bottom_left,
        horizontal,
        bottom_right,
        crate::colors::RESET,
        layout_str
    );

    println!("{}", visual_preview);
}

/// Get appropriate icon for theme based on name patterns
fn get_icon_for_theme(name: &str) -> &'static str {
    match name {
        n if n.contains("error") => "❌",
        n if n.contains("success") => "✅",
        n if n.contains("warning") || n.contains("warn") => "⚠️",
        n if n.contains("info") => "ℹ️",
        n if n.contains("critical") => "⛔",
        n if n.contains("debug") => "🐛",
        n if n.contains("magic") => "✨",
        n if n.contains("silly") => "🎉",
        n if n.contains("blueprint") => "📐",
        n if n.contains("fatal") => "💀",
        n if n.contains("base") => "🔧",
        _ => "🎨", // Default theme icon
    }
}

/// Get box drawing characters for the given style
#[allow(dead_code)] // Retained for future theme style previews.
fn get_box_chars_for_style(style: &str) -> (char, char, char) {
    match style {
        "rounded" => ('╭', '─', '╮'),
        "double" => ('╔', '═', '╗'),
        "heavy" => ('┏', '━', '┓'),
        "ascii" => ('+', '-', '+'),
        _ => ('┌', '─', '┐'), // normal
    }
}

/// Get all box drawing characters for a complete box preview
fn get_box_chars_for_style_full(style: &str) -> (char, char, char, char, char, char) {
    match style {
        "rounded" => ('╭', '─', '╮', '│', '╰', '╯'),
        "double" => ('╔', '═', '╗', '║', '╚', '╝'),
        "heavy" => ('┏', '━', '┓', '┃', '┗', '┛'),
        "ascii" => ('+', '-', '+', '|', '+', '+'),
        _ => ('┌', '─', '┐', '│', '└', '┘'), // normal
    }
}

/// Generate default engine configuration with essential built-in themes
fn get_default_engine_config() -> String {
    r#"# Boxy Default Engine Configuration
# Auto-generated by `boxy engine init`
# This file contains the core built-in themes

metadata:
  name: "Default Boxy Themes"
  version: "0.12.2"
  description: "Built-in theme definitions for Boxy"
  created: "auto-generated"

themes:
  # Core success/error themes
  success:
    icon: "✅"
    color: "green"
    text_color: "auto"
    style: "rounded"
    description: "Success messages with rounded borders"

  error:
    icon: "❌"
    color: "red"
    text_color: "white"
    style: "heavy"
    description: "Error messages with heavy borders"

  warning:
    icon: "⚠️"
    color: "orange"
    text_color: "auto"
    style: "normal"
    description: "Warning messages"

  info:
    icon: "ℹ️"
    color: "blue"
    text_color: "auto"
    style: "normal"
    description: "Informational messages"

  critical:
    icon: "⛔"
    color: "crimson"
    text_color: "white"
    style: "double"
    description: "Critical alerts with double borders"

  debug:
    icon: "🐛"
    color: "purple"
    text_color: "auto"
    style: "ascii"
    description: "Debug messages with ASCII borders"

  # Special themes
  magic:
    icon: "✨"
    color: "magenta"
    text_color: "auto"
    style: "rounded"
    description: "Magical sparkly theme"

  silly:
    icon: "🎉"
    color: "rainbow"
    text_color: "auto"
    style: "rounded"
    description: "Fun party theme"

  # Base style templates (for inheritance)
  base_rounded:
    style: "rounded"
    color: "blue"
    description: "Base template for rounded borders"

  base_heavy:
    style: "heavy"
    color: "red"
    description: "Base template for heavy borders"

  base_double:
    style: "double"
    color: "purple"
    description: "Base template for double borders"
"#
    .to_string()
}
