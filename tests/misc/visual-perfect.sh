#!/bin/bash

# Visual Perfect Test - Single box showcasing ALL boxy features
# Shows everything in one comprehensive demonstration

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BOXY="$ROOT_DIR/target/release/boxy"

if [ ! -f "$BOXY" ]; then
    echo "Building release version..."
    (cd "$ROOT_DIR" && cargo build --release)
fi

echo "🎯 VISUAL PERFECT TEST - All Features in One Box"
echo "================================================"

# Single comprehensive box showing ALL features
cat << 'EOF' | $BOXY --theme blueprint \
                    --style rounded \
                    --title "🎯 VISUAL PERFECT TEST" \
                    --header "All Boxy Features Demonstration" \
                    --footer "Height: 20 | Width: 60 | Style: Rounded | Theme: Blueprint" \
                    --status "✅ All systems operational - UAT passed" \
                    --width 60 \
                    --height 20 \
                    --layout "dt,ds,stn,ssn"
📋 CONTENT FEATURES:
✓ Multi-line text rendering
✓ Emoji support: 🚀🎉🌟✨❤️⚡
✓ Unicode symbols: ←→↑↓⟲⟳⊕⊗
✓ Alignment and padding
✓ Text wrapping and truncation

🎨 VISUAL FEATURES:
✓ Title positioning (top)
✓ Header integration
✓ Footer integration
✓ Status bar (bottom, centered)
✓ Rounded border style
✓ Blueprint theme coloring

📐 DIMENSION CONTROLS:
✓ Fixed width: 60 characters
✓ Fixed height: 20 lines total
✓ Height padding: blank lines added
✓ Content properly centered

🔧 LAYOUT CONTROLS:
✓ Dividers after title (dt)
✓ Dividers before status (ds)
✓ Title padding (stn)
✓ Status padding (ssn)
EOF

echo ""
echo "✅ If this box renders correctly with:"
echo "   • Exact 60 character width"
echo "   • Exact 20 line height"
echo "   • Proper status alignment"
echo "   • All text properly padded"
echo "   • Rounded borders intact"
echo "   • Colors applied correctly"
echo "Then all visual features are working!"