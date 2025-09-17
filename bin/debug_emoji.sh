#!/bin/bash

# Emoji debugging toolkit for boxy development
# Usage: ./debug_emoji.sh "ℹ️"

///   cargo run --bin emoji_debug compare "✅" "ℹ️" "🚀" "🟢" "⚠" "✗"
///   echo -en 'á' | iconv -f utf-8 -t UNICODEBIG | xxd -g 2


if [ $# -eq 0 ]; then
    echo "Usage: $0 'emoji_string'"
    echo "Example: $0 'ℹ️'"
    exit 1
fi

EMOJI="$1"

echo "🔍 EMOJI DEBUG ANALYSIS: '$EMOJI'"
echo "=================================="

echo -e "\n📊 BASIC STATS:"
echo "Input: '$EMOJI'"
echo -n "Byte count: "
echo -n "$EMOJI" | wc -c
echo -n "Char count (wc): "
echo -n "$EMOJI" | wc -m

echo -e "\n🔢 HEX DUMP:"
echo -n "$EMOJI" | xxd

echo -e "\n🐍 PYTHON UNICODE ANALYSIS:"
python3 -c "
import unicodedata
emoji = '$EMOJI'
print(f'Python len(): {len(emoji)}')
print(f'Unicode codepoints: {[hex(ord(c)) for c in emoji]}')
print(f'Unicode names:')
for i, c in enumerate(emoji):
    print(f'  [{i}] {repr(c)} = {unicodedata.name(c, \"UNKNOWN\")}')
"

echo -e "\n🦀 RUST UNICODE-WIDTH TEST:"
if command -v cargo >/dev/null 2>&1; then
    echo "Testing with our new emoji debug utility..."
    echo -n "Unicode width: "
    timeout 5s cargo run --bin emoji_debug "$EMOJI" 2>/dev/null | grep "Unicode Width:" | cut -d: -f2 | tr -d ' ' || echo "N/A"
fi

echo -e "\n📐 TERMINAL WIDTH TEST:"
echo "Terminal display test:"
echo "[$EMOJI] <- should be 1 visual width"
echo "[${EMOJI}${EMOJI}] <- comparison with double"

echo -e "\n✅ BOXY INTEGRATION TEST:"
echo "Testing emoji in boxy:"
if [ -f "./target/release/boxy" ]; then
    echo -e "Simple:\n$(timeout 5s bash -c "echo 'test' | ./target/release/boxy --icon '$EMOJI'" 2>/dev/null | head -2 || echo "Timeout or error")"
else
    echo "Simple: Build boxy first with 'cargo build --release'"
fi
