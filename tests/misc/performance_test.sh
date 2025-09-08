#!/bin/bash
# Performance testing script for boxy v0.6.0
# Tests theme system performance and overall rendering speed

set -e

echo "🚀 Boxy v0.6.0 Performance Test Suite"
echo "====================================="

# Build optimized binary first
echo "📦 Building optimized binary..."
cargo build --release > /dev/null 2>&1
BOXY_BIN="./target/release/boxy"

if [ ! -f "$BOXY_BIN" ]; then
    echo "❌ Failed to build optimized binary"
    exit 1
fi

echo "✅ Optimized binary ready"
echo ""

# Test 1: Basic rendering performance
echo "🧪 Test 1: Basic Rendering Performance"
echo "---------------------------------------"
echo "Testing 1000 basic box renderings..."

start_time=$(date +%s%3N)
for i in {1..1000}; do
    echo "Render test $i" | $BOXY_BIN > /dev/null 2>&1
done
end_time=$(date +%s%3N)

basic_duration=$((end_time - start_time))
basic_avg=$(echo "scale=3; $basic_duration / 1000" | bc -l)

echo "⏱️  Total time: ${basic_duration}ms"
echo "📊 Average per render: ${basic_avg}ms"
echo ""

# Test 2: Theme system performance
echo "🧪 Test 2: Theme System Performance"
echo "------------------------------------"
echo "Testing 500 theme-based renderings..."

themes=("error" "success" "warning" "info")
start_time=$(date +%s%3N)

for i in {1..500}; do
    theme_idx=$((i % 4))
    theme=${themes[$theme_idx]}
    echo "Theme test $i" | $BOXY_BIN --theme "$theme" > /dev/null 2>&1
done

end_time=$(date +%s%3N)
theme_duration=$((end_time - start_time))
theme_avg=$(echo "scale=3; $theme_duration / 500" | bc -l)

echo "⏱️  Total time: ${theme_duration}ms"
echo "📊 Average per render: ${theme_avg}ms"
echo ""

# Test 3: Color system performance
echo "🧪 Test 3: Color System Performance"
echo "------------------------------------"
echo "Testing 300 color variations..."

colors=("crimson" "emerald" "azure" "amber" "violet" "coral" "sage" "steel")
start_time=$(date +%s%3N)

for i in {1..300}; do
    color_idx=$((i % 8))
    color=${colors[$color_idx]}
    echo "Color test $i" | $BOXY_BIN --color "$color" > /dev/null 2>&1
done

end_time=$(date +%s%3N)
color_duration=$((end_time - start_time))
color_avg=$(echo "scale=3; $color_duration / 300" | bc -l)

echo "⏱️  Total time: ${color_duration}ms"
echo "📊 Average per render: ${color_avg}ms"
echo ""

# Test 4: Complex feature combination performance
echo "🧪 Test 4: Complex Feature Performance"
echo "---------------------------------------"
echo "Testing 200 complex renderings with multiple features..."

start_time=$(date +%s%3N)

for i in {1..200}; do
    theme_idx=$((i % 4))
    theme=${themes[$theme_idx]}
    echo "Complex feature test with multiple options and longer content that should trigger various rendering pathways including width constraints and status bars" | \
        $BOXY_BIN --theme "$theme" --header "Performance Test #$i" --width 60 --status "sc:Test $i/200" > /dev/null 2>&1
done

end_time=$(date +%s%3N)
complex_duration=$((end_time - start_time))
complex_avg=$(echo "scale=3; $complex_duration / 200" | bc -l)

echo "⏱️  Total time: ${complex_duration}ms"
echo "📊 Average per render: ${complex_avg}ms"
echo ""

# Test 5: Large content performance
echo "🧪 Test 5: Large Content Performance"
echo "-------------------------------------"
echo "Testing 100 large content renderings..."

# Create large test content
large_content=""
for i in {1..50}; do
    large_content+="This is line $i of large content that should test the performance of boxy with substantial text input including unicode characters ✅ 🚀 📦 and various symbols. "
done

start_time=$(date +%s%3N)

for i in {1..100}; do
    echo "$large_content" | $BOXY_BIN --theme info --width 80 > /dev/null 2>&1
done

end_time=$(date +%s%3N)
large_duration=$((end_time - start_time))
large_avg=$(echo "scale=3; $large_duration / 100" | bc -l)

echo "⏱️  Total time: ${large_duration}ms"
echo "📊 Average per render: ${large_avg}ms"
echo ""

# Test 6: Help system performance
echo "🧪 Test 6: Help System Performance"
echo "-----------------------------------"
echo "Testing help command performance..."

start_time=$(date +%s%3N)

for i in {1..50}; do
    $BOXY_BIN --help > /dev/null 2>&1
    $BOXY_BIN --examples > /dev/null 2>&1
    $BOXY_BIN theme list > /dev/null 2>&1
done

end_time=$(date +%s%3N)
help_duration=$((end_time - start_time))
help_avg=$(echo "scale=3; $help_duration / 150" | bc -l)

echo "⏱️  Total time: ${help_duration}ms"
echo "📊 Average per command: ${help_avg}ms"
echo ""

## Test 7 removed: Migration commands no longer exist in v0.8

# Memory usage test
echo "🧪 Test 8: Memory Usage Analysis"
echo "---------------------------------"
echo "Analyzing memory usage patterns..."

# Run a complex command and check memory
memory_output=$(echo "Memory usage test with complex features" | /usr/bin/time -v $BOXY_BIN --theme error --header "Memory Test" --status "sc:Testing" --width 60 2>&1 | grep "Maximum resident set size")
memory_kb=$(echo "$memory_output" | grep -o '[0-9]*')
memory_mb=$(echo "scale=2; $memory_kb / 1024" | bc -l)

echo "🧠 Peak memory usage: ${memory_mb}MB"
echo ""

# Performance summary
echo "📋 PERFORMANCE SUMMARY"
echo "======================"
echo "Basic rendering:      ${basic_avg}ms/render"
echo "Theme system:         ${theme_avg}ms/render"
echo "Color system:         ${color_avg}ms/render"
echo "Complex features:     ${complex_avg}ms/render"
echo "Large content:        ${large_avg}ms/render"
echo "Help commands:        ${help_avg}ms/command"
echo "Migration commands:   ${migration_avg}ms/command"
echo "Peak memory usage:    ${memory_mb}MB"
echo ""

# Performance benchmarks and recommendations
echo "🎯 PERFORMANCE ANALYSIS"
echo "========================"

# Check if basic rendering is fast enough (target: <5ms)
if (( $(echo "$basic_avg < 5" | bc -l) )); then
    echo "✅ Basic rendering performance: EXCELLENT (<5ms target)"
else
    echo "⚠️  Basic rendering performance: Could be improved (${basic_avg}ms > 5ms target)"
fi

# Check theme system overhead
theme_overhead=$(echo "scale=3; $theme_avg - $basic_avg" | bc -l)
echo "📊 Theme system overhead: +${theme_overhead}ms"

if (( $(echo "$theme_overhead < 2" | bc -l) )); then
    echo "✅ Theme overhead: ACCEPTABLE (<2ms)"
else
    echo "⚠️  Theme overhead: HIGH (${theme_overhead}ms > 2ms target)"
fi

# Check memory usage
if (( $(echo "$memory_mb < 10" | bc -l) )); then
    echo "✅ Memory usage: EXCELLENT (<10MB)"
elif (( $(echo "$memory_mb < 25" | bc -l) )); then
    echo "✅ Memory usage: GOOD (<25MB)"
else
    echo "⚠️  Memory usage: HIGH (${memory_mb}MB > 25MB)"
fi

echo ""

# Optimization recommendations
echo "🔧 OPTIMIZATION RECOMMENDATIONS"
echo "================================"

if (( $(echo "$theme_avg > 10" | bc -l) )); then
    echo "• Consider optimizing theme lookup and resolution"
fi

if (( $(echo "$complex_avg > 15" | bc -l) )); then
    echo "• Complex feature combinations could be optimized"
fi

if (( $(echo "$large_avg > 20" | bc -l) )); then
    echo "• Large content processing could be optimized"
fi

if (( $(echo "$memory_mb > 20" | bc -l) )); then
    echo "• Memory usage optimization recommended"
fi

# Calculate overall performance score
total_tests=7
excellent_count=0

(( $(echo "$basic_avg < 5" | bc -l) )) && ((excellent_count++))
(( $(echo "$theme_avg < 8" | bc -l) )) && ((excellent_count++))
(( $(echo "$color_avg < 8" | bc -l) )) && ((excellent_count++))
(( $(echo "$complex_avg < 12" | bc -l) )) && ((excellent_count++))
(( $(echo "$large_avg < 15" | bc -l) )) && ((excellent_count++))
(( $(echo "$help_avg < 10" | bc -l) )) && ((excellent_count++))
(( $(echo "$memory_mb < 15" | bc -l) )) && ((excellent_count++))

performance_score=$(echo "scale=1; $excellent_count * 100 / $total_tests" | bc -l)

echo ""
echo "🏆 OVERALL PERFORMANCE SCORE: ${performance_score}%"

if (( $(echo "$performance_score >= 85" | bc -l) )); then
    echo "🌟 RATING: EXCELLENT - Production ready with outstanding performance"
elif (( $(echo "$performance_score >= 70" | bc -l) )); then
    echo "✅ RATING: GOOD - Production ready with solid performance"
elif (( $(echo "$performance_score >= 50" | bc -l) )); then
    echo "⚠️  RATING: ACCEPTABLE - Some optimizations recommended"
else
    echo "❌ RATING: NEEDS IMPROVEMENT - Optimization required before production"
fi

echo ""
echo "✨ Performance testing complete!"

# Store results for comparison
echo "Storing performance results for future comparison..."
echo "timestamp,basic_avg,theme_avg,color_avg,complex_avg,large_avg,help_avg,memory_mb,score" > performance_results.csv
echo "$(date '+%Y-%m-%d %H:%M:%S'),$basic_avg,$theme_avg,$color_avg,$complex_avg,$large_avg,$help_avg,$memory_mb,$performance_score" >> performance_results.csv

echo "📊 Results saved to performance_results.csv"
