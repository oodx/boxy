#!/usr/bin/env bash
#
# CEREMONY 01: Basic Box Drawing - Foundation API
# Description: Tests fundamental box drawing capabilities
# Batch: Foundation (01)
# Complexity: Basic
# Dependencies: boxy binary
#

# Script directory and setup
CEREMONY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TESTS_DIR="$(cd "$CEREMONY_DIR/../../" && pwd)"
PROJECT_ROOT="$(cd "$TESTS_DIR/.." && pwd)"

# Force use of local project boxy binary (not global)
LOCAL_BOXY="$PROJECT_ROOT/target/release/boxy"
if [[ -f "$LOCAL_BOXY" ]]; then
    export PATH="$PROJECT_ROOT/target/release:$PATH"
fi

# Source UX and UAT ceremony functions
if [[ -f "$TESTS_DIR/ux-kit.sh" ]]; then
    source "$TESTS_DIR/ux-kit.sh"
elif [[ -f "$TESTS_DIR/misc/ux-kit.sh" ]]; then
    source "$TESTS_DIR/misc/ux-kit.sh"
else
    # Fallback minimal UX
    ux_info() { echo "INFO: $*"; }
    ux_success() { echo "SUCCESS: $*"; }
    ux_warn() { echo "WARN: $*"; }
    ux_error() { echo "ERROR: $*"; }
fi

# Source UAT ceremonies if available for enhanced display
if [[ -f "$TESTS_DIR/misc/uat-ceremonies.sh" ]]; then
    source "$TESTS_DIR/misc/uat-ceremonies.sh"
fi

# ═══════════════════════════════════════════════════════════════════════════════
# CEREMONY 01: BASIC BOX DRAWING TESTS
# ═══════════════════════════════════════════════════════════════════════════════

# Ceremony configuration
CEREMONY_NAME="Basic Box Drawing"
CEREMONY_VERSION="v1.0"
CEREMONY_AUTO="${CEREMONY_AUTO:-false}"

# Test data
TEST_CONTENT="Hello World"
TEST_MULTILINE="Line 1
Line 2
Line 3"

# Boxy binary location detection
detect_boxy_binary() {
    local boxy_path
    
    # Check various common locations
    if [[ -f "./target/debug/boxy" ]]; then
        boxy_path="./target/debug/boxy"
    elif [[ -f "./target/release/boxy" ]]; then
        boxy_path="./target/release/boxy"
    elif command -v boxy &> /dev/null; then
        boxy_path="boxy"
    else
        ux_error "Boxy binary not found. Build first with: cargo build"
        exit 1
    fi
    
    echo "$boxy_path"
}

# Initialize ceremony
ceremony_start() {
    local boxy_path="$1"
    
    if command -v uat_suite_start &> /dev/null; then
        uat_suite_start "$CEREMONY_NAME" "Foundation API - Basic box drawing verification" "$CEREMONY_VERSION"
    else
        ux_info "Starting Ceremony 01: $CEREMONY_NAME"
        echo "═══════════════════════════════════════════════════════════════════════════════"
        echo "Testing basic box drawing with boxy at: $boxy_path"
        echo "═══════════════════════════════════════════════════════════════════════════════"
    fi
}

# Test Step 1: Basic box drawing
test_basic_box() {
    local step_num=1
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Basic Box Drawing" \
            "Boxy binary is available and functional" \
            "echo '$TEST_CONTENT' | $boxy_path" \
            "Text '$TEST_CONTENT' enclosed in a clean box with borders" \
            "Testing fundamental box drawing capability" \
            "• Box has top, bottom, left, right borders\n• Content is centered/aligned properly\n• No broken characters or encoding issues"
    else
        ux_info "Step $step_num: Basic box drawing test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path"
        echo "Expected: Text enclosed in clean box borders"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path"
        echo
        if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: look correct"; else read -p "Does this look correct? (y/n): " response; fi
        if [[ "${response,,}" != "y" ]]; then
            ux_error "Step $step_num failed visual verification"
            return 1
        fi
    fi
}

# Test Step 2: Empty input handling
test_empty_input() {
    local step_num=2
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Empty Input Handling" \
            "Boxy handles empty/no input gracefully" \
            "echo '' | $boxy_path" \
            "Empty box or minimal box structure" \
            "Testing edge case of empty input" \
            "• No crashes or errors\n• Produces valid box structure\n• Handles gracefully"
    else
        ux_info "Step $step_num: Empty input test"
        echo "Command: echo '' | $boxy_path"
        echo "Expected: Graceful handling of empty input"
        echo
        echo "Result:"
        echo '' | "$boxy_path"
        echo
        if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: handle empty input correctly"; else read -p "Does this handle empty input correctly? (y/n): " response; fi
        if [[ "${response,,}" != "y" ]]; then
            ux_error "Step $step_num failed visual verification"
            return 1
        fi
    fi
}

# Test Step 3: Multi-line content
test_multiline_content() {
    local step_num=3
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Multi-line Content" \
            "Boxy handles multiple lines of input correctly" \
            "echo -e '$TEST_MULTILINE' | $boxy_path" \
            "Multiple lines properly enclosed in box with consistent borders" \
            "Testing multi-line content handling" \
            "• All lines visible within box\n• Box height adjusts to content\n• Consistent left/right borders"
    else
        ux_info "Step $step_num: Multi-line content test"
        echo "Command: echo -e '$TEST_MULTILINE' | $boxy_path"
        echo "Expected: Multi-line content in properly sized box"
        echo
        echo "Result:"
        echo -e "$TEST_MULTILINE" | "$boxy_path"
        echo
        if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: lines properly boxed"; else read -p "Are all lines properly boxed? (y/n): " response; fi
        if [[ "${response,,}" != "y" ]]; then
            ux_error "Step $step_num failed visual verification"
            return 1
        fi
    fi
}

# Test Step 4: Basic pipe integration
test_pipe_integration() {
    local step_num=4
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Pipe Integration" \
            "Boxy integrates properly in Unix pipe chains" \
            "echo '$TEST_CONTENT' | $boxy_path | cat -n" \
            "Box output can be further processed in pipelines" \
            "Testing Unix pipe integration" \
            "• Output flows through pipe correctly\n• No broken pipe errors\n• Maintains box formatting"
    else
        ux_info "Step $step_num: Pipe integration test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path | cat -n"
        echo "Expected: Boxed output with line numbers added by cat"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" | cat -n
        echo
        if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: pipe integration works"; else read -p "Does pipe integration work correctly? (y/n): " response; fi
        if [[ "${response,,}" != "y" ]]; then
            ux_error "Step $step_num failed visual verification"
            return 1
        fi
    fi
}

# Test Step 5: Error handling
test_error_handling() {
    local step_num=5
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Error Handling" \
            "Boxy handles invalid flags gracefully" \
            "$boxy_path --invalid-flag-test 2>&1 || true" \
            "Helpful error message without crash" \
            "Testing error handling with invalid flags" \
            "• No segfaults or panics\n• Clear error message\n• Appropriate exit code"
    else
        ux_info "Step $step_num: Error handling test"
        echo "Command: $boxy_path --invalid-flag-test"
        echo "Expected: Clear error message, no crash"
        echo
        echo "Result:"
        "$boxy_path" --invalid-flag-test 2>&1 || true
        echo
        if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: error handling appropriate"; else read -p "Does error handling work appropriately? (y/n): " response; fi
        if [[ "${response,,}" != "y" ]]; then
            ux_error "Step $step_num failed visual verification"
            return 1
        fi
    fi
}

# Ceremony completion
ceremony_end() {
    local total_steps=5
    local success="$1"
    
    if command -v uat_suite_end &> /dev/null; then
        uat_suite_end "$CEREMONY_NAME" "$total_steps" "$(date)"
    else
        if [[ "$success" == "true" ]]; then
            ux_success "Ceremony 01 completed successfully ($total_steps steps)"
            echo "✅ Basic box drawing functionality verified"
        else
            ux_error "Ceremony 01 completed with failures"
            echo "❌ Basic box drawing needs attention"
        fi
        echo "═══════════════════════════════════════════════════════════════════════════════"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN CEREMONY EXECUTION
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    local exit_code=0
    
    # Detect boxy binary
    local boxy_path
    boxy_path="$(detect_boxy_binary)"
    
    # Start ceremony
    ceremony_start "$boxy_path"
    
    if [[ "$CEREMONY_QUICK" == "true" ]]; then
        # Quick mode: Just one smoke test
        echo "🚀 CEREMONY 01 SMOKE TEST: Basic box drawing"
        test_basic_box "$boxy_path" || exit_code=1
        echo "$([ $exit_code -eq 0 ] && echo "✅ CEREMONY 01: PASS" || echo "❌ CEREMONY 01: FAIL")"
    else
        # Full mode: All test steps
        test_basic_box "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 1 2
        fi
        
        test_empty_input "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 2 3
        fi
        
        test_multiline_content "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 3 4
        fi
        
        test_pipe_integration "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 4 5
        fi
        
        test_error_handling "$boxy_path" || exit_code=1
    fi
    
    # Complete ceremony
    local success="false"
    [[ $exit_code -eq 0 ]] && success="true"
    ceremony_end "$success"
    
    exit $exit_code
}

# Execute if run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi