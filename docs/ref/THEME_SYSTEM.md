# Boxy Theme System v0.6+

## Overview

Transform boxy from a simple box drawing tool into a **rich semantic formatting system** with predefined themes, extended color palette, advanced text styling, and intelligent color inheritance. **Inherits complete theme architecture from jynx's proven XDG+ system and enhances it with API-driven color propagation.**

> **Note**: jynx now has COMPLETE theme specifications with XDG+ management, color templating, and 90+ semantic colors. Boxy should inherit this proven architecture.

## jynx Theme Architecture Inherited

### **Complete XDG+ System (PROVEN)**
- ✅ **XDG+ directories**: `~/.local/etc/rsb/jynx/themes/`
- ✅ **Smart resolution**: `--theme=rebel` → `theme_rebel.yml`
- ✅ **Theme management**: import/export/create/edit commands
- ✅ **Color Inheritance**: Automatic border and text color propagation with BOX_CHARS
- ✅ **Color templating**: `%c:colorname(text)` declarative system
- ✅ **90+ semantic colors**: crimson, emerald, azure, amber, etc.
- ✅ **Dual mode**: Dynamic YAML + compiled binary optimization

## Current vs Enhanced

### **Current (v0.5)**
```bash
echo "content" | boxy --color blue --style rounded
```

### **Enhanced (v0.6+)**  
```bash
echo "content" | boxy --theme error                    # Semantic theme
echo "content" | boxy --theme todo_urgent              # Domain-specific  
echo "content" | boxy --header "Error Report"          # External header (was --title)
echo "content" | boxy --title "❌ Critical Issue"      # Internal title with icon
echo "content" | boxy --layout=hr,fc                   # Header-right, footer-center
```

## Critical v0.6 Changes Required

### **🚨 Breaking Changes**
- **`--title` → `--header`**: External box header (correct pattern)
- **New `--title`**: Internal box title with leading icon (inside content)
- **Alignment flags**: `--layout=hr,fc` instead of multiple alignment flags

### **🐛 Critical Bugs to Fix**
- **Header overflow**: Icons (crane emoji) cause overflow when header > box width
- **Icon counting fragility**: DO NOT break existing icon width calculation mechanism
- **Theme icon conflicts**: Research if themes already insert icons vs manual --title icons

---

## Theme System Architecture

### **Theme Storage Structure**
```
~/.local/etc/rsb/boxy/
├── theme.yml              # Active theme configuration
├── themes/                 # Built-in theme collection
│   ├── semantic.yml        # Semantic themes (error, success, warning)
│   ├── domain.yml          # Domain themes (todo, code, docs)  
│   ├── visual.yml          # Visual themes (minimal, rich, retro)
│   └── accessibility.yml   # High contrast, colorblind-safe
├── custom/                 # User custom themes
│   ├── my_project.yml
│   └── dark_mode.yml
└── compiled/               # Performance cache (like jynx)
    └── *.bin
```

### **Theme Configuration Format**
```yaml
# ~/.local/etc/rsb/boxy/theme.yml
metadata:
  name: "boxy-semantic-v1"
  version: "1.0.0"
  description: "Rich semantic themes for boxy v0.6"

# Extended color palette (from jynx design)
colors:
  # Core palette
  crimson: "\x1B[38;5;196m"
  emerald: "\x1B[38;5;34m" 
  amber: "\x1B[38;5;220m"
  azure: "\x1B[38;5;33m"
  orchid: "\x1B[38;5;170m"
  
  # Full extended palette...
  
# Semantic theme definitions  
themes:
  # === SYSTEM STATUS THEMES ===
  error:
    color: "crimson"
    text_color: "white"
    style: "heavy"
    text_style: "bold"
    title: "❌ Error"
    
  warning:
    color: "amber" 
    text_color: "auto"
    style: "heavy"
    text_style: "italic"
    title: "⚠️ Warning"
    
  success:
    color: "emerald"
    text_color: "auto" 
    style: "rounded"
    text_style: "bold"
    title: "✅ Success"
    
  info:
    color: "azure"
    text_color: "auto"
    style: "normal"
    text_style: "normal"
    title: "ℹ️ Info"
    
  # === PRIORITY THEMES ===
  critical:
    color: "crimson"
    text_color: "white"
    style: "double"
    text_style: "bold_underline"
    title: "🚨 Critical"
    width: 60
    
  urgent:
    color: "tangerine"
    text_color: "auto"
    style: "heavy" 
    text_style: "bold"
    title: "⚡ Urgent"
    
  normal:
    color: "slate" 
    text_color: "auto"
    style: "normal"
    text_style: "normal"
    
  low:
    color: "charcoal"
    text_color: "silver"
    style: "ascii"
    text_style: "dim"
    
  # === DOMAIN-SPECIFIC THEMES ===
  todo_urgent:
    color: "crimson"
    text_color: "white"
    style: "double"
    text_style: "bold_underline" 
    title: "🔥 Urgent Task"
    footer: "action_required"
    
  todo_complete:
    color: "emerald"
    text_color: "auto"
    style: "rounded"
    text_style: "bold"
    title: "✅ Completed" 
    
  todo_blocked:
    color: "rust"
    text_color: "auto"
    style: "heavy"
    text_style: "strikethrough_dim"
    title: "⛔ Blocked"
    
  code_block:
    color: "steel"
    text_color: "auto"
    style: "ascii"
    text_style: "mono"
    title: "💻 Code"
    width: 80
    
  code_error:
    color: "ruby"
    text_color: "white"
    style: "heavy"
    text_style: "bold"
    title: "🐛 Error"
    
  documentation:
    color: "azure"
    text_color: "auto"
    style: "rounded"
    text_style: "normal"
    title: "📚 Documentation"
    width: 70
    
  # === VISUAL STYLE THEMES ===
  minimal:
    color: "slate"
    text_color: "auto"
    style: "normal"
    text_style: "normal"
    
  rich:
    color: "orchid"
    text_color: "auto"
    style: "double"
    text_style: "italic"
    title: "✨"
    
  retro:
    color: "amber"
    text_color: "auto" 
    style: "ascii"
    text_style: "bold"
    title: "▓▓▓"
    
  # === ACCESSIBILITY THEMES ===
  high_contrast:
    color: "white"
    text_color: "black"
    style: "heavy"
    text_style: "bold"
    
  colorblind_safe:
    color: "azure"  # Blue-based, universally accessible
    text_color: "auto"
    style: "heavy"
    text_style: "bold"

# Text style definitions
text_styles:
  normal: ""
  bold: "\x1B[1m"
  italic: "\x1B[3m"
  underline: "\x1B[4m" 
  dim: "\x1B[2m"
  strikethrough: "\x1B[9m"
  mono: "\x1B[2m"  # Dim for monospace feel
  
  # Combined styles
  bold_underline: "\x1B[1;4m"
  italic_dim: "\x1B[3;2m"
  strikethrough_dim: "\x1B[9;2m"
  bold_italic: "\x1B[1;3m"

# Style presets for quick access
presets:
  error_box: "error"
  warn_box: "warning"
  success_box: "success"
  code: "code_block"
  urgent: "todo_urgent"
  note: "info"
```

---

## CLI Integration

### **New Theme Commands (Inherit from jynx)**
```bash
# Theme management (same as jynx)
boxy theme list                    # Show available themes
boxy theme import <name>           # Import theme from current dir to XDG+
boxy theme export <name>           # Export XDG+ theme to current dir
boxy theme create <name>           # Create new theme from default
boxy theme edit <name>             # Edit theme in $EDITOR

# Using themes with new flags
echo "content" | boxy --theme error --header "Critical Alert"
echo "content" | boxy --theme success --title "✅ Completed" 
echo "content" | boxy --layout=hr,fc --header "Status" --footer "2024"

# Layout flag combinations
--layout=hr        # header-right only
--layout=fc        # footer-center only  
--layout=hr,fc     # header-right, footer-center
--layout=hl,fr     # header-left, footer-right
--layout=hc,fc,tl  # header-center, footer-center, title-left
```

### **Layout Flag Specification**
```bash
# Header alignment: h + l/c/r
hl = header-left
hc = header-center (default)  
hr = header-right

# Footer alignment: f + l/c/r
fl = footer-left
fc = footer-center (default)
fr = footer-right  

# Title alignment: t + l/c/r (internal box title)
tl = title-left
tc = title-center (default)
tr = title-right
```

### **Enhanced CLI Arguments**
```bash
# Current arguments (backward compatible)
--color <COLOR>           # Basic color
--style <STYLE>           # Border style  
--text <COLOR>            # Text color
--title <TEXT>            # Title text
--footer <TEXT>           # Footer text
--width <NUM>             # Fixed width

# New theme arguments
--theme <NAME>            # Apply semantic theme
--preset <NAME>           # Quick preset
--text-style <STYLE>      # Text formatting (bold, italic, etc.)
--auto-theme              # Auto-detect theme from content
--theme-override <KEY=VAL> # Override theme properties

# Extended colors (from jynx palette)
--color crimson           # Rich red
--color emerald           # Rich green  
--color amber             # Rich orange
--color azure             # Rich blue
--color orchid            # Rich purple
# ... all 90+ colors from extended palette
```

---

## Implementation Design

### **Theme Engine Structure**
```rust
// Core theme system
pub struct ThemeEngine {
    themes: HashMap<String, BoxyTheme>,
    active_theme: String,
    color_palette: ColorPalette,
}

#[derive(Debug, Clone)]
pub struct BoxyTheme {
    pub metadata: ThemeMetadata,
    pub color: String,
    pub text_color: Option<String>,
    pub style: BorderStyle,
    pub text_style: TextStyle,
    pub title: Option<String>,
    pub footer: Option<String>, 
    pub width: Option<usize>,
    pub padding: Option<usize>,
}

#[derive(Debug, Clone)]  
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
}

impl TextStyle {
    pub fn to_ansi(&self) -> String {
        let mut codes = Vec::new();
        if self.bold { codes.push("1"); }
        if self.dim { codes.push("2"); }  
        if self.italic { codes.push("3"); }
        if self.underline { codes.push("4"); }
        if self.strikethrough { codes.push("9"); }
        
        if codes.is_empty() {
            String::new()
        } else {
            format!("\x1B[{}m", codes.join(";"))
        }
    }
}
```

### **Auto-Theme Detection**
```rust
pub fn detect_theme_from_content(content: &str) -> Option<String> {
    // Detect semantic meaning from content
    if content.to_lowercase().contains("error") || content.contains("❌") {
        Some("error".to_string())
    } else if content.to_lowercase().contains("warning") || content.contains("⚠️") {
        Some("warning".to_string()) 
    } else if content.to_lowercase().contains("success") || content.contains("✅") {
        Some("success".to_string())
    } else if content.contains("```") || content.contains("function") {
        Some("code_block".to_string())
    } else if content.to_lowercase().contains("urgent") || content.contains("🔥") {
        Some("urgent".to_string())
    } else {
        None
    }
}

// Usage
echo "❌ Database connection failed" | boxy --auto-theme
// Automatically applies "error" theme
```

---

## Integration Examples

### **Knowledge Management Integration**
```bash
# KB system integration  
kb_display() {
    local filter_type="$1"
    local content="$2"
    
    case "$filter_type" in
        "todo")
            if echo "$content" | grep -q "URGENT\|CRITICAL"; then
                echo "$content" | boxy --theme todo_urgent
            elif echo "$content" | grep -q "DONE\|COMPLETE"; then
                echo "$content" | boxy --theme todo_complete  
            else
                echo "$content" | boxy --theme info
            fi
            ;;
        "troubleshoot")
            if echo "$content" | grep -q "ERROR\|FATAL"; then
                echo "$content" | boxy --theme code_error
            else
                echo "$content" | boxy --theme warning
            fi
            ;;
        *)
            echo "$content" | boxy --theme info
            ;;
    esac
}
```

### **Development Workflow Integration**
```bash
# Git status with themed output
git_status_themed() {
    local status=$(git status --porcelain)
    
    if [[ -n "$status" ]]; then
        echo "$status" | boxy --theme warning --title "⚡ Git Status" 
    else
        echo "Working directory clean" | boxy --theme success --title "✅ Git Status"
    fi
}

# Test results with themed display
test_results_themed() {
    if ./run_tests.sh; then
        echo "All tests passed!" | boxy --theme success --title "🧪 Test Results"
    else
        echo "Tests failed - see log for details" | boxy --theme error --title "🚨 Test Results"
    fi
}
```

### **System Monitoring Integration** 
```bash
# System health with automatic theming
system_health() {
    local cpu_usage=$(top -bn1 | grep "Cpu(s)" | sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | awk '{print 100 - $1}')
    
    if (( $(echo "$cpu_usage > 80" | bc -l) )); then
        echo "CPU Usage: ${cpu_usage}% - High load detected!" | boxy --theme critical
    elif (( $(echo "$cpu_usage > 50" | bc -l) )); then
        echo "CPU Usage: ${cpu_usage}% - Moderate load" | boxy --theme warning
    else
        echo "CPU Usage: ${cpu_usage}% - System healthy" | boxy --theme success
    fi
}
```

---

## Theme Development

### **Custom Theme Creation**
```bash
# Create a new theme interactively
boxy theme create my_project

# Opens editor with template:
```

```yaml
# ~/.local/etc/rsb/boxy/custom/my_project.yml
metadata:
  name: "my_project"
  description: "Custom theme for my project"

themes:
  my_error:
    color: "crimson"
    text_color: "white" 
    style: "double"
    text_style: "bold"
    title: "🔥 Project Error"
    
  my_success:
    color: "emerald"
    style: "rounded"
    text_style: "bold"
    title: "🚀 Project Success"
```

### **Theme Testing & Validation**
```bash
# Preview theme with sample content
boxy theme preview my_error

# Test all themes in current config
boxy theme test-all

# Validate theme syntax
boxy theme validate custom/my_project.yml

# Performance benchmark themes
boxy theme benchmark  # Test rendering performance
```

---

## Compatibility

The theme system introduced in v0.6 is the current, stable approach in v0.8.
Use the documented options and semantic themes; legacy migration helpers have been removed.

### Color Inheritance Architecture

- Border colors automatically propagate across components
- Text colors inherit with intelligent fallback
- `text_color: "none"` prevents inheritance
- Uses BOX_CHARS for consistent rendering

**Key Benefits**:
- Simplified theme configuration
- Consistent visual hierarchy
- Flexible color propagation rules

---

## Performance & Optimization

### **Theme Compilation**
- Themes compiled to binary format for speed (like jynx)
- ~0.1ms theme lookup vs ~5ms YAML parsing
- Smart caching with file change detection
- Incremental compilation for theme development

### **Memory Efficiency**
- Lazy loading of unused themes  
- Shared color palette across themes
- Optimized ANSI sequence generation
- Minimal allocation during rendering

---

This theme system transforms boxy from a simple box tool into a **rich semantic formatting system** perfect for modern CLI applications! 🎨✨
