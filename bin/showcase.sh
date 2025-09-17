#!/bin/bash

# Boxy Theme Showcase Ceremony
# A proper bash ceremony to demonstrate all themes without breaking the width calculations!

set -e

BOXY_BIN="./target/release/boxy"
if [ ! -f "$BOXY_BIN" ]; then
    echo "Building boxy..."
    cargo build --release
fi

# Sample texts for showcase
LOREM="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore."
SHORT="Boxy Showcase"
LONG="This is a longer demonstration text to show how boxy handles different content lengths and wrapping behaviors."

# Get theme list (extract just the theme names)
THEMES=$(cargo run --release --bin boxy theme list 2>/dev/null | grep "^  " | awk '{print $1}')

echo "🎭 Boxy Theme Showcase Ceremony"
echo "=================================="
echo

for theme in $THEMES; do
    echo "🎨 Theme: $theme"
    echo "─────────────────"

    # Short demo
    echo "$SHORT" | $BOXY_BIN --theme "$theme" --title "Theme: $theme"
    echo

    # Medium demo with some features
    echo "$LOREM" | $BOXY_BIN --theme "$theme" --title "Lorem Demo" --status "Theme: $theme"
    echo

    # Long text demo
    echo "$LONG" | $BOXY_BIN --theme "$theme" --title "Wrapping Demo"
    echo

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo
done

echo "✨ Showcase complete! All themes demonstrated using the working boxy binary."
echo "🛡️  Width calculations protected by macros - no internal modifications needed!"