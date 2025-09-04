# Boxy Theme System (v0.6+ → v0.8)

Note: While originally introduced in v0.6, this theme system is the
current API in v0.8. Migration tooling has been removed; use the options
documented here.

## Overview

The Boxy Theme System v0.6 transforms boxy from a simple box drawing utility into a comprehensive semantic formatting system with predefined themes, rich color palette, and advanced typography. It inherits proven architecture patterns from jynx with XDG+ directory support.

## Architecture

### Theme Engine Components

```
~/.local/etc/rsb/boxy/
├── themes/                 # XDG+ theme directory
│   ├── error.yml           # Built-in semantic themes
│   ├── success.yml
│   ├── warning.yml
│   └── *.yml               # User themes
├── cache/                  # Performance cache
└── config.yml              # Global configuration
```

### Theme Structure

Each theme is a YAML file with comprehensive configuration:

```yaml
# Theme metadata
metadata:
  name: "error-theme"
  version: "1.0.0"
  description: "Critical error display theme"
  author: "boxy"
  compatibility: "boxy v0.6+"

# Theme definitions
themes:
  error:
    # === VISUAL PROPERTIES ===
    color: "crimson"                # Border color (required)
    text_color: "white"             # Text color: "auto", "none", or color
    style: "heavy"                  # Border: normal, rounded, double, heavy, ascii
    text_style: "bold"              # Text formatting: normal, bold, italic, etc.
    
    # === CONTENT ===
    title: "❌ Error"               # Internal title with icon
    header: null                    # External header (above box)
    footer: null                    # Footer (below box)
    icon: "❌"                      # Content icon (legacy support)
    
    # === LAYOUT ===
    width: 60                       # Fixed width in characters
    padding: 1                      # Internal padding
    
    # === ALIGNMENT ===
    title_align: "center"           # left, center, right
    header_align: "center"
    footer_align: "center"
    status_align: "left"
    
    # === ADVANCED ===
    status_bar: null                # Default status bar
    inherits: null                  # Inherit from parent theme
```

## Built-in Themes

### System Status Themes

**Error Theme** (`--theme error`)
- Color: Crimson (bright red)
- Style: Heavy borders
- Icon: ❌ Error
- Use: Critical failures, system errors

**Success Theme** (`--theme success`)  
- Color: Emerald (rich green)
- Style: Rounded borders
- Icon: ✅ Success
- Use: Successful operations, completions

**Warning Theme** (`--theme warning`)
- Color: Amber (golden orange)
- Style: Heavy borders  
- Icon: ⚠️ Warning
- Use: Cautionary messages, deprecations

**Info Theme** (`--theme info`)
- Color: Azure (sky blue)
- Style: Normal borders
- Icon: ℹ️ Info
- Use: Informational messages, status updates

### Advanced Themes

**Critical Theme** (`--theme critical`)
- Enhanced error theme with double borders and bold text
- Fixed width for consistent critical alerts
- Bright white text on crimson background

## CLI Integration

### Basic Theme Usage

```bash
# Apply semantic themes
echo "Database error" | boxy --theme error
echo "Backup complete" | boxy --theme success  
echo "Deprecated API" | boxy --theme warning
echo "Server status" | boxy --theme info

# Combine themes with additional options
echo "Critical alert" | boxy --theme error --header "System Alert"
echo "All tests passed" | boxy --theme success --status "sc:Completed $(date)"
```

### Enhanced Features (v0.6)

**Header vs Title Distinction:**
```bash
# External header (above box) + internal title (in border)
echo "content" | boxy --header "Application Name" --title "✅ Ready"

# Header for app identity, title for status
echo "data" | boxy --header "MyWebApp v2.1" --title "🟢 Online"
```

**Unified Icon System:**
```bash
# Old way (v0.5): --icon ✅ --title "Status" 
# New way (v0.6): --title "✅ Status"
echo "Success" | boxy --title "✅ Operation Complete"
```

**Status Bar Alignment:**
```bash
# Left, center, right alignment
echo "data" | boxy --status "sl:Left aligned"
echo "data" | boxy --status "sc:Center aligned"  
echo "data" | boxy --status "sr:Right aligned"
```

### Theme Management Commands

```bash
# List all available themes
boxy theme list

# Show detailed theme information  
boxy theme show error

# Create new theme interactively
boxy theme create my_project

# Import theme from file
boxy theme import ~/my_theme.yml

# Export theme to file  
boxy theme export error > error_backup.yml

# Edit existing theme
boxy theme edit my_project

# Help and examples
boxy theme help
```

## Color System

### Extended Color Palette (90+ Colors)

**Legacy Colors (v0.5 compatibility):**
- Basic: red, green, blue, yellow, orange, purple, cyan, magenta
- Extended: red2, blue2, grey, grey2, grey3, white, white2

**Rich Spectrum Colors:**
- **Red:** crimson, ruby, coral, salmon, rose, brick
- **Orange:** amber, tangerine, peach, rust, bronze, gold
- **Yellow:** lemon, mustard, sand, cream, khaki
- **Green:** lime, emerald, forest, mint, sage, jade, olive
- **Blue:** azure, navy, royal, ice, steel, teal, indigo  
- **Purple:** violet, plum, lavender, orchid, mauve, amethyst
- **Cyan:** aqua, turquoise, sky, ocean
- **Monochrome:** black, charcoal, slate, silver, pearl, snow

**Semantic Colors:**
- **Status:** error, success, warning, info, critical
- **Priority:** high, medium, low, trivial
- **State:** active, inactive, pending, progress, blocked

**Preview Colors:**
```bash
# Show all available colors with previews
boxy --colors
```

## Text Styling System

### Text Style Options

**Basic Styles:**
- `normal` - Standard text (default)
- `bold` - Bold text weight
- `italic` - Italic text style
- `underline` - Underlined text
- `dim` - Dimmed text
- `strikethrough` - Strikethrough text

**Combined Styles:**
- `bold_underline` - Bold and underlined
- `italic_dim` - Italic and dimmed
- `bold_italic` - Bold and italic

**Usage:**
```bash
# Apply text styles with themes or directly
echo "content" | boxy --theme error --text-style bold
echo "content" | boxy --color azure --text-style italic_dim
```

## Theme Creation and Customization

### Interactive Theme Creation

```bash
# Start interactive theme creation
boxy theme create my_app_theme

# Follow prompts to configure:
# - Color selection from 90+ palette
# - Border style (normal, rounded, double, heavy, ascii)  
# - Text styling (bold, italic, etc.)
# - Icon/emoji selection
# - Width and padding settings
```

### Custom Theme Example

Create `~/.local/etc/rsb/boxy/themes/my_project.yml`:

```yaml
metadata:
  name: "My Project Theme Collection"
  version: "1.0.0"
  description: "Custom themes for my application"
  author: "developer"
  created: "2024-09-03"

themes:
  # Development themes
  dev_error:
    color: "ruby"
    text_color: "white"
    style: "double"
    text_style: "bold"
    title: "🔥 Dev Error"
    width: 80
    
  dev_success:
    color: "jade"
    text_color: "auto"
    style: "rounded"
    text_style: "bold"
    title: "🚀 Dev Success"
    
  # Production themes  
  prod_alert:
    color: "crimson"
    text_color: "white"
    style: "heavy"
    text_style: "bold_underline"
    title: "🚨 Production Alert"
    width: 100
    footer: "IMMEDIATE ACTION REQUIRED"
```

### Theme Inheritance

```yaml
# Base theme
base_app:
  color: "azure"
  text_color: "auto"
  style: "normal"
  padding: 1

# Inherited theme
app_error:
  inherits: "base_app"        # Inherits base properties
  color: "crimson"            # Override color
  title: "❌ App Error"       # Add title
  text_style: "bold"          # Add text styling
```

## Version Notes

The theme system described here was introduced in v0.6 and continues in v0.8.
All migration tooling has been removed; use the current CLI options shown above.

## Performance and Optimization

### Theme Engine Performance

- **Lazy Loading**: Themes loaded only when needed
- **Smart Caching**: XDG+ directory monitoring with change detection  
- **Memory Efficiency**: Shared color palette across themes
- **Fast Lookups**: Optimized theme resolution (~0.1ms)

### Best Practices

**Theme Selection:**
```bash
# Use semantic themes for consistency
echo "error" | boxy --theme error          # Good
echo "error" | boxy --color red --icon ❌  # Works but verbose

# Combine themes with specific overrides
echo "data" | boxy --theme info --header "Custom App"

# Use descriptive headers and titles
echo "status" | boxy --header "Database Service" --title "🟢 Online"
```

**Performance Tips:**
- Prefer built-in themes over custom colors for faster rendering
- Use fixed widths sparingly - auto-sizing is more responsive
- Cache theme lookups in scripts with multiple boxy calls

## Integration Examples

### CI/CD Integration

```bash
# Build status reporting
if ./build.sh; then
    echo "Build completed successfully" | boxy --theme success --header "CI/CD Pipeline"
else  
    echo "Build failed - check logs" | boxy --theme error --header "CI/CD Pipeline"
fi

# Test results with detailed status
test_results() {
    local passed=$1
    local total=$2
    if [ "$passed" -eq "$total" ]; then
        echo "All $total tests passed!" | boxy --theme success --status "sc:$(date)"
    else
        echo "$((total - passed)) tests failed" | boxy --theme error --status "sc:$(date)"
    fi
}
```

### Application Monitoring

```bash
# System health dashboard
system_status() {
    local cpu=$(get_cpu_usage)
    local memory=$(get_memory_usage)
    
    if [ "$cpu" -gt 80 ]; then
        echo "CPU: ${cpu}% | Memory: ${memory}%" | boxy --theme critical --header "System Status"
    elif [ "$cpu" -gt 60 ]; then
        echo "CPU: ${cpu}% | Memory: ${memory}%" | boxy --theme warning --header "System Status"  
    else
        echo "CPU: ${cpu}% | Memory: ${memory}%" | boxy --theme success --header "System Status"
    fi
}
```

### Development Workflow

```bash
# Git status with themes
git_themed_status() {
    if git diff --quiet && git diff --staged --quiet; then
        echo "Working directory clean" | boxy --theme success --title "✅ Git Status"
    else
        git status --short | boxy --theme warning --title "⚡ Git Status" --header "Uncommitted Changes"
    fi
}

# Package management
npm_install_themed() {
    if npm install; then
        echo "Dependencies installed successfully" | boxy --theme success --title "📦 NPM"
    else
        echo "Failed to install dependencies" | boxy --theme error --title "❌ NPM"
    fi  
}
```

## Troubleshooting

### Common Issues

**Theme Not Found:**
```bash
$ echo "test" | boxy --theme unknown_theme
Error: Theme 'unknown_theme' not found
Available themes: error, success, warning, info, critical

# Solution: List available themes
$ boxy theme list
```

**Icon Spacing Issues:**
```bash
# Problem: Manual icon spacing
$ echo "text" | boxy --icon "📦 " --title "Status"  # Extra space

# Solution: Use unified title
$ echo "text" | boxy --title "📦 Status"  # Automatic spacing
```

**Status Bar Alignment:**
```bash
# Problem: Long status without alignment  
$ echo "text" | boxy --status "Very long status message here"

# Solution: Add alignment prefix
$ echo "text" | boxy --status "sc:Very long status message here"
```

### Debugging Tools

```bash
# Validate theme files
boxy theme validate ~/.local/etc/rsb/boxy/themes/my_theme.yml

# Test theme rendering  
boxy theme show error

# Check color availability
boxy --colors | grep crimson

# Verbose theme loading (if implemented)
BOXY_DEBUG=1 boxy --theme error
```

## API Reference

### Command Line Interface

```bash
# Core theming
--theme <name>              # Apply semantic theme
--color <color>             # Border color (90+ options)
--text <color>              # Text color or "auto"/"none" 
--style <style>             # Border style: normal|rounded|double|heavy|ascii

# Layout and content
--header <text>             # External header (above box)
--title <text>              # Internal title (with icon support)
--footer <text>             # Footer text
--status <text>             # Status bar (with alignment: sl:|sc:|sr:)
--width <num>               # Fixed width in characters

# Theme management  
theme list                  # List available themes
theme show <name>           # Show theme details
theme create <name>         # Create new theme
theme import <file>         # Import theme file
theme export <name>         # Export theme
theme edit <name>           # Edit theme
theme help                  # Theme system help

# Migration assistance
# (Removed in v0.8 — use current CLI options above)
```

### Theme File Format

Complete YAML schema for theme files:

```yaml
# Required metadata section
metadata:
  name: string              # Theme collection name
  version: string           # Semantic version
  description: string       # Human description
  author: string           # Author name
  created: string          # Creation date
  updated: string          # Last update date  
  compatibility: string    # Compatible versions

# Optional custom colors
colors:
  color_name: "ansi_code"  # Custom color definitions

# Theme definitions (required)
themes:
  theme_name:
    # Required
    color: string                    # Border color
    
    # Optional visual
    text_color: string              # "auto"|"none"|color_name  
    style: string                   # normal|rounded|double|heavy|ascii
    text_style: string              # normal|bold|italic|underline|etc
    
    # Optional content
    title: string                   # Internal title with icon
    header: string                  # External header
    footer: string                  # Footer text  
    icon: string                    # Legacy icon support
    
    # Optional layout
    width: integer                  # Fixed width (10-200)
    padding: integer                # Padding (default: 1)
    
    # Optional alignment  
    title_align: string             # left|center|right
    header_align: string            # left|center|right
    footer_align: string            # left|center|right
    status_align: string            # left|center|right
    
    # Optional advanced
    status_bar: string              # Default status text
    inherits: string                # Parent theme name

# Optional presets for quick access
presets:
  preset_name: "theme_name"        # Quick theme aliases
  
# Optional text style definitions
text_styles:
  style_name: "ansi_codes"         # Custom text styles

# Optional global settings
settings:
  default_theme: string            # Default theme name
  fallback_color: string           # Fallback color
  max_width: integer               # Maximum box width
  min_width: integer               # Minimum box width
  cache_themes: boolean            # Enable theme caching
  validate_colors: boolean         # Validate color names
```

## Advanced Topics

### Custom Color Definitions

```yaml
# Define custom colors in theme files
colors:
  company_blue: "\x1B[38;5;25m"
  company_green: "\x1B[38;5;29m"
  
themes:
  company_theme:
    color: "company_blue"
    text_color: "company_green"
```

### Theme Inheritance Patterns

```yaml
# Base organizational theme
base_corporate:
  color: "slate" 
  style: "normal"
  text_color: "auto"
  padding: 1

# Department-specific themes inheriting base
engineering:
  inherits: "base_corporate"
  color: "azure"
  title: "🔧 Engineering"
  
marketing:
  inherits: "base_corporate"  
  color: "orchid"
  title: "📈 Marketing"
  
operations:
  inherits: "base_corporate"
  color: "emerald"
  title: "⚙️ Operations"
```

### Dynamic Theme Selection

```bash
# Environment-based theme selection
THEME=${NODE_ENV:-info}
echo "Server started" | boxy --theme "$THEME"

# Content-based theme selection
select_theme() {
    local content="$1"
    case "$content" in
        *error*|*failed*|*crash*) echo "error" ;;
        *success*|*complete*|*done*) echo "success" ;;
        *warning*|*deprecated*) echo "warning" ;;
        *) echo "info" ;;
    esac
}

# Usage
THEME=$(select_theme "$MESSAGE")
echo "$MESSAGE" | boxy --theme "$THEME"
```

This comprehensive theme system provides the foundation for consistent, semantic, and visually appealing CLI output across all your applications and scripts.
