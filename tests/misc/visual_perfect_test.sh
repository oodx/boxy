#!/bin/bash

# VISUAL PERFECT TEST - Standalone unit test for all boxy rendering features
# Shows each feature clearly and systematically for visual UAT

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
BOXY="$ROOT_DIR/target/release/boxy"

if [ ! -f "$BOXY" ]; then
    echo "Building release version..."
    (cd "$ROOT_DIR" && cargo build --release)
fi

clear
echo "========================================="
echo "🎯 BOXY VISUAL PERFECT TEST"
echo "Unit test for all rendering features"
echo "========================================="

# 1. BASIC RENDERING
echo -e "\n1️⃣ BASIC RENDERING:"
echo "Simple content" | $BOXY
echo "Multi-line content" | $BOXY
echo -e "Line 1\nLine 2\nLine 3" | $BOXY

# 2. WIDTH CONTROL
echo -e "\n2️⃣ WIDTH CONTROL:"
echo "Fixed width test" | $BOXY --width 30
echo "Very long text that should be properly handled with width control" | $BOXY --width 25

# 3. HEIGHT CONTROL
echo -e "\n3️⃣ HEIGHT CONTROL:"
echo "Height test" | $BOXY --height 8
echo "Height + width test" | $BOXY --height 10 --width 30

# 4. TITLE AND STATUS
echo -e "\n4️⃣ TITLE AND STATUS:"
echo "Content with title" | $BOXY --title "📋 Test Title"
echo "Content with status" | $BOXY --status "✅ Status OK"
echo "Title + status" | $BOXY --title "📋 Title" --status "✅ Status"

# 5. HEADER AND FOOTER
echo -e "\n5️⃣ HEADER AND FOOTER:"
echo "With header" | $BOXY --header "Header Text"
echo "With footer" | $BOXY --footer "Footer Text"
echo "Header + footer" | $BOXY --header "Top" --footer "Bottom"

# 6. ALL COMPONENTS TOGETHER
echo -e "\n6️⃣ ALL COMPONENTS:"
echo "Complete test" | $BOXY --title "Title" --header "Header" --footer "Footer" --status "Status" --width 40

# 7. STYLE VARIATIONS
echo -e "\n7️⃣ BOX STYLES:"
echo "Normal style" | $BOXY --style normal
echo "Rounded style" | $BOXY --style rounded
echo "Double style" | $BOXY --style double
echo "Heavy style" | $BOXY --style heavy
echo "ASCII style" | $BOXY --style ascii

# 8. COLOR VARIATIONS
echo -e "\n8️⃣ COLORS:"
echo "Red box" | $BOXY --color red
echo "Green box" | $BOXY --color green
echo "Blue box" | $BOXY --color blue

# 9. THEME TESTS
echo -e "\n9️⃣ THEMES:"
echo "Success theme" | $BOXY --theme success
echo "Error theme" | $BOXY --theme error
echo "Warning theme" | $BOXY --theme warning
echo "Info theme" | $BOXY --theme info

# 10. EMOJI AND UNICODE
echo -e "\n🔟 EMOJI & UNICODE:"
echo -e "🚀 Rocket\n✅ Checkmark\n🎯 Target" | $BOXY
echo -e "Wide emoji: 🌟🎉🚀\nNarrow: ✨❤️⚡" | $BOXY

# 11. PARAMS SYNTAX
echo -e "\n1️⃣1️⃣ PARAMS SYNTAX:"
echo "Params test" | $BOXY --params "h=8; w=25; tl='Params Title';"
echo "Full params" | $BOXY --params "h=10; w=35; tl='Full Test'; st='Status'; hd='Header';"

# 12. HEIGHT + STATUS INTEGRATION (Critical test)
echo -e "\n1️⃣2️⃣ HEIGHT + STATUS (Critical):"
echo "Height with status" | $BOXY --height 8 --status "✅ Should be aligned"
echo "Height + title + status" | $BOXY --height 10 --title "🎯 Title" --status "✅ Status"

# 13. COMPLEX COMBINATIONS
echo -e "\n1️⃣3️⃣ COMPLEX COMBINATIONS:"
echo -e "Multi\nLine\nContent" | $BOXY --height 12 --width 30 --title "Complex" --status "OK" --theme success

# 14. ALIGNMENT REGRESSION TEST
echo -e "\n1️⃣4️⃣ ALIGNMENT REGRESSION TEST:"
echo -e "✓ Pass\n⚠ Warning\n✗ Fail" | $BOXY --title "Alignment Test" --status "All aligned?"

echo -e "\n========================================="
echo "🎯 VISUAL PERFECT TEST COMPLETE"
echo "Check each section for proper rendering:"
echo "• Proper padding and alignment"
echo "• Correct width/height dimensions"
echo "• Status bar positioning"
echo "• Border integrity"
echo "• Text centering and spacing"
echo "========================================="