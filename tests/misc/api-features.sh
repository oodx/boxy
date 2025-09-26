#!/bin/bash

# API Features Test - demonstrates all BoxBuilder API capabilities
# Runs the complete_features example to showcase the full API surface

# Resolve repo root
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

echo "========================================="
echo "BOXY API FEATURES DEMONSTRATION"
echo "========================================="
echo
echo "Running complete_features.rs example..."
echo "This demonstrates all API capabilities:"
echo "  - Header/Footer/Status alignment"
echo "  - Padding control (h/v)"
echo "  - Text wrapping"
echo "  - Height constraints"
echo "  - Status divider control"
echo "  - Real-world use cases"
echo
echo "-----------------------------------------"
echo

cd "$ROOT_DIR"
cargo run --example complete_features