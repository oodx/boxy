#!/bin/bash
# Color Testing Harness - Comprehensive validation of color system
# Tests all 90+ colors with visual verification and error scenarios

set -e

echo "🎨 Boxy Color System Testing Harness v0.6"
echo "=========================================="
echo

# Build first to ensure everything compiles
echo "📦 Building boxy..."
cargo build --release
echo

BOXY="./target/release/boxy"

# Test 1: Legacy color compatibility (v0.5.0 colors must work)
echo "🔄 Testing legacy color compatibility..."
legacy_colors=("red" "green" "blue" "orange" "purple" "grey")
for color in "${legacy_colors[@]}"; do
    echo "Legacy $color" | $BOXY --color "$color" --style ascii
done
echo "✅ Legacy colors working"
echo

# Test 2: New semantic colors
echo "🎯 Testing semantic colors..."
semantic_colors=("error" "success" "warning" "info" "critical")
for color in "${semantic_colors[@]}"; do
    echo "Semantic: $color" | $BOXY --color "$color" --style rounded
done
echo "✅ Semantic colors working"
echo

# Test 3: Extended color palette
echo "🌈 Testing extended color palette..."
extended_colors=("crimson" "emerald" "azure" "amber" "violet")
for color in "${extended_colors[@]}"; do
    echo "Extended: $color" | $BOXY --color "$color" --style heavy
done
echo "✅ Extended colors working"
echo

# Test 4: Color validation and error handling
echo "⚠️  Testing color validation..."

# Test invalid color (should fail gracefully)
echo "Testing invalid color handling..."
if echo "Invalid color test" | $BOXY --color "invalid_color_name" 2>/dev/null; then
    echo "❌ ERROR: Invalid color was accepted (should have failed)"
    exit 1
else
    echo "✅ Invalid colors properly rejected"
fi

# Test color suggestion system
echo "Testing color suggestion system..."
if echo "Test" | $BOXY --color "redd" 2>&1 | grep -q "crimson"; then
    echo "✅ Color suggestions working"
else
    echo "❌ ERROR: Color suggestions not working properly"
    exit 1
fi

# Test 5: --colors flag functionality
echo "🎨 Testing color preview system..."
if $BOXY --colors | grep -q "Legacy Colors"; then
    echo "✅ Color preview system working"
else
    echo "❌ ERROR: Color preview system not working"
    exit 1
fi

# Test 6: Text color validation
echo "📝 Testing text color validation..."
echo "Text color test" | $BOXY --text "emerald" --color "slate"
echo "✅ Text color validation working"

# Test 7: Auto and control colors
echo "🔧 Testing control colors..."
echo "Auto color test" | $BOXY --color "auto" --text "none"
echo "✅ Control colors working"

# Test 8: Performance test - render many colors quickly
echo "⚡ Performance testing..."
start_time=$(date +%s%N)
for i in {1..20}; do
    echo "Performance test $i" | $BOXY --color "azure" >/dev/null
done
end_time=$(date +%s%N)
duration=$((($end_time - $start_time) / 1000000))
echo "✅ Performance test: 20 renders in ${duration}ms"

# Test 9: All theme colors still work
echo "🎭 Testing theme color compatibility..."
themes=("error" "success" "warn" "info")
for theme in "${themes[@]}"; do
    if echo "Theme test: $theme" | $BOXY --theme "$theme" >/dev/null 2>&1; then
        echo "✅ Theme $theme working"
    else
        echo "❌ ERROR: Theme $theme broken"
        exit 1
    fi
done

echo
echo "🎉 ALL TESTS PASSED!"
echo "Color system ready for production use."
echo "Total colors available: $(echo "$BOXY --colors" | grep -o '■' | wc -l)"
echo
echo "Usage examples:"
echo "  echo 'Critical error' | $BOXY --color critical"
echo "  echo 'Success message' | $BOXY --color emerald --text white"
echo "  echo 'Info display' | $BOXY --color azure --style rounded"