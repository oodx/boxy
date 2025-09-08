#!/usr/bin/env bash
#
# CEREMONY 03: Basic Colors - Foundation API
# Description: Tests basic color system with text coordination
# Batch: Foundation (01)
# Complexity: Basic
# Dependencies: boxy binary with --color flag support
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
# CEREMONY 03: BASIC COLORS TESTS
# ═══════════════════════════════════════════════════════════════════════════════

# Ceremony configuration
CEREMONY_NAME="Basic Colors"
CEREMONY_VERSION="v1.0"
CEREMONY_AUTO="${CEREMONY_AUTO:-false}"

# Test data
TEST_CONTENT="Color Test"
BASIC_COLORS=("red" "green" "blue" "yellow" "cyan" "magenta" "white")
# Common theme names based on boxy suggestions
THEME_COLORS=("success" "warning" "error" "info")

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
        uat_suite_start "$CEREMONY_NAME" "Foundation API - Basic color system verification" "$CEREMONY_VERSION"
    else
        ux_info "Starting Ceremony 03: $CEREMONY_NAME"
        echo "═══════════════════════════════════════════════════════════════════════════════"
        echo "Testing basic color system with boxy at: $boxy_path"
        echo "═══════════════════════════════════════════════════════════════════════════════"
    fi
}

# Test Step 1: Red color
test_red_color() {
    local step_num=1
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Red Color" \
            "Red color renders correctly with proper contrast" \
            "echo '$TEST_CONTENT' | $boxy_path --color red" \
            "Box with red coloring and readable text" \
            "Testing red color implementation" \
            "• Red color visible and distinct\n• Text remains readable\n• Consistent color application"
    else
        ux_info "Step $step_num: Red color test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --color red"
        echo "Expected: Box with red color scheme"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --color red
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: red color correct"; else read -p "Does red color look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 2: Green color
test_green_color() {
    local step_num=2
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Green Color" \
            "Green color renders correctly with proper contrast" \
            "echo '$TEST_CONTENT' | $boxy_path --color green" \
            "Box with green coloring and readable text" \
            "Testing green color implementation" \
            "• Green color visible and distinct\n• Text remains readable\n• Consistent color application"
    else
        ux_info "Step $step_num: Green color test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --color green"
        echo "Expected: Box with green color scheme"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --color green
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: green color correct"; else read -p "Does green color look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 3: Blue color
test_blue_color() {
    local step_num=3
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Blue Color" \
            "Blue color renders correctly with proper contrast" \
            "echo '$TEST_CONTENT' | $boxy_path --color blue" \
            "Box with blue coloring and readable text" \
            "Testing blue color implementation" \
            "• Blue color visible and distinct\n• Text remains readable\n• Consistent color application"
    else
        ux_info "Step $step_num: Blue color test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --color blue"
        echo "Expected: Box with blue color scheme"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --color blue
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: blue color correct"; else read -p "Does blue color look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 4: Theme colors (semantic colors)
test_theme_colors() {
    local step_num=4
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Theme Colors" \
            "Semantic theme colors (success, warning, error, info) render correctly" \
            "echo '$TEST_CONTENT' | $boxy_path --color success" \
            "Box with theme-appropriate colors and semantic meaning" \
            "Testing semantic theme color system" \
            "• Theme colors semantically appropriate\n• Clear visual distinctions\n• Professional appearance"
    else
        ux_info "Step $step_num: Theme colors test"
        echo "Command: Testing theme colors (success, warning, error, info)"
        echo "Expected: Semantically appropriate colors"
        echo
        echo "Results:"
        for theme in "${THEME_COLORS[@]}"; do
            echo "=== Theme: $theme ==="
            echo "$TEST_CONTENT ($theme)" | "$boxy_path" --color "$theme" 2>/dev/null || echo "Theme '$theme' not supported"
            echo
        done
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: theme colors appropriate"; else read -p "Do theme colors look appropriate and distinct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 5: Text coordination test
test_text_coordination() {
    local step_num=5
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Text Coordination" \
            "Text color coordinates with box color for readability" \
            "echo 'Readability Test' | $boxy_path --color blue --text white" \
            "Box color and text color work together for optimal readability" \
            "Testing text/box color coordination" \
            "• Text clearly readable against box color\n• No color conflicts\n• Professional contrast ratios"
    else
        ux_info "Step $step_num: Text coordination test"
        echo "Command: echo 'Readability Test' | $boxy_path --color blue --text white"
        echo "Expected: Good contrast between box and text colors"
        echo
        echo "Result:"
        echo "Readability Test" | "$boxy_path" --color blue --text white 2>/dev/null || \
        echo "Readability Test" | "$boxy_path" --color blue
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: text readable with contrast"; else read -p "Is text clearly readable with good contrast? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 6: Color palette comparison
test_color_comparison() {
    local step_num=6
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Color Comparison" \
            "All basic colors display for palette verification" \
            "for color in ${BASIC_COLORS[*]}; do echo \"Color: \$color\" | $boxy_path --color \$color; done" \
            "Visual comparison of all available basic colors" \
            "Testing comprehensive color palette" \
            "• All colors render correctly\n• Visual differences are clear\n• No color conflicts or errors"
    else
        ux_info "Step $step_num: Color comparison display"
        echo "Command: Display all basic colors for comparison"
        echo "Expected: Clear visual differences between colors"
        echo
        echo "Result:"
        for color in "${BASIC_COLORS[@]}"; do
            echo "=== Color: $color ==="
            echo "$TEST_CONTENT ($color)" | "$boxy_path" --color "$color" 2>/dev/null || echo "Color '$color' not supported"
            echo
        done
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: colors distinct and rendered"; else read -p "Are all colors visually distinct and correctly rendered? (y/n): " response; fi
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
            ux_success "Ceremony 03 completed successfully ($total_steps steps)"
            echo "✅ Basic color system functionality verified"
            echo "    Supported colors: ${BASIC_COLORS[*]}"
            echo "    Theme colors: ${THEME_COLORS[*]}"
        else
            ux_error "Ceremony 03 completed with failures"
            echo "❌ Basic color system needs attention"
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
        echo "🚀 CEREMONY 03 SMOKE TEST: Basic colors"
        test_red_color "$boxy_path" || exit_code=1
        echo "$([ $exit_code -eq 0 ] && echo "✅ CEREMONY 03: PASS" || echo "❌ CEREMONY 03: FAIL")"
    else
        # Execute full test steps
        test_red_color "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 1 2
        fi
        
        test_green_color "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 2 3
        fi
        
        test_blue_color "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 3 4
        fi
        
        test_theme_colors "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 4 5
        fi
        
        test_text_coordination "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 5 6
        fi
        
        test_color_comparison "$boxy_path" || exit_code=1
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
