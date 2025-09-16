# Boxy Width Calculation System

## 🎯 Overview

Boxy has **two width calculation implementations** that can be easily swapped:

1. **Unicode-width crate** (default) - External dependency
2. **Custom implementation** (optional) - Our own emoji-aware calculation

## 🔄 Easy Swapping

### Environment Variable Method
```bash
# Use default (unicode-width crate)
echo "test" | boxy --theme info

# Use custom implementation
BOXY_USE_CUSTOM_WIDTH=1 echo "test" | boxy --theme info
```

### Code-level Method
In `src/width_plugin.rs`, change line 156:
```rust
// Current (env-based switching)
if std::env::var("BOXY_USE_CUSTOM_WIDTH").is_ok() {
    get_display_width_custom(text)
} else {
    get_display_width_unicode_crate(text)
}

// To force custom implementation:
get_display_width_custom(text)

// To force unicode-width crate:
get_display_width_unicode_crate(text)
```

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

### Unicode-width Crate
- **Pros**: Well-tested, handles edge cases
- **Cons**: External dependency, black box
- **Size**: Adds ~50KB to binary

### Custom Implementation
- **Pros**: Full control, no dependencies, emoji-optimized
- **Cons**: May miss edge cases
- **Features**:
  - Comprehensive emoji ranges
  - CJK character support
  - Variation selector handling
  - Zero-width character support

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

- ✅ Both implementations give **identical results**
- ✅ Custom handles compound emojis like `ℹ️` correctly
- ✅ CJK characters (世界) work properly
- ✅ Easy environment-based switching
- ✅ Comprehensive test coverage

**Ready for production testing!**

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