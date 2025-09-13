use crate::BoxStyle;

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
}

impl Default for WidthConfig {
    fn default() -> Self {
        Self {
            fixed_width: None,
            h_padding: 1,
            v_padding: 1,
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
