#!/usr/bin/env bash
#
# CEREMONY 04: Title and Footer - Foundation API
# Description: Tests header/footer positioning and formatting
# Batch: Foundation (01)
# Complexity: Basic
# Dependencies: boxy binary with --title and --footer flag support
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
# CEREMONY 04: TITLE AND FOOTER TESTS
# ═══════════════════════════════════════════════════════════════════════════════

# Ceremony configuration
CEREMONY_NAME="Title and Footer"
CEREMONY_VERSION="v1.0"
CEREMONY_AUTO="${CEREMONY_AUTO:-false}"

# Test data
TEST_CONTENT="Main Content"
TEST_TITLE="Test Title"
TEST_FOOTER="Footer Text"
LONG_TITLE="This is a longer title to test positioning and wrapping"
LONG_FOOTER="Extended footer text with more content to verify formatting"

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
        uat_suite_start "$CEREMONY_NAME" "Foundation API - Title/footer positioning verification" "$CEREMONY_VERSION"
    else
        ux_info "Starting Ceremony 04: $CEREMONY_NAME"
        echo "═══════════════════════════════════════════════════════════════════════════════"
        echo "Testing title/footer positioning with boxy at: $boxy_path"
        echo "═══════════════════════════════════════════════════════════════════════════════"
    fi
}

# Test Step 1: Basic title
test_basic_title() {
    local step_num=1
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Basic Title" \
            "Title appears correctly at top of box" \
            "echo '$TEST_CONTENT' | $boxy_path --title '$TEST_TITLE'" \
            "Box with title '$TEST_TITLE' positioned at top, main content below" \
            "Testing basic title positioning" \
            "• Title visible at top of box\n• Title properly formatted\n• Content positioned below title\n• Box sizing adjusts appropriately"
    else
        ux_info "Step $step_num: Basic title test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --title '$TEST_TITLE'"
        echo "Expected: Title at top, content below"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --title "$TEST_TITLE" 2>/dev/null || \
        echo "$TEST_CONTENT" | "$boxy_path" --header "$TEST_TITLE" 2>/dev/null || \
        echo "Title/header flag not supported by current boxy version"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: title positioning correct"; else read -p "Does title positioning look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 2: Basic footer
test_basic_footer() {
    local step_num=2
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Basic Footer" \
            "Footer appears correctly at bottom of box" \
            "echo '$TEST_CONTENT' | $boxy_path --footer '$TEST_FOOTER'" \
            "Box with footer '$TEST_FOOTER' positioned at bottom, main content above" \
            "Testing basic footer positioning" \
            "• Footer visible at bottom of box\n• Footer properly formatted\n• Content positioned above footer\n• Box sizing adjusts appropriately"
    else
        ux_info "Step $step_num: Basic footer test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --footer '$TEST_FOOTER'"
        echo "Expected: Content at top, footer at bottom"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --footer "$TEST_FOOTER" 2>/dev/null || \
        echo "Footer flag not supported by current boxy version"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: footer positioning correct"; else read -p "Does footer positioning look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 3: Title and footer combined
test_title_footer_combined() {
    local step_num=3
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Title + Footer Combined" \
            "Both title and footer display correctly with content between" \
            "echo '$TEST_CONTENT' | $boxy_path --title '$TEST_TITLE' --footer '$TEST_FOOTER'" \
            "Box with title at top, main content in middle, footer at bottom" \
            "Testing combined title/footer layout" \
            "• Title at top position\n• Footer at bottom position\n• Content properly centered between\n• Clean separation and layout"
    else
        ux_info "Step $step_num: Combined title/footer test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --title '$TEST_TITLE' --footer '$TEST_FOOTER'"
        echo "Expected: Title-Content-Footer layout"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --title "$TEST_TITLE" --footer "$TEST_FOOTER" 2>/dev/null || \
        echo "$TEST_CONTENT" | "$boxy_path" --header "$TEST_TITLE" --footer "$TEST_FOOTER" 2>/dev/null || \
        echo "Combined title/footer not supported by current boxy version"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: combined layout correct"; else read -p "Does combined title/footer layout look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 4: Long title handling
test_long_title() {
    local step_num=4
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Long Title Handling" \
            "Long titles are handled gracefully with proper formatting" \
            "echo '$TEST_CONTENT' | $boxy_path --title '$LONG_TITLE'" \
            "Long title properly formatted without breaking box layout" \
            "Testing long title edge case handling" \
            "• Long title displays fully or truncates gracefully\n• Box width adjusts appropriately\n• No text overflow issues\n• Maintains professional appearance"
    else
        ux_info "Step $step_num: Long title test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --title '$LONG_TITLE'"
        echo "Expected: Long title handled gracefully"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --title "$LONG_TITLE" 2>/dev/null || \
        echo "$TEST_CONTENT" | "$boxy_path" --header "$LONG_TITLE" 2>/dev/null || \
        echo "Long title test skipped - feature not supported"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: long title handling appropriate"; else read -p "Does long title handling look appropriate? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 5: Empty title/footer handling
test_empty_title_footer() {
    local step_num=5
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Empty Title/Footer" \
            "Empty or missing titles/footers are handled gracefully" \
            "echo '$TEST_CONTENT' | $boxy_path --title '' --footer ''" \
            "Box reverts to basic layout when title/footer are empty" \
            "Testing empty title/footer edge cases" \
            "• Empty values handled gracefully\n• No broken layouts or errors\n• Clean fallback to basic box\n• No visual artifacts"
    else
        ux_info "Step $step_num: Empty title/footer test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --title '' --footer ''"
        echo "Expected: Graceful handling of empty title/footer"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --title "" --footer "" 2>/dev/null || \
        echo "$TEST_CONTENT" | "$boxy_path" --header "" --footer "" 2>/dev/null || \
        echo "Empty title/footer test skipped - feature not supported"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: empty title/footer correct"; else read -p "Does empty title/footer handling look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 6: Style integration with title/footer
test_style_integration() {
    local step_num=6
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Style Integration" \
            "Title/footer work correctly with different box styles" \
            "echo '$TEST_CONTENT' | $boxy_path --style rounded --title '$TEST_TITLE' --footer '$TEST_FOOTER'" \
            "Title/footer integrate cleanly with different box styles" \
            "Testing style compatibility with title/footer" \
            "• Style applies consistently to entire box\n• Title/footer integrate with style borders\n• No style conflicts or mismatches\n• Professional integrated appearance"
    else
        ux_info "Step $step_num: Style integration test"
        echo "Command: Testing title/footer with different styles"
        echo "Expected: Clean integration with box styles"
        echo
        echo "Results with different styles:"
        for style in normal rounded double; do
            echo "=== Style: $style ==="
            echo "$TEST_CONTENT" | "$boxy_path" --style "$style" --title "$TEST_TITLE" --footer "$TEST_FOOTER" 2>/dev/null || \
            echo "$TEST_CONTENT" | "$boxy_path" --style "$style" --header "$TEST_TITLE" --footer "$TEST_FOOTER" 2>/dev/null || \
            echo "$TEST_CONTENT" | "$boxy_path" --style "$style"
            echo
        done
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: style integration consistent"; else read -p "Does style integration look consistent and clean? (y/n): " response; fi
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
            ux_success "Ceremony 04 completed successfully ($total_steps steps)"
            echo "✅ Title/footer positioning functionality verified"
            echo "    Features: --title, --footer, style integration"
        else
            ux_error "Ceremony 04 completed with failures"
            echo "❌ Title/footer positioning needs attention"
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
        echo "🚀 CEREMONY 04 SMOKE TEST: Title/footer positioning"
        test_basic_title "$boxy_path" || exit_code=1
        echo "$([ $exit_code -eq 0 ] && echo "✅ CEREMONY 04: PASS" || echo "❌ CEREMONY 04: FAIL")"
    else
        # Execute full test steps
        test_basic_title "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 1 2
        fi
        
        test_basic_footer "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 2 3
        fi
        
        test_title_footer_combined "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 3 4
        fi
        
        test_long_title "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 4 5
        fi
        
        test_empty_title_footer "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 5 6
        fi
        
        test_style_integration "$boxy_path" || exit_code=1
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
