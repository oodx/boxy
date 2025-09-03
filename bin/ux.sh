#!/bin/bash
# Boxy v0.6.0 UAT – Phase Ceremonies (M1–M4)
# Demonstrates features in milestone phases with explicit, numbered steps

set -e

# Configuration
BOXY="./target/release/boxy"
DEMO_DELAY=1.2
SECTION_DELAY=1

echo_demo() {
    echo "$ $1"
    eval "$1"
    sleep $DEMO_DELAY
    echo
}

section_header() {
    echo
    echo "┌─────────────────────────────────────────────────────────────────────┐"
    echo "│ $1"
    echo "└─────────────────────────────────────────────────────────────────────┘"
    echo
    sleep $SECTION_DELAY
}

phase_header() {
    local title="$1"
    echo
    echo "$title" | $BOXY --title "🎯 Phase Ceremony" --style heavy --width 72
    echo
}

step_card() {
    local step="$1"; shift
    local desc="$*"
    printf "%s\n%s\n" "Step ${step}" "• ${desc}" | $BOXY --style rounded --width 72 --title "📋 UAT Step ${step}"
    echo
}

# Build first to ensure everything works
echo "🔨 Building boxy v0.6.0..."
cargo build --release
echo

# Welcome
echo "🎁 BOXY v0.6.0 – UAT Ceremonies" | $BOXY --theme success --title "✅ UAT Start" --width 64
echo
sleep 1

# ========== M1: Foundation Architecture ==========
phase_header "🏗️ M1: Foundation Architecture"

step_card 1.1 "Header/Title semantics (spec vs current)"
echo "Spec Note: Current engine renders --header above box; --title inside top border. Feedback requests header inside border (deferred)." | $BOXY --style ascii --width 72
echo_demo "echo 'System operational' | $BOXY --header '🖥️ MyApp v2.1' --title '✅ Online'"

step_card 1.2 "Status bar visibility and alignment"
echo_demo "echo 'Status visibly demonstrated' | $BOXY --title '📊 Status Demo' --status 'sl:LEFT | sc:CENTER | sr:RIGHT' --width 72"

step_card 1.3 "Width + truncation"
echo_demo "echo 'This is a very long message that will be automatically truncated' | $BOXY --width 30 --color azure"

step_card 1.4 "Color palette ceremony"
echo_demo "echo 'Legacy color' | $BOXY --color red --style rounded"
echo_demo "echo 'Semantic error' | $BOXY --color error --text white"
echo_demo "echo 'Rich crimson' | $BOXY --color crimson --text white --style heavy"

# ========== M2: Theme Management ==========
phase_header "🎨 M2: Theme Management"

step_card 2.1 "List available themes"
echo_demo "$BOXY theme list | head -12"

step_card 2.2 "Apply built-in themes"
echo_demo "echo 'Critical system failure detected' | $BOXY --theme error"
echo_demo "echo 'Backup completed successfully' | $BOXY --theme success"
echo_demo "echo 'API endpoint deprecated' | $BOXY --theme warning"

step_card 2.3 "Preview theme details (export preview)"
echo_demo "$BOXY theme export error | sed -n '1,24p'"
echo "Note: 'theme show' is deferred; export used to preview." | $BOXY --style ascii --width 72

step_card 2.4 "Create/edit/import (non-interactive demo)"
echo_demo "mkdir -p /tmp/boxy-uat 2>/dev/null || true"
echo "Demo: theme create/edit/import available; interactive prompts are deferred in UAT." | $BOXY --style ascii --width 72

# ========== M3: Migration & Compatibility ==========
phase_header "🔧 M3: Migration & Compatibility"

step_card 3.1 "Icon+Title migration"
echo "OLD: --icon ✅ --title 'Status'  →  NEW: --title '✅ Status'" | $BOXY --width 72 --style rounded
echo_demo "$BOXY migrate-commands --examples | head -12"

step_card 3.2 "Deprecation warning demo"
echo_demo "echo 'Check' | $BOXY --icon '📊' --title 'Analytics'"

step_card 3.3 "Input precedence"
echo_demo "echo 'Ignored pipe' | $BOXY theme list | head -5"
echo_demo "echo 'Used pipe' | $BOXY --theme info --title '📥 Pipe'"

# ========== M4: Production Readiness ==========
phase_header "🚀 M4: Production Readiness"

step_card 4.1 "Jynx integration"
echo_demo "$BOXY --version"
echo_demo "$BOXY --help | head -10"

step_card 4.2 "Pipeline utility ceremony (--no-boxy)"
BOXED=$(echo 'Pipeline content' | $BOXY --theme info --title '📦 Boxed')
echo "$BOXED" | sed -n '1,4p'
echo_demo "echo \"\"" # spacer
echo_demo "echo \"$BOXED\" | $BOXY --no-boxy | sed -n '1,3p'"
echo_demo "echo \"$BOXED\" | $BOXY --no-boxy=strict | sed -n '1,3p'"

step_card 4.3 "Performance sanity (short)"
echo_demo "bash -lc 'for i in {1..10}; do echo t | $BOXY --theme success >/dev/null; done; echo Done'"

section_header "🎉 DEMONSTRATION COMPLETE"
echo "Phase ceremonies completed (M1–M4). See CODEX_FEEDBACK1.yml for deferred items." | $BOXY --theme success --title "✅ UAT Complete" --width 64
echo
echo "Next steps: --colors, theme list, migrate-commands --examples, THEME_SYSTEM_v0.6.md" | $BOXY --style ascii --width 64

