//! Built-in theme definitions for Boxy
//!
//! This module contains all hardcoded theme definitions organized by defaults level:
//! - Level 0: Minimal themes (basic box styles only)
//! - Level 1: Standard themes (semantic themes: error, success, warning, info)
//! - Level 2: Extended themes (standard + legacy unique themes)
//!
//! Environment variable BOXY_DEFAULTS_LEVEL controls which themes are loaded.

use std::env;
use std::collections::HashMap;
use crate::theme_engine::BoxyTheme;

/// Parse BOXY_DEFAULTS_LEVEL environment variable with validation and warnings
/// If override_level is provided, it takes precedence over the environment variable
pub fn parse_defaults_level(override_level: Option<u8>) -> u8 {
    // Check for CLI override first
    if let Some(level) = override_level {
        return level;
    }
    match env::var("BOXY_DEFAULTS_LEVEL") {
        Ok(value) => {
            match value.parse::<u8>() {
                Ok(0) => 0,
                Ok(1) => 1,
                Ok(2) => 2,
                Ok(n) if n > 2 => {
                    eprintln!("Warning: BOXY_DEFAULTS_LEVEL={} invalid (>2), using level 2", n);
                    2
                },
                Ok(n) => {
                    eprintln!("Warning: BOXY_DEFAULTS_LEVEL={} invalid, using level 0", n);
                    0
                },
                Err(_) => {
                    eprintln!("Warning: BOXY_DEFAULTS_LEVEL='{}' invalid (not a number), using level 0", value);
                    0
                }
            }
        },
        Err(_) => 2 // Default to extended themes (level 2)
    }
}

/// Get builtin themes based on defaults level
pub fn get_builtin_themes(override_level: Option<u8>) -> (HashMap<String, BoxyTheme>, Vec<String>) {
    let level = parse_defaults_level(override_level);

    match level {
        0 => get_minimal_themes(),
        1 => get_standard_themes(),
        2 => get_extended_themes(),
        _ => unreachable!() // parse_defaults_level handles validation
    }
}

/// Level 0: Minimal themes - basic box styles only
fn get_minimal_themes() -> (HashMap<String, BoxyTheme>, Vec<String>) {
    let mut themes = HashMap::new();
    let trail = vec!["  📦 Built-in themes (Level 0 - Minimal): default styles, blueprint".to_string()];

    // Basic default theme (plain white box)
    themes.insert("default".to_string(), BoxyTheme {
        color: "white".to_string(),
        text_color: "auto".to_string(),
        style: "normal".to_string(),
        text_style: "normal".to_string(),
        ..Default::default()
    });

    // Box style variants
    themes.insert("default_rounded".to_string(), BoxyTheme {
        color: "white".to_string(),
        text_color: "auto".to_string(),
        style: "rounded".to_string(),
        text_style: "normal".to_string(),
        ..Default::default()
    });

    themes.insert("default_double".to_string(), BoxyTheme {
        color: "white".to_string(),
        text_color: "auto".to_string(),
        style: "double".to_string(),
        text_style: "normal".to_string(),
        ..Default::default()
    });

    themes.insert("default_heavy".to_string(), BoxyTheme {
        color: "white".to_string(),
        text_color: "auto".to_string(),
        style: "heavy".to_string(),
        text_style: "normal".to_string(),
        ..Default::default()
    });

    themes.insert("default_ascii".to_string(), BoxyTheme {
        color: "white".to_string(),
        text_color: "auto".to_string(),
        style: "ascii".to_string(),
        text_style: "normal".to_string(),
        ..Default::default()
    });

    // Blueprint as the one "styled" theme
    themes.insert("blueprint".to_string(), BoxyTheme {
        color: "blue".to_string(),
        text_color: "auto".to_string(),
        style: "ascii".to_string(),
        text_style: "normal".to_string(),
        title: Some("📐 Blueprint".to_string()),
        icon: Some("📐".to_string()),
        ..Default::default()
    });

    (themes, trail)
}

/// Level 1: Standard themes - current semantic themes
fn get_standard_themes() -> (HashMap<String, BoxyTheme>, Vec<String>) {
    let mut themes = HashMap::new();
    let trail = vec!["  📦 Built-in themes (Level 1 - Standard): error, success, warning, info".to_string()];

    themes.insert("error".to_string(), BoxyTheme {
        color: "crimson".to_string(),
        text_color: "white".to_string(),
        style: "heavy".to_string(),
        text_style: "bold".to_string(),
        title: Some("❌ Error".to_string()),
        icon: Some("❌".to_string()),
        width: Some(60),
        ..Default::default()
    });

    themes.insert("success".to_string(), BoxyTheme {
        color: "emerald".to_string(),
        text_color: "auto".to_string(),
        style: "rounded".to_string(),
        text_style: "bold".to_string(),
        title: Some("✅ Success".to_string()),
        icon: Some("✅".to_string()),
        ..Default::default()
    });

    themes.insert("warning".to_string(), BoxyTheme {
        color: "amber".to_string(),
        text_color: "auto".to_string(),
        style: "heavy".to_string(),
        text_style: "italic".to_string(),
        title: Some("⚠️ Warning".to_string()),
        icon: Some("⚠️".to_string()),
        ..Default::default()
    });

    themes.insert("info".to_string(), BoxyTheme {
        color: "azure".to_string(),
        text_color: "auto".to_string(),
        style: "normal".to_string(),
        text_style: "normal".to_string(),
        title: Some("ℹ️ Info".to_string()),
        icon: Some("ℹ️".to_string()),
        ..Default::default()
    });

    (themes, trail)
}

/// Level 2: Extended themes - standard + unique legacy themes
fn get_extended_themes() -> (HashMap<String, BoxyTheme>, Vec<String>) {
    let (mut themes, mut trail) = get_standard_themes();

    // Update trail to reflect extended set
    trail[0] = "  📦 Built-in themes (Level 2 - Extended): error, success, warning, info + legacy themes".to_string();

    // Add unique legacy themes (no overlaps with YAML themes)
    let legacy_themes = vec![
        ("trace", BoxyTheme {
            color: "grey2".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("👣 Trace".to_string()),
            icon: Some("👣".to_string()),
            ..Default::default()
        }),
        ("dev", BoxyTheme {
            color: "cyan".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("🪛 Dev".to_string()),
            icon: Some("🪛".to_string()),
            ..Default::default()
        }),
        ("new", BoxyTheme {
            color: "green2".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("✨ New".to_string()),
            icon: Some("✨".to_string()),
            ..Default::default()
        }),
        ("think", BoxyTheme {
            color: "cyan".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("💭 Think".to_string()),
            icon: Some("💭".to_string()),
            ..Default::default()
        }),
        ("notif", BoxyTheme {
            color: "green".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("📣 Notification".to_string()),
            icon: Some("📣".to_string()),
            ..Default::default()
        }),
        ("lore", BoxyTheme {
            color: "grey".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("🪬 Lore".to_string()),
            icon: Some("🪬".to_string()),
            ..Default::default()
        }),
        ("blocked", BoxyTheme {
            color: "orange".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("🚧 Blocked".to_string()),
            icon: Some("🚧".to_string()),
            ..Default::default()
        }),
        ("help", BoxyTheme {
            color: "blue2".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("💡 Help".to_string()),
            icon: Some("💡".to_string()),
            ..Default::default()
        }),
        ("oops", BoxyTheme {
            color: "purple".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("👻 Oops".to_string()),
            icon: Some("👻".to_string()),
            ..Default::default()
        }),
        ("lab", BoxyTheme {
            color: "cyan".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("🧪 Lab".to_string()),
            icon: Some("🧪".to_string()),
            ..Default::default()
        }),
        ("lock", BoxyTheme {
            color: "grey2".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("🔒 Lock".to_string()),
            icon: Some("🔒".to_string()),
            ..Default::default()
        }),
        ("unlock", BoxyTheme {
            color: "green".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("🔓 Unlock".to_string()),
            icon: Some("🔓".to_string()),
            ..Default::default()
        }),
        ("key", BoxyTheme {
            color: "orange".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: Some("🔑 Key".to_string()),
            icon: Some("🔑".to_string()),
            ..Default::default()
        }),
    ];

    for (name, theme) in legacy_themes {
        themes.insert(name.to_string(), theme);
    }

    (themes, trail)
}