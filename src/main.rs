// Removed stray migration assistant block (v0.8)

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
    println!("    -w, --width <WIDTH|max|auto>  Set width: number, 'max' (terminal), or 'auto'");
    println!();
    
    println!("  {}Content & Layout:{}", get_color_code("cyan"), RESET);
    println!("    --header <TEXT>            External header (above the box)");
    println!("    --title <TEXT>             Title line (first in-box line; emoji-aware icon)");
    println!("    --footer <TEXT>            Footer text (inside bottom border)");
    println!("    --status <TEXT>            Status line inside box (use sl:|sc:|sr: prefixes)");
    println!("    --layout <spec>            Align/divide/pad: hl|hc|hr, fl|fc|fr, sl|sc|sr, dt|dtn, ds|dsn, stn|ptn|psn|ssn, bl|bc|br, bp");
    println!("    --pad <a|b>               Blank line above (a) and/or below (b) the body");
    println!("    --title-color <COLOR>      Color for title line (overrides --text)");
    println!("    --status-color <COLOR>     Color for status line (overrides --text)");
    println!("    --header-color <COLOR>     Color for header line");
    println!("    --footer-color <COLOR>     Color for footer line");
    println!();
    
    println!("  {}Theme System:{}", get_color_code("cyan"), RESET);
    println!("    --theme <THEME>            Apply semantic theme (error, success, warning, info)");
    println!();
    
    println!("  {}Utility:{}", get_color_code("cyan"), RESET);
    println!("    --no-boxy[=strict]         Strip box decoration (strict removes all formatting)");
    println!("    --no-color                 Disable jynx integration and color output");
    println!("    --params <stream>          Param stream: k='v'; pairs (hd, tl, st, ft, ic). Body comes from stdin");
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
    println!("    Env: BOXY_THEME=<name>      Set default theme (overridden by --theme)");
    println!();
    
    // Alignment and color palette are covered above; no migration section in v0.8
    
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
    
    // Migration content removed in v0.8 help
    
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
    
    // Migration subcommand removed in v0.8
    
    // PRIORITY 2: Check for other subcommands that should prevent stdin reading
    // This explicit check ensures no ambiguity about input precedence
    let has_subcommand = args.len() >= 2 && matches!(args[1].as_str(), "theme");
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
    let mut title_color: Option<String> = None;
    let mut status_color: Option<String> = None;
    let mut header_color: Option<String> = None;
    let mut footer_color: Option<String> = None;
    let mut header_align: &str = "center";
    let mut footer_align: &str = "center";
    let mut status_align_override: Option<String> = None;
    let mut body_align: &str = "left";
    let mut body_pad_emoji = false;
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
                                std::process::exit(1);
                            }
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
    
    let text = input.trim_end_matches('\n').to_string();

    // Params stream parsing: ONLY via --params flag. Piped stdin remains the body.
    if let Some(ref blob) = params_flag {
        if let Some(pc) = parse_content_stream(blob) {
            if header.is_none() { header = pc.header; }
            if footer.is_none() { footer = pc.footer; }
            if status_bar.is_none() { status_bar = pc.status; }
            if title.is_none() { title = pc.title; }
            if let Some(ic) = pc.icon { icon = Some(ic); }
            if title_color.is_none() { title_color = pc.title_color; }
            if status_color.is_none() { status_color = pc.status_color; }
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
    
    // Apply theme if specified - using new theme engine
    if let Some(theme_name_str) = &theme_name {
        match ThemeEngine::new() {
            Ok(theme_engine) => {
                if let Some(boxy_theme) = theme_engine.get_theme(theme_name_str) {
                    // Theme overrides defaults but explicit flags override theme
                    if color == "none" {
                        color = Box::leak(boxy_theme.color.clone().into_boxed_str());
                    }
                    // Prefer to use theme icon as icon decoration (first content line), not a separate line
                    if icon.is_none() {
                        if let Some(icon_str) = &boxy_theme.icon {
                            icon = Some(icon_str.clone());
                        } else if let Some(title_str) = &boxy_theme.title {
                            let emoji_part: String = title_str.chars().take_while(|c| !c.is_ascii()).collect();
                            if !emoji_part.trim().is_empty() { icon = Some(emoji_part.trim().to_string()); }
                        }
                    }
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
    // No longer prepend icon to the raw text; icon is injected on first line in draw_box
    
    if no_boxy {
        let stripped = strip_box(&text, strict_mode);
        println!("{}", stripped);
    } else {
        draw_box(&text, 1, 1, style, color, text_color, title.as_deref(), footer.as_deref(), icon.as_deref(), fixed_width, status_bar.as_deref(), header.as_deref(), header_align, footer_align, status_align_override.as_deref(), divider_after_title, divider_before_status, pad_after_title_divider, pad_before_status_divider, pad_before_title, pad_after_status, pad_after_title, pad_before_status, title_color.as_deref(), status_color.as_deref(), body_align, body_pad_emoji, pad_body_above, pad_body_below);
    }
}
fn handle_migrate_command(_args: &[String], _jynx: &JynxIntegration) {
    println!("{} {} - Migration Overview", NAME, VERSION);
    println!();
    println!("CHANGES IN v0.6.x:");
    println!("  • Header/Footer render inside borders");
    println!("  • Title is first in-box line; Status is in-box line");
    println!("  • Layout tokens: align (hl/hc/hr, fl/fc/fr, sl/sc/sr)");
    println!("    Dividers (dt/dtn, ds/dsn), Padding (stn/ptn/psn/ssn)");
    println!("    Body controls (bl/bc/br for align, bp for emoji pad)");
    println!("  • Param stream (--params): hd/tl/st/ft/ic + tc/sc + ly");
    println!("  • Title/Status color overrides (--title-color/--status-color)");
    println!("  • Width keywords: --width max|auto");
    println!("  • BOXY_THEME default theme");
}
