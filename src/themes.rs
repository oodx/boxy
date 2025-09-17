// Legacy v0.5.0 theme system - maintained for backward compatibility
// v0.6.0+ uses theme_engine.rs with YAML themes

use std::path::PathBuf;

use crate::{ fs, io };
use crate::{ HashMap, Write, boxes::*, help::*};
use crate::{ JynxPlugin, jynx_println };
use crate::{ validate_color, validate_width};

use crate::theme_engine::{ThemeEngine, BoxyTheme, ThemeFile, ThemeMetadata, ThemeSettings};

// RSB framework imports  
use rsb::param;


/// Validate theme file before import
pub fn validate_theme_file(path: &PathBuf) -> Result<(), String> {
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


/// Handle theme subcommands: list, show, etc.
pub fn handle_theme_command(args: &[String], jynx: &JynxPlugin) {
    if args.is_empty() {
        eprintln!("Theme command requires an action. Usage: {} theme <action>", NAME);
        eprintln!("Available actions: list, show <theme>, hierarchy, dryrun <theme>, init, help");
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
        "hierarchy" => {
            match ThemeEngine::new() {
                Ok(theme_engine) => {
                    theme_engine.print_theme_hierarchy();
                }
                Err(e) => {
                    eprintln!("Error: Failed to load theme engine: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "init" => {
            handle_theme_init();
        }
        "dryrun" | "test" => {
            if args.len() < 2 {
                eprintln!("Error: Theme dryrun requires a theme name. Usage: {} theme dryrun <theme>", NAME);
                std::process::exit(1);
            }
            handle_theme_dryrun(&args[1]);
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
            println!("    hierarchy         Show theme loading hierarchy and sources");
            println!("    dryrun <theme>    Test theme application with sample content");
            println!("    init              Initialize local .themes/boxy-custom.yaml template");
            println!("    create <name>     Create a new theme interactively");
            println!("    import <path>     Import theme from file");
            println!("    export <name>     Export theme to file");
            println!("    edit <name>       Edit existing theme");
            println!("    help              Show this help message");
            println!();
            println!("EXAMPLES:");
            println!("    {} theme list", NAME);
            println!("    {} theme show error", NAME);
            println!("    {} theme hierarchy", NAME);
            println!("    {} theme dryrun error", NAME);
            println!("    {} theme init", NAME);
            println!("    {} theme create my_theme", NAME);
            println!("    {} theme export error > error.yml", NAME);
        }
        action => {
            eprintln!("Unknown theme action: {}", action);
            eprintln!("Available actions: list, show, hierarchy, dryrun, init, create, import, export, edit, help");
            eprintln!("Use '{} theme help' for more information", NAME);
            std::process::exit(1);
        }
    }
}

/// Handle engine subcommands: init, import, export, list, debug, etc.
pub fn handle_engine_command(args: &[String], jynx: &JynxPlugin) {
    if args.is_empty() {
        eprintln!("Engine command requires an action. Usage: {} engine <action>", NAME);
        eprintln!("Available actions: init, import <name>, export <name>, list, debug, status, edit <name>, help");
        std::process::exit(1);
    }

    match args[0].as_str() {
        "init" => {
            handle_engine_init();
        }
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
        "import" => {
            if args.len() < 2 {
                eprintln!("Error: Engine import requires a name. Usage: {} engine import <name>", NAME);
                std::process::exit(1);
            }
            handle_engine_import(&args[1]);
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("Error: Engine export requires a name. Usage: {} engine export <name>", NAME);
                std::process::exit(1);
            }
            handle_engine_export(&args[1]);
        }
        "edit" => {
            if args.len() < 2 {
                eprintln!("Error: Engine edit requires a name. Usage: {} engine edit <name>", NAME);
                std::process::exit(1);
            }
            handle_engine_edit(&args[1]);
        }
        "debug" => {
            match ThemeEngine::new() {
                Ok(theme_engine) => {
                    theme_engine.print_theme_hierarchy();
                }
                Err(e) => {
                    eprintln!("Error: Failed to load theme engine: {}", e);
                    std::process::exit(1);
                }
            }
        }
        "status" => {
            handle_engine_status();
        }
        "help" => {
            print_engine_help();
        }
        _ => {
            let action = &args[0];
            eprintln!("Unknown engine action: {}", action);
            eprintln!("Available actions: init, import, export, list, debug, status, edit, help");
            eprintln!("Use '{} engine help' for more information", NAME);
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

// TODO: WHAT IS THIS??
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
    let text_color = if text_color.is_empty() { "auto" } else { text_color };
    
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
pub fn edit_theme_interactively(name: &str, existing: &BoxyTheme) -> BoxyTheme {
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
pub fn save_theme_to_file(path: &PathBuf, name: &str, theme: &BoxyTheme) -> Result<(), String> {
    let theme_file = ThemeFile {
        metadata: ThemeMetadata {
            name: format!("{} Theme File", name),
            version: "1.0.0".to_string(),
            description: format!("Custom theme: {}", name),
            author: {
                let user = param!("USER");
                if user.is_empty() { "boxy".to_string() } else { user }
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
    
    fs::write(path, yaml_content)
        .map_err(|e| format!("Failed to write theme file: {}", e))?;
    
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

/// Handle `boxy theme create <name>` command
pub fn handle_theme_create(name: &str, jynx: &JynxPlugin) {
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
pub fn handle_theme_import(path: &str) {
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
pub fn handle_theme_export(name: &str) {
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
pub fn handle_theme_edit(name: &str) {
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





/// Legacy theme structure for v0.5.0 compatibility
//todo: incomplete/broken implementation
#[derive(Debug, Clone)]
pub struct Theme {
    #[allow(dead_code)]
    pub icon: &'static str,
    #[allow(dead_code)]
    pub color: &'static str,
    #[allow(dead_code)]
    pub width: Option<usize>,
}



//todo: incomplete/broken implementation
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

//todo: incomplete/broken implementation
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

//todo: incomplete/broken implementation
/// Hardcoded legacy themes as fallback (preserves v0.5.0 exact compatibility)
#[allow(dead_code)]
pub fn get_fallback_legacy_themes() -> HashMap<&'static str, Theme> {
    let mut themes = HashMap::new();
    
    themes.insert("fatal", Theme { icon: "💀", color: "red2", width: None });
    themes.insert("error", Theme { icon: "❌", color: "red", width: None });
    themes.insert("warn", Theme { icon: "⚠️", color: "orange", width: None });
    themes.insert("success", Theme { icon: "✅", color: "green", width: None });
    themes.insert("info", Theme { icon: "ℹ️", color: "blue2", width: None });
    themes.insert("debug", Theme { icon: "🐛", color: "grey", width: None });
    themes.insert("trace", Theme { icon: "👣", color: "grey2", width: None });
    themes.insert("dev", Theme { icon: "🪛", color: "cyan", width: None });
    themes.insert("new", Theme { icon: "✨", color: "green2", width: None });
    themes.insert("silly", Theme { icon: "🪀", color: "purple", width: None });
    themes.insert("magic", Theme { icon: "🌈", color: "purple2", width: None });
    themes.insert("think", Theme { icon: "💭", color: "cyan", width: None });
    themes.insert("notif", Theme { icon: "📣", color: "green", width: None });
    themes.insert("lore", Theme { icon: "🪬", color: "grey", width: None });
    themes.insert("blocked", Theme { icon: "🚧", color: "orange", width: None });
    themes.insert("help", Theme { icon: "💡", color: "blue2", width: None });
    themes.insert("oops", Theme { icon: "👻", color: "purple", width: None });
    themes.insert("lab", Theme { icon: "🧪", color: "cyan", width: None });
    themes.insert("lock", Theme { icon: "🔒", color: "grey2", width: None });
    themes.insert("unlock", Theme { icon: "🔓", color: "green", width: None });
    themes.insert("key", Theme { icon: "🔑", color: "orange", width: None });

    themes
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
                    "🎯 Unicode and emoji test: ℹ️ 📦 ✅ ❌ ⚠️"
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

                println!("💡 Use: echo \"your text\" | boxy --theme {}", theme_name);
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
    println!("  1. Edit {} to customize your themes", target_file.display());
    println!("  2. Test with: {} theme dryrun <theme_name>", NAME);
    println!("  3. Use with: echo \"text\" | {} --theme <theme_name>", NAME);
    println!();
    println!("💡 The .themes/ directory has the highest priority after individual boxy*.yaml files");
    println!("   Use: {} theme hierarchy to see the complete loading order", NAME);
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
"#.to_string()
}

// ==================== ENGINE COMMAND HANDLERS ====================

/// Handle `boxy engine init` command - creates global theme directory and default config
pub fn handle_engine_init() {
    // TODO: This will be implemented in ENGINE-008
    eprintln!("Error: Engine init command not yet implemented.");
    eprintln!("This will create the global theme directory: ~/.local/etc/rsb/boxy/themes/");
    eprintln!("Coming in ENGINE-008 implementation.");
    std::process::exit(1);
}

/// Handle `boxy engine import <name>` command - imports boxy_<name>.yml to global location
pub fn handle_engine_import(name: &str) {
    // TODO: Rename from theme import, implement proper file operations
    eprintln!("Error: Engine import command not yet fully implemented.");
    eprintln!("This will import boxy_{}.yml from local to global themes directory.", name);
    eprintln!("Coming in ENGINE-004 implementation.");
    std::process::exit(1);
}

/// Handle `boxy engine export <name>` command - exports boxy_<name>.yml from global to local
pub fn handle_engine_export(name: &str) {
    // TODO: Rename from theme export, implement proper file operations
    eprintln!("Error: Engine export command not yet fully implemented.");
    eprintln!("This will export boxy_{}.yml from global to local directory.", name);
    eprintln!("Coming in ENGINE-005 implementation.");
    std::process::exit(1);
}

/// Handle `boxy engine edit <name>` command - edits a config file
pub fn handle_engine_edit(name: &str) {
    // TODO: Move from theme edit, implement proper file operations
    eprintln!("Error: Engine edit command not yet fully implemented.");
    eprintln!("This will edit the boxy_{}.yml config file.", name);
    eprintln!("Coming in ENGINE-015 implementation.");
    std::process::exit(1);
}

/// Handle `boxy engine status` command - shows engine health
pub fn handle_engine_status() {
    // TODO: Implement engine status check
    eprintln!("Error: Engine status command not yet implemented.");
    eprintln!("This will show engine health, config count, themes count, etc.");
    eprintln!("Coming in ENGINE-012 implementation.");
    std::process::exit(1);
}

/// Print engine help
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
    println!("    edit <name>       Edit a theme config file");
    println!("    help              Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Engine commands manage theme configuration files (boxy_*.yml).");
    println!("    These are separate from individual theme usage commands.");
    println!();
    println!("EXAMPLES:");
    println!("    {} engine init                   # Set up global theme system", NAME);
    println!("    {} engine list                   # Show all available themes", NAME);
    println!("    {} engine debug                  # Debug theme loading", NAME);
    println!("    {} engine import myproject       # Import boxy_myproject.yml", NAME);
    println!("    {} engine export default         # Export boxy_default.yml", NAME);
}

