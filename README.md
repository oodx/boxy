# boxy 📦

A fast command-line utility that draws Unicode boxes around text with proper emoji/Unicode width handling.

## Features

- ✨ Custom Unicode/emoji width calculation system (no external dependencies)
- 🎨 Multiple box styles (normal, rounded, double, heavy, ascii)
- 🌈 Colored borders and text with predefined color schemes
- 🎨 Text color control with auto-matching and explicit colors
- 🎭 **5-level theme hierarchy system** with flexible theme loading
- 🎯 **Theme management commands** (hierarchy, dryrun, list, show)
- 📋 Title and footer support with emoji/variable expansion (inside borders)
- 🎯 Icon decorations for content
- 📏 Fixed width boxes with smart content truncation
- 🔄 Pipeline integration with box stripping modes
- 🧩 Advanced layout control (align/dividers/padding for header/title/body/status/footer)
- 🛠️ Param stream (--params) to set header/title/status/footer/layout/colors alongside piped body
- 🎛️ Title/status color overrides (--title-color/--status-color)
- 🧪 BOXY_THEME default theme (env)
- 🔍 **Comprehensive emoji debugging system** for development
- 🚀 Written in Rust for speed
- 📝 Handles multi-line text and ANSI color codes

## Installation

```bash
# Build from source
cargo build --release

# Deploy to local bin
./deploy.sh
```

## Usage

```bash
# Basic usage
echo "Hello World" | boxy

# With style and color
echo "Hello World" | boxy --style rounded --color blue
echo "Hello World" | boxy -s double -c red

# With title and footer
echo "Hello World" | boxy --title "🚀 My App" --footer "v1.0"
echo "Content" | boxy --header "Header" --footer "✅ Done"

# With icon decoration and text colors
echo "Important message" | boxy --icon "⚠️" --color yellow --text red
echo "Success!" | boxy --icon "✅" --color green --text auto  # Text matches box color

# Using themes (includes icon, color, and styling)
echo "Something went wrong" | boxy --theme error
echo "Build successful" | boxy --theme success --text auto

# Fixed width boxes
echo "This is a long message that will be truncated" | boxy --width 20

# Full width or auto width
echo "Use full terminal width" | boxy --width max
echo "Use content-based width (default)" | boxy --width auto

# Status inside the box (with alignment)
echo "Body" | boxy --status "sc:centered status" --width 40

# Layout control (align/dividers/padding)
# hl|hc|hr (header align), fl|fc|fr (footer), sl|sc|sr (status)
# dt|dtn (divider after title), ds|dsn (divider before status)
# stn (space before title), ptn (space after title), psn (space before status), ssn (space after status)
# bl|bc|br (body align), bp (pad body to match title emoji/icon)
echo "Body" | boxy --header H --title "😀 Title" --status Status --footer F \
    --layout "bp,bc,stn,ptn,psn,ssn" --width 50

# Variable expansion in titles
export VERSION="v1.2.3"
echo "Build complete" | boxy --title "🏗️ Build $VERSION" --color green

# Multi-line text
echo -e "Line 1\nLine 2\nLine 3" | boxy

# Pipeline integration - strip box decoration
echo "Content" | boxy | boxy --no-boxy          # Preserves colors
echo "Content" | boxy | boxy --no-boxy=strict   # Pure ASCII output

# With emojis (handles width correctly!)
echo -e "🎉 Party Time\n🚀 Launch\n🔥 Fire" | boxy -s rounded -c orange

# Param stream (metadata alongside piped body)
echo -e "Line 1\nLine 2" | boxy --params "hd='Header'; tl='Title'; st='Status'; ly='bl,bp,stn,ptn,psn,ssn'" --width max

# Title/Status color overrides
echo "Body" | boxy --title "Title" --status Status --title-color crimson --status-color jade
```

### CLI Reference (v0.9)

- Input: Pipe content to `boxy` or pass via `--params` (metadata only).
- Visual: `--style`, `--color`, `--text`, `--width <N|max|auto>`.
- Sections: `--header`, `--title`, `--status` (sl|sc|sr), `--footer`.
- Layout: `--layout` tokens `hl|hc|hr, fl|fc|fr, sl|sc|sr, dt|dtn, ds|dsn, stn|ptn|psn|ssn, bl|bc|br, bp`.
- Themes: `--theme <name>`; manage via `boxy theme <command>`. Env: `BOXY_THEME`.
  - `boxy theme list` - List all available themes
  - `boxy theme show <name>` - Show theme details
  - `boxy theme hierarchy` - Display theme loading hierarchy
  - `boxy theme dryrun <name>` - Test theme with sample content
- Utility: `--no-boxy[=strict]`, `--colors`, `--examples`, `--help`, `--version`.

## Box Styles

- `normal` - Standard box drawing characters `┌─┐│└┘`
- `rounded` - Rounded corners `╭─╮│╰╯`
- `double` - Double lines `╔═╗║╚╝`
- `heavy` - Heavy lines `┏━┓┃┗┛`
- `ascii` - ASCII compatible `+-+|++`

## Colors & Text Styling

### Available Colors
- Basic: `red`, `green`, `blue`, `cyan`, `yellow`, `magenta`
- Extended: `red2`, `green2`, `blue2`, `purple`, `purple2`
- Special: `orange`, `deep`, `deep_green`
- Grays: `white`, `white2`, `grey`, `grey2`, `grey3`

### Text Colors (`--text`)
- Use any color from the list above: `--text red`, `--text blue2`
- Use `auto` to match box color: `--text auto`
- Omit flag for default terminal text color

### Themes
Predefined combinations of icon, color, and styling:
```bash
--theme error      # ❌ with red styling
--theme success    # ✅ with green styling
--theme warning    # ⚠️ with orange styling
--theme info       # ℹ️ with blue styling
--theme debug      # 🐛 with dark green styling
# ... and many more
```

## Theme Hierarchy System

Boxy uses a **5-level theme hierarchy** that searches for themes in the following priority order:

### 1. Local boxy files (highest priority)
- Files like `boxy*.yaml` or `boxy*.yml` in the current directory
- Alphabetically first file is selected if multiple exist
- Example: `boxy_alpha.yaml`, `boxy_custom.yml`

### 2. Local .themes directory
- Hidden `.themes/` directory in current working directory
- Contains project-specific themes
- Example: `.themes/my_project_theme.yml`

### 3. Local themes directory
- Public `themes/` directory in current working directory
- Shared themes for the project
- Example: `themes/default.yml`, `themes/custom.yml`

### 4. XDG themes directory
- System-wide themes in XDG config location
- Path: `~/.local/etc/rsb/boxy/themes/`
- Global user themes

### 5. Built-in themes (lowest priority)
- Compiled fallback themes
- Always available as last resort

### Theme Management Commands

```bash
# View the theme loading hierarchy
boxy theme hierarchy

# List all available themes from all levels
boxy theme list

# Show details for a specific theme
boxy theme show success

# Test a theme with sample content before using
boxy theme dryrun error
```

### Creating Custom Themes

You can create themes at any level of the hierarchy:

```yaml
# Example: ./themes/my_theme.yml
themes:
  my_custom:
    color: "blue"
    text_color: "white"
    style: "rounded"
    text_style: "bold"
    title: "🎯 Custom"
```

Then use with: `echo "Hello" | boxy --theme my_custom`

## Examples

```bash
# System info box with title  
echo -e "🦀 Rust powered\n⚡ Lightning fast\n🔒 Memory safe" | boxy --title "📦 boxy v0.5.0" -s rounded -c blue

# Error alert with themed styling and auto text color
echo "File not found: config.json" | boxy --theme error --text auto --title "🚨 Error"

# Status dashboard with custom text colors
echo -e "✅ Tests passing\n🔧 Build complete\n📦 Ready to deploy" | boxy --title "🎯 CI/CD Status" --footer "✅ All systems go" -c green --text white

# Mixed styling approach
echo "Deploy to production?" | boxy --theme warning --text auto --width 25

# Interactive menu
echo -e "1. Deploy to staging\n2. Deploy to production\n3. Rollback\n4. Exit" | boxy --title "🚀 Deployment Menu" -s rounded

# Theme management and testing
boxy theme hierarchy                           # View theme loading priority
boxy theme dryrun success                     # Test theme before using
echo "Task completed" | boxy --theme success  # Apply the tested theme

# Pipeline processing
command_output | boxy --title "📋 Results" | tee results.txt
cat results.txt | boxy --no-boxy | grep "ERROR"
```

## Integration with Bash

Add to your `.bashrc` or script:

```bash
box() {
    echo "$1" | $HOME/.local/bin/odx/boxy "$@"
}

# Usage examples  
box "Hello World" -s rounded -c blue --text auto
box "Deploy complete" --theme success --text auto
box "Error occurred" --theme error --width 30
```

## Advanced Features

### Title and Footer
- Support emoji and environment variable expansion
- Auto-truncation with `...` when too long
- Centered alignment within box

### Icon Decoration
- Adds visual flair to first content line
- Supports emoji and colored characters
- Suppresses theme icon automatically when title begins with an emoji

### Pipeline Integration
- `--no-boxy`: Strip box while preserving colors/formatting
- `--no-boxy=strict`: Pure ASCII output for script processing
- Perfect for command chains and text processing

### Param Stream (--params)
- Keys: `hd` (header), `tl` (title), `st` (status), `ft` (footer), `ic` (icon), `tc` (title color), `sc` (status color), `ly` (layout tokens)
- Body is always taken from stdin; params only set metadata

### Default Theme
- Set `BOXY_THEME` to a valid theme name to apply by default (overridden by `--theme`)

## Why boxy?

Unlike bash-based box drawing tools, boxy correctly handles:
- **Custom emoji width calculation** (🚀 = 2 columns) - no external dependencies
- Unicode variation selectors (ℹ️ vs ℹ)
- Zero-width joiners and modifiers
- CJK characters (中文, 日本語, 한국어)
- Mixed ASCII and Unicode content
- ANSI color preservation in pipeline modes
- **Comprehensive emoji debugging** for development and troubleshooting

### Custom Width Calculation System
Boxy implements its own Unicode width calculation system, removing the dependency on external crates like `unicode-width`. This provides:
- More accurate emoji width detection
- Better handling of complex Unicode sequences
- Reduced binary size and dependencies
- Custom debugging capabilities for width issues

## Known Issues & Solutions

### Emoji & Unicode Alignment Issues

During development, several critical alignment and width calculation issues were identified and resolved. If you encounter similar problems when working with emoji or Unicode content, these solutions may help:

#### 1. **ANSI Color Code Width Issue**
**Problem**: ANSI escape sequences (color codes) were being counted in width calculations, causing misaligned boxes.

**Solution**: Strip ANSI codes before width calculation using the `strip_ansi_escapes` function. Width calculations should only consider visible characters.

```rust
// Before width calculation, strip ANSI codes
let clean_text = strip_ansi_escapes(&text);
let width = calculate_width(&clean_text);
```

#### 2. **Emoji Truncation Issue**
**Problem**: Multi-codepoint emoji (like ℹ️ - information symbol with variation selector) were being split during text truncation, breaking visual alignment and rendering.

**Solution**: Handle emoji as complete grapheme clusters during truncation operations. Never split emoji in the middle of their codepoint sequence.

```rust
// Use grapheme-aware truncation
use unicode_segmentation::UnicodeSegmentation;
let graphemes: Vec<&str> = text.graphemes(true).collect();
// Truncate by grapheme clusters, not bytes or chars
```

#### 3. **Mixed Width Padding Issue**
**Problem**: Lines containing different emoji and symbol widths weren't getting consistent padding, resulting in ragged box edges.

**Solution**:
- Use unicode-width library for accurate emoji width detection
- Calculate the maximum content width across all lines
- Pad all lines to this maximum width for uniform alignment
- Account for emoji width (typically 2 columns) vs ASCII (1 column)

```rust
// Calculate max width across all content lines
let max_width = content_lines.iter()
    .map(|line| unicode_width::UnicodeWidthStr::width(&strip_ansi_escapes(line)))
    .max()
    .unwrap_or(0);

// Pad each line to max_width for uniform alignment
```

#### 4. **Complex Unicode Sequence Handling**
**Problem**: Unicode sequences with variation selectors, zero-width joiners, and modifiers weren't handled consistently.

**Solution**:
- Process text as grapheme clusters rather than individual codepoints
- Use proper Unicode normalization
- Test with complex emoji sequences during development

#### Development Tips
- Always test with mixed emoji and ASCII content
- Use the built-in `emoji_debug` binary to analyze problematic characters
- Verify alignment with sequences like `"✅ Success"` and `"ℹ️ Info"`
- Check that ANSI color codes don't affect width calculations

## Emoji Debugging System

For developers working with Unicode and emoji, boxy includes comprehensive debugging tools:

### Built-in Emoji Debug Binary
```bash
# Debug a single emoji or character
cargo run --bin emoji_debug "✅"
cargo run --bin emoji_debug "ℹ️"

# Compare multiple characters side by side
cargo run --bin emoji_debug compare "✅" "ℹ️" "🚀" "X"
```

### Debug Information Provided
- Character width calculations
- Unicode codepoint breakdowns
- Visual alignment testing
- Grapheme cluster analysis
- Comparison utilities for troubleshooting

This debugging system is invaluable when working with complex Unicode sequences or when boxy's width calculations don't match expectations.

## License

RSB Framework, Oxidex (ODX), and REBEL libraries, services, and software are offered under a **multi-license model**:

| License | Who it’s for | Obligations |
|---------|--------------|-------------|
| [AGPL-3.0](./LICENSE) | Open-source projects that agree to release their own source code under the AGPL. | Must comply with the AGPL for any distribution or network service. |
| [Community Edition License](./docs/LICENSE_COMMUNITY.txt) | Personal, educational, or academic use **only**. Not for companies, organizations, or anyone acting for the benefit of a business. | Must meet all CE eligibility requirements and follow its terms. |
| [Commercial License](./docs/LICENSE_COMMERCIAL.txt) | Companies, contractors, or anyone needing to embed the software in closed-source, SaaS, or other commercial products. | Requires a signed commercial agreement with Dr. Vegajunk Hackware. |

By **downloading, installing, linking to, or otherwise using RSB Framework, Oxidex, or REBEL libraries, services, and software**, you:

1. **Accept** the terms of one of the licenses above, **and**  
2. **Represent that you meet all eligibility requirements** for the license you have chosen.

> Questions about eligibility or commercial licensing: **licensing@vegajunk.com**
