#!/bin/bash

# Resolve repo root and binary path
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BOXY="$ROOT_DIR/target/release/boxy"

# Build release version if it doesn't exist
if [ ! -f "$BOXY" ]; then
    echo "Building release version..."
    (cd "$ROOT_DIR" && cargo build --release)
fi

# Mixed content
echo -e "Wide emojis (2 cols): 🚀🌟🔥\nNarrow symbols (1 col): ✨❤️\nMixed: Orange text 🚀 and ✨" | $BOXY

# Your test case
echo -e "Red text\nGreen text\nBlue text with emoji 🚀\nNormal text\nMixed red and green 🌟" | $BOXY

# Fun example
echo -e "📦 boxy v0.3.0\n🎯 Perfect alignment\n🦀 Written in Rust\n⚡ Lightning fast" | $BOXY

# Emoji overload
echo -e "🍕🍔🌮🌯🥙\n🍎🍊🍋🍌🍉\n🚗🚕🚙🚌🚎" | $BOXY

# Colored boxes
echo "Hello World" | $BOXY --color red
echo "Hello World" | $BOXY -c orange

# Combine style and color
echo "Hello World" | $BOXY --style rounded --color blue
echo "Hello World" | $BOXY -s double -c purple

# Your special colors
echo "Deep color" | $BOXY -c deep
echo "Deep green" | $BOXY -c deep_green
echo "Grey shades" | $BOXY -c grey2

# With emojis and colors
echo -e "🎉 Party\n🚀 Time" | $BOXY -s rounded -c cyan

# New width feature
echo "This is a long message that will be truncated" | $BOXY --width 20

# Theme examples - now working with fixed YAML loading
echo "Build failed!" | $BOXY --theme error
echo "Success!" | $BOXY --theme success
echo "Warning message" | $BOXY --theme warn
echo "System crash" | $BOXY --theme fatal
echo "Debug info" | $BOXY --theme debug
echo "Magic happening" | $BOXY --theme magic

# Theme with overrides
echo "Custom theme" | $BOXY --theme error --color blue --icon "🚀"

# Complex combinations
echo "Deploy complete" | $BOXY --theme success --title "🚀 Status" --footer "v0.3.0" --width 30

echo "Success!" | $BOXY --color green --text auto --icon "✅"
echo "Success!" | $BOXY --color red --text white --icon "🚀"

# Theme with overrides
echo "Custom theme" | $BOXY --theme error --color auto --icon "🚀"

# Theme with overrides
echo "Custom theme" | $BOXY --color blue --icon "✅"


# Colored boxes
echo "Hello World" | $BOXY --color red --icon "🚀"
echo "Hello World" | $BOXY -c orange --icon "🚀"
