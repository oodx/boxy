#!/bin/bash

# Comprehensive Feature Test Suite for Boxy v0.8
# Tests all major features identified in help and REPAIRS.md

# Resolve repo root and binary path
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BOXY="$ROOT_DIR/target/release/boxy"

# Build release version if it doesn't exist
if [ ! -f "$BOXY" ]; then
    echo "Building release version..."
    (cd "$ROOT_DIR" && cargo build --release)
fi

echo "========================================="
echo "BOXY v0.8 COMPREHENSIVE FEATURE TEST"
echo "========================================="

# 1. BASIC BOX STYLES
echo -e "\n[1] BASIC BOX STYLES:"
echo "Testing normal style" | $BOXY --style normal
echo "Testing rounded style" | $BOXY --style rounded  
echo "Testing double style" | $BOXY --style double
echo "Testing heavy style" | $BOXY --style heavy
echo "Testing ASCII style" | $BOXY --style ascii

# 2. COLOR SYSTEM (90+ colors)
echo -e "\n[2] COLOR SYSTEM:"
echo "Red box" | $BOXY --color red
echo "Deep purple" | $BOXY --color deep_purple
echo "Cyan box" | $BOXY --color cyan
echo "Orange box" | $BOXY --color orange
echo "Grey shades" | $BOXY --color grey2

# 3. TEXT COLOR WITH AUTO-MATCHING
echo -e "\n[3] TEXT COLOR AUTO-MATCHING:"
echo "Auto text color" | $BOXY --color green --text auto
echo "White text on red" | $BOXY --color red --text white
echo "Custom text color" | $BOXY --color blue --text yellow

# 4. WIDTH MANAGEMENT
echo -e "\n[4] WIDTH MANAGEMENT:"
echo "Fixed width at 30 chars" | $BOXY --width 30
echo "This is a very long message that will be truncated at exactly 20 characters" | $BOXY --width 20
echo "Auto width" | $BOXY --width auto
# Note: --width max would use full terminal width

# 5. HEADERS AND FOOTERS
echo -e "\n[5] HEADERS, TITLES, AND FOOTERS:"
echo "Content here" | $BOXY --header "External Header"
echo "Main content" | $BOXY --title "📦 Title with Icon" 
echo "Body text" | $BOXY --footer "Version 0.8.1"
echo "Complete box" | $BOXY --header "HEADER" --title "TITLE" --footer "FOOTER"

# 6. TITLE AND FOOTER COLORS
echo -e "\n[6] TITLE/FOOTER COLORS:"
echo "Colored elements" | $BOXY --title "Red Title" --title-color red --footer "Blue Footer" --footer-color blue
echo "Header colors" | $BOXY --header "Green Header" --header-color green

# 7. STATUS LINE
echo -e "\n[7] STATUS LINE WITH ALIGNMENT:"
echo "Main content" | $BOXY --status "sl:Left aligned status"
echo "Main content" | $BOXY --status "sc:Center aligned status"  
echo "Main content" | $BOXY --status "sr:Right aligned status"
echo "Main content" | $BOXY --status "Status with color" --status-color cyan

# 8. LAYOUT CONTROL (from --help)
echo -e "\n[8] LAYOUT CONTROL:"
echo "Left header" | $BOXY --layout "hl" --title "Left Title"
echo "Center header" | $BOXY --layout "hc" --title "Center Title"
echo "Right header" | $BOXY --layout "hr" --title "Right Title"
echo "Left footer" | $BOXY --layout "fl" --footer "Left Footer"
echo "Center footer" | $BOXY --layout "fc" --footer "Center Footer"
echo "Right footer" | $BOXY --layout "fr" --footer "Right Footer"

# 9. PADDING (from REPAIRS.md)
echo -e "\n[9] PADDING CONTROL:"
echo "Padded above" | $BOXY --pad a
echo "Padded below" | $BOXY --pad b
echo "Padded both" | $BOXY --pad ab

# 10. THEME SYSTEM
echo -e "\n[10] THEME SYSTEM:"
echo "Error occurred!" | $BOXY --theme error
echo "Operation successful" | $BOXY --theme success
echo "Warning message" | $BOXY --theme warning
echo "Information" | $BOXY --theme info
echo "Critical failure" | $BOXY --theme critical

# 11. THEME WITH OVERRIDES
echo -e "\n[11] THEME OVERRIDES:"
echo "Custom error" | $BOXY --theme error --color blue --icon "🚀"
echo "Custom success" | $BOXY --theme success --style double --text yellow

# 12. ICON INTEGRATION (deprecated but test)
echo -e "\n[12] ICON INTEGRATION:"
echo "With rocket icon" | $BOXY --icon "🚀"
echo "With checkmark" | $BOXY --icon "✅"
echo "Combined" | $BOXY --icon "⚡" --color purple

# 13. MULTILINE CONTENT
echo -e "\n[13] MULTILINE CONTENT:"
echo -e "Line 1\nLine 2\nLine 3" | $BOXY
echo -e "🚀 Emojis\n🌟 In rows\n✨ Aligned" | $BOXY --color cyan

# 14. ANSI PRESERVATION
echo -e "\n[14] ANSI COLOR PRESERVATION:"
echo -e "\033[31mRed text\033[0m\n\033[32mGreen text\033[0m\n\033[34mBlue text\033[0m" | $BOXY

# 15. VARIABLE EXPANSION (from title/footer)
echo -e "\n[15] VARIABLE EXPANSION:"
USER="TestUser" echo "Testing vars" | $BOXY --title "\${USER} Dashboard"
echo "Path test" | $BOXY --footer "Path: \${PATH:0:20}..."

# 16. PARAMETER STREAMS
echo -e "\n[16] PARAMETER STREAMS:"
echo "Body content" | $BOXY --params "hd='Stream Header'; tl='Stream Title'; ft='Stream Footer'"

# 17. NO-BOXY MODE (Pipeline integration)
echo -e "\n[17] NO-BOXY MODE:"
echo "Strip decorations" | $BOXY --theme error --no-boxy
echo "Strict strip" | $BOXY --theme success --no-boxy=strict

# 18. COMPLEX COMBINATIONS
echo -e "\n[18] COMPLEX COMBINATIONS:"
echo -e "🎯 Project Status\n✅ Tests passing\n📦 Ready to deploy" | \
    $BOXY --theme success \
          --title "CI/CD Pipeline" \
          --footer "Build #142" \
          --width 40 \
          --status "sc:All systems operational" \
          --pad ab

# 19. EMOJI WIDTH HANDLING
echo -e "\n[19] EMOJI WIDTH HANDLING:"
echo -e "🍕🍔🌮🌯🥙 Food\n🚗🚕🚙🚌🚎 Cars\n✨⭐💫🌟✨ Stars" | $BOXY --width 25

# 20. EDGE CASES
echo -e "\n[20] EDGE CASES:"
echo "" | $BOXY --title "Empty body"
echo "Single" | $BOXY --width 10
echo "Very very very very very long single line that should wrap or truncate appropriately" | $BOXY --width 15

# 21. THEME MANAGEMENT COMMANDS
echo -e "\n[21] THEME MANAGEMENT:"
$BOXY theme list | head -5
# Note: Other theme commands (show, create, etc) are interactive

# 22. DIAGNOSTIC COMMANDS
echo -e "\n[22] DIAGNOSTICS:"
$BOXY width

echo -e "\n========================================="
echo "COMPREHENSIVE TEST COMPLETE"
echo "========================================="