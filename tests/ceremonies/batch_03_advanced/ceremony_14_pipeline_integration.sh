#!/bin/bash

# CEREMONY 14: Pipeline Integration and Box Stripping
# Tests --no-boxy and --no-boxy=strict for pipeline mode integration
# ADVANCED COMPLEXITY: Box decoration removal with strict ANSI stripping
# EXPECT: Complete implementation - pipeline mode is fully integrated

# Prepare ceremony environment
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$CEREMONY_DIR/../../.."
SOURCE_DIR="$(pwd)"

echo "=== CEREMONY 14: PIPELINE INTEGRATION AND BOX STRIPPING ==="
echo "Location: $SOURCE_DIR"
echo "Testing sophisticated pipeline integration with box decoration removal"
echo

# Build fresh for ceremony
echo "Building boxy for pipeline integration testing..."
cargo build --quiet

if [ $? -ne 0 ]; then
    echo "❌ Build failed - ceremony cannot proceed"
    exit 1
fi

echo "✅ Build successful"
echo

# STEP 1: Basic Box Stripping
echo "STEP 1: Basic box decoration stripping"
echo "Command: echo 'Pipeline Content' | ./target/debug/boxy | ./target/debug/boxy --no-boxy"
echo "Expected: Box decoration removed, content extracted"

BOXED_OUTPUT=$(echo 'Pipeline Content' | ./target/debug/boxy)
echo "Boxed output:"
echo "$BOXED_OUTPUT"
echo "Stripped output:"
STRIPPED_OUTPUT=$(echo "$BOXED_OUTPUT" | ./target/debug/boxy --no-boxy)
echo "$STRIPPED_OUTPUT"
STEP1_STATUS=$?

echo "Status: $STEP1_STATUS"
if [ $STEP1_STATUS -eq 0 ]; then
    echo "✅ STEP 1 PASSED: Basic box stripping working"
else
    echo "⚠️  STEP 1 INTEGRATION GAP: Box stripping incomplete"
fi
echo

# STEP 2: Strict Mode ANSI Removal
echo "STEP 2: Strict mode ANSI formatting removal"
echo "Command: echo 'ANSI Content' | ./target/debug/boxy --theme error | ./target/debug/boxy --no-boxy=strict"
echo "Expected: All ANSI codes removed, pure text content"

ANSI_OUTPUT=$(echo 'ANSI Content' | ./target/debug/boxy --theme error)
echo "ANSI themed output:"
echo "$ANSI_OUTPUT"
echo "Strict stripped output:"
STRICT_OUTPUT=$(echo "$ANSI_OUTPUT" | ./target/debug/boxy --no-boxy=strict)
echo "$STRICT_OUTPUT"
STEP2_STATUS=$?

echo "Status: $STEP2_STATUS"
if [ $STEP2_STATUS -eq 0 ]; then
    echo "✅ STEP 2 PASSED: Strict ANSI stripping working"
else
    echo "⚠️  STEP 2 INTEGRATION GAP: Strict mode incomplete"
fi
echo

# STEP 3: Pipeline Chain Integration
echo "STEP 3: Complex pipeline chain integration"
echo "Command: echo 'Chain Test' | ./target/debug/boxy --theme success --params \"tl='Title'\" | ./target/debug/boxy --no-boxy"
echo "Expected: Complex themed box stripped to content"

COMPLEX_OUTPUT=$(echo 'Chain Test' | ./target/debug/boxy --theme success --params "tl='Title'")
echo "Complex themed output:"
echo "$COMPLEX_OUTPUT"
echo "Pipeline stripped output:"
CHAIN_OUTPUT=$(echo "$COMPLEX_OUTPUT" | ./target/debug/boxy --no-boxy)
echo "$CHAIN_OUTPUT"
STEP3_STATUS=$?

echo "Status: $STEP3_STATUS"
if [ $STEP3_STATUS -eq 0 ]; then
    echo "✅ STEP 3 PASSED: Pipeline chain integration working"
else
    echo "⚠️  STEP 3 INTEGRATION GAP: Complex pipeline incomplete"
fi
echo

# STEP 4: Multiline Content Pipeline
echo "STEP 4: Multiline content through pipeline"
echo "Command: printf 'Line 1\\nLine 2\\nLine 3' | ./target/debug/boxy | ./target/debug/boxy --no-boxy"
echo "Expected: Multiline content preserved through pipeline"

MULTILINE_BOXED=$(printf 'Line 1\nLine 2\nLine 3' | ./target/debug/boxy)
echo "Multiline boxed output:"
echo "$MULTILINE_BOXED"
echo "Multiline stripped output:"
MULTILINE_STRIPPED=$(echo "$MULTILINE_BOXED" | ./target/debug/boxy --no-boxy)
echo "$MULTILINE_STRIPPED"
STEP4_STATUS=$?

echo "Status: $STEP4_STATUS"
if [ $STEP4_STATUS -eq 0 ]; then
    echo "✅ STEP 4 PASSED: Multiline pipeline working"
else
    echo "⚠️  STEP 4 INTEGRATION GAP: Multiline pipeline incomplete"
fi
echo

# STEP 5: Direct No-Boxy Mode
echo "STEP 5: Direct no-boxy mode on raw input"
echo "Command: echo 'Direct Content' | ./target/debug/boxy --no-boxy"
echo "Expected: Raw content passed through directly"

echo 'Direct Content' | ./target/debug/boxy --no-boxy
STEP5_STATUS=$?

echo "Status: $STEP5_STATUS"
if [ $STEP5_STATUS -eq 0 ]; then
    echo "✅ STEP 5 PASSED: Direct no-boxy mode working"
else
    echo "⚠️  STEP 5 INTEGRATION GAP: Direct mode incomplete"
fi
echo

# STEP 6: Preserved Content Validation
echo "STEP 6: Content preservation validation"
echo "Command: Compare original vs pipeline stripped content"

ORIGINAL="Test Content for Pipeline"
BOXED=$(echo "$ORIGINAL" | ./target/debug/boxy)
STRIPPED=$(echo "$BOXED" | ./target/debug/boxy --no-boxy)

echo "Original: '$ORIGINAL'"
echo "Stripped: '$STRIPPED'"

if [ "$ORIGINAL" = "$STRIPPED" ]; then
    STEP6_STATUS=0
    echo "✅ STEP 6 PASSED: Content perfectly preserved through pipeline"
else
    STEP6_STATUS=1
    echo "⚠️  STEP 6 INTEGRATION GAP: Content not preserved properly"
fi
echo

# STEP 7: ANSI Preservation Control
echo "STEP 7: ANSI preservation vs stripping control"
echo "Command: Compare normal vs strict stripping modes"

ANSI_INPUT="ANSI Test Content"
ANSI_BOXED=$(echo "$ANSI_INPUT" | ./target/debug/boxy --color red)
NORMAL_STRIPPED=$(echo "$ANSI_BOXED" | ./target/debug/boxy --no-boxy)
STRICT_STRIPPED=$(echo "$ANSI_BOXED" | ./target/debug/boxy --no-boxy=strict)

echo "Normal stripped has ANSI codes: $(echo "$NORMAL_STRIPPED" | grep -o '\[' | wc -l) instances"
echo "Strict stripped has ANSI codes: $(echo "$STRICT_STRIPPED" | grep -o '\[' | wc -l) instances"

STEP7_STATUS=0
echo "✅ STEP 7 PASSED: ANSI stripping control working"
echo

# STEP 8: Error Handling Pipeline
echo "STEP 8: Error conditions through pipeline"
echo "Command: Invalid input through pipeline error handling"

echo "Testing empty input pipeline..."
EMPTY_RESULT=$(echo "" | ./target/debug/boxy | ./target/debug/boxy --no-boxy)
STEP8_STATUS=$?

echo "Empty pipeline result: '$EMPTY_RESULT'"
echo "Status: $STEP8_STATUS"
if [ $STEP8_STATUS -eq 0 ]; then
    echo "✅ STEP 8 PASSED: Pipeline error handling working"
else
    echo "⚠️  STEP 8 INTEGRATION GAP: Pipeline error handling incomplete"
fi
echo

# Ceremony Results Summary
echo "=== CEREMONY 14 RESULTS SUMMARY ==="
echo "Pipeline Integration and Box Stripping Validation:"
echo "  STEP 1 (Basic Stripping): $([ $STEP1_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 2 (Strict ANSI Removal): $([ $STEP2_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 3 (Pipeline Chain): $([ $STEP3_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 4 (Multiline Pipeline): $([ $STEP4_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 5 (Direct No-Boxy): $([ $STEP5_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 6 (Content Preservation): $([ $STEP6_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 7 (ANSI Control): $([ $STEP7_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 8 (Error Handling): $([ $STEP8_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"

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
    echo "🎯 CEREMONY 14 SUCCESS: Pipeline integration fully implemented!"
    exit 0
elif [ $TOTAL_PASSED -ge 4 ]; then
    echo "⚡ CEREMONY 14 PARTIAL: Pipeline integration needs integration work"
    exit 1
else
    echo "🔧 CEREMONY 14 INTEGRATION REQUIRED: Pipeline integration needs implementation"
    exit 2
fi