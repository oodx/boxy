#!/bin/bash
# Pantheon Test Entry Point
# Unified interface for running all pantheon tests

set -e

# Configuration
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TEST_DIR="$ROOT_DIR/tests"


BOXY="./target/release/boxy"


# Parse optional flags (can be anywhere in arguments)
TEST_SLEEP=""
NO_SLEEP="false"
QUICK_MODE="true"  # Default to quick mode
COMPREHENSIVE_MODE="false"
ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --sleep)
            TEST_SLEEP="$2"
            shift 2
            ;;
        --no-sleep)
            NO_SLEEP="true"
            shift 1
            ;;
        --quick)
            QUICK_MODE="true"
            COMPREHENSIVE_MODE="false"
            shift 1
            ;;
        --comprehensive|--full)
            QUICK_MODE="false"
            COMPREHENSIVE_MODE="true"
            shift 1
            ;;
        *)
            ARGS+=("$1")
            shift 1
            ;;
    esac
done

# Restore non-flag arguments
set -- "${ARGS[@]}"

# Available tests
declare -A TESTS=(
    # Canonical names
    ["suite:all"]="misc/all-ceremonies.sh"
    ["batch:foundation"]="misc/foundation-batch.sh"
    ["ceremony:01"]="misc/ceremony-01.sh"
    ["demo:ux"]="misc/ux.sh"
    ["smoke:minimal"]="misc/sanity-test.sh"
    ["comprehensive"]="misc/comprehensive-features.sh"
    ["varied"]="misc/perfect-demo.sh"
    ["perfect"]="misc/perfect.sh"
    ["uat:pantheon"]="uat/ceremony.sh"
    ["suite:uat-helpers"]="misc/uat-ceremonies.sh"

    # Legacy aliases (kept for compatibility)
    ["all-ceremonies"]="misc/all-ceremonies.sh"
    ["foundation"]="misc/foundation-batch.sh"
    ["ceremony-01"]="misc/ceremony-01.sh"
    ["boxy-demo"]="misc/ux.sh"
    ["minimal"]="misc/sanity-test.sh"
    ["features"]="../bin/feature-test.sh"
    ["ceremony"]="uat/ceremony.sh"
    ["uat"]="misc/uat-ceremonies.sh"
)

show_help() {
    if command -v $BOXY >/dev/null 2>&1; then
        cat <<-EOF | $BOXY --theme info --title "🧪 Pantheon Test Runner" --width max
Available Commands:
  test.sh [--comprehensive] [--sleep N|--no-sleep] run <test>   Run specific test (quick by default)
  test.sh list                                       List available tests
  test.sh help                                       Show this help

Options:
  --comprehensive        Run full validation (200+ tests per ceremony)
  --quick                Force quick mode (default, 1 test per ceremony)  
  --sleep N              Add sleep/timeout of N seconds between demo steps
  --no-sleep             Disable all sleeps (default behavior)

Available Tests:
  uat                    Comprehensive UAT ceremonies with boxy integration
  ceremony               Security-hardened pantheon ceremony
  minimal                Basic boxy functionality tests
  comprehensive          Complete feature coverage test (all v0.8 features)
  perfect                The Perfect Unit Test
	varied 								 Beautiful showcase demo with realistic scenarios
  boxy-demo              Full v0.6.0 milestone demonstration
  foundation             Complete foundation API ceremony batch (5 ceremonies)
  ceremony-01            Single foundation ceremony demo
  all-ceremonies         Run ALL 15 ceremonies (Foundation+Intermediate+Advanced)
EOF
    else
        echo "🧪 PANTHEON TEST RUNNER"
        echo "======================"
        echo
        echo "Available Commands:"
        echo "  test.sh [--comprehensive] [--sleep N|--no-sleep] run <test>   Run specific test (quick by default)"
        echo "  test.sh list                                       List available tests" 
        echo "  test.sh help                                       Show this help"
        echo ""
        echo "Options:"
        echo "  --comprehensive        Run full validation (200+ tests per ceremony)"
        echo "  --quick                Force quick mode (default, 1 test per ceremony)"
        echo "  --sleep N              Add sleep/timeout of N seconds between demo steps"
        echo "  --no-sleep             Disable all sleeps (default behavior)"
        echo
        echo "Available Tests:"
        echo "  uat                    Comprehensive UAT ceremonies with boxy integration"
        echo "  ceremony               Security-hardened pantheon ceremony" 
        echo "  minimal                Basic boxy functionality tests
  comprehensive          Complete feature coverage test (all v0.8 features)
  perfect                Beautiful showcase demo with realistic scenarios"
        echo "  boxy-demo              Full v0.6.0 milestone demonstration"
        echo "  foundation             Complete foundation API ceremony batch (5 ceremonies)"
        echo "  ceremony-01            Single foundation ceremony demo"
        echo "  all-ceremonies         Run ALL 15 ceremonies (Foundation+Intermediate+Advanced)"
    fi
}

list_tests() {
    if command -v $BOXY >/dev/null 2>&1; then
        {
            echo "Available Tests:"
            echo
            for test_name in $(printf "%s\n" "${!TESTS[@]}" | sort); do
                test_file="${TESTS[$test_name]}"
                if [[ -f "$TEST_DIR/$test_file" ]]; then
                    echo "✅ $test_name → $test_file"
                else
                    echo "❌ $test_name → $test_file (missing)"
                fi
            done
        } | $BOXY --theme info --title "🗂️ Available Tests" --width max
    else
        echo "🗂️ AVAILABLE TESTS"
        echo "=================="
        for test_name in $(printf "%s\n" "${!TESTS[@]}" | sort); do
            test_file="${TESTS[$test_name]}"
            if [[ -f "$TEST_DIR/$test_file" ]]; then
                echo "✅ $test_name → $test_file"
            else
                echo "❌ $test_name → $test_file (missing)"
            fi
        done
    fi
}

run_test() {
    local test_name="$1"
    
    if [[ -z "$test_name" ]]; then
        echo "❌ Error: Test name required"
        echo "Use: test.sh run <test>"
        echo "Available tests: ${!TESTS[*]}"
        exit 1
    fi
    
    if [[ ! "${TESTS[$test_name]+exists}" ]]; then
        echo "❌ Error: Unknown test '$test_name'"
        echo "Available tests: ${!TESTS[*]}"
        exit 1
    fi
    
    local test_file="${TESTS[$test_name]}"
    local test_path="$TEST_DIR/$test_file"
    
    if [[ ! -f "$test_path" ]]; then
        echo "❌ Error: Test file not found: $test_path"
        exit 1
    fi
    
    if command -v $BOXY >/dev/null 2>&1; then
        echo "🚀 Running test: $test_name" | $BOXY --theme success --title "🧪 Test Runner" --width max
    else
        echo "🚀 Running test: $test_name"
        echo "===================="
    fi
    echo
    
    # Change to project root and run test
    cd "$ROOT_DIR"
    
    # Export flags if provided
    if [[ "$NO_SLEEP" == "true" ]]; then
        export TEST_SLEEP=0
        export TEST_SECTION_DELAY=0
        export TEST_STEP_DELAY=0
    elif [[ -n "$TEST_SLEEP" ]]; then
        # Apply same delay to both step and section unless overridden upstream
        export TEST_SLEEP="$TEST_SLEEP"
        export TEST_SECTION_DELAY="${TEST_SECTION_DELAY:-$TEST_SLEEP}"
        export TEST_STEP_DELAY="${TEST_STEP_DELAY:-$TEST_SLEEP}"
    fi
    if [[ "$QUICK_MODE" == "true" ]]; then
        export QUICK_MODE="true"
        export CEREMONY_QUICK="true"
    fi
    if [[ "$COMPREHENSIVE_MODE" == "true" ]]; then
        export COMPREHENSIVE_MODE="true"
        export CEREMONY_QUICK="false"
    fi
    
    exec bash "$test_path"
}

# Main command dispatch
case "${1:-help}" in
    "run")
        run_test "$2"
        ;;
    "list")
        list_tests
        ;;
    "help"|"--help"|"-h")
        show_help
        ;;
    *)
        echo "❌ Unknown command: $1"
        echo "Use: test.sh help"
        exit 1
        ;;
esac
