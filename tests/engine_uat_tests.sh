#!/bin/bash

# ENGINE UAT (User Acceptance Testing) Suite
# Tests all ENGINE features: init, import, export, validate, debug, list, dry-run

set -e

BOLD="\033[1m"
GREEN="\033[0;32m"
RED="\033[0;31m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
RESET="\033[0m"

TEST_DIR="/tmp/boxy_engine_uat_$$"
BOXY_BIN="${BOXY_BIN:-cargo run --bin boxy --}"

echo -e "${BOLD}${BLUE}════════════════════════════════════════════════════${RESET}"
echo -e "${BOLD}       BOXY ENGINE UAT TEST SUITE${RESET}"
echo -e "${BOLD}${BLUE}════════════════════════════════════════════════════${RESET}"
echo ""

# Setup test environment
setup_test_env() {
    echo -e "${YELLOW}Setting up test environment...${RESET}"
    mkdir -p "$TEST_DIR"
    cd "$TEST_DIR"

    # Create test theme files
    cat > boxy_test_theme.yml << 'EOF'
name: "Test Theme Collection"
version: "1.0.0"
author: "UAT Test Suite"
description: "Test theme for UAT validation"

themes:
  test_success:
    description: "Test success theme"
    color: "green"
    style: "rounded"
    icon: "✅"

  test_error:
    description: "Test error theme"
    color: "red"
    style: "double"
    icon: "❌"
EOF

    cat > boxy_invalid.yml << 'EOF'
name: "Invalid Theme"
# Missing version and themes
EOF

    echo -e "${GREEN}✓ Test environment ready${RESET}"
    echo ""
}

# Cleanup function
cleanup() {
    cd /
    rm -rf "$TEST_DIR"
    # Clean up any test themes from global directory
    rm -f ~/.local/etc/odx/boxy/themes/boxy_test_*.yml
}

trap cleanup EXIT

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Test function wrapper
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_pattern="$3"

    echo -e "${BOLD}Testing: $test_name${RESET}"

    if eval "$test_command" 2>&1 | grep -q "$expected_pattern"; then
        echo -e "${GREEN}✅ PASS${RESET}: $test_name"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${RESET}: $test_name"
        echo "  Expected pattern: $expected_pattern"
        echo "  Command: $test_command"
        ((TESTS_FAILED++))
    fi
    echo ""
}

# Test function for commands that should fail
run_fail_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_pattern="$3"

    echo -e "${BOLD}Testing (should fail): $test_name${RESET}"

    if eval "$test_command" 2>&1 | grep -q "$expected_pattern"; then
        echo -e "${GREEN}✅ PASS${RESET}: $test_name (correctly failed)"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${RESET}: $test_name (should have shown error)"
        echo "  Expected error pattern: $expected_pattern"
        ((TESTS_FAILED++))
    fi
    echo ""
}

# Setup
setup_test_env

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}1. ENGINE INIT TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

run_test "ENGINE init creates directory" \
    "$BOXY_BIN engine init" \
    "Created directory"

run_test "ENGINE init is idempotent" \
    "$BOXY_BIN engine init" \
    "already exists"

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}2. ENGINE VALIDATION TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

run_test "Validate valid theme file" \
    "$BOXY_BIN engine validate boxy_test_theme.yml" \
    "Validation passed"

run_fail_test "Validate invalid theme file" \
    "$BOXY_BIN engine validate boxy_invalid.yml" \
    "Validation errors"

run_fail_test "Validate non-existent file" \
    "$BOXY_BIN engine validate boxy_nonexistent.yml" \
    "not found"

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}3. ENGINE IMPORT TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

run_test "Import theme file" \
    "$BOXY_BIN engine import test_theme" \
    "Successfully imported"

run_test "Import with overwrite protection" \
    "$BOXY_BIN engine import test_theme" \
    "already exists"

run_test "Import with --overwrite flag" \
    "$BOXY_BIN engine import test_theme --overwrite" \
    "Backup created"

run_fail_test "Import non-existent file" \
    "$BOXY_BIN engine import nonexistent" \
    "not found"

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}4. ENGINE EXPORT TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

# First, ensure we have something to export
$BOXY_BIN engine import test_theme --overwrite >/dev/null 2>&1 || true

run_test "Export theme file" \
    "rm -f boxy_test_theme.yml && $BOXY_BIN engine export test_theme" \
    "Successfully exported"

run_test "Export with overwrite protection" \
    "$BOXY_BIN engine export test_theme" \
    "already exists"

run_test "Export with --overwrite flag" \
    "$BOXY_BIN engine export test_theme --overwrite" \
    "Backup created"

run_fail_test "Export non-existent theme" \
    "$BOXY_BIN engine export nonexistent_theme" \
    "not found"

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}5. ENGINE DRY-RUN TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

run_test "Dry-run import" \
    "$BOXY_BIN engine import test_theme --dry-run" \
    "DRY RUN"

run_test "Dry-run export" \
    "$BOXY_BIN engine export test_theme --dry-run" \
    "DRY RUN"

run_test "Dry-run shows preview" \
    "$BOXY_BIN engine import test_theme --dry-run" \
    "No changes will be made"

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}6. ENGINE LIST/DEBUG TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

run_test "ENGINE list shows themes" \
    "$BOXY_BIN engine list" \
    "ENGINE THEME CATALOG"

run_test "ENGINE debug shows hierarchy" \
    "$BOXY_BIN engine debug" \
    "Loading Hierarchy"

run_test "ENGINE status shows system info" \
    "$BOXY_BIN engine status" \
    "System Status"

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}7. ENGINE HELP TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

run_test "ENGINE help shows all commands" \
    "$BOXY_BIN engine help" \
    "import.*export.*validate"

run_test "ENGINE help shows examples" \
    "$BOXY_BIN engine help" \
    "Examples"

echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}8. INTEGRATION TESTS${RESET}"
echo -e "${BOLD}${BLUE}══════════════════════════════════════════${RESET}"
echo ""

# Full workflow test
echo -e "${BOLD}Running full workflow test...${RESET}"

# Create a new theme file
cat > boxy_workflow_test.yml << 'EOF'
name: "Workflow Test"
version: "1.0.0"
author: "UAT"
description: "Full workflow test"

themes:
  workflow_theme:
    description: "Test theme"
    color: "blue"
    style: "sharp"
    icon: "🔵"
EOF

# Validate it
if $BOXY_BIN engine validate boxy_workflow_test.yml 2>&1 | grep -q "Validation passed"; then
    echo -e "${GREEN}✓ Validation step passed${RESET}"
else
    echo -e "${RED}✗ Validation step failed${RESET}"
    ((TESTS_FAILED++))
fi

# Dry-run import
if $BOXY_BIN engine import workflow_test --dry-run 2>&1 | grep -q "DRY RUN"; then
    echo -e "${GREEN}✓ Dry-run import step passed${RESET}"
else
    echo -e "${RED}✗ Dry-run import step failed${RESET}"
    ((TESTS_FAILED++))
fi

# Actual import
if $BOXY_BIN engine import workflow_test 2>&1 | grep -q "Successfully imported"; then
    echo -e "${GREEN}✓ Import step passed${RESET}"
else
    echo -e "${RED}✗ Import step failed${RESET}"
    ((TESTS_FAILED++))
fi

# Verify it appears in list
if $BOXY_BIN engine list 2>&1 | grep -q "workflow_test"; then
    echo -e "${GREEN}✓ Theme appears in list${RESET}"
else
    echo -e "${RED}✗ Theme missing from list${RESET}"
    ((TESTS_FAILED++))
fi

# Export it back
rm -f boxy_workflow_test.yml
if $BOXY_BIN engine export workflow_test 2>&1 | grep -q "Successfully exported"; then
    echo -e "${GREEN}✓ Export step passed${RESET}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Export step failed${RESET}"
    ((TESTS_FAILED++))
fi

echo ""

# Summary
echo -e "${BOLD}${BLUE}════════════════════════════════════════════════════${RESET}"
echo -e "${BOLD}                    TEST SUMMARY${RESET}"
echo -e "${BOLD}${BLUE}════════════════════════════════════════════════════${RESET}"
echo ""
echo -e "${GREEN}Passed: $TESTS_PASSED${RESET}"
echo -e "${RED}Failed: $TESTS_FAILED${RESET}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${BOLD}${GREEN}✅ ALL TESTS PASSED!${RESET}"
    exit 0
else
    echo -e "${BOLD}${RED}❌ SOME TESTS FAILED${RESET}"
    exit 1
fi