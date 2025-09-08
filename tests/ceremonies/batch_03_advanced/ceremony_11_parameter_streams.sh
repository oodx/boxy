#!/bin/bash

# CEREMONY 11: Parameter Streams Processing
# Tests --params "key=value,key=value" streaming for dynamic configuration
# ADVANCED COMPLEXITY: Integration-level parameter processing with validation
# EXPECT: Potential integration gaps in parameter parsing->application flow

# Prepare ceremony environment
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$CEREMONY_DIR/../../.."
SOURCE_DIR="$(pwd)"

echo "=== CEREMONY 11: PARAMETER STREAMS PROCESSING ==="
echo "Location: $SOURCE_DIR"
echo "Testing advanced parameter stream parsing and application"
echo

# Build fresh for ceremony
echo "Building boxy for parameter stream testing..."
cargo build --quiet

if [ $? -ne 0 ]; then
    echo "❌ Build failed - ceremony cannot proceed"
    exit 1
fi

echo "✅ Build successful"
echo

# STEP 1: Basic Parameter Stream Processing
echo "STEP 1: Basic parameter stream with header"
echo "Command: echo 'Parameter Test' | ./target/debug/boxy --params \"hd='Header Text'\""
echo "Expected: Parameter parsing success, header application"

echo 'Parameter Test' | ./target/debug/boxy --params "hd='Header Text'"
STEP1_STATUS=$?

echo "Status: $STEP1_STATUS"
if [ $STEP1_STATUS -eq 0 ]; then
    echo "✅ STEP 1 PASSED: Basic parameter stream processed"
else
    echo "⚠️  STEP 1 INTEGRATION GAP: Parameter stream processing incomplete"
fi
echo

# STEP 2: Multiple Parameter Stream Processing  
echo "STEP 2: Multiple parameters in stream"
echo "Command: echo 'Multi Param Test' | ./target/debug/boxy --params \"hd='Header'; ft='Footer'\""
echo "Expected: Multiple parameter parsing and coordinated application"

echo 'Multi Param Test' | ./target/debug/boxy --params "hd='Header'; ft='Footer'"
STEP2_STATUS=$?

echo "Status: $STEP2_STATUS"
if [ $STEP2_STATUS -eq 0 ]; then
    echo "✅ STEP 2 PASSED: Multiple parameter stream coordination"
else
    echo "⚠️  STEP 2 INTEGRATION GAP: Multiple parameter processing incomplete"
fi
echo

# STEP 3: Theme Override via Parameters
echo "STEP 3: Theme parameter overrides with title"
echo "Command: echo 'Theme Override Test' | ./target/debug/boxy --theme error --params \"tl='Custom Title'; ic='⚡'\""
echo "Expected: Error theme with title and icon parameters applied"

echo 'Theme Override Test' | ./target/debug/boxy --theme error --params "tl='Custom Title'; ic='⚡'"
STEP3_STATUS=$?

echo "Status: $STEP3_STATUS"
if [ $STEP3_STATUS -eq 0 ]; then
    echo "✅ STEP 3 PASSED: Theme parameter override integration"
else
    echo "⚠️  STEP 3 INTEGRATION GAP: Theme-parameter override incomplete"
fi
echo

# STEP 4: Status Bar Parameter Integration
echo "STEP 4: Status bar parameters via stream"
echo "Command: echo 'Status Stream Test' | ./target/debug/boxy --params \"st='Status Bar'; sc='cyan'\""
echo "Expected: Status bar parameters processed and applied"

echo 'Status Stream Test' | ./target/debug/boxy --params "st='Status Bar'; sc='cyan'"
STEP4_STATUS=$?

echo "Status: $STEP4_STATUS"
if [ $STEP4_STATUS -eq 0 ]; then
    echo "✅ STEP 4 PASSED: Layout parameter stream integration"
else
    echo "⚠️  STEP 4 INTEGRATION GAP: Layout parameter processing incomplete"
fi
echo

# STEP 5: Parameter Validation Testing
echo "STEP 5: Invalid parameter key handling"
echo "Command: echo 'Validation Test' | ./target/debug/boxy --params \"invalid_key='value'; hd='Valid Header'\""
echo "Expected: Valid parameters applied, invalid keys ignored"

echo 'Validation Test' | ./target/debug/boxy --params "invalid_key='value'; hd='Valid Header'"
STEP5_STATUS=$?

echo "Status: $STEP5_STATUS"
if [ $STEP5_STATUS -ne 0 ]; then
    echo "✅ STEP 5 PASSED: Parameter validation working (expected failure)"
else
    echo "⚠️  STEP 5 INTEGRATION GAP: Parameter validation needs improvement"
fi
echo

# STEP 6: Complex Parameter Stream Composition
echo "STEP 6: Complex parameter stream with quoted values and Unicode"
echo "Command: echo 'Complex Stream Test' | ./target/debug/boxy --params \"tl='Complex \"Title\"; ic='⚡'; st='Status \"Info\"'\""
echo "Expected: Complex parameter parsing with nested quotes and Unicode"

echo 'Complex Stream Test' | ./target/debug/boxy --params "tl='Complex \"Title\"'; ic='⚡'; st='Status \"Info\"'"
STEP6_STATUS=$?

echo "Status: $STEP6_STATUS"
if [ $STEP6_STATUS -eq 0 ]; then
    echo "✅ STEP 6 PASSED: Complex parameter stream composition"
else
    echo "⚠️  STEP 6 INTEGRATION GAP: Complex parameter parsing incomplete"
fi
echo

# STEP 7: Complete Parameter Stream Testing
echo "STEP 7: Full parameter stream with all components"
echo "Command: echo 'Complete Test' | ./target/debug/boxy --params \"hd='Header'; ft='Footer'; tl='Title'; st='Status'; ic='📦'\""
echo "Expected: All parameter components applied systematically"

echo 'Complete Test' | ./target/debug/boxy --params "hd='Header'; ft='Footer'; tl='Title'; st='Status'; ic='📦'"
STEP7_STATUS=$?

echo "Status: $STEP7_STATUS"
if [ $STEP7_STATUS -eq 0 ]; then
    echo "✅ STEP 7 PASSED: Parameter priority handling"
else
    echo "⚠️  STEP 7 INTEGRATION GAP: Parameter precedence incomplete"
fi
echo

# STEP 8: Multi-line Parameter Stream Impact
echo "STEP 8: Parameter streams with multiline content"
echo "Command: printf 'Line One\\nLine Two\\nLine Three' | ./target/debug/boxy --params \"hd='Multi Header'; ft='Multi Footer'\""
echo "Expected: Parameter application across multiline content"

printf 'Line One\nLine Two\nLine Three' | ./target/debug/boxy --params "hd='Multi Header'; ft='Multi Footer'"
STEP8_STATUS=$?

echo "Status: $STEP8_STATUS"
if [ $STEP8_STATUS -eq 0 ]; then
    echo "✅ STEP 8 PASSED: Multiline parameter stream integration"
else
    echo "⚠️  STEP 8 INTEGRATION GAP: Multiline parameter processing incomplete"
fi
echo

# Ceremony Results Summary
echo "=== CEREMONY 11 RESULTS SUMMARY ==="
echo "Parameter Streams Processing Validation:"
echo "  STEP 1 (Basic Header Stream): $([ $STEP1_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 2 (Multiple Streams): $([ $STEP2_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 3 (Theme + Parameters): $([ $STEP3_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 4 (Status Parameters): $([ $STEP4_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 5 (Invalid Key Handling): $([ $STEP5_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 6 (Complex Parsing): $([ $STEP6_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 7 (Complete Stream): $([ $STEP7_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 8 (Multiline Content): $([ $STEP8_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"

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
    echo "🎯 CEREMONY 11 SUCCESS: Parameter streams fully integrated!"
    exit 0
elif [ $TOTAL_PASSED -ge 4 ]; then
    echo "⚡ CEREMONY 11 PARTIAL: Parameter streams need integration work"
    exit 1
else
    echo "🔧 CEREMONY 11 INTEGRATION REQUIRED: Parameter streams need implementation"
    exit 2
fi