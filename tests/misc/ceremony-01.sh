#!/bin/bash
# Ceremony 01 Demo - Wrapper for test.sh integration

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
export CEREMONY_AUTOMATED=true

# Default to quick mode unless comprehensive requested
if [[ "$COMPREHENSIVE_MODE" == "true" ]]; then
    export CEREMONY_QUICK=false
else
    export CEREMONY_QUICK=true
fi

exec "$ROOT_DIR/tests/ceremonies/ceremony_runner.sh" ceremony_01