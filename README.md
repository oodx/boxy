# boxy 📦

A fast command-line utility that draws Unicode boxes around text with proper emoji/Unicode width handling.

## Features

- ✨ Accurate Unicode/emoji width calculation
- 🎨 Multiple box styles (normal, rounded, double, heavy, ascii)
- 🌈 Colored borders and text with predefined color schemes
- 🎨 Text color control with auto-matching and explicit colors
- 🎭 Theme system with predefined visual styles
- 📋 Title and footer support with emoji/variable expansion (inside borders)
- 🎯 Icon decorations for content
- 📏 Fixed width boxes with smart content truncation
- 🔄 Pipeline integration with box stripping modes
- 🧩 Advanced layout control (align/dividers/padding for header/title/body/status/footer)
- 🛠️ Param stream (--params) to set header/title/status/footer/layout/colors alongside piped body
- 🎛️ Title/status color overrides (--title-color/--status-color)
- 🧪 BOXY_THEME default theme (env)
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

### CLI Reference (v0.8)

- Input: Pipe content to `boxy` or pass via `--params` (metadata only).
- Visual: `--style`, `--color`, `--text`, `--width <N|max|auto>`.
- Sections: `--header`, `--title`, `--status` (sl|sc|sr), `--footer`.
- Layout: `--layout` tokens `hl|hc|hr, fl|fc|fr, sl|sc|sr, dt|dtn, ds|dsn, stn|ptn|psn|ssn, bl|bc|br, bp`.
- Themes: `--theme <name>`; manage via `boxy theme list|show <name>`. Env: `BOXY_THEME`.
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
- Emoji width (🚀 = 2 columns)
- Unicode variation selectors
- Zero-width joiners
- CJK characters
- Mixed ASCII and Unicode content
- ANSI color preservation in pipeline modes

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
