# Boxy Width Calculation System

## 🎯 Overview

Boxy uses a **protected macro system** with dual width calculation implementations:

1. **Unicode-width crate** (primary/default) - Accurate external dependency
2. **Custom implementation** (fallback) - Environment-switchable backup
3. **Protected macros** - Abstraction layer preventing regressions

## 🛑 Protected Macro System

Three critical macros abstract width calculations and prevent regressions:

### box_width! (src/draw.rs)
```rust
// Main box width calculation wrapper
box_width!(text, h_padding, fixed_width)
```
**Purpose**: Calculates optimal box width based on content and terminal constraints
**Location**: Used in calculate_box_width() function
**Protection**: Prevents direct width calculation calls

### max_width! (src/components.rs)
```rust
// Content maximum width calculation
let content_max_width = max_width!(composed_lines);
```
**Purpose**: Determines maximum width across all content lines
**Location**: Body component rendering
**Protection**: Ensures consistent multi-line width handling

### inner_target_width! (src/components.rs)
```rust
// Inner content target width calculation
let inner_content_target_width = inner_target_width!(inner_width, self.config.width.h_padding);
```
**Purpose**: Calculates proper inner content width including padding
**Location**: Component layout calculations
**Protection**: Prevents padding calculation errors

## 🔄 Implementation Switching

### Primary Method: Environment Variable
```bash
# Use primary (unicode-width crate) - DEFAULT
echo "test" | boxy --theme info

# Use fallback (custom implementation)
BOXY_USE_CUSTOM_WIDTH=1 echo "test" | boxy --theme info
```

### Advanced Method: Code-level Override
In `src/width_plugin.rs`, the switching logic:
```rust
// Current architecture (env-based switching with unicode-width primary)
if std::env::var("BOXY_USE_CUSTOM_WIDTH").is_ok() {
    get_display_width_custom(text)  // Fallback implementation
} else {
    get_display_width_unicode_crate(text)  // Primary implementation
}
```

**Note**: Direct modification not recommended due to protected macro system.

## 🧪 Testing & Comparison

### Compare Implementations
```bash
cargo run --bin width_compare compare "X" "✅" "ℹ️" "🚀"
```

### Test Individual Characters
```bash
cargo run --bin width_compare "ℹ️"
```

### Test with Emoji Debug Tool
```bash
cargo run --bin emoji_debug compare "✅" "ℹ️" "🚀"
```

## 📊 Implementation Details

### Unicode-width Crate (Primary)
- **Pros**: Well-tested, handles edge cases, comprehensive Unicode support
- **Cons**: External dependency (~50KB binary size)
- **Features**:
  - Complete Unicode width tables
  - Grapheme cluster handling
  - Variation selector support
  - CJK character support
  - **ANSI escape sequence stripping** (critical fix)

### Custom Implementation (Fallback)
- **Pros**: Full control, dependency-free, emoji-optimized
- **Cons**: May miss edge cases, maintenance overhead
- **Features**:
  - Comprehensive emoji ranges
  - CJK character support
  - Variation selector handling
  - Zero-width character support
  - Custom debugging capabilities

### Protected Macro Benefits
- **Regression Prevention**: Abstracts implementation details
- **Consistent Interface**: Same API across implementations
- **Easy Testing**: Switch implementations via environment
- **Future-proof**: Can add new implementations without code changes

## 🚀 Removing Unicode-width Dependency

If custom implementation proves stable:

1. **Test thoroughly**:
   ```bash
   BOXY_USE_CUSTOM_WIDTH=1 ./bin/test.sh run minimal
   ```

2. **Update Cargo.toml**:
   ```toml
   # Remove this line:
   # unicode-width = "0.2"
   ```

3. **Update get_display_width()** to use custom by default

4. **Remove unicode_width imports** from:
   - `src/lib.rs`
   - `src/main.rs`
   - `src/emoji_debug.rs`

## 🎯 Current Status

- ✅ Unicode-width crate is **primary implementation** (stable)
- ✅ Custom implementation provides **reliable fallback**
- ✅ Protected macros **prevent regressions**
- ✅ Both implementations give **identical results**
- ✅ ANSI color code handling **fixed** (major regression resolved)
- ✅ Emoji truncation issues **resolved** (grapheme cluster handling)
- ✅ Empty box rendering **improved**
- ✅ Mixed emoji/ASCII alignment **perfected**
- ✅ Comprehensive test coverage

**Production ready with protected architecture!**

## 📈 Performance

Custom implementation may be faster due to:
- No external crate overhead
- Optimized for common emoji patterns
- Direct character matching vs general Unicode tables

## 🔍 Debugging

Use these tools to debug width issues:
- `cargo run --bin width_compare` - Compare implementations
- `cargo run --bin emoji_debug` - Detailed emoji analysis
- `./debug_emoji.sh` - Shell-based debugging
- `boxy width` - Terminal width diagnostics

### Common Issues Fixed
1. **ANSI Color Codes**: Width calculation now strips escape sequences
2. **Emoji Truncation**: Proper grapheme cluster handling prevents splits
3. **Mixed Content**: Uniform padding across emoji/ASCII combinations
4. **Empty Boxes**: Improved minimum width handling