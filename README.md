# boxy 📦

A fast command-line utility that draws Unicode boxes around text with proper emoji/Unicode width handling.

## Features

- ✨ Accurate Unicode/emoji width calculation
- 🎨 Multiple box styles (normal, rounded, double, heavy, ascii)
- 🌈 Colored borders with predefined color schemes
- 📋 Title and footer support with emoji/variable expansion
- 🎯 Icon decorations for content
- 🔄 Pipeline integration with box stripping modes
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
echo "Content" | boxy --title "Header" --footer "✅ Done"

# With icon decoration
echo "Important message" | boxy --icon "⚠️" --color yellow

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
```

## Box Styles

- `normal` - Standard box drawing characters `┌─┐│└┘`
- `rounded` - Rounded corners `╭─╮│╰╯`
- `double` - Double lines `╔═╗║╚╝`
- `heavy` - Heavy lines `┏━┓┃┗┛`
- `ascii` - ASCII compatible `+-+|++`

## Colors

Available colors:
- Basic: `red`, `green`, `blue`, `cyan`, `yellow`, `magenta`
- Extended: `red2`, `green2`, `blue2`, `purple`, `purple2`
- Special: `orange`, `deep`, `deep_green`
- Grays: `white`, `white2`, `grey`, `grey2`, `grey3`

## Examples

```bash
# System info box with title
echo -e "🦀 Rust powered\n⚡ Lightning fast\n🔒 Memory safe" | boxy --title "📦 boxy v0.3.0" -s rounded -c blue

# Error alert with icon
echo "File not found: config.json" | boxy --icon "❌" --title "🚨 Error" -s double -c red

# Status dashboard
echo -e "✅ Tests passing\n🔧 Build complete\n📦 Ready to deploy" | boxy --title "🎯 CI/CD Status" --footer "✅ All systems go" -c green

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
box "Hello World" -s rounded -c blue
box "Deploy complete" --title "🚀 Status" --icon "✅"
```

## Advanced Features

### Title and Footer
- Support emoji and environment variable expansion
- Auto-truncation with `...` when too long
- Centered alignment within box

### Icon Decoration
- Adds visual flair to first content line
- Supports emoji and colored characters

### Pipeline Integration
- `--no-boxy`: Strip box while preserving colors/formatting
- `--no-boxy=strict`: Pure ASCII output for script processing
- Perfect for command chains and text processing

## Why boxy?

Unlike bash-based box drawing tools, boxy correctly handles:
- Emoji width (🚀 = 2 columns)
- Unicode variation selectors
- Zero-width joiners
- CJK characters
- Mixed ASCII and Unicode content
- ANSI color preservation in pipeline modes

## License

MIT