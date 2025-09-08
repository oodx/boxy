#!/bin/bash
# Foundation Batch Test - Wrapper for test.sh integration

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
export CEREMONY_AUTOMATED=true
exec "$ROOT_DIR/tests/ceremonies/ceremony_runner.sh" batch_01