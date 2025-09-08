#!/bin/bash

# CEREMONY 12: Theme Composition and Color Overrides
# Tests --theme error/success/info/warning with granular color override capabilities  
# ADVANCED COMPLEXITY: Theme inheritance + color composition + override precedence
# EXPECT: Potential integration gaps in theme color override application

# Prepare ceremony environment
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$CEREMONY_DIR/../../.."
SOURCE_DIR="$(pwd)"

echo "=== CEREMONY 12: THEME COMPOSITION AND COLOR OVERRIDES ==="
echo "Location: $SOURCE_DIR"
echo "Testing advanced theme composition with sophisticated color override capabilities"
echo

# Build fresh for ceremony
echo "Building boxy for theme composition testing..."
cargo build --quiet

if [ $? -ne 0 ]; then
    echo "❌ Build failed - ceremony cannot proceed"
    exit 1
fi

echo "✅ Build successful"
echo

# STEP 1: Built-in Theme Validation
echo "STEP 1: Built-in semantic theme validation"
echo "Command: echo 'Error Theme Test' | ./target/debug/boxy --theme error"
echo "Expected: Error theme applied with crimson colors and heavy borders"

echo 'Error Theme Test' | ./target/debug/boxy --theme error
STEP1_STATUS=$?

echo "Status: $STEP1_STATUS"
if [ $STEP1_STATUS -eq 0 ]; then
    echo "✅ STEP 1 PASSED: Built-in error theme working"
else
    echo "⚠️  STEP 1 INTEGRATION GAP: Built-in theme loading incomplete"
fi
echo

# STEP 2: Success Theme with Contrast
echo "STEP 2: Success theme validation"
echo "Command: echo 'Success Theme Test' | ./target/debug/boxy --theme success"
echo "Expected: Success theme with emerald colors and rounded borders"

echo 'Success Theme Test' | ./target/debug/boxy --theme success
STEP2_STATUS=$?

echo "Status: $STEP2_STATUS"
if [ $STEP2_STATUS -eq 0 ]; then
    echo "✅ STEP 2 PASSED: Built-in success theme working"
else
    echo "⚠️  STEP 2 INTEGRATION GAP: Success theme loading incomplete"
fi
echo

# STEP 3: Theme + Color Override Composition
echo "STEP 3: Theme with color override composition"
echo "Command: echo 'Override Test' | ./target/debug/boxy --theme error --color blue"
echo "Expected: Error theme structure with blue color override applied"

echo 'Override Test' | ./target/debug/boxy --theme error --color blue
STEP3_STATUS=$?

echo "Status: $STEP3_STATUS"
if [ $STEP3_STATUS -eq 0 ]; then
    echo "✅ STEP 3 PASSED: Theme + color override composition working"
else
    echo "⚠️  STEP 3 INTEGRATION GAP: Color override integration incomplete"
fi
echo

# STEP 4: Warning Theme with Width Integration
echo "STEP 4: Warning theme with width constraint"
echo "Command: echo 'Warning Width Test' | ./target/debug/boxy --theme warning --width 40"
echo "Expected: Warning theme with amber colors constrained to 40 characters"

echo 'Warning Width Test' | ./target/debug/boxy --theme warning --width 40
STEP4_STATUS=$?

echo "Status: $STEP4_STATUS"
if [ $STEP4_STATUS -eq 0 ]; then
    echo "✅ STEP 4 PASSED: Theme + width integration working"
else
    echo "⚠️  STEP 4 INTEGRATION GAP: Theme width integration incomplete"
fi
echo

# STEP 5: Info Theme with Layout Composition
echo "STEP 5: Info theme with layout positioning"
echo "Command: echo 'Info Layout Test' | ./target/debug/boxy --theme info --layout hc"
echo "Expected: Info theme with azure colors and header center alignment"

echo 'Info Layout Test' | ./target/debug/boxy --theme info --layout hc
STEP5_STATUS=$?

echo "Status: $STEP5_STATUS"  
if [ $STEP5_STATUS -eq 0 ]; then
    echo "✅ STEP 5 PASSED: Theme + layout composition working"
else
    echo "⚠️  STEP 5 INTEGRATION GAP: Theme layout integration incomplete"
fi
echo

# STEP 6: Complex Theme Composition
echo "STEP 6: Complex multi-parameter theme composition"
echo "Command: echo 'Complex Theme Test' | ./target/debug/boxy --theme success --color cyan --width 50 --layout fc"
echo "Expected: Success theme + cyan override + width + footer center layout"

echo 'Complex Theme Test' | ./target/debug/boxy --theme success --color cyan --width 50 --layout fc
STEP6_STATUS=$?

echo "Status: $STEP6_STATUS"
if [ $STEP6_STATUS -eq 0 ]; then
    echo "✅ STEP 6 PASSED: Complex theme composition working"
else
    echo "⚠️  STEP 6 INTEGRATION GAP: Complex composition incomplete"
fi
echo

# STEP 7: Theme + Parameter Stream Integration
echo "STEP 7: Theme with parameter stream overrides"
echo "Command: echo 'Theme Stream Test' | ./target/debug/boxy --theme warning --params \"tl='Stream Title'; ic='🔥'\""
echo "Expected: Warning theme + parameter stream title and icon integration"

echo 'Theme Stream Test' | ./target/debug/boxy --theme warning --params "tl='Stream Title'; ic='🔥'"
STEP7_STATUS=$?

echo "Status: $STEP7_STATUS"
if [ $STEP7_STATUS -eq 0 ]; then
    echo "✅ STEP 7 PASSED: Theme + parameter stream integration working"
else
    echo "⚠️  STEP 7 INTEGRATION GAP: Theme parameter stream integration incomplete"
fi
echo

# STEP 8: Invalid Theme Handling
echo "STEP 8: Invalid theme graceful handling"
echo "Command: echo 'Invalid Theme Test' | ./target/debug/boxy --theme nonexistent_theme"
echo "Expected: Graceful fallback or error message for invalid theme"

echo 'Invalid Theme Test' | ./target/debug/boxy --theme nonexistent_theme
STEP8_STATUS=$?

echo "Status: $STEP8_STATUS"
if [ $STEP8_STATUS -ne 0 ]; then
    echo "✅ STEP 8 PASSED: Invalid theme handling working (expected failure)"
else
    echo "⚠️  STEP 8 INTEGRATION GAP: Invalid theme validation incomplete"
fi
echo

# Ceremony Results Summary
echo "=== CEREMONY 12 RESULTS SUMMARY ==="
echo "Theme Composition and Color Override Validation:"
echo "  STEP 1 (Error Theme): $([ $STEP1_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 2 (Success Theme): $([ $STEP2_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 3 (Theme + Color Override): $([ $STEP3_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 4 (Theme + Width): $([ $STEP4_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 5 (Theme + Layout): $([ $STEP5_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 6 (Complex Composition): $([ $STEP6_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 7 (Theme + Parameter Stream): $([ $STEP7_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 8 (Invalid Theme Handling): $([ $STEP8_STATUS -ne 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"

TOTAL_PASSED=$(( $([ $STEP1_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP2_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP3_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP4_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP5_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP6_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP7_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP8_STATUS -ne 0 ] && echo 1 || echo 0) ))

echo
echo "CEREMONY COMPLETION: $TOTAL_PASSED/8 validations successful"

if [ $TOTAL_PASSED -eq 8 ]; then
    echo "🎯 CEREMONY 12 SUCCESS: Theme composition fully integrated!"
    exit 0
elif [ $TOTAL_PASSED -ge 4 ]; then
    echo "⚡ CEREMONY 12 PARTIAL: Theme composition needs integration work"
    exit 1
else
    echo "🔧 CEREMONY 12 INTEGRATION REQUIRED: Theme composition needs implementation"
    exit 2
fi