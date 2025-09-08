#!/bin/bash

# CEREMONY 13: Layout Mastery and Advanced Positioning
# Tests --layout with sophisticated positioning tokens (tl,br,cf,hr,fc,etc)
# ADVANCED COMPLEXITY: Multi-component layout coordination with alignment mastery
# EXPECT: Potential integration gaps in advanced layout token processing

# Prepare ceremony environment
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$CEREMONY_DIR/../../.."
SOURCE_DIR="$(pwd)"

echo "=== CEREMONY 13: LAYOUT MASTERY AND ADVANCED POSITIONING ==="
echo "Location: $SOURCE_DIR"
echo "Testing sophisticated layout positioning with advanced token coordination"
echo

# Build fresh for ceremony
echo "Building boxy for layout mastery testing..."
cargo build --quiet

if [ $? -ne 0 ]; then
    echo "❌ Build failed - ceremony cannot proceed"
    exit 1
fi

echo "✅ Build successful"
echo

# STEP 1: Header Alignment Tokens
echo "STEP 1: Header alignment positioning tokens"
echo "Command: echo 'Header Layout Test' | ./target/debug/boxy --layout hr --params \"hd='Right Header'\""
echo "Expected: Header aligned to right position"

echo 'Header Layout Test' | ./target/debug/boxy --layout hr --params "hd='Right Header'"
STEP1_STATUS=$?

echo "Status: $STEP1_STATUS"
if [ $STEP1_STATUS -eq 0 ]; then
    echo "✅ STEP 1 PASSED: Header alignment token working"
else
    echo "⚠️  STEP 1 INTEGRATION GAP: Header layout token incomplete"
fi
echo

# STEP 2: Footer Alignment Coordination
echo "STEP 2: Footer alignment positioning"  
echo "Command: echo 'Footer Layout Test' | ./target/debug/boxy --layout fc --params \"ft='Center Footer'\""
echo "Expected: Footer aligned to center position"

echo 'Footer Layout Test' | ./target/debug/boxy --layout fc --params "ft='Center Footer'"
STEP2_STATUS=$?

echo "Status: $STEP2_STATUS"
if [ $STEP2_STATUS -eq 0 ]; then
    echo "✅ STEP 2 PASSED: Footer alignment token working"
else
    echo "⚠️  STEP 2 INTEGRATION GAP: Footer layout token incomplete"
fi
echo

# STEP 3: Status Bar Positioning
echo "STEP 3: Status bar positioning tokens"
echo "Command: echo 'Status Layout Test' | ./target/debug/boxy --layout sl --params \"st='Left Status'\""
echo "Expected: Status bar aligned to left position"

echo 'Status Layout Test' | ./target/debug/boxy --layout sl --params "st='Left Status'"
STEP3_STATUS=$?

echo "Status: $STEP3_STATUS"
if [ $STEP3_STATUS -eq 0 ]; then
    echo "✅ STEP 3 PASSED: Status positioning token working"
else
    echo "⚠️  STEP 3 INTEGRATION GAP: Status layout token incomplete"
fi
echo

# STEP 4: Body Content Alignment
echo "STEP 4: Body content alignment tokens"
echo "Command: echo 'Body Alignment Test' | ./target/debug/boxy --layout bc"
echo "Expected: Body content center aligned"

echo 'Body Alignment Test' | ./target/debug/boxy --layout bc
STEP4_STATUS=$?

echo "Status: $STEP4_STATUS"
if [ $STEP4_STATUS -eq 0 ]; then
    echo "✅ STEP 4 PASSED: Body alignment token working"
else
    echo "⚠️  STEP 4 INTEGRATION GAP: Body layout token incomplete"
fi
echo

# STEP 5: Combined Layout Token Coordination
echo "STEP 5: Multiple layout tokens coordination"
echo "Command: echo 'Multi Layout Test' | ./target/debug/boxy --layout hr,fc,sl --params \"hd='Right H'; ft='Center F'; st='Left S'\""
echo "Expected: Complex multi-component layout coordination"

echo 'Multi Layout Test' | ./target/debug/boxy --layout hr,fc,sl --params "hd='Right H'; ft='Center F'; st='Left S'"
STEP5_STATUS=$?

echo "Status: $STEP5_STATUS"
if [ $STEP5_STATUS -eq 0 ]; then
    echo "✅ STEP 5 PASSED: Multi-token layout coordination working"
else
    echo "⚠️  STEP 5 INTEGRATION GAP: Multi-token coordination incomplete"
fi
echo

# STEP 6: Layout + Theme Integration
echo "STEP 6: Layout positioning with theme integration"
echo "Command: echo 'Theme Layout Test' | ./target/debug/boxy --theme success --layout hc,br --params \"hd='Success Header'\""
echo "Expected: Success theme with header center and body right alignment"

echo 'Theme Layout Test' | ./target/debug/boxy --theme success --layout hc,br --params "hd='Success Header'"
STEP6_STATUS=$?

echo "Status: $STEP6_STATUS"
if [ $STEP6_STATUS -eq 0 ]; then
    echo "✅ STEP 6 PASSED: Theme + layout integration working"
else
    echo "⚠️  STEP 6 INTEGRATION GAP: Theme layout integration incomplete"
fi
echo

# STEP 7: Complex Layout with Width Constraints
echo "STEP 7: Advanced layout with width constraints"
echo "Command: echo 'Constrained Layout Test' | ./target/debug/boxy --layout hr,fc,bl --width 50 --params \"hd='Header'; ft='Footer'\""
echo "Expected: Complex layout within width constraints"

echo 'Constrained Layout Test' | ./target/debug/boxy --layout hr,fc,bl --width 50 --params "hd='Header'; ft='Footer'"
STEP7_STATUS=$?

echo "Status: $STEP7_STATUS"
if [ $STEP7_STATUS -eq 0 ]; then
    echo "✅ STEP 7 PASSED: Layout + width constraint integration working"
else
    echo "⚠️  STEP 7 INTEGRATION GAP: Layout width integration incomplete"
fi
echo

# STEP 8: Multiline Content with Layout Control
echo "STEP 8: Layout tokens with multiline content"
echo "Command: printf 'Line One\\nLine Two\\nLine Three' | ./target/debug/boxy --layout hc,fc,bc --params \"hd='Multi H'; ft='Multi F'\""
echo "Expected: Layout positioning applied to multiline content"

printf 'Line One\nLine Two\nLine Three' | ./target/debug/boxy --layout hc,fc,bc --params "hd='Multi H'; ft='Multi F'"
STEP8_STATUS=$?

echo "Status: $STEP8_STATUS"
if [ $STEP8_STATUS -eq 0 ]; then
    echo "✅ STEP 8 PASSED: Multiline layout positioning working"
else
    echo "⚠️  STEP 8 INTEGRATION GAP: Multiline layout incomplete"
fi
echo

# Ceremony Results Summary
echo "=== CEREMONY 13 RESULTS SUMMARY ==="
echo "Layout Mastery and Positioning Validation:"
echo "  STEP 1 (Header Alignment): $([ $STEP1_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 2 (Footer Alignment): $([ $STEP2_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 3 (Status Positioning): $([ $STEP3_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 4 (Body Alignment): $([ $STEP4_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 5 (Multi-Token Coordination): $([ $STEP5_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 6 (Theme + Layout): $([ $STEP6_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 7 (Layout + Width): $([ $STEP7_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 8 (Multiline Layout): $([ $STEP8_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"

TOTAL_PASSED=$(( $([ $STEP1_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP2_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP3_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP4_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP5_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP6_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP7_STATUS -eq 0 ] && echo 1 || echo 0) + \
                $([ $STEP8_STATUS -eq 0 ] && echo 1 || echo 0) ))

echo
echo "CEREMONY COMPLETION: $TOTAL_PASSED/8 validations successful"

if [ $TOTAL_PASSED -eq 8 ]; then
    echo "🎯 CEREMONY 13 SUCCESS: Layout mastery fully integrated!"
    exit 0
elif [ $TOTAL_PASSED -ge 4 ]; then
    echo "⚡ CEREMONY 13 PARTIAL: Layout mastery needs integration work"
    exit 1
else
    echo "🔧 CEREMONY 13 INTEGRATION REQUIRED: Layout positioning needs implementation"
    exit 2
fi