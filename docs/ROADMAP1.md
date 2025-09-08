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

## ✅ Phase 3: Width Control & Content Management (COMPLETED)
### ✅ 3.1 Fixed Width Feature (`--width`/`-w`) 
Control box dimensions with intelligent content handling.

**Specifications:**
- Set exact box width: `--width=30` or `-w 30`
- Content truncation with Unicode ellipsis (`…`) when overflowing
- Minimum width validation (≥4 for basic box structure)
- Seamless integration with title, footer, and icon features
- Preserved content alignment and padding

### ✅ 3.2 Theme System (`--theme`)
Predefined visual styling with unified icon/color combinations.

**Specifications:**
- Pre-built themes: `error`, `success`, `warn`, `info`, `debug`, etc.
- Unified icon handling approach for consistent spacing
- Theme inheritance with explicit flag overrides
- Expandable theme system for future customization

### ✅ 3.3 Text Color Control (`--text`)
Independent text color control with smart auto-matching.

**Specifications:**
- Support for all existing color palette: `--text red`, `--text blue2`
- Auto-mode for box color matching: `--text auto`
- Default terminal color preservation when omitted
- Seamless integration with themes and manual icons
- Unified icon rendering approach for consistent behavior

## Phase 4: Content Enhancement (Future)
### 4.1 Content Overflow Enhancements
- Smart word-wrapping within fixed width
- Multi-line content balancing
- Overflow indicators beyond ellipsis

### 4.2 Responsive Width
- Terminal-aware width adjustment
- Percentage-based widths (`--width=50%`)
- Auto-fit content with max width limits

## Phase 5: Advanced Features (Future)
### 5.1 Multi-column Support
- Side-by-side boxes with `--columns=2`
- Column separator styles

### 5.2 Interactive Mode
- Box resizing based on terminal width
- Dynamic color themes

### 5.3 Configuration File
- `.boxyrc` for default styles
- Custom color palettes
- Preset box templates

## Phase 6: Ecosystem Integration
### 6.1 Shell Completions
- Bash, Zsh, Fish autocomplete
- Dynamic color/style suggestions

### 6.2 Package Distribution
- Homebrew formula
- AUR package
- Cargo install optimization

## Design Principles
1. **Composability**: Every feature works in pipelines
2. **Performance**: Minimal overhead for large inputs
3. **Simplicity**: Intuitive flags, sensible defaults
4. **Compatibility**: UTF-8 aware, terminal-agnostic