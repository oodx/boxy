// Legacy v0.5.0 theme system - maintained for backward compatibility
// v0.6.0+ uses theme_engine.rs with YAML themes

use std::collections::HashMap;
use crate::theme_engine::{ThemeEngine, BoxyTheme};

/// Legacy theme structure for v0.5.0 compatibility
#[derive(Debug, Clone)]
pub struct Theme {
    pub icon: &'static str,
    pub color: &'static str,
    pub width: Option<usize>,
}

/// Get legacy themes (v0.5.0 style) - converts from new YAML system when available
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
fn convert_boxy_theme_to_legacy(boxy_theme: BoxyTheme) -> Theme {
    let icon = boxy_theme.icon.as_deref().unwrap_or("📦");
    Theme {
        icon: Box::leak(icon.to_string().into_boxed_str()),
        color: Box::leak(boxy_theme.color.into_boxed_str()),
        width: boxy_theme.width,
    }
}

/// Hardcoded legacy themes as fallback (preserves v0.5.0 exact compatibility)
fn get_fallback_legacy_themes() -> HashMap<&'static str, Theme> {
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