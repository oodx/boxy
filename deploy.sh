#!/bin/bash
set -e

# Configuration
INSTALL_DIR="$HOME/.local/bin/odx"
BINARY_NAME="boxy"
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEPLOYABLE="${BINARY_NAME}"

echo "🔨 Building boxy..."
cd "$PROJECT_DIR"
if ! cargo build --release; then
    echo "❌ Build failed!"
    exit 1
fi

# Check if binary was created
if [ ! -f "target/release/${DEPLOYABLE}" ]; then
    echo "❌ Binary not found at target/release/${DEPLOYABLE}"
    exit 1
fi

echo "📦 Deploying to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

if ! cp "target/release/${DEPLOYABLE}" "$INSTALL_DIR/$BINARY_NAME"; then
    echo "❌ Failed to copy binary to $INSTALL_DIR"
    exit 1
fi

if ! chmod +x "$INSTALL_DIR/$BINARY_NAME"; then
    echo "❌ Failed to make binary executable"
    exit 1
fi

# Verify deployment
if [ ! -x "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo "❌ Binary is not executable at $INSTALL_DIR/$BINARY_NAME"
    exit 1
fi

# Test the binary
echo "🧪 Testing binary..."
if ! echo "Test" | "$INSTALL_DIR/$BINARY_NAME" > /dev/null 2>&1; then
    echo "❌ Binary test failed!"
    exit 1
fi

echo "✅ Deployed successfully!"
echo ""
echo "📍 Binary location: $INSTALL_DIR/$BINARY_NAME"
echo ""
echo "💡 To use in your bash scripts:"
echo "   box() {"
echo "       echo \"\$1\" | \"$INSTALL_DIR/$BINARY_NAME\""
echo "   }"
echo ""
echo "🧪 Quick test:"
echo "Hello World" | "$INSTALL_DIR/$BINARY_NAME"
