//! Core utilities - RSB MODULE_SPEC compliant core system
//!
//! This module provides the public API for core boxy functionality including:
//! - Configuration management (BoxyConfig and related types)
//! - Content parsing and processing
//! - Text wrapping and formatting
//! - Help system and documentation
//!
//! Consolidated from config.rs, parser.rs, and help.rs following RSB MODULE_SPEC.
//!
//! CRITICAL: Preserves icon detection logic from parser.rs:385-410
//!
//! Version: boxy v0.16.0+ (RSB MODULE_SPEC reorganization)

use crate::visual::BoxStyle;
use crate::colors::*;
use crate::jynx_plugin::*;
use crate::{HashMap, Regex};
use crate::width_plugin::*;

// =============== CONSTANTS ===============
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// =============== CONFIGURATION TYPES ===============

/// Body content alignment options
#[derive(Debug, Clone, PartialEq)]
pub enum BodyAlignment {
    Left,
    Center,
    Right,
}

impl Default for BodyAlignment {
    fn default() -> Self {
        Self::Left
    }
}

impl From<&str> for BodyAlignment {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "center" => Self::Center,
            "right" => Self::Right,
            _ => Self::Left,
        }
    }
}

/// Width configuration for box sizing
#[derive(Debug, Clone)]
pub struct WidthConfig {
    pub fixed_width: Option<usize>,
    pub h_padding: usize,
    #[allow(dead_code)] // Future feature: vertical padding support
    pub v_padding: usize,
    pub enable_wrapping: bool,
}

impl Default for WidthConfig {
    fn default() -> Self {
        Self {
            fixed_width: None,
            h_padding: 1,
            v_padding: 1,
            enable_wrapping: false,
        }
    }
}

/// Color configuration for different box elements
#[derive(Debug, Clone)]
pub struct BoxColors {
    pub box_color: String,
    pub text_color: String,
    pub title_color: Option<String>,
    pub status_color: Option<String>,
    #[allow(dead_code)] // Future feature: header color customization
    pub header_color: Option<String>,
    #[allow(dead_code)] // Future feature: footer color customization
    pub footer_color: Option<String>,
}

impl Default for BoxColors {
    fn default() -> Self {
        Self {
            box_color: "white".to_string(),
            text_color: "none".to_string(),
            title_color: None,
            status_color: None,
            header_color: None,
            footer_color: None,
        }
    }
}

/// Divider configuration between sections
#[derive(Debug, Clone)]
pub struct DividerConfig {
    pub divider_after_title: bool,
    pub divider_before_status: bool,
    pub pad_after_title_divider: bool,
    pub pad_before_status_divider: bool,
}

impl Default for DividerConfig {
    fn default() -> Self {
        Self {
            divider_after_title: false,
            divider_before_status: false,
            pad_after_title_divider: false,
            pad_before_status_divider: false,
        }
    }
}

/// Padding configuration around different elements
#[derive(Debug, Clone)]
pub struct PaddingConfig {
    pub pad_before_title: bool,
    pub pad_after_title: bool,
    pub pad_before_status: bool,
    pub pad_after_status: bool,
    #[allow(dead_code)] // Future feature: body vertical padding
    pub pad_body_above: bool,
    #[allow(dead_code)] // Future feature: body vertical padding
    pub pad_body_below: bool,
}

impl Default for PaddingConfig {
    fn default() -> Self {
        Self {
            pad_before_title: false,
            pad_after_title: false,
            pad_before_status: false,
            pad_after_status: false,
            pad_body_above: false,
            pad_body_below: false,
        }
    }
}

/// Alignment configuration for headers and footers
#[derive(Debug, Clone)]
pub struct AlignmentConfig {
    pub header_align: String,
    pub footer_align: String,
    pub status_align_override: Option<String>,
}

impl Default for AlignmentConfig {
    fn default() -> Self {
        Self {
            header_align: "left".to_string(),
            footer_align: "left".to_string(),
            status_align_override: None,
        }
    }
}

/// Main configuration struct that replaces the 28-parameter draw_box function
#[derive(Debug, Clone)]
pub struct BoxyConfig {
    // Content
    pub text: String,
    pub title: Option<String>,
    pub footer: Option<String>,
    pub header: Option<String>,
    pub status_bar: Option<String>,
    pub icon: Option<String>,

    // Layout
    #[allow(dead_code)] // Future feature: body text alignment
    pub body_align: BodyAlignment,
    #[allow(dead_code)] // Future feature: emoji padding in body text
    pub body_pad_emoji: bool,

    // Styling
    pub style: BoxStyle,
    pub colors: BoxColors,
    pub width: WidthConfig,
    // Optional fixed height; only honored when BOXY_MULTIPLEX_MODE is enabled
    #[allow(dead_code)]
    pub fixed_height: Option<usize>,
    pub padding: PaddingConfig,

    // Advanced layout
    pub dividers: DividerConfig,
    pub alignment: AlignmentConfig,
}

impl Default for BoxyConfig {
    fn default() -> Self {
        Self {
            text: String::new(),
            title: None,
            footer: None,
            header: None,
            status_bar: None,
            icon: None,
            body_align: BodyAlignment::default(),
            body_pad_emoji: false,
            style: BoxStyle::default(),
            colors: BoxColors::default(),
            width: WidthConfig::default(),
            fixed_height: None,
            padding: PaddingConfig::default(),
            dividers: DividerConfig::default(),
            alignment: AlignmentConfig::default(),
        }
    }
}

// =============== PARSER TYPES ===============

#[derive(Default, Debug)]
pub struct ParsedContent {
    pub header: Option<String>,
    pub footer: Option<String>,
    pub status: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub icon: Option<String>,
    pub layout: Option<String>,
    pub title_color: Option<String>,
    pub status_color: Option<String>,
    pub header_color: Option<String>,
    pub footer_color: Option<String>
}

// =============== CONFIGURATION FUNCTIONS ===============

/// RSB-compliant config resolver that builds BoxyConfig from CLI arguments
pub fn resolve_box_config(
    text: &str,
    h_padding: usize,
    v_padding: usize,
    style: &BoxStyle,
    color: &str,
    text_color: &str,
    title: Option<&str>,
    footer: Option<&str>,
    icon: Option<&str>,
    fixed_width: Option<usize>,
    status_bar: Option<&str>,
    header: Option<&str>,
    header_align: &str,
    footer_align: &str,
    status_align_override: Option<&str>,
    divider_after_title: bool,
    divider_before_status: bool,
    pad_after_title_divider: bool,
    pad_before_status_divider: bool,
    pad_before_title: bool,
    pad_after_status: bool,
    pad_after_title: bool,
    pad_before_status: bool,
    title_color_name: Option<&str>,
    status_color_name: Option<&str>,
    body_align: &str,
    body_pad_emoji: bool,
    pad_body_above: bool,
    pad_body_below: bool,
    header_color: Option<&str>,
    footer_color: Option<&str>,
    enable_wrapping: bool,
) -> BoxyConfig {
    BoxyConfig {
        text: text.to_string(),
        title: title.map(|s| s.to_string()),
        footer: footer.map(|s| s.to_string()),
        header: header.map(|s| s.to_string()),
        status_bar: status_bar.map(|s| s.to_string()),
        icon: icon.map(|s| s.to_string()),
        body_align: BodyAlignment::from(body_align),
        body_pad_emoji,
        style: *style,
        colors: BoxColors {
            box_color: color.to_string(),
            text_color: text_color.to_string(),
            title_color: title_color_name.map(|s| s.to_string()),
            status_color: status_color_name.map(|s| s.to_string()),
            header_color: header_color.map(|s| s.to_string()),
            footer_color: footer_color.map(|s| s.to_string()),
        },
        width: WidthConfig {
            fixed_width,
            h_padding,
            v_padding,
            enable_wrapping,
        },
        fixed_height: None,
        padding: PaddingConfig {
            pad_before_title,
            pad_after_title,
            pad_before_status,
            pad_after_status,
            pad_body_above,
            pad_body_below,
        },
        dividers: DividerConfig {
            divider_after_title,
            divider_before_status,
            pad_after_title_divider,
            pad_before_status_divider,
        },
        alignment: AlignmentConfig {
            header_align: header_align.to_string(),
            footer_align: footer_align.to_string(),
            status_align_override: status_align_override.map(|s| s.to_string()),
        },
    }
}

// =============== PARSER FUNCTIONS ===============

pub fn expand_variables(text: &str) -> String {
    let mut result = text.to_string();
    let var_regex = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for cap in var_regex.captures_iter(text) {
        if let Some(var_name) = cap.get(1) {
            let value = std::env::var(var_name.as_str()).unwrap_or_default();
            if !value.is_empty() {
                result = result.replace(&cap[0], &value);
            }
        }
    }
    result
}

pub fn unescape_stream_value(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some(other) => { out.push(other); },
                None => break,
            }
        } else if c == '/' {
            if let Some('n') = chars.peek().copied() { chars.next(); out.push('\n'); } else { out.push(c); }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn parse_content_stream(input: &str) -> Option<ParsedContent> {
    // Matches k='v' with single quotes; non-greedy across newlines; optional trailing semicolon
    let re = Regex::new(r"(?s)([A-Za-z]{2})\s*=\s*'(.+?)'\s*;?").ok()?;
    let mut map: HashMap<String, String> = HashMap::new();
    for cap in re.captures_iter(input) {
        let k = cap.get(1).map(|m| m.as_str().to_lowercase()).unwrap_or_default();
        let v_raw = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let v = unescape_stream_value(v_raw);
        map.insert(k, v);
    }
    if map.is_empty() {
        return None;
    }
    let mut pc = ParsedContent::default();
    if let Some(v) = map.remove("hd") { pc.header = Some(v); }
    if let Some(v) = map.remove("ft") { pc.footer = Some(v); }
    if let Some(v) = map.remove("st") { pc.status = Some(v); }
    if let Some(v) = map.remove("tl") { pc.title = Some(v); }
    // Body (bd) intentionally ignored; body should come from piped stdin
    if let Some(v) = map.remove("ic") { pc.icon = Some(v); }
    if let Some(v) = map.remove("tc") { pc.title_color = Some(v); }
    if let Some(v) = map.remove("sc") { pc.status_color = Some(v); }
    // If nothing recognized, return None to avoid hijacking arbitrary input
    if pc.header.is_none() && pc.footer.is_none() && pc.status.is_none() && pc.title.is_none() && pc.body.is_none() && pc.icon.is_none() {
        None
    } else {
        Some(pc)
    }
}

/// Wrap text at word boundaries with intelligent hint support
///
/// Hint markers:
/// - #W# : Ideal wrap point if needed
/// - #T# : Truncate everything before this point, wrap content after
/// - #NL# : Explicit newline
pub fn wrap_text_at_word_boundaries(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![String::new()];
    }

    // First, convert #NL# markers to actual newlines
    let text_with_newlines = text.replace("#NL#", "\n");

    let mut lines = Vec::new();

    for line in text_with_newlines.lines() {
        // Process each line separately to preserve original line breaks
        let wrapped_lines = crate::core::helpers::wrap_single_line(line, max_width);
        lines.extend(wrapped_lines);
    }

    // Ensure we return at least one line
    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

pub fn truncate_with_ellipsis(text: &str, max_width: usize) -> String {
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

    // Process character by character, but handle multi-codepoint sequences properly
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        // Build the complete grapheme cluster (handle variation selectors, etc.)
        let mut grapheme = String::new();
        grapheme.push(ch);

        // Check for variation selectors or other combining characters
        while let Some(&next_ch) = chars.peek() {
            if matches!(next_ch, '\u{FE0E}' | '\u{FE0F}' | '\u{200D}') {
                grapheme.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        let grapheme_width = get_display_width(&grapheme);
        if current_width + grapheme_width > target_width {
            break;
        }
        result.push_str(&grapheme);
        current_width += grapheme_width;
    }

    result.push_str(ELLIPSIS);
    result
}

/// CRITICAL: Enhanced title/footer rendering with auto-detection and formatting of icons
///
/// This function contains the PROTECTED icon detection logic (lines 385-410 in original parser.rs)
/// that automatically detects and formats icons in titles and footers.
pub fn render_title_or_footer(text: &str, total_width: usize, style_char: &str, align: &str) -> String {
    if total_width < 4 {
        // Minimum viable box: just return style chars
        return style_char.repeat(total_width);
    }

    // *** CRITICAL PROTECTED LOGIC START ***
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
    // *** CRITICAL PROTECTED LOGIC END ***

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
    let (left_pad, right_pad) = match align {
        "left" => (0, remaining_width),
        "right" => (remaining_width, 0),
        _ => {
            let lp = remaining_width / 2;
            (lp, remaining_width.saturating_sub(lp))
        }
    };

    format!("{} {} {}",
        style_char.repeat(left_pad),
        final_text,
        style_char.repeat(right_pad))
}

// =============== HELP FUNCTIONS ===============

/// Show comprehensive CLI help with examples and usage patterns
pub fn show_comprehensive_help(jynx: &JynxPlugin) {
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
    println!("  Boxy v0.11.0 introduces a comprehensive theme system with semantic formatting,");
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
    println!("    --icon <ICON>              Add icon to content (deprecated - use --title)"); //??
    println!("    --status <TEXT>            Status line inside box (use sl:|sc:|sr: prefixes)");
    println!("    --layout <spec>            Align/divide/pad: hl|hc|hr, fl|fc|fr, sl|sc|sr, dt|dtn, ds|dsn, stn|ptn|psn|ssn, bl|bc|br, bp");
    println!("    --pad <a|b>               Blank line above (a) and/or below (b) the body");
    println!("    --title-color <COLOR>      Color for title line (overrides --text)");
    println!("    --status-color <COLOR>     Color for status line (overrides --text)");
    println!("    --header-color <COLOR>     Color for header line");
    println!("    --footer-color <COLOR>     Color for footer line");
    println!();

    println!("  {}Theme System:{}", get_color_code("cyan"), RESET);
    println!("    --use <THEME>              Apply theme by name (error, success, warning, info)");
    println!("    --theme <THEME>            Alias for --use (legacy compatibility)");
    println!();

    println!("  {}Utility:{}", get_color_code("cyan"), RESET);
    println!("    --no-boxy[=strict]         Strip box decoration (strict removes all formatting)");
    println!("    --no-color                 Disable jynx integration and color output");
    println!("    width                      Show terminal width diagnostics");
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
    println!("    blueprint  Technical blue theme with ASCII style (📐)");
    println!();

    println!("  {}Engine Management:{}", get_color_code("cyan"), RESET);
    println!("    {} engine init               Initialize global theme directory", NAME);
    println!("    {} engine import <name>      Import boxy_<name>.yml to global location", NAME);
    println!("    {} engine export <name>      Export boxy_<name>.yml from global to local", NAME);
    println!("    {} engine list               List all available themes from all configs", NAME);
    println!("    {} engine debug              Show loading hierarchy and diagnostics", NAME);
    println!("    {} engine status             Show engine health and statistics", NAME);
    println!("    {} engine edit <name>        Edit a theme config file", NAME);
    println!("    {} engine help               Show engine commands help", NAME);
    println!();

    println!("  {}Theme Usage:{}", get_color_code("cyan"), RESET);
    println!("    {} theme show <name>         Show individual theme properties", NAME);
    println!("    {} theme dryrun <name>       Test theme with sample content", NAME);
    println!("    {} theme create <name>       Create new theme within a config", NAME);
    println!("    Env: BOXY_THEME=<name>      Set default theme (overridden by --theme)");
    println!();

    // =============== NEW IN V0.6 =============== //TODO:CLEANUP
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
    println!("  Documentation: See docs/tech/THEME_SYSTEM.md");
    println!();

    println!("{}Transform your CLI output with semantic themes and professional formatting!{}", get_color_code("emerald"), RESET);
}

/// Show practical usage examples for different scenarios
pub fn show_usage_examples() {
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

    // =============== ENGINE MANAGEMENT ===============
    println!("{}ENGINE MANAGEMENT:{}", get_color_code("coral"), RESET);
    println!("  # Set up global theme system");
    println!("  {} engine init", NAME);
    println!();
    println!("  # List available themes from all configs");
    println!("  {} engine list", NAME);
    println!();
    println!("  # Debug theme loading hierarchy");
    println!("  {} engine debug", NAME);
    println!();
    println!("  # Import/export theme config files");
    println!("  {} engine export default        # Copy global to local", NAME);
    println!("  {} engine import myproject       # Copy local boxy_myproject.yml to global", NAME);
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