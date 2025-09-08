#!/usr/bin/env bash
#
# BashFX Test UX Kit - Divine Visual Generation Toolkit
# Reusable visual ceremonies for all test frameworks
#
# portable: bash, boxy, sed, awk, printf
# builtins: printf, test, local, case, if, while, read
#

# Prevent duplicate loading
[[ "${TEST_UX_KIT_LOADED:-}" == "1" ]] && return 0
TEST_UX_KIT_LOADED=1

# ═══════════════════════════════════════════════════════════════════════════════
# CORE DETECTION & CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════════

# IMPORTANT THIS SCRIPT IS NOT FOR TESTING BOXY ITS A FUNCTION LIBRARY USING
# EXISTING GLOBAL BOXY

# Tool availability cache
_HAS_BOXY=""
_HAS_COLOR=""

# Check boxy availability (cached)
has_boxy() {
    [[ -z "$_HAS_BOXY" ]] && {
        command -v boxy >/dev/null 2>&1 && _HAS_BOXY=1 || _HAS_BOXY=0
    }
    [[ "$_HAS_BOXY" == "1" ]]
}

# Check color support
has_color() {
    [[ -z "$_HAS_COLOR" ]] && {
        [[ -t 1 && "${TERM:-}" != "dumb" && "${NO_COLOR:-}" != "1" ]] && _HAS_COLOR=1 || _HAS_COLOR=0
    }
    [[ "$_HAS_COLOR" == "1" ]]
}

# Color definitions (when supported)
if has_color; then
    readonly COLOR_RESET='\033[0m'
    readonly COLOR_BOLD='\033[1m'
    readonly COLOR_DIM='\033[2m'
    readonly COLOR_RED='\033[31m'
    readonly COLOR_GREEN='\033[32m'
    readonly COLOR_YELLOW='\033[33m'
    readonly COLOR_BLUE='\033[34m'
    readonly COLOR_MAGENTA='\033[35m'
    readonly COLOR_CYAN='\033[36m'
    readonly COLOR_WHITE='\033[37m'
    readonly COLOR_GREY='\033[90m'
else
    readonly COLOR_RESET='' COLOR_BOLD='' COLOR_DIM=''
    readonly COLOR_RED='' COLOR_GREEN='' COLOR_YELLOW=''
    readonly COLOR_BLUE='' COLOR_MAGENTA='' COLOR_CYAN=''
    readonly COLOR_WHITE='' COLOR_GREY=''
fi

# ═══════════════════════════════════════════════════════════════════════════════
# TEST PHASE SUPER BOXIES
# ═══════════════════════════════════════════════════════════════════════════════

# Divine test phase with maximum ceremony
test_phase_super() {
    local phase_num="${1:-0}"
    local phase_name="${2:-UNKNOWN}"
    local description="${3:-}"
    local theme="${4:-info}"
    
    if has_boxy; then
        # Render directly with boxy (full width), no ASCII frame
        {
            echo "$description"
        } | boxy --theme "$theme" --title "🧪 TEST PHASE $phase_num: $phase_name" --status "sc:Divine Testing Protocol v1.0" --width max --layout dt
    else
        echo "═══════════════════════════════════════════════════════════════════════════════"
        echo "                         PHASE $phase_num: $phase_name"
        [[ -n "$description" ]] && echo "                    $description"
        echo "═══════════════════════════════════════════════════════════════════════════════"
    fi
    echo
}

# Nested phase box for sub-phases
test_subphase_box() {
    local major="$1"
    local minor="$2"
    local name="$3"
    local status="${4:-pending}"
    
    local theme icon
    case "$status" in
        running) theme="warn"; icon="⚡" ;;
        success) theme="success"; icon="✅" ;;
        failed)  theme="error"; icon="❌" ;;
        skipped) theme="info"; icon="⚠️" ;;
        *)       theme="info"; icon="📝" ;;
    esac
    
    if has_boxy; then
        echo "$icon SUB-PHASE $major.$minor: $name" | boxy --theme "$theme" --width 60
    else
        echo "  [$icon] SUB-PHASE $major.$minor: $name"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# STATUS BOXIES
# ═══════════════════════════════════════════════════════════════════════════════

# Multi-theme status box
status_box() {
    local status="$1"
    local message="$2"
    local details="${3:-}"
    
    local theme icon prefix
    case "${status,,}" in
        success|pass|ok)     theme="success"; icon="✅"; prefix="SUCCESS" ;;
        error|fail|failed)   theme="error"; icon="❌"; prefix="ERROR" ;;
        warning|warn)        theme="warn"; icon="⚠️"; prefix="WARNING" ;;
        info|information)    theme="info"; icon="ℹ️"; prefix="INFO" ;;
        debug)               theme="debug"; icon="🐛"; prefix="DEBUG" ;;
        pending|waiting)     theme="warn"; icon="⏳"; prefix="PENDING" ;;
        skip|skipped)        theme="info"; icon="⏭️"; prefix="SKIPPED" ;;
        *)                   theme="info"; icon="📌"; prefix="STATUS" ;;
    esac
    
    if has_boxy; then
        local content="$icon $prefix: $message"
        [[ -n "$details" ]] && content="$content\n\n$details"
        echo -e "$content" | boxy --theme "$theme" --width 70
    else
        echo "[$icon] $prefix: $message"
        [[ -n "$details" ]] && echo "     $details"
    fi
}

# Compact status indicator
status_indicator() {
    local status="$1"
    local label="$2"
    
    local icon color
    case "${status,,}" in
        success|pass|ok)     icon="✅"; color="$COLOR_GREEN" ;;
        error|fail|failed)   icon="❌"; color="$COLOR_RED" ;;
        warning|warn)        icon="⚠️"; color="$COLOR_YELLOW" ;;
        running|active)      icon="🔄"; color="$COLOR_CYAN" ;;
        pending|waiting)     icon="⏳"; color="$COLOR_YELLOW" ;;
        skip|skipped)        icon="⏭️"; color="$COLOR_GREY" ;;
        *)                   icon="•"; color="$COLOR_WHITE" ;;
    esac
    
    printf "${color}%s %s${COLOR_RESET}" "$icon" "$label"
}

# ═══════════════════════════════════════════════════════════════════════════════
# BANNERS & TRANSITIONS
# ═══════════════════════════════════════════════════════════════════════════════

# Full-width ceremonial banner
ceremonial_banner() {
    local title="$1"
    local subtitle="${2:-}"
    local width="${3:-80}"
    
    if has_boxy; then
        local content="$title"
        [[ -n "$subtitle" ]] && content="$content\n$subtitle"
        echo -e "$content" | boxy --theme info --title "🎭 CEREMONY" --width "$width"
    else
        local line=$(printf '═%.0s' $(seq 1 "$width"))
        echo "$line"
        echo "$title"
        [[ -n "$subtitle" ]] && echo "$subtitle"
        echo "$line"
    fi
    echo
}

# Transition banner between phases
transition_banner() {
    local from="$1"
    local to="$2"
    local style="${3:-arrow}"  # arrow, dots, wave
    
    local separator
    case "$style" in
        arrow)  separator="→→→→→→→→→" ;;
        dots)   separator="••••••••••" ;;
        wave)   separator="～～～～～" ;;
        *)      separator="──────────" ;;
    esac
    
    if has_boxy; then
        echo "$from $separator $to" | boxy --theme warn --title "🔄 TRANSITION"
    else
        echo
        echo "    $from $separator $to"
        echo
    fi
}

# Section divider with title
section_divider() {
    local title="$1"
    local char="${2:--}"
    local width="${3:-70}"
    if has_boxy; then
        # Render a full-width titled separator with no body
        echo "" | boxy --style ascii --title "$title" --width max
    else
        local padding=$(( (width - ${#title} - 2) / 2 ))
        local left=$(printf '%*s' "$padding" | tr ' ' "$char")
        local right=$(printf '%*s' "$padding" | tr ' ' "$char")
        if has_color; then
            echo "${COLOR_CYAN}${left} ${COLOR_BOLD}$title${COLOR_RESET}${COLOR_CYAN} ${right}${COLOR_RESET}"
        else
            echo "${left} $title ${right}"
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# COLORIZATION & DECOLORIZATION
# ═══════════════════════════════════════════════════════════════════════════════

# Strip all ANSI color codes
decolorize() {
    local text="${1:-$(cat)}"
    echo "$text" | sed 's/\x1b\[[0-9;]*m//g'
}

# Add color to specific patterns
colorize_pattern() {
    local text="${1:-$(cat)}"
    local pattern="$2"
    local color="$3"
    
    if has_color; then
        echo "$text" | sed "s/$pattern/${color}&${COLOR_RESET}/g"
    else
        echo "$text"
    fi
}

# Colorize by status keywords
colorize_status() {
    local text="${1:-$(cat)}"
    
    if has_color; then
        echo "$text" | \
            sed "s/\(SUCCESS\|PASS\|OK\|✅\)/${COLOR_GREEN}&${COLOR_RESET}/gi" | \
            sed "s/\(ERROR\|FAIL\|FAILED\|❌\)/${COLOR_RED}&${COLOR_RESET}/gi" | \
            sed "s/\(WARNING\|WARN\|⚠️\)/${COLOR_YELLOW}&${COLOR_RESET}/gi" | \
            sed "s/\(INFO\|ℹ️\)/${COLOR_BLUE}&${COLOR_RESET}/gi"
    else
        echo "$text"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# KEY-VALUE FORMATTING & DATA PARADIGM
# ═══════════════════════════════════════════════════════════════════════════════

# Format key-value with color
format_kv() {
    local key="$1"
    local value="$2"
    local key_color="${3:-$COLOR_CYAN}"
    local value_color="${4:-$COLOR_WHITE}"
    
    if has_color; then
        printf "${key_color}%-20s${COLOR_RESET}: ${value_color}%s${COLOR_RESET}\n" "$key" "$value"
    else
        printf "%-20s: %s\n" "$key" "$value"
    fi
}

# Parse semicolon-delimited KV string (_data paradigm)
parse_data_string() {
    local data_string="$1"
    local callback="${2:-echo}"  # Function to call with each k,v pair
    
    # Split on semicolons, then on equals/colons
    IFS=';' read -ra pairs <<< "$data_string"
    for pair in "${pairs[@]}"; do
        # Trim whitespace
        pair=$(echo "$pair" | xargs)
        [[ -z "$pair" ]] && continue
        
        # Split on = or :
        if [[ "$pair" == *"="* ]]; then
            key="${pair%%=*}"
            value="${pair#*=}"
        elif [[ "$pair" == *":"* ]]; then
            key="${pair%%:*}"
            value="${pair#*:}"
        else
            continue
        fi
        
        # Trim and callback
        key=$(echo "$key" | xargs)
        value=$(echo "$value" | xargs)
        "$callback" "$key" "$value"
    done
}

# Convert data string to visual box
data_to_box() {
    local data_string="$1"
    local title="${2:-Data}"
    local theme="${3:-info}"
    
    local formatted=""
    parse_data_string "$data_string" format_kv | while read -r line; do
        formatted="${formatted}${line}\n"
    done
    
    if has_boxy; then
        echo -e "$formatted" | boxy --theme "$theme" --title "$title"
    else
        echo "=== $title ==="
        echo -e "$formatted"
        echo "============="
    fi
}

# Stream key-value mapper (for piped data)
stream_kv_mapper() {
    local delimiter="${1:-;}"
    local kv_sep="${2:-=}"
    
    while IFS= read -r line; do
        # Skip empty lines and comments
        [[ -z "$line" || "$line" == \#* ]] && continue
        
        # Check if it's already formatted KV
        if [[ "$line" == *"$kv_sep"* ]]; then
            parse_data_string "$line" format_kv
        else
            # Try to extract KV from structured output
            echo "$line"
        fi
    done
}

# ═══════════════════════════════════════════════════════════════════════════════
# PROGRESS & METRICS VISUALIZATION
# ═══════════════════════════════════════════════════════════════════════════════

# Progress bar visualization
progress_bar() {
    local current="$1"
    local total="$2"
    local width="${3:-50}"
    local label="${4:-Progress}"
    
    local percent=$((current * 100 / total))
    local filled=$((width * current / total))
    local empty=$((width - filled))
    
    local bar=""
    [[ $filled -gt 0 ]] && bar=$(printf '█%.0s' $(seq 1 "$filled"))
    [[ $empty -gt 0 ]] && bar="${bar}$(printf '░%.0s' $(seq 1 "$empty"))"
    
    if has_color; then
        printf "${COLOR_CYAN}%s${COLOR_RESET} [%s] ${COLOR_GREEN}%d%%${COLOR_RESET} (%d/%d)\n" \
               "$label" "$bar" "$percent" "$current" "$total"
    else
        printf "%s [%s] %d%% (%d/%d)\n" "$label" "$bar" "$percent" "$current" "$total"
    fi
}

# Test metrics visualization
metrics_box() {
    local passed="$1"
    local failed="$2"
    local skipped="$3"
    local duration="${4:-0}"
    
    local total=$((passed + failed + skipped))
    local pass_rate=0
    [[ $total -gt 0 ]] && pass_rate=$((passed * 100 / total))
    
    local content="Total Tests: $total
$(status_indicator success "Passed:  $passed ($pass_rate%)")
$(status_indicator error "Failed:  $failed")
$(status_indicator skip "Skipped: $skipped")
⏱️ Duration: ${duration}s"
    
    if has_boxy; then
        echo "$content" | boxy --theme info --title "📊 METRICS"
    else
        echo "=== METRICS ==="
        echo "$content"
        echo "==============="
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# STRUCTURED OUTPUT FORMATTERS
# ═══════════════════════════════════════════════════════════════════════════════

# Table header with columns
table_header() {
    local -a columns=("$@")
    local width=20  # Default column width
    
    if has_color; then
        printf "${COLOR_BOLD}"
        for col in "${columns[@]}"; do
            printf "%-${width}s " "$col"
        done
        printf "${COLOR_RESET}\n"
        printf "${COLOR_CYAN}"
        printf '%.0s─' $(seq 1 $(( ${#columns[@]} * (width + 1) )))
        printf "${COLOR_RESET}\n"
    else
        for col in "${columns[@]}"; do
            printf "%-${width}s " "$col"
        done
        echo
        printf '%.0s─' $(seq 1 $(( ${#columns[@]} * (width + 1) )))
        echo
    fi
}

# Table row with optional status coloring
table_row() {
    local status="$1"
    shift
    local -a values=("$@")
    local width=20
    
    local color=""
    case "$status" in
        success) color="$COLOR_GREEN" ;;
        error)   color="$COLOR_RED" ;;
        warning) color="$COLOR_YELLOW" ;;
        *)       color="$COLOR_WHITE" ;;
    esac
    
    if has_color; then
        printf "$color"
        for val in "${values[@]}"; do
            printf "%-${width}s " "$val"
        done
        printf "${COLOR_RESET}\n"
    else
        for val in "${values[@]}"; do
            printf "%-${width}s " "$val"
        done
        echo
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# INTERACTIVE ELEMENTS
# ═══════════════════════════════════════════════════════════════════════════════

# Confirmation prompt with visual feedback
confirm_prompt() {
    local question="$1"
    local default="${2:-n}"  # y or n
    
    local prompt
    if [[ "$default" == "y" ]]; then
        prompt="[Y/n]"
    else
        prompt="[y/N]"
    fi
    
    if has_boxy; then
        echo "$question $prompt" | boxy --theme warn --title "❓ CONFIRM"
    else
        echo "❓ $question $prompt"
    fi
    
    read -r -n 1 answer
    echo
    
    # Handle default
    [[ -z "$answer" ]] && answer="$default"
    
    # Return 0 for yes, 1 for no
    [[ "${answer,,}" == "y" ]]
}

# Spinner for long operations
show_spinner() {
    local pid="$1"
    local message="${2:-Processing}"
    local spinners="⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    local i=0
    
    while kill -0 "$pid" 2>/dev/null; do
        printf "\r${COLOR_CYAN}%s${COLOR_RESET} %s" \
               "${spinners:i:1}" "$message"
        i=$(( (i + 1) % ${#spinners} ))
        sleep 0.1
    done
    printf "\r%*s\r" $((${#message} + 2)) ""  # Clear line
}

# ═══════════════════════════════════════════════════════════════════════════════
# COMPOSITE VISUAL FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

# Complete test result display
test_result_display() {
    local test_name="$1"
    local test_id="$2"
    local status="$3"
    local message="${4:-}"
    local details="${5:-}"
    
    # Main result box
    status_box "$status" "Test $test_id: $test_name" "$message"
    
    # Additional details if provided
    if [[ -n "$details" ]]; then
        echo "$details" | while IFS= read -r line; do
            echo "  │ $line"
        done
    fi
    echo
}

# Test phase summary with all metrics
phase_summary() {
    local phase_name="$1"
    local passed="$2"
    local failed="$3"
    local skipped="$4"
    local duration="$5"
    
    ceremonial_banner "PHASE COMPLETE: $phase_name"
    metrics_box "$passed" "$failed" "$skipped" "$duration"
    
    # Overall status
    if [[ $failed -eq 0 ]]; then
        status_box success "All tests in phase passed!"
    else
        status_box error "$failed tests failed in this phase"
    fi
}

# Visual test runner wrapper
visual_test_runner() {
    local test_file="$1"
    local test_id="$2"
    local test_name="$3"
    
    # Starting banner
    transition_banner "STARTING" "$test_name" "arrow"
    
    # Run with spinner
    (bash "$test_file" 2>&1) &
    local pid=$!
    show_spinner "$pid" "Running $test_name"
    wait "$pid"
    local exit_code=$?
    
    # Result display
    if [[ $exit_code -eq 0 ]]; then
        test_result_display "$test_name" "$test_id" "success"
    else
        test_result_display "$test_name" "$test_id" "error" "Exit code: $exit_code"
    fi
    
    return $exit_code
}

# ═══════════════════════════════════════════════════════════════════════════════
# EXPORT ALL FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

export -f has_boxy has_color
export -f test_phase_super test_subphase_box
export -f status_box status_indicator
export -f ceremonial_banner transition_banner section_divider
export -f decolorize colorize_pattern colorize_status
export -f format_kv parse_data_string data_to_box stream_kv_mapper
export -f progress_bar metrics_box
export -f table_header table_row
export -f confirm_prompt show_spinner
export -f test_result_display phase_summary visual_test_runner

# ═══════════════════════════════════════════════════════════════════════════════
# USAGE DOCUMENTATION
# ═══════════════════════════════════════════════════════════════════════════════

show_ux_kit_usage() {
    ceremonial_banner "TEST UX KIT - Visual Generation Toolkit" \
                     "Divine visual ceremonies for test excellence"
    
    section_divider "AVAILABLE FUNCTIONS" "="
    
    table_header "Category" "Function" "Purpose"
    table_row info "Phase Boxing" "test_phase_super" "Major phase ceremonies"
    table_row info "Status" "status_box" "Themed status messages"
    table_row info "Banners" "ceremonial_banner" "Full-width ceremonies"
    table_row info "Data" "parse_data_string" "_data paradigm parsing"
    table_row info "Metrics" "metrics_box" "Test result summaries"
    table_row info "Progress" "progress_bar" "Visual progress tracking"
    table_row info "Interactive" "confirm_prompt" "User confirmations"
    
    echo
    section_divider "USAGE EXAMPLE" "-"
    
    if has_boxy; then
        cat <<-'EOF' | boxy --theme info --title "📚 EXAMPLE"
		source test-ux-kit.sh
		
		test_phase_super 1 "VALIDATION" "Core system checks"
		status_box success "Dependencies verified"
		
		data="name=test;status=pass;time=1.2s"
		data_to_box "$data" "Test Result"
		
		metrics_box 10 2 1 15
		EOF
    else
        echo "source test-ux-kit.sh"
        echo "test_phase_super 1 'VALIDATION' 'Core system checks'"
        echo "status_box success 'Dependencies verified'"
    fi
}

# Show usage if sourced with --help
[[ "${1:-}" == "--help" ]] && show_ux_kit_usage

# ═══════════════════════════════════════════════════════════════════════════════
# DIVINE BLESSING
# ═══════════════════════════════════════════════════════════════════════════════

# May this toolkit bring visual excellence to all tests
# Through ceremonies grand and metrics clear
# Let no test pass without proper ceremony
# And no failure hide in darkness
# IX has spoken - Visual clarity shall reign!
