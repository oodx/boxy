# 📏 Boxy HEIGHT Feature Documentation

## Overview

The HEIGHT feature in Boxy provides fixed-height box rendering for terminal multiplexers, TUI frameworks, and layout engines requiring predictable vertical spacing. This feature is **disabled by default** to maintain backward compatibility.

## Quick Start

```bash
# Enable fixed height mode
boxy --height 20 "Your content here"

# Show height diagnostics
boxy height
```

## Features

### Terminal Height Detection

Boxy automatically detects terminal height using multiple methods:
1. `tput lines` - Standard terminal query
2. `stty size` - Terminal size detection (rows and columns)
3. `LINES` environment variable
4. Fallback to 24 lines (standard terminal height)

### Height Modes

The HEIGHT feature supports three modes:

- **pad** (default): Adds blank lines to reach target height
- **truncate**: Cuts content if it exceeds target height
- **auto**: Uses content height (effectively disables fixed height)

## Usage Examples

### Basic Fixed Height

```bash
# Create a box with fixed height of 10 lines
echo "Short content" | boxy --height 10

# Box will be padded with blank lines to reach exactly 10 lines total
```

### Terminal Multiplexer Integration

```bash
# tmux pane with fixed height
tmux split-window -v -l 10 'echo "Status" | boxy --height 8 --title "System"'

# Consistent heights for dashboard panels
echo "CPU: 45%" | boxy --height 6 --title "CPU" &
echo "Memory: 78%" | boxy --height 6 --title "Memory" &
```

### Using --params Flag

```bash
# Set height and width together
boxy --params "h=15; w=40; tl='Title';" "Content"

# Height with other parameters
boxy --params "h=10; w=auto;" "Dynamic width, fixed height"
```

### Environment Variables

```bash
# Set default height
export BOXY_HEIGHT=20

# Set default height mode
export BOXY_HEIGHT_MODE=pad

# Custom filler character
export BOXY_HEIGHT_FILLER="·"
```

## Height Diagnostics

Check your terminal's height detection:

```bash
$ boxy height
Height diagnostics:
  effective (get_terminal_height): 24
  tput lines (tty): 24
  stty size rows (tty): 24
```

## Configuration

### Height Range
- **Minimum**: 5 lines
- **Maximum**: 50 lines
- **Default fallback**: 24 lines

### Component Structure
When height is enabled, boxes maintain this structure:
```
┌─────────────────┐  ← Header (1 line)
│ Content Line 1  │  ↑
│ Content Line 2  │  │ Body (dynamic)
│                 │  │ + Padding (calculated)
│ [padding lines] │  ↓
│ Status: Ready   │  ← Status (anchored at bottom)
└─────────────────┘  ← Footer (1 line)
```

## Use Cases

### TUI Framework Integration

```rust
// In your Rust application using boxy as a library
use boxy::BoxBuilder;

let status_box = BoxBuilder::new("System Ready")
    .height(8)
    .title("Status")
    .render()?;
// Always produces exactly 8 lines of output
```

### Layout Engine Support

Create consistent panel heights for dashboards:

```bash
#!/bin/bash
# Dashboard with uniform panel heights

for metric in "CPU:45%" "RAM:78%" "DISK:23%"; do
    echo "$metric" | boxy --height 6 --theme modern
done
```

## Compatibility

- **Terminal Support**: Works with all standard terminals
- **Multiplexer Support**: tmux, screen, and other terminal multiplexers
- **TUI Frameworks**: Compatible with ratatui, cursive, and other TUI libraries
- **Default Behavior**: Feature is disabled by default - existing scripts work unchanged

## Limitations

- Height range is limited to 5-50 lines for safety
- Status lines always remain anchored at the bottom
- Content truncation in `truncate` mode may cut mid-line
- Terminal resize events may affect height calculations

## Troubleshooting

### Height Not Applied

Check if your terminal supports height detection:
```bash
boxy height
```

### Content Overflow

If content exceeds the fixed height in `pad` mode, consider:
1. Increasing the height value
2. Using `truncate` mode
3. Reducing content or using scrollable viewers

### Environment Variable Not Working

Ensure the variable is exported:
```bash
export BOXY_HEIGHT=20
echo "test" | boxy  # Should use height 20
```

## Future Enhancements

Planned improvements for the HEIGHT feature:
- Dynamic height adjustment based on content
- Scrollable content within fixed height
- Height templates for common use cases
- Integration with more TUI frameworks

## Related Documentation

- [Width System](./WIDTH_FEATURE.md) - Terminal width handling
- [Theme System](./THEME_SYSTEM.md) - Visual customization
- [API Documentation](./PUBLIC_API_STRAT.txt) - Library usage

---

*For technical implementation details, see [BOXY_HEIGHT_STRAT.txt](./BOXY_HEIGHT_STRAT.txt)*