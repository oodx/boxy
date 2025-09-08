#!/bin/bash

# CEREMONY 15: Environment Variable Configuration  
# Tests BOXY_THEME and other environment-driven configuration
# ADVANCED COMPLEXITY: Environment variable integration with CLI override precedence
# EXPECT: Integration gaps - BOXY_THEME documented but not implemented

# Prepare ceremony environment
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$CEREMONY_DIR/../../.."
SOURCE_DIR="$(pwd)"

echo "=== CEREMONY 15: ENVIRONMENT VARIABLE CONFIGURATION ==="
echo "Location: $SOURCE_DIR"
echo "Testing environment-driven configuration with CLI override precedence"
echo

# Build fresh for ceremony
echo "Building boxy for environment testing..."
cargo build --quiet

if [ $? -ne 0 ]; then
    echo "❌ Build failed - ceremony cannot proceed"
    exit 1
fi

echo "✅ Build successful"
echo

# STEP 1: BOXY_THEME Environment Variable
echo "STEP 1: BOXY_THEME environment variable"
echo "Command: BOXY_THEME=error echo 'Theme Env Test' | ./target/debug/boxy"
echo "Expected: Error theme applied via environment variable"

export BOXY_THEME=error
echo 'Theme Env Test' | ./target/debug/boxy
STEP1_STATUS=$?

echo "Status: $STEP1_STATUS"
# Check if output shows error theme (red/crimson colors)
OUTPUT1=$(echo 'Theme Env Test' | ./target/debug/boxy 2>&1)
if echo "$OUTPUT1" | grep -q "38;5;196"; then
    echo "✅ STEP 1 PASSED: BOXY_THEME environment variable working"
else
    echo "⚠️  STEP 1 INTEGRATION GAP: BOXY_THEME environment variable not implemented"
    STEP1_STATUS=1
fi
echo

# STEP 2: Environment vs CLI Precedence
echo "STEP 2: Environment variable vs CLI argument precedence"
echo "Command: BOXY_THEME=error echo 'Precedence Test' | ./target/debug/boxy --theme success"
echo "Expected: Success theme overrides environment (CLI precedence)"

export BOXY_THEME=error
echo 'Precedence Test' | ./target/debug/boxy --theme success
STEP2_STATUS=$?

echo "Status: $STEP2_STATUS"
# Check if output shows success theme (green colors) not error theme
OUTPUT2=$(echo 'Precedence Test' | ./target/debug/boxy --theme success 2>&1)
if echo "$OUTPUT2" | grep -q "38;5;34" && ! echo "$OUTPUT2" | grep -q "38;5;196"; then
    echo "✅ STEP 2 PASSED: CLI argument precedence over environment working"
else
    echo "⚠️  STEP 2 INTEGRATION GAP: Environment precedence handling incomplete"
    STEP2_STATUS=1
fi
echo

# STEP 3: Environment Reset Testing
echo "STEP 3: Environment variable reset behavior"
echo "Command: unset BOXY_THEME; echo 'Reset Test' | ./target/debug/boxy"
echo "Expected: Default theme when environment variable unset"

unset BOXY_THEME
echo 'Reset Test' | ./target/debug/boxy
STEP3_STATUS=$?

echo "Status: $STEP3_STATUS"
# Should show default theme (no special coloring)
OUTPUT3=$(echo 'Reset Test' | ./target/debug/boxy 2>&1)
if ! echo "$OUTPUT3" | grep -q "38;5;"; then
    echo "✅ STEP 3 PASSED: Environment reset to default working"
else
    echo "⚠️  STEP 3 INTEGRATION GAP: Environment reset behavior incomplete"
fi
echo

# STEP 4: Multiple Environment Themes
echo "STEP 4: Different environment theme values"
echo "Command: BOXY_THEME=warning echo 'Warning Env Test' | ./target/debug/boxy"
echo "Expected: Warning theme applied via environment"

export BOXY_THEME=warning
echo 'Warning Env Test' | ./target/debug/boxy
STEP4_STATUS=$?

echo "Status: $STEP4_STATUS"
# Check for warning theme (amber colors)
OUTPUT4=$(echo 'Warning Env Test' | ./target/debug/boxy 2>&1)
if echo "$OUTPUT4" | grep -q "38;5;220"; then
    echo "✅ STEP 4 PASSED: Warning theme environment working"
else
    echo "⚠️  STEP 4 INTEGRATION GAP: Warning environment theme incomplete"
    STEP4_STATUS=1
fi
echo

# STEP 5: Invalid Environment Theme
echo "STEP 5: Invalid environment theme handling"
echo "Command: BOXY_THEME=nonexistent echo 'Invalid Env Test' | ./target/debug/boxy"
echo "Expected: Graceful fallback to default theme"

export BOXY_THEME=nonexistent
echo 'Invalid Env Test' | ./target/debug/boxy
STEP5_STATUS=$?

echo "Status: $STEP5_STATUS"
# Should not crash, should use default theme
if [ $STEP5_STATUS -eq 0 ]; then
    echo "✅ STEP 5 PASSED: Invalid environment theme handled gracefully"
else
    echo "⚠️  STEP 5 INTEGRATION GAP: Invalid environment theme error handling incomplete"
fi
echo

# STEP 6: Environment + Parameter Streams  
echo "STEP 6: Environment theme with parameter streams"
echo "Command: BOXY_THEME=success echo 'Env Stream Test' | ./target/debug/boxy --params \"tl='Env Title'\""
echo "Expected: Environment theme + parameter stream integration"

export BOXY_THEME=success
echo 'Env Stream Test' | ./target/debug/boxy --params "tl='Env Title'"
STEP6_STATUS=$?

echo "Status: $STEP6_STATUS"
# Check for success theme with title parameter
OUTPUT6=$(echo 'Env Stream Test' | ./target/debug/boxy --params "tl='Env Title'" 2>&1)
if echo "$OUTPUT6" | grep -q "38;5;34" && echo "$OUTPUT6" | grep -q "Env Title"; then
    echo "✅ STEP 6 PASSED: Environment + parameter stream integration working"
else
    echo "⚠️  STEP 6 INTEGRATION GAP: Environment parameter integration incomplete"
    STEP6_STATUS=1
fi
echo

# STEP 7: Environment Documentation Validation
echo "STEP 7: Environment variable documentation validation"
echo "Command: ./target/debug/boxy --help | grep BOXY_THEME"
echo "Expected: BOXY_THEME documented in help output"

HELP_OUTPUT=$(./target/debug/boxy --help | grep BOXY_THEME)
if [ -n "$HELP_OUTPUT" ]; then
    STEP7_STATUS=0
    echo "Documentation found: $HELP_OUTPUT"
    echo "✅ STEP 7 PASSED: BOXY_THEME documented in help"
else
    STEP7_STATUS=1
    echo "⚠️  STEP 7 INTEGRATION GAP: BOXY_THEME documentation missing"
fi
echo

# STEP 8: Environment Persistence Testing
echo "STEP 8: Environment variable persistence across operations"
echo "Command: BOXY_THEME=info with multiple operations"
echo "Expected: Theme persists across multiple boxy operations"

export BOXY_THEME=info
FIRST_OUTPUT=$(echo 'First Operation' | ./target/debug/boxy 2>&1)
SECOND_OUTPUT=$(echo 'Second Operation' | ./target/debug/boxy 2>&1)
STEP8_STATUS=0

echo "First operation uses info theme: $(echo "$FIRST_OUTPUT" | grep -q "38;5;33" && echo "YES" || echo "NO")"
echo "Second operation uses info theme: $(echo "$SECOND_OUTPUT" | grep -q "38;5;33" && echo "YES" || echo "NO")"

if echo "$FIRST_OUTPUT" | grep -q "38;5;33" && echo "$SECOND_OUTPUT" | grep -q "38;5;33"; then
    echo "✅ STEP 8 PASSED: Environment persistence working"
else
    echo "⚠️  STEP 8 INTEGRATION GAP: Environment persistence incomplete"
    STEP8_STATUS=1
fi
echo

# Clean up environment
unset BOXY_THEME

# Ceremony Results Summary
echo "=== CEREMONY 15 RESULTS SUMMARY ==="
echo "Environment Variable Configuration Validation:"
echo "  STEP 1 (BOXY_THEME Basic): $([ $STEP1_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 2 (CLI Precedence): $([ $STEP2_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 3 (Environment Reset): $([ $STEP3_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 4 (Warning Theme Env): $([ $STEP4_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 5 (Invalid Theme Handling): $([ $STEP5_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 6 (Env + Parameter Stream): $([ $STEP6_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 7 (Documentation): $([ $STEP7_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"
echo "  STEP 8 (Environment Persistence): $([ $STEP8_STATUS -eq 0 ] && echo "✅ PASSED" || echo "⚠️ INTEGRATION GAP")"

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
    echo "🎯 CEREMONY 15 SUCCESS: Environment configuration fully implemented!"
    exit 0
elif [ $TOTAL_PASSED -ge 4 ]; then
    echo "⚡ CEREMONY 15 PARTIAL: Environment configuration needs integration work"
    exit 1
else
    echo "🔧 CEREMONY 15 INTEGRATION REQUIRED: Environment configuration needs implementation"
    exit 2
fi