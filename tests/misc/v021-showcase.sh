#!/bin/bash

# Comprehensive v0.21.0 Feature Showcase
# Demonstrates all new box styles, barmode, and CLI features

# Resolve repo root and binary path
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BOXY="$ROOT_DIR/target/release/boxy"

# Build release version if it doesn't exist
if [ ! -f "$BOXY" ]; then
    echo "Building release version..."
    (cd "$ROOT_DIR" && cargo build --release)
fi

echo "========================================="
echo "BOXY v0.21.0 FEATURE SHOWCASE"
echo "========================================="
echo

# 1. ALL 10 BOX STYLES
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[1] ALL 10 BOX STYLES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "Classic styles:" | $BOXY --style normal --title "NORMAL"
echo "Rounded corners" | $BOXY --style rounded --title "ROUNDED"
echo "Strong borders" | $BOXY --style double --title "DOUBLE"
echo "Bold design" | $BOXY --style heavy --title "HEAVY"
echo "Terminal safe" | $BOXY --style ascii --title "ASCII"
echo

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[2] NEW v0.20.0 STYLES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "Thick double corners" | $BOXY --style thicksii --title "THICKSII"
echo "Subtle colons" | $BOXY --style colon --title "COLON"
echo "Dotted style" | $BOXY --style dot --title "DOT"
echo "Star borders" | $BOXY --style star --title "STAR"
echo "Dashed lines" | $BOXY --style dashed --title "DASHED"
echo

# 2. BARMODE LAYOUTS
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[3] BARMODE LAYOUTS (v0.20.0)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "This is a document section with horizontal bars only.
No vertical borders - perfect for text integration.
Looks professional in documentation." | $BOXY --layout bm --header "Document Section"

echo "Clean separation between content blocks
Works great for logs, reports, and terminals" | $BOXY --layout bm --footer "End of Section"

echo "Barmode with different styles:" | $BOXY --layout bm --style heavy --title "Heavy Bars"
echo "Subtle dashed separators" | $BOXY --layout bm --style dashed --footer "Status: Complete"
echo

# 3. STYLE + COLOR COMBINATIONS
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[4] STYLE + COLOR COMBINATIONS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "Success notification" | $BOXY --style rounded --color green --title "✓ Complete"
echo "Error alert" | $BOXY --style heavy --color red --title "⚠ Error"
echo "Information" | $BOXY --style double --color blue --title "ℹ Info"
echo "Warning message" | $BOXY --style thicksii --color orange --title "⚡ Warning"
echo

# 4. BARMODE + THEMES
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[5] BARMODE + THEMES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "Operation completed successfully
All tests passed without errors" | $BOXY --layout bm --theme success --header "Success Report"

echo "Critical system failure detected
Immediate attention required" | $BOXY --layout bm --theme error --footer "Error Code: 500"

echo "System maintenance scheduled
Services may be temporarily unavailable" | $BOXY --layout bm --theme warning --header "Maintenance Notice"

echo "New features available in v0.21.0
Check documentation for details" | $BOXY --layout bm --theme info --footer "Learn More"
echo

# 5. UNICODE AND EMOJI SUPPORT
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[6] UNICODE & EMOJI SUPPORT"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "🚀 Rocket Launch
🌟 Star Quality
🎉 Celebration
💡 Bright Ideas
🔥 Hot Topics" | $BOXY --style rounded --title "Emoji Support"

echo "中文字符支持 (Chinese)
日本語サポート (Japanese)
한국어 지원 (Korean)" | $BOXY --style double --title "CJK Characters"
echo

# 6. STATUS LINES WITH STYLES
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[7] STATUS LINES + NEW STYLES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "Processing data..." | $BOXY --style colon --status "sl:Progress: 75%" --status-color cyan
echo "Server running" | $BOXY --style dot --status "sc:Status: Online" --status-color green
echo "Build complete" | $BOXY --style star --status "sr:Time: 2.3s" --status-color yellow
echo

# 7. COMPLEX LAYOUTS
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[8] COMPLEX LAYOUTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "This is a complete example showing
all components working together:
- Header with alignment
- Main content area
- Status line with info
- Footer with version" | $BOXY \
    --style thicksii \
    --color purple \
    --header "System Dashboard" \
    --title "📊 Metrics" \
    --status "sc:All Systems Operational" \
    --status-color green \
    --footer "v0.21.0"
echo

# 8. WIDTH VARIATIONS WITH NEW STYLES
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[9] WIDTH VARIATIONS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "Narrow" | $BOXY --style dashed --width 20
echo "Medium width box" | $BOXY --style colon --width 40
echo "Wide box with plenty of space for longer content" | $BOXY --style dot --width 60
echo

# 9. BARMODE WITH ALL NEW STYLES
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[10] BARMODE × ALL NEW STYLES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "THICKSII barmode - strong horizontal separators" | $BOXY --layout bm --style thicksii --title "Strong Bars"
echo "COLON barmode - subtle dotted separators" | $BOXY --layout bm --style colon --title "Subtle"
echo "DOT barmode - minimalist point separators" | $BOXY --layout bm --style dot --title "Minimal"
echo "STAR barmode - decorative star separators" | $BOXY --layout bm --style star --title "Decorative"
echo "DASHED barmode - clean dashed separators" | $BOXY --layout bm --style dashed --title "Clean"
echo

# 10. REAL-WORLD USE CASES
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[11] REAL-WORLD USE CASES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "Use Case: Build Log Entry" | $BOXY --layout bm --style dashed --color grey3
echo

echo "[2025-01-15 14:32:15] INFO: Starting deployment
[2025-01-15 14:32:16] INFO: Building containers
[2025-01-15 14:32:20] SUCCESS: Deployment complete" | $BOXY --style colon --title "📝 Deployment Log" --footer "Exit Code: 0"
echo

echo "database_backup_2025_01_15.sql    [✓] Complete
server_logs_2025_01_15.tar.gz      [✓] Complete
config_snapshot.json               [✓] Complete" | $BOXY --style dot --theme success --title "Backup Status"
echo

echo "CPU: 45% [████░░░░░░]
RAM: 78% [███████░░░]
DISK: 23% [██░░░░░░░░]" | $BOXY --style heavy --color cyan --title "System Resources" --status "sr:Updated: 2s ago"
echo

# 11. COMBINING EVERYTHING
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "[12] THE ULTIMATE BOX"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo

echo "This box demonstrates:
✓ New THICKSII style
✓ Custom colors
✓ Header alignment
✓ Title with emoji
✓ Status line with color
✓ Footer information
✓ Unicode content support
✓ Perfect for production use

All v0.21.0 features working together!" | $BOXY \
    --style thicksii \
    --color deep_purple \
    --text white \
    --layout "hc" \
    --header "BOXY v0.21.0 SHOWCASE" \
    --title "🎨 Feature Complete" \
    --status "sc:All Tests Passing ✓" \
    --status-color green \
    --footer "Built with ❤️ by qodeninja" \
    --footer-color cyan
echo

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ SHOWCASE COMPLETE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo
echo "All 10 box styles tested ✓"
echo "Barmode layouts demonstrated ✓"
echo "Unicode support verified ✓"
echo "Real-world use cases shown ✓"
echo
echo "Run with: ./bin/test.sh run showcase"