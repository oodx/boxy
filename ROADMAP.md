# Boxy Roadmap

## ✅ Phase 1: Enhanced Visual Components (COMPLETED)
### ✅ 1.1 Title Feature (`--title`)
Add a title that renders in the top border of the box.

**Specifications:**
- Center-aligned within the top border
- Support for emojis, colors, and environment variables
- Format: `--title="🚀 $PROJECT_NAME [blue]v1.0[/blue]"`
- Truncation with `…` when exceeding box width
- Preserve box corner characters

### ✅ 1.2 Footer Feature (`--footer`)
Mirror of title functionality for the bottom border.

**Specifications:**
- Same formatting capabilities as title
- Format: `--footer="Status: ✅ Complete"`
- No dedicated icon (manual emoji inclusion)
- Truncation logic identical to title

### ✅ 1.3 Icon Feature (`--icon`)
Decorative element for visual enhancement.

**Specifications:**
- Placement: Top-left corner (after corner character)
- Support single emoji or colored character
- Format: `--icon="🔥"` or `--icon="[red]★[/red]"`
- Optional position flag: `--icon-pos=left|right`

## ✅ Phase 2: Pipeline Integration (COMPLETED)
### ✅ 2.1 Box Stripping (`--no-boxy`)
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

## Phase 3: Width Control & Content Management
### 3.1 Fixed Width Feature (`--width`/`-w`) ✅ COMPLETED
Control box dimensions with intelligent content handling.

**Specifications:**
- Set exact box width: `--width=30` or `-w 30`
- Content truncation with Unicode ellipsis (`…`) when overflowing
- Minimum width validation (≥4 for basic box structure)
- Seamless integration with title, footer, and icon features
- Preserved content alignment and padding

### 3.2 Content Overflow Enhancements
- Smart word-wrapping within fixed width
- Multi-line content balancing
- Overflow indicators beyond ellipsis

### 3.3 Responsive Width
- Terminal-aware width adjustment
- Percentage-based widths (`--width=50%`)
- Auto-fit content with max width limits

## Phase 4: Advanced Features (Future)
### 4.1 Multi-column Support
- Side-by-side boxes with `--columns=2`
- Column separator styles

### 4.2 Interactive Mode
- Box resizing based on terminal width
- Dynamic color themes

### 4.3 Configuration File
- `.boxyrc` for default styles
- Custom color palettes
- Preset box templates

## Phase 5: Ecosystem Integration
### 5.1 Shell Completions
- Bash, Zsh, Fish autocomplete
- Dynamic color/style suggestions

### 5.2 Package Distribution
- Homebrew formula
- AUR package
- Cargo install optimization

## Design Principles
1. **Composability**: Every feature works in pipelines
2. **Performance**: Minimal overhead for large inputs
3. **Simplicity**: Intuitive flags, sensible defaults
4. **Compatibility**: UTF-8 aware, terminal-agnostic