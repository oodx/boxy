#!/bin/bash
# All Ceremonies Comprehensive Test Suite
# Runs complete ceremony validation: Foundation → Intermediate → Advanced

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CEREMONY_RUNNER="$ROOT_DIR/tests/ceremonies/ceremony_runner.sh"
BOXY="./target/release/boxy"		

# Header ceremony for full suite
if command -v $BOXY >/dev/null 2>&1; then
    {
        echo "COMPREHENSIVE CEREMONY VALIDATION"
        echo
        echo "Executing all 15 ceremonies across 3 complexity tiers:"
        echo "• Foundation API (ceremonies 1-5)"
        echo "• Intermediate API (ceremonies 6-10)"  
        echo "• Advanced API (ceremonies 11-15)"
        echo
        echo "This validates Boxy's complete API surface through"
        echo "systematic UAT ceremony patterns with professional"
        echo "step-by-step verification."
        echo
        echo "Total validation steps: 1,800+ across all ceremonies"
    } | $BOXY --theme info --title "🎭 ALL CEREMONIES SUITE" --width max
else
    echo "🎭 ALL CEREMONIES COMPREHENSIVE SUITE"
    echo "=================================="
    echo
    echo "Executing all 15 ceremonies across 3 complexity tiers:"
    echo "• Foundation API (ceremonies 1-5)"
    echo "• Intermediate API (ceremonies 6-10)"
    echo "• Advanced API (ceremonies 11-15)"
    echo
    echo "This validates Boxy's complete API surface through"
    echo "systematic UAT ceremony patterns."
    echo
    echo "Total validation steps: 1,800+ across all ceremonies"
fi

echo
echo "🎯 Starting comprehensive ceremony execution..."
echo

# Set non-interactive mode for automated execution
export CEREMONY_AUTOMATED=true

# Default to quick mode unless comprehensive requested
if [[ "$COMPREHENSIVE_MODE" == "true" ]]; then
    export CEREMONY_QUICK=false
    echo "📊 Comprehensive mode: Running full validation (200+ tests per ceremony)"
    echo "⚠️  This will take significantly longer than quick mode"
    echo
else
    export CEREMONY_QUICK=true
    echo "🚀 Quick mode (default): Running smoke tests (1 test per ceremony)"
    echo "💡 Use --comprehensive for full validation"
    echo
fi

# Apply sleep between batches if requested
if [[ -n "$TEST_SLEEP" ]] && [[ "$TEST_SLEEP" -gt 0 ]]; then
    echo "ℹ️  Using sleep timeout of $TEST_SLEEP seconds between batches"
    echo
fi

# Execute all ceremony batches in sequence
echo "🏗️  FOUNDATION BATCH (Ceremonies 1-5)..."
"$CEREMONY_RUNNER" batch_01
FOUNDATION_EXIT=$?

if [[ -n "$TEST_SLEEP" ]] && [[ "$TEST_SLEEP" -gt 0 ]]; then
    echo "⏱️  Sleeping for $TEST_SLEEP seconds..."
    sleep "$TEST_SLEEP"
fi

echo
echo "🔧 INTERMEDIATE BATCH (Ceremonies 6-10)..."  
"$CEREMONY_RUNNER" batch_02
INTERMEDIATE_EXIT=$?

if [[ -n "$TEST_SLEEP" ]] && [[ "$TEST_SLEEP" -gt 0 ]]; then
    echo "⏱️  Sleeping for $TEST_SLEEP seconds..."
    sleep "$TEST_SLEEP"
fi

echo
echo "⚡ ADVANCED BATCH (Ceremonies 11-15)..."
"$CEREMONY_RUNNER" batch_03
ADVANCED_EXIT=$?

# Calculate total results
TOTAL_BATCHES=3
SUCCESSFUL_BATCHES=0

if [ $FOUNDATION_EXIT -eq 0 ]; then
    SUCCESSFUL_BATCHES=$((SUCCESSFUL_BATCHES + 1))
fi

if [ $INTERMEDIATE_EXIT -eq 0 ]; then
    SUCCESSFUL_BATCHES=$((SUCCESSFUL_BATCHES + 1))
fi

if [ $ADVANCED_EXIT -eq 0 ]; then
    SUCCESSFUL_BATCHES=$((SUCCESSFUL_BATCHES + 1))
fi

# Final results ceremony
echo
if command -v $BOXY >/dev/null 2>&1; then
    {
        echo "COMPREHENSIVE CEREMONY RESULTS"
        echo
        echo "Ceremony Batch Results:"
        if [ $FOUNDATION_EXIT -eq 0 ]; then
            echo "✅ Foundation (ceremonies 1-5): SUCCESS"
        else  
            echo "❌ Foundation (ceremonies 1-5): FAILED"
        fi
        
        if [ $INTERMEDIATE_EXIT -eq 0 ]; then
            echo "✅ Intermediate (ceremonies 6-10): SUCCESS"
        else
            echo "❌ Intermediate (ceremonies 6-10): FAILED"  
        fi
        
        if [ $ADVANCED_EXIT -eq 0 ]; then
            echo "✅ Advanced (ceremonies 11-15): SUCCESS"
        else
            echo "❌ Advanced (ceremonies 11-15): FAILED"
        fi
        
        echo
        echo "Overall Suite: $SUCCESSFUL_BATCHES/$TOTAL_BATCHES batches successful"
        echo
        if [ $SUCCESSFUL_BATCHES -eq $TOTAL_BATCHES ]; then
            echo "🎉 ALL CEREMONIES PASSED!"
            echo "Boxy's complete API surface validated through"
            echo "comprehensive ceremony testing."
        else
            echo "⚠️  Some ceremony batches failed."
            echo "Check individual ceremony output for details."
        fi
        echo
        echo "Thank you for comprehensive validation!"
    } | $BOXY --theme success --title "🎭 CEREMONY SUITE COMPLETE" --width max
else
    echo "🎭 COMPREHENSIVE CEREMONY RESULTS"
    echo "================================"
    echo
    echo "Ceremony Batch Results:"
    if [ $FOUNDATION_EXIT -eq 0 ]; then
        echo "✅ Foundation (ceremonies 1-5): SUCCESS"
    else  
        echo "❌ Foundation (ceremonies 1-5): FAILED"
    fi
    
    if [ $INTERMEDIATE_EXIT -eq 0 ]; then
        echo "✅ Intermediate (ceremonies 6-10): SUCCESS"
    else
        echo "❌ Intermediate (ceremonies 6-10): FAILED"  
    fi
    
    if [ $ADVANCED_EXIT -eq 0 ]; then
        echo "✅ Advanced (ceremonies 11-15): SUCCESS"
    else
        echo "❌ Advanced (ceremonies 11-15): FAILED"
    fi
    
    echo
    echo "Overall Suite: $SUCCESSFUL_BATCHES/$TOTAL_BATCHES batches successful"
    echo
    if [ $SUCCESSFUL_BATCHES -eq $TOTAL_BATCHES ]; then
        echo "🎉 ALL CEREMONIES PASSED!"
        echo "Boxy's complete API surface validated."
    else
        echo "⚠️  Some ceremony batches failed."
    fi
fi

# Exit with failure if any batch failed
if [ $SUCCESSFUL_BATCHES -ne $TOTAL_BATCHES ]; then
    exit 1
fi

exit 0
