#!/bin/bash

# Boxy Comprehensive Feature Test
# Tests each feature progressively to spot issues at any level

set -e

BOXY_BIN="./target/release/boxy"
if [ ! -f "$BOXY_BIN" ]; then
    echo "Building boxy..."
    cargo build --release
fi

# Colors for test output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
DGREY='\033[38;5;240m'  # Dark grey
GREY='\033[38;5;245m'   # Medium grey
NC='\033[0m' # No Color

test_header() {
    echo
    echo -e "${DGREY}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${DGREY}🧪 $1${NC}"
    echo -e "${DGREY}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

test_section() {
    echo
    echo -e "${GREY}📋 $1${NC}"
    echo -e "${GREY}────────────────────────────────────────────────────────────────────────────────${NC}"
}

test_case() {
    echo -e "${GREEN}✓ $1${NC}"
}

echo -e "${DGREY}🎯 BOXY COMPREHENSIVE FEATURE TEST${NC}"
echo -e "${DGREY}═══════════════════════════════════════════════════════════════════════════════════${NC}"

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "1. BASIC CONTENT RENDERING"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "1.1 Simple Content"
test_case "Single line"
echo "Hello World" | $BOXY_BIN

test_case "Multi-line content"
echo -e "Line 1\nLine 2\nLine 3" | $BOXY_BIN

test_case "Empty content"
echo "" | $BOXY_BIN

test_case "Very long single line (auto-width)"
echo "This is a very long line that should demonstrate auto-width calculation and wrapping behavior" | $BOXY_BIN

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "2. WIDTH CALCULATION (CRITICAL)"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "2.1 Auto-width with content only"
test_case "Short content"
echo "Short" | $BOXY_BIN

test_case "Medium content"
echo "This is medium length content" | $BOXY_BIN

test_case "Long content"
echo "This is a much longer piece of content that should expand the box width automatically" | $BOXY_BIN

test_section "2.2 Fixed width"
test_case "Fixed width 20"
echo "Content" | $BOXY_BIN --width 20

test_case "Fixed width 50"
echo "This content should fit within the fixed width" | $BOXY_BIN --width 50

test_case "Content longer than fixed width"
echo "This is very long content that exceeds the fixed width and should wrap or truncate" | $BOXY_BIN --width 30

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "3. TITLE FEATURE (WIDTH CRITICAL)"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "3.1 Title shorter than content"
test_case "Short title with long content"
echo "This is a long piece of content that is longer than the title" | $BOXY_BIN --title "Short"

test_section "3.2 Title longer than content"
test_case "Long title with short content (CRITICAL TEST)"
echo "Short" | $BOXY_BIN --title "This is a very long title that exceeds content width"

test_case "Very long title with medium content"
echo "Medium content here" | $BOXY_BIN --title "This is an extremely long title that should force the box to expand properly"

test_section "3.3 Title with fixed width"
test_case "Long title with fixed width"
echo "Content" | $BOXY_BIN --title "Long title here" --width 25

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "4. STATUS BAR FEATURE (WIDTH CRITICAL)"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "4.1 Status shorter than content"
test_case "Short status with long content"
echo "This is a long piece of content that is longer than the status" | $BOXY_BIN --status "v1.0"

test_section "4.2 Status longer than content"
test_case "Long status with short content (CRITICAL TEST)"
echo "Short" | $BOXY_BIN --status "This is a very long status that exceeds content width"

test_section "4.3 Status with title combinations"
test_case "Both title and status longer than content"
echo "Short" | $BOXY_BIN --title "Very long title here" --status "Very long status message"

test_case "Title longer, status shorter"
echo "Medium content" | $BOXY_BIN --title "Very long title exceeds everything" --status "v1.0"

test_case "Title shorter, status longer"
echo "Medium content" | $BOXY_BIN --title "Short" --status "Very long status message that exceeds content"

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "5. THEME SYSTEM"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "5.1 Basic themes"
for theme in success error warning info; do
    test_case "Theme: $theme"
    echo "Testing $theme theme" | $BOXY_BIN --theme $theme
done

test_section "5.2 Border styles"
for theme in base base_rounded base_double base_heavy; do
    test_case "Style: $theme"
    echo "Testing $theme style" | $BOXY_BIN --theme $theme
done

test_section "5.3 Themes with width issues"
test_case "Success theme with long title (regression test)"
echo "Content" | $BOXY_BIN --theme success --title "Very long title that previously caused width issues"

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "6. EMOJI AND UNICODE"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "6.1 Emoji width calculation"
test_case "Wide emojis"
echo "🚀🌟🔥" | $BOXY_BIN

test_case "Mixed emoji and text"
echo "Deploy 🚀 complete ✅" | $BOXY_BIN

test_case "Emoji in title"
echo "Content" | $BOXY_BIN --title "🚀 Deployment Status"

test_case "Emoji in status"
echo "Content" | $BOXY_BIN --status "✅ Complete"

test_section "6.2 Unicode and special characters"
test_case "Unicode symbols"
echo "✓ ✗ ⚠ ℹ" | $BOXY_BIN

test_case "Mixed unicode"
echo "Status: ✓ Tests passed ⚠ 3 warnings" | $BOXY_BIN

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "7. WRAPPING AND HINTS"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "7.1 Auto-wrapping (default)"
test_case "Long content auto-wrap"
echo "This is a very long line that should automatically wrap at word boundaries when it exceeds terminal width" | $BOXY_BIN

test_section "7.2 Wrap hints with --wrap flag"
test_case "Ideal wrap point #W#"
echo "Text with ideal #W# wrap point for testing" | $BOXY_BIN --wrap

test_case "Truncate hint #T#"
echo "Remove this part #T# keep this content" | $BOXY_BIN --wrap

test_case "Newline hint #NL#"
echo "Line one #NL# Line two #NL# Line three" | $BOXY_BIN --wrap

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "8. LAYOUT AND PADDING"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "8.1 Layout combinations"
test_case "Title with divider"
echo -e "Content line 1\nContent line 2" | $BOXY_BIN --title "Section Header" --layout "dt"

test_case "Status with divider"
echo -e "Content line 1\nContent line 2" | $BOXY_BIN --status "Footer" --layout "ds"

test_case "Both title and status with dividers"
echo -e "Content line 1\nContent line 2" | $BOXY_BIN --title "Header" --status "Footer" --layout "dt,ds"

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "9. EDGE CASES AND STRESS TESTS"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "9.1 Extreme width scenarios"
test_case "Very long title, very long status, short content"
echo "X" | $BOXY_BIN --title "This is an extremely long title that should definitely exceed any reasonable content width" --status "This is also an extremely long status that should exceed content width"

test_case "Multiple long elements with fixed width"
echo "Content" | $BOXY_BIN --title "Long title" --status "Long status" --width 20

test_section "9.2 Special content"
test_case "Content with ANSI codes"
echo -e "\033[31mRed text\033[0m and \033[32mgreen text\033[0m" | $BOXY_BIN

test_case "Mixed content types"
echo -e "Text\n🚀 Emoji\n✓ Symbols\n\033[33mColors\033[0m" | $BOXY_BIN --title "Mixed Content Test"

# ═══════════════════════════════════════════════════════════════════════════════════
test_header "10. REGRESSION TESTS"
# ═══════════════════════════════════════════════════════════════════════════════════

test_section "10.1 Known issue regression tests"
test_case "Showcase scenario (title longer than content)"
echo "Test showcase" | $BOXY_BIN --theme success --title "Test Theme: success"

test_case "Complex showcase scenario"
echo "Lorem ipsum" | $BOXY_BIN --theme magic --title "Theme Demonstration: magic" --status "Showcase v1.0"

test_section "10.2 Width calculation validation"
test_case "Auto-width with long title"
echo "Short content" | $BOXY_BIN --title "This title is significantly longer than the content and should expand the box"

test_case "Auto-width with long status"
echo "Short content" | $BOXY_BIN --status "This status is significantly longer than the content and should expand the box"

test_case "Auto-width with both long title and status"
echo "Short" | $BOXY_BIN --title "Very long title that exceeds content" --status "Very long status that also exceeds content"

echo
echo -e "${GREEN}✅ COMPREHENSIVE FEATURE TEST COMPLETE${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}🎯 All features tested progressively${NC}"
echo -e "${GREEN}🔍 Any rendering issues should be visible above${NC}"
echo -e "${GREEN}🛡️  Width calculation specifically stress-tested${NC}"
echo