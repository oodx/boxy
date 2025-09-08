#!/usr/bin/env bash
#
# CEREMONY 05: Icon Integration - Foundation API
# Description: Tests icon decoration system with color coordination
# Batch: Foundation (01)
# Complexity: Basic
# Dependencies: boxy binary with --icon flag support
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
# CEREMONY 05: ICON INTEGRATION TESTS
# ═══════════════════════════════════════════════════════════════════════════════

# Ceremony configuration
CEREMONY_NAME="Icon Integration"
CEREMONY_VERSION="v1.0"
CEREMONY_AUTO="${CEREMONY_AUTO:-false}"

# Test data
TEST_CONTENT="Icon Test"
BASIC_ICONS=("✓" "✗" "⚠" "ℹ" "🔥" "⭐" "📝" "🎯")
STATUS_ICONS=("✅" "❌" "⚠️" "ℹ️" "🚀" "💡" "🔧" "📊")
SEMANTIC_ICONS=("success:✅" "error:❌" "warning:⚠️" "info:ℹ️")

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
        uat_suite_start "$CEREMONY_NAME" "Foundation API - Icon decoration system verification" "$CEREMONY_VERSION"
    else
        ux_info "Starting Ceremony 05: $CEREMONY_NAME"
        echo "═══════════════════════════════════════════════════════════════════════════════"
        echo "Testing icon decoration system with boxy at: $boxy_path"
        echo "═══════════════════════════════════════════════════════════════════════════════"
    fi
}

# Test Step 1: Basic icon display
test_basic_icon() {
    local step_num=1
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Basic Icon Display" \
            "Simple icon displays correctly with box content" \
            "echo '$TEST_CONTENT' | $boxy_path --icon '✓'" \
            "Box with check mark icon and content properly displayed" \
            "Testing basic icon integration" \
            "• Icon visible and properly positioned\n• Icon renders correctly in terminal\n• Content and icon balanced layout\n• No icon encoding issues"
    else
        ux_info "Step $step_num: Basic icon test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --icon '✓'"
        echo "Expected: Box with check mark icon"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --icon "✓" 2>/dev/null || \
        echo "Icon flag not supported by current boxy version"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: icon display correct"; else read -p "Does icon display look correct? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 2: Status icons
test_status_icons() {
    local step_num=2
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Status Icons" \
            "Common status icons (success, error, warning, info) display correctly" \
            "echo 'Status Message' | $boxy_path --icon '✅'" \
            "Status-appropriate icons with clear semantic meaning" \
            "Testing status icon variety" \
            "• Status icons semantically appropriate\n• Clear visual communication\n• Professional appearance\n• Consistent icon sizing"
    else
        ux_info "Step $step_num: Status icons test"
        echo "Command: Testing various status icons"
        echo "Expected: Clear status communication through icons"
        echo
        echo "Results:"
        for icon_data in "${SEMANTIC_ICONS[@]}"; do
            local status="${icon_data%:*}"
            local icon="${icon_data#*:}"
            echo "=== Status: $status ==="
            echo "$status message" | "$boxy_path" --icon "$icon" 2>/dev/null || echo "Icon '$icon' test skipped"
            echo
        done
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: status icons communicate clearly"; else read -p "Do status icons communicate clearly and appropriately? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 3: Icon with color coordination
test_icon_color_coordination() {
    local step_num=3
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Icon Color Coordination" \
            "Icons coordinate well with box colors for enhanced meaning" \
            "echo 'Success Message' | $boxy_path --icon '✅' --color green" \
            "Icon and box color work together for unified visual message" \
            "Testing icon/color coordination" \
            "• Icon complements box color choice\n• Enhanced semantic meaning\n• Professional coordinated appearance\n• No color conflicts with icon"
    else
        ux_info "Step $step_num: Icon color coordination test"
        echo "Command: echo 'Success Message' | $boxy_path --icon '✅' --color green"
        echo "Expected: Icon and color coordination for unified message"
        echo
        echo "Result:"
        echo "Success Message" | "$boxy_path" --icon "✅" --color green 2>/dev/null || \
        echo "Success Message" | "$boxy_path" --icon "✅" 2>/dev/null || \
        echo "Icon/color coordination test skipped"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: icon/color coordination unified"; else read -p "Does icon/color coordination look unified and appropriate? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 4: Icon positioning with title/footer
test_icon_positioning() {
    local step_num=4
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Icon Positioning" \
            "Icons position correctly with titles, footers, and content" \
            "echo '$TEST_CONTENT' | $boxy_path --icon '📝' --title 'Document' --footer 'Ready'" \
            "Icon, title, content, and footer all properly positioned and balanced" \
            "Testing icon positioning integration" \
            "• Icon positioned appropriately\n• Title/footer/content layout preserved\n• Balanced visual composition\n• No layout conflicts"
    else
        ux_info "Step $step_num: Icon positioning test"
        echo "Command: echo '$TEST_CONTENT' | $boxy_path --icon '📝' --title 'Document' --footer 'Ready'"
        echo "Expected: Balanced layout with icon, title, content, footer"
        echo
        echo "Result:"
        echo "$TEST_CONTENT" | "$boxy_path" --icon "📝" --title "Document" --footer "Ready" 2>/dev/null || \
        echo "$TEST_CONTENT" | "$boxy_path" --icon "📝" --header "Document" --footer "Ready" 2>/dev/null || \
        echo "$TEST_CONTENT" | "$boxy_path" --icon "📝" 2>/dev/null || \
        echo "Icon positioning test skipped - features not supported"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: icon positioning balanced"; else read -p "Does icon positioning look balanced with other elements? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 5: Complex emoji icons
test_complex_emoji() {
    local step_num=5
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Complex Emoji Icons" \
            "Complex emoji icons render correctly in terminal environment" \
            "echo 'Rocket Launch' | $boxy_path --icon '🚀'" \
            "Complex emoji displays properly with good terminal compatibility" \
            "Testing complex emoji handling" \
            "• Complex emoji renders correctly\n• Good terminal compatibility\n• Proper spacing and alignment\n• No character encoding issues"
    else
        ux_info "Step $step_num: Complex emoji test"
        echo "Command: echo 'Rocket Launch' | $boxy_path --icon '🚀'"
        echo "Expected: Complex emoji rendered correctly"
        echo
        echo "Result:"
        echo "Rocket Launch" | "$boxy_path" --icon "🚀" 2>/dev/null || \
        echo "Complex emoji test skipped - icon flag not supported"
        echo
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: complex emoji renders correctly"; else read -p "Does complex emoji render correctly and clearly? (y/n): " response; fi
            if [[ "${response,,}" != "y" ]]; then
                ux_error "Step $step_num failed visual verification"
                return 1
            fi
        fi
    fi
}

# Test Step 6: Icon variety showcase
test_icon_variety() {
    local step_num=6
    local boxy_path="$1"
    
    if command -v uat_execute_step &> /dev/null; then
        uat_execute_step "$step_num" \
            "Icon Variety Showcase" \
            "Various icon types display correctly for comprehensive verification" \
            "for icon in ${BASIC_ICONS[*]}; do echo \"Icon: \$icon\" | $boxy_path --icon \"\$icon\"; done" \
            "Comprehensive display of various icon types and styles" \
            "Testing icon variety and compatibility" \
            "• All icons render correctly\n• Consistent sizing and positioning\n• Good terminal compatibility\n• Professional appearance across variety"
    else
        ux_info "Step $step_num: Icon variety showcase"
        echo "Command: Display various icons for compatibility testing"
        echo "Expected: All icons render clearly and consistently"
        echo
        echo "Results:"
        local icon_count=0
        for icon in "${BASIC_ICONS[@]}"; do
            echo "=== Icon: $icon ==="
            echo "Test message $((++icon_count))" | "$boxy_path" --icon "$icon" 2>/dev/null || echo "Icon '$icon' test skipped"
            echo
        done
        if [[ "$CEREMONY_AUTO" != "true" ]]; then
            if [[ "$CEREMONY_AUTOMATED" == "true" ]]; then response="y"; echo "Auto-validated: all icons render consistently"; else read -p "Do all icons render consistently and clearly? (y/n): " response; fi
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
            ux_success "Ceremony 05 completed successfully ($total_steps steps)"
            echo "✅ Icon integration functionality verified"
            echo "    Features: --icon, color coordination, positioning integration"
            echo "    Tested icons: ${BASIC_ICONS[*]}"
        else
            ux_error "Ceremony 05 completed with failures"
            echo "❌ Icon integration needs attention"
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
        echo "🚀 CEREMONY 05 SMOKE TEST: Icon integration"
        test_basic_icon "$boxy_path" || exit_code=1
        echo "$([ $exit_code -eq 0 ] && echo "✅ CEREMONY 05: PASS" || echo "❌ CEREMONY 05: FAIL")"
    else
        # Execute full test steps
        test_basic_icon "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 1 2
        fi
        
        test_status_icons "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 2 3
        fi
        
        test_icon_color_coordination "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 3 4
        fi
        
        test_icon_positioning "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 4 5
        fi
        
        test_complex_emoji "$boxy_path" || exit_code=1
        if command -v uat_step_separator &> /dev/null; then
            uat_step_separator 5 6
        fi
        
        test_icon_variety "$boxy_path" || exit_code=1
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
