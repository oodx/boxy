#!/usr/bin/env bash
# Benchmark Suite - Comprehensive Performance Testing
set -euo pipefail

# Configuration
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

GLOBAL_BOXY="/home/xnull/.local/bin/odx/boxy"
LOCAL_BOXY="./target/release/boxy"
BENCHMARK_THRESHOLD_MS=2.0

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}🚀 BENCHMARK SUITE${NC}"
    echo "===================="
}

print_section() {
    echo -e "\n${YELLOW}$1${NC}"
    echo "$(printf '%.0s-' {1..40})"
}

# Ensure release build exists
ensure_release_build() {
    if [[ ! -f "$LOCAL_BOXY" ]]; then
        echo -e "${YELLOW}📦 Building release binary...${NC}"
        cargo build --release
    fi
}

# Run criterion benchmarks
run_criterion_benchmarks() {
    print_section "📊 CRITERION MICRO-BENCHMARKS"

    echo "Running cargo bench..."
    cargo bench --bench status_render -- --quiet

    if [[ "${SNAP_BENCHMARKS:-false}" == "true" ]]; then
        echo -e "${GREEN}📸 Preserving benchmark snapshots...${NC}"
        bash bin/snap.sh
    fi
}

# Extract time value from /usr/bin/time output (just the number)
parse_time_output() {
    local time_output="$1"
    # With /usr/bin/time -f "%e", output is just the elapsed time in seconds
    echo "$time_output" | grep -o '^[0-9.]*'
}

# Run time-based comparison tests
run_time_comparison() {
    print_section "⏱️  TIME COMPARISON: Global vs Local"

    # Test configuration - using simple realistic test case
    local test_input="Line 1\nLine 2 with emoji 🚀\nLine 3 final"
    local test_args="--theme base_rounded --title 'Benchmark Test' --width 60"

    echo "Test input: Short realistic content with emoji"
    echo "Test args: $test_args"
    echo

    # Run global boxy
    if [[ -x "$GLOBAL_BOXY" ]]; then
        echo -e "${BLUE}Global boxy:${NC} $GLOBAL_BOXY"
        local global_time_output
        global_time_output=$( /usr/bin/time -f "%e" sh -c "echo -e '$test_input' | $GLOBAL_BOXY $test_args >/dev/null" 2>&1 )
        local global_time=$(parse_time_output "$global_time_output")
        echo "Time: ${global_time}s"
        if [[ -z "$global_time" ]]; then
            echo "Failed to parse time from: $global_time_output"
            local global_time="N/A"
        fi
    else
        echo -e "${RED}❌ Global boxy not found at $GLOBAL_BOXY${NC}"
        local global_time="N/A"
    fi

    echo

    # Run local boxy
    echo -e "${BLUE}Local boxy:${NC} $LOCAL_BOXY"
    local local_time_output
    local_time_output=$( /usr/bin/time -f "%e" sh -c "echo -e '$test_input' | $LOCAL_BOXY $test_args >/dev/null" 2>&1 )
    local local_time=$(parse_time_output "$local_time_output")
    echo "Time: ${local_time}s"
    if [[ -z "$local_time" ]]; then
        echo "Failed to parse time from: $local_time_output"
        local local_time="N/A"
    fi

    echo

    # Performance comparison
    if [[ "$global_time" != "N/A" ]]; then
        echo -e "${YELLOW}📈 PERFORMANCE COMPARISON${NC}"
        local comparison_result
        comparison_result=$(awk -v local="$local_time" -v global="$global_time" '
            BEGIN {
                if (global > 0) {
                    ratio = local / global
                    percent_diff = (ratio - 1) * 100
                    if (percent_diff > 5) {
                        print "🔴 LOCAL SLOWER: " percent_diff "% slower than global"
                    } else if (percent_diff < -5) {
                        print "🟢 LOCAL FASTER: " (-percent_diff) "% faster than global"
                    } else {
                        print "🟡 SIMILAR PERFORMANCE: " percent_diff "% difference"
                    }
                } else {
                    print "Unable to compare (global time invalid)"
                }
            }
        ')
        echo "$comparison_result"
    fi

    # Threshold check
    echo
    echo -e "${YELLOW}🎯 THRESHOLD CHECK${NC}"
    local threshold_check
    threshold_check=$(awk -v time="$local_time" -v threshold="$BENCHMARK_THRESHOLD_MS" '
        BEGIN {
            if (time > threshold) {
                print "❌ FAIL: " time "s > " threshold "s threshold"
                exit 1
            } else {
                print "✅ PASS: " time "s ≤ " threshold "s threshold"
                exit 0
            }
        }
    ')
    echo "$threshold_check"

    # Return exit code based on threshold
    awk -v time="$local_time" -v threshold="$BENCHMARK_THRESHOLD_MS" 'BEGIN { exit (time > threshold) }'
}

# Analyze criterion output
analyze_criterion_output() {
    print_section "📋 CRITERION ANALYSIS"

    local criterion_dir="target/criterion"
    if [[ ! -d "$criterion_dir" ]]; then
        echo "❌ No criterion output found"
        return 1
    fi

    echo "Recent benchmark results:"
    find "$criterion_dir" -name "*.html" -type f -printf "📊 %f: %TY-%Tm-%Td %TH:%TM\n" | sort -r | head -5

    # Check for performance regression markers in estimates.json
    local estimates_files=($(find "$criterion_dir" -name "estimates.json" -type f))
    if [[ ${#estimates_files[@]} -gt 0 ]]; then
        echo
        echo "Latest performance estimates:"
        for estimates_file in "${estimates_files[@]}"; do
            local bench_name=$(dirname "$estimates_file" | xargs basename)
            local mean_estimate=$(grep -o '"mean":{"estimate":[0-9.]*' "$estimates_file" | cut -d: -f3)
            if [[ -n "$mean_estimate" ]]; then
                # Convert nanoseconds to milliseconds
                local mean_ms=$(awk -v ns="$mean_estimate" 'BEGIN { printf "%.3f", ns / 1000000 }')
                echo "  $bench_name: ${mean_ms}ms"
            fi
        done
    fi
}

# Main execution
main() {
    print_header
    echo "Benchmark mode: ${BENCHMARK_MODE:-false}"
    echo "Snap benchmarks: ${SNAP_BENCHMARKS:-false}"
    echo "Threshold: ${BENCHMARK_THRESHOLD_MS}ms"
    echo

    ensure_release_build

    # Run benchmarks
    run_criterion_benchmarks
    run_time_comparison
    analyze_criterion_output

    echo
    echo -e "${GREEN}✅ Benchmark suite completed successfully${NC}"
}

# Execute main function
main "$@"