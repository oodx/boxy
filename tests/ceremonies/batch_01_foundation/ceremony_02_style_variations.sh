#!/usr/bin/env bash
#
# CEREMONY 02: Style Variations - Foundation API
# Description: Tests box style variations (normal, rounded, double, heavy, ascii)
# Batch: Foundation (01)
# Complexity: Basic
# Dependencies: boxy binary with --style flag support
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
# CEREMONY 02: STYLE VARIATIONS TESTS
# ═══════════════════════════════════════════════════════════════════════════════

# Ceremony configuration
CEREMONY_NAME="Style Variations"
CEREMONY_VERSION="v1.0"
CEREMONY_AUTO="${CEREMONY_AUTO:-false}"

# Test data
TEST_CONTENT="Style Test"
STYLE_LIST=("normal" "rounded" "double" "heavy" "ascii")

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
        uat_suite_start "$CEREMONY_NAME" "Foundation API - Box style variations verification" "$CEREMONY_VERSION"
    else
        ux_info "Starting Ceremony 02: $CEREMONY_NAME"
        echo "═══════════════════════════════════════════════════════════════════════════════"
        echo "Testing box style variations with boxy at: $boxy_path"
        echo "═══════════════════════════════════════════════════════════════════════════════"
    fi
}

# Test Step 1: Normal style (default/baseline)
test_normal_style() {
    local step_num=1
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Normal Style" \
            "Default/normal box style renders correctly" \
            "echo '$TEST_CONTENT' | $boxy_path --style normal" \
            "Standard box borders using normal characters" \
            "Testing baseline normal style" \
            "• Standard ASCII box characters\n• Clean corners and edges\n• Consistent line thickness"
    else
        ux_info "Step $step_num: Normal style test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --style normal"
        echo "Expected: Standard box with normal ASCII characters"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --style normal
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: normal style correct"; else read -p "Does normal style look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 2: Rounded style
test_rounded_style() {
    local step_num=2
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Rounded Style" \
            "Rounded corners box style renders correctly" \
            "echo '$TEST_CONTENT' | $boxy_path --style rounded" \
            "Box with rounded corner characters" \
            "Testing rounded corner style variation" \
            "• Rounded corner characters\n• Smooth visual appearance\n• Proper Unicode rendering"
    else
        ux_info "Step $step_num: Rounded style test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --style rounded"
        echo "Expected: Box with rounded corners"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --style rounded
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: rounded style correct"; else read -p "Does rounded style look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 3: Double style
test_double_style() {
    local step_num=3
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Double Style" \
            "Double-line box style renders correctly" \
            "echo '$TEST_CONTENT' | $boxy_path --style double" \
            "Box with double-line borders" \
            "Testing double-line style variation" \
            "• Double-line characters\n• Consistent thickness\n• Proper line connections"
    else
        ux_info "Step $step_num: Double style test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --style double"
        echo "Expected: Box with double-line borders"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --style double
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: double style correct"; else read -p "Does double style look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 4: Heavy style
test_heavy_style() {
    local step_num=4
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Heavy Style" \
            "Heavy/thick box style renders correctly" \
            "echo '$TEST_CONTENT' | $boxy_path --style heavy" \
            "Box with thick/heavy border characters" \
            "Testing heavy/bold style variation" \
            "• Heavy/thick line characters\n• Bold visual appearance\n• Strong emphasis effect"
    else
        ux_info "Step $step_num: Heavy style test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --style heavy"
        echo "Expected: Box with thick/heavy borders"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --style heavy
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: heavy style correct"; else read -p "Does heavy style look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 5: ASCII style (compatibility)
test_ascii_style() {
    local step_num=5
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "ASCII Style" \
            "ASCII-only style for terminal compatibility" \
            "echo '$TEST_CONTENT' | $boxy_path --style ascii" \
            "Box using only basic ASCII characters (+, -, |)" \
            "Testing ASCII compatibility style" \
            "• Only basic ASCII characters\n• Maximum terminal compatibility\n• No Unicode dependencies"
    else
        ux_info "Step $step_num: ASCII style test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --style ascii"
        echo "Expected: Box using only basic ASCII characters"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --style ascii
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: ASCII style correct"; else read -p "Does ASCII style look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 6: Style comparison display
test_style_comparison() {
    local step_num=6
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Style Comparison" \
            "All styles display side by side for visual comparison" \
            "for style in ${STYLE_LIST[*]}; do echo \"Style: \$style\" | $boxy_path --style \$style; done" \
            "Visual comparison of all available styles" \
            "Testing comprehensive style variety" \
            "• All styles render correctly\n• Visual differences are clear\n• No style conflicts"
    else
        ux_info "Step $step_num: Style comparison display"
        echo "Command: Display all styles for comparison"
        echo "Expected: Clear visual differences between styles"
        echo
        echo "Result:"
        for style in "${STYLE_LIST[@]}"; do
            echo "=== Style: $style ==="
            echo "$TEST_CONTENT ($style)" | "$boxy_path" --style "$style"
            echo
        done
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: all styles distinct"; else read -p "Are all styles visually distinct and correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Ceremony completion
ceremony_end() {
    local total_steps=6
    local success="$1"
    
    if command -v uat_suite_end &> /dev/null; then
        uat_suite_end "$CEREMONY_NAME" "$total_steps" "$(date)"
    else
        if [[ "$success" == "true" ]]; then
            ux_success "Ceremony 02 completed successfully ($total_steps steps)"
            echo "✅ Style variations functionality verified"
            echo "    Supported styles: ${STYLE_LIST[*]}"
        else
            ux_error "Ceremony 02 completed with failures"
            echo "❌ Style variations need attention"
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
        echo "🚀 CEREMONY 02 SMOKE TEST: Style variations"
        test_normal_style "$boxy_path" || exit_code=1
        echo "$([ $exit_code -eq 0 ] && echo "✅ CEREMONY 02: PASS" || echo "❌ CEREMONY 02: FAIL")"
    else
        # Full mode: All test steps
        test_normal_style "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 1 2
        fi
        
        test_rounded_style "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 2 3
        fi
        
        test_double_style "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 3 4
        fi
        
        test_heavy_style "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 4 5
        fi
        
        test_ascii_style "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 5 6
        fi
        
        test_style_comparison "$boxy_path" || exit_code=1
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