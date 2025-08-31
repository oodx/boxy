# Boxy Roadmap

## Phase 1: Enhanced Visual Components
### 1.1 Title Feature (`--title`)
Add a title that renders in the top border of the box.

**Specifications:**
- Center-aligned within the top border
- Support for emojis, colors, and environment variables
- Format: `--title="🚀 $PROJECT_NAME [blue]v1.0[/blue]"`
- Truncation with `...` when exceeding box width
- Preserve box corner characters

### 1.2 Footer Feature (`--footer`)
Mirror of title functionality for the bottom border.

**Specifications:**
- Same formatting capabilities as title
- Format: `--footer="Status: ✅ Complete"`
- No dedicated icon (manual emoji inclusion)
- Truncation logic identical to title

### 1.3 Icon Feature (`--icon`)
Decorative element for visual enhancement.

**Specifications:**
- Placement: Top-left corner (after corner character)
- Support single emoji or colored character
- Format: `--icon="🔥"` or `--icon="[red]★[/red]"`
- Optional position flag: `--icon-pos=left|right`

## Phase 2: Pipeline Integration
### 2.1 Box Stripping (`--no-boxy`)
Remove box decoration while preserving content formatting.

**Default Mode:**
- Strip: Box drawing characters, box colors, external padding
- Preserve: Content colors, emojis, internal formatting
- Use case: Re-boxing with different styles

**Strict Mode (`--no-boxy=strict`):**
- Strip: ALL ANSI codes, emojis, Unicode decorations
- Trim: Leading/trailing whitespace
- Output: Pure ASCII text only
- Use case: Script processing, grep/awk pipelines

## Phase 3: Advanced Features (Future)
### 3.1 Multi-column Support
- Side-by-side boxes with `--columns=2`
- Column separator styles

### 3.2 Interactive Mode
- Box resizing based on terminal width
- Dynamic color themes

### 3.3 Configuration File
- `.boxyrc` for default styles
- Custom color palettes
- Preset box templates

## Phase 4: Ecosystem Integration
### 4.1 Shell Completions
- Bash, Zsh, Fish autocomplete
- Dynamic color/style suggestions

### 4.2 Package Distribution
- Homebrew formula
- AUR package
- Cargo install optimization

## Design Principles
1. **Composability**: Every feature works in pipelines
2. **Performance**: Minimal overhead for large inputs
3. **Simplicity**: Intuitive flags, sensible defaults
4. **Compatibility**: UTF-8 aware, terminal-agnostic