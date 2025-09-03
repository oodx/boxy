// Advanced Theme Engine - YAML-based theme system for boxy v0.6+
// Inherits complete jynx architecture patterns with XDG+ directory support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::colors::*;

/// Main theme engine - manages all theme operations
pub struct ThemeEngine {
    themes: HashMap<String, BoxyTheme>,
    theme_files: Vec<PathBuf>,
    default_theme: String,
    xdg_base_dir: PathBuf,
}

/// Complete theme definition with all v0.6+ features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxyTheme {
    // === CORE VISUAL PROPERTIES ===
    pub color: String,                    // Box border color (required)
    #[serde(default = "default_text_color")]
    pub text_color: String,               // Text color: "auto", "none", or color name
    #[serde(default = "default_style")]
    pub style: String,                    // Border style: normal, rounded, double, heavy, ascii
    
    // === TEXT STYLING (v0.6+ feature) ===
    #[serde(default = "default_text_style")]
    pub text_style: String,               // Text formatting: normal, bold, italic, etc.
    
    // === BOX CONTENT ===
    pub title: Option<String>,            // Internal title with icon
    pub header: Option<String>,           // External header above box
    pub footer: Option<String>,           // Footer below box
    pub icon: Option<String>,             // Leading icon for content
    
    // === LAYOUT PROPERTIES ===
    pub width: Option<usize>,             // Fixed width in characters
    #[serde(default = "default_padding")]
    pub padding: usize,                   // Internal padding (default: 1)
    
    // === LAYOUT ALIGNMENT ===
    #[serde(default = "default_align")]
    pub title_align: String,              // Title alignment: left, center, right
    #[serde(default = "default_align")]
    pub header_align: String,             // Header alignment: left, center, right
    #[serde(default = "default_align")]
    pub footer_align: String,             // Footer alignment: left, center, right
    
    // === ADVANCED FEATURES ===
    pub status_bar: Option<String>,       // Status bar below box
    #[serde(default = "default_status_align")]
    pub status_align: String,             // Status alignment: left, center, right
    
    // === INHERITANCE ===
    pub inherits: Option<String>,         // Inherit from another theme
    
    // === METADATA ===
    #[serde(skip_serializing, skip_deserializing)]
    pub metadata: Option<ThemeMetadata>,  // Theme metadata (populated at runtime)
}

/// Theme file structure - complete YAML theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeFile {
    pub metadata: ThemeMetadata,
    #[serde(default)]
    pub colors: HashMap<String, String>,     // Custom color definitions
    pub themes: HashMap<String, BoxyTheme>,  // Theme definitions
    #[serde(default)]
    pub presets: HashMap<String, String>,    // Quick preset mappings
    #[serde(default)]
    pub text_styles: HashMap<String, String>, // Text style definitions
    #[serde(default)]
    pub settings: ThemeSettings,             // Theme file settings
}

/// Theme metadata for versioning and attribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
    #[serde(default)]
    pub compatibility: String,
}

/// Theme file settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeSettings {
    #[serde(default = "default_theme_name")]
    pub default_theme: String,
    #[serde(default = "default_fallback_color")]
    pub fallback_color: String,
    #[serde(default = "default_max_width")]
    pub max_width: usize,
    #[serde(default = "default_min_width")]
    pub min_width: usize,
    #[serde(default = "default_true")]
    pub cache_themes: bool,
    #[serde(default = "default_true")]
    pub validate_colors: bool,
}

// Default value functions for serde
fn default_text_color() -> String { "auto".to_string() }
fn default_style() -> String { "normal".to_string() }
fn default_text_style() -> String { "normal".to_string() }
fn default_padding() -> usize { 1 }
fn default_align() -> String { "center".to_string() }
fn default_status_align() -> String { "left".to_string() }
fn default_theme_name() -> String { "info".to_string() }
fn default_fallback_color() -> String { "slate".to_string() }
fn default_max_width() -> usize { 120 }
fn default_min_width() -> usize { 10 }
fn default_true() -> bool { true }

impl ThemeEngine {
    /// Create new theme engine with XDG+ directory support
    pub fn new() -> Result<Self, String> {
        let xdg_base_dir = Self::get_xdg_base_dir();
        
        let mut engine = ThemeEngine {
            themes: HashMap::new(),
            theme_files: Vec::new(),
            default_theme: "info".to_string(),
            xdg_base_dir,
        };
        
        // Load built-in themes first (fallback)
        engine.load_builtin_themes();
        
        // Load YAML theme files from XDG+ directories
        if let Err(e) = engine.load_theme_files() {
            eprintln!("Warning: Failed to load theme files: {}", e);
            // Continue with built-in themes
        }
        
        Ok(engine)
    }
    
    /// Get XDG+ base directory following jynx architecture
    fn get_xdg_base_dir() -> PathBuf {
        // Follow jynx XDG+ pattern: ~/.local/etc/rsb/boxy/
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join(".local/etc/rsb/boxy")
    }
    
    /// Load built-in themes as fallback (converted from current themes.rs)
    fn load_builtin_themes(&mut self) {
        let builtin_themes = vec![
            ("error", BoxyTheme {
                color: "crimson".to_string(),
                text_color: "white".to_string(),
                style: "heavy".to_string(),
                text_style: "bold".to_string(),
                title: Some("❌ Error".to_string()),
                icon: Some("❌".to_string()),
                width: Some(60),
                ..Default::default()
            }),
            ("success", BoxyTheme {
                color: "emerald".to_string(),
                text_color: "auto".to_string(),
                style: "rounded".to_string(),
                text_style: "bold".to_string(),
                title: Some("✅ Success".to_string()),
                icon: Some("✅".to_string()),
                ..Default::default()
            }),
            ("warning", BoxyTheme {
                color: "amber".to_string(),
                text_color: "auto".to_string(),
                style: "heavy".to_string(),
                text_style: "italic".to_string(),
                title: Some("⚠️ Warning".to_string()),
                icon: Some("⚠️".to_string()),
                ..Default::default()
            }),
            ("info", BoxyTheme {
                color: "azure".to_string(),
                text_color: "auto".to_string(),
                style: "normal".to_string(),
                text_style: "normal".to_string(),
                title: Some("ℹ️ Info".to_string()),
                icon: Some("ℹ️".to_string()),
                ..Default::default()
            }),
        ];
        
        for (name, theme) in builtin_themes {
            self.themes.insert(name.to_string(), theme);
        }
    }
    
    /// Load theme files from XDG+ directories
    fn load_theme_files(&mut self) -> Result<(), String> {
        // Create XDG+ directory structure if it doesn't exist
        let themes_dir = self.xdg_base_dir.join("themes");
        if !themes_dir.exists() {
            fs::create_dir_all(&themes_dir)
                .map_err(|e| format!("Failed to create themes directory: {}", e))?;
        }
        
        // Load theme files from themes directory
        if themes_dir.exists() {
            let entries = fs::read_dir(&themes_dir)
                .map_err(|e| format!("Failed to read themes directory: {}", e))?;
                
            for entry in entries {
                let path = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?.path();
                if path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                    if let Err(e) = self.load_theme_file(&path) {
                        eprintln!("Warning: Failed to load theme file {:?}: {}", path, e);
                        // Continue loading other files
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Load individual theme file
    fn load_theme_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read theme file: {}", e))?;
            
        let theme_file: ThemeFile = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;
            
        // Validate themes before adding
        for (name, theme) in &theme_file.themes {
            if let Err(e) = self.validate_theme(theme) {
                eprintln!("Warning: Invalid theme '{}': {}", name, e);
                continue;
            }
        }
        
        // Add themes to engine
        for (name, mut theme) in theme_file.themes {
            // Set metadata
            theme.metadata = Some(theme_file.metadata.clone());
            self.themes.insert(name, theme);
        }
        
        self.theme_files.push(path.clone());
        Ok(())
    }
    
    /// Validate theme configuration
    pub fn validate_theme(&self, theme: &BoxyTheme) -> Result<(), String> {
        // Validate color
        validate_color(&theme.color)?;
        
        // Validate text color
        if theme.text_color != "auto" && theme.text_color != "none" {
            validate_color(&theme.text_color)?;
        }
        
        // Validate style
        let valid_styles = vec!["normal", "rounded", "double", "heavy", "ascii"];
        if !valid_styles.contains(&theme.style.as_str()) {
            return Err(format!("Invalid style '{}'. Valid styles: {:?}", theme.style, valid_styles));
        }
        
        // Validate width constraints
        if let Some(width) = theme.width {
            if width < 10 || width > 200 {
                return Err(format!("Width {} out of range (10-200)", width));
            }
        }
        
        Ok(())
    }
    
    /// Get theme by name with inheritance resolution
    pub fn get_theme(&self, name: &str) -> Option<BoxyTheme> {
        // Direct theme lookup
        if let Some(theme) = self.themes.get(name) {
            return Some(self.resolve_inheritance(theme.clone()));
        }
        
        // Smart resolution: "error" -> "theme_error" -> "error_theme"
        let variations = vec![
            format!("theme_{}", name),
            format!("{}_theme", name),
            format!("{}_box", name),
        ];
        
        for variation in variations {
            if let Some(theme) = self.themes.get(&variation) {
                return Some(self.resolve_inheritance(theme.clone()));
            }
        }
        
        None
    }
    
    /// Resolve theme inheritance
    fn resolve_inheritance(&self, mut theme: BoxyTheme) -> BoxyTheme {
        if let Some(parent_name) = &theme.inherits {
            if let Some(parent_theme) = self.themes.get(parent_name) {
                // Merge parent theme with child theme (child overrides parent)
                theme = self.merge_themes(parent_theme.clone(), theme);
            }
        }
        theme
    }
    
    /// Merge parent and child themes (child takes precedence)
    fn merge_themes(&self, parent: BoxyTheme, child: BoxyTheme) -> BoxyTheme {
        BoxyTheme {
            color: if child.color.is_empty() { parent.color } else { child.color },
            text_color: if child.text_color == "auto" && parent.text_color != "auto" { parent.text_color } else { child.text_color },
            style: if child.style == "normal" && parent.style != "normal" { parent.style } else { child.style },
            text_style: if child.text_style == "normal" && parent.text_style != "normal" { parent.text_style } else { child.text_style },
            title: child.title.or(parent.title),
            header: child.header.or(parent.header),
            footer: child.footer.or(parent.footer),
            icon: child.icon.or(parent.icon),
            width: child.width.or(parent.width),
            padding: if child.padding == 1 && parent.padding != 1 { parent.padding } else { child.padding },
            title_align: if child.title_align == "center" && parent.title_align != "center" { parent.title_align } else { child.title_align },
            header_align: if child.header_align == "center" && parent.header_align != "center" { parent.header_align } else { child.header_align },
            footer_align: if child.footer_align == "center" && parent.footer_align != "center" { parent.footer_align } else { child.footer_align },
            status_bar: child.status_bar.or(parent.status_bar),
            status_align: if child.status_align == "left" && parent.status_align != "left" { parent.status_align } else { child.status_align },
            inherits: None, // Clear inheritance to prevent cycles
            metadata: child.metadata.or(parent.metadata),
        }
    }
    
    /// List all available themes
    pub fn list_themes(&self) -> Vec<(String, String)> {
        let mut themes: Vec<(String, String)> = self.themes.iter()
            .map(|(name, theme)| {
                let description = theme.metadata.as_ref()
                    .map(|m| m.description.clone())
                    .unwrap_or_else(|| format!("Theme with {} border", theme.color));
                (name.clone(), description)
            })
            .collect();
            
        themes.sort_by(|a, b| a.0.cmp(&b.0));
        themes
    }
    
    /// Get XDG+ themes directory path
    pub fn get_themes_directory(&self) -> PathBuf {
        self.xdg_base_dir.join("themes")
    }
}

impl Default for BoxyTheme {
    fn default() -> Self {
        BoxyTheme {
            color: "azure".to_string(),
            text_color: "auto".to_string(),
            style: "normal".to_string(),
            text_style: "normal".to_string(),
            title: None,
            header: None,
            footer: None,
            icon: None,
            width: None,
            padding: 1,
            title_align: "center".to_string(),
            header_align: "center".to_string(),
            footer_align: "center".to_string(),
            status_bar: None,
            status_align: "left".to_string(),
            inherits: None,
            metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_engine_creation() {
        let engine = ThemeEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_builtin_themes() {
        let engine = ThemeEngine::new().unwrap();
        
        // Test that built-in themes are loaded
        assert!(engine.get_theme("error").is_some());
        assert!(engine.get_theme("success").is_some());
        assert!(engine.get_theme("warning").is_some());
        assert!(engine.get_theme("info").is_some());
    }

    #[test]
    fn test_theme_validation() {
        let engine = ThemeEngine::new().unwrap();
        
        // Valid theme
        let valid_theme = BoxyTheme {
            color: "crimson".to_string(),
            style: "heavy".to_string(),
            ..Default::default()
        };
        assert!(engine.validate_theme(&valid_theme).is_ok());
        
        // Invalid color
        let invalid_theme = BoxyTheme {
            color: "invalid_color".to_string(),
            ..Default::default()
        };
        assert!(engine.validate_theme(&invalid_theme).is_err());
    }

    #[test]
    fn test_smart_theme_resolution() {
        let engine = ThemeEngine::new().unwrap();
        
        // Direct lookup
        assert!(engine.get_theme("error").is_some());
        
        // Should fallback gracefully for unknown themes
        assert!(engine.get_theme("unknown_theme").is_none());
    }
}