use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Theme {
    pub icon: &'static str,
    pub color: &'static str,
    pub width: Option<usize>,
}

pub fn get_themes() -> HashMap<&'static str, Theme> {
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