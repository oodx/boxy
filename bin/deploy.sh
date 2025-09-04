#!/bin/bash
set -e

# Configuration
INSTALL_DIR="$HOME/.local/bin/odx"
BINARY_NAME="boxy"
# Resolve repository root from bin/
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
DEPLOYABLE="${BINARY_NAME}"

# Extract version from Cargo.toml at repo root
VERSION=$(grep '^version' "$ROOT_DIR/Cargo.toml" | head -1 | cut -d'"' -f2)

# Display deployment ceremony
echo "╔════════════════════════════════════════════════╗"
echo "║             BOXY DEPLOYMENT CEREMONY           ║"
echo "╠════════════════════════════════════════════════╣"
echo "║ Package: $BINARY_NAME                          ║"
echo "║ Version: v$VERSION (Theme System + 90+ Colors) ║"
echo "║ Target:  $INSTALL_DIR/$BINARY_NAME             ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

echo "🔨 Building boxy v$VERSION..."
cd "$ROOT_DIR"
if ! cargo build --release; then
    echo "❌ Build failed!"
    exit 1
fi

# Check if binary was created (at repo root)
if [ ! -f "$ROOT_DIR/target/release/${DEPLOYABLE}" ]; then
    echo "❌ Binary not found at target/release/${DEPLOYABLE}"
    exit 1
fi

echo "📦 Deploying to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

if ! cp "$ROOT_DIR/target/release/${DEPLOYABLE}" "$INSTALL_DIR/$BINARY_NAME"; then
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

echo ""
echo "╔════════════════════════════════════════════════╗"
echo "║          DEPLOYMENT SUCCESSFUL!                ║"
echo "╠════════════════════════════════════════════════╣"
echo "║  Deployed: boxy v$VERSION                      ║"
echo "║  Location: $INSTALL_DIR/$BINARY_NAME           ║"
echo "║  Features: Theme System, 90+ Colors, Headers   ║"
echo "╚════════════════════════════════════════════════╝"
echo ""
echo "💡 To use in your bash scripts:"
echo "   box() {"
echo "       echo \"\$1\" | \"$INSTALL_DIR/$BINARY_NAME\""
echo "   }"
echo ""
echo "🎨 Quick test of boxy v$VERSION theme system:"
echo "Deploy successful!" | "$INSTALL_DIR/$BINARY_NAME" --theme success --header "🚀 Boxy v$VERSION"
echo ""
echo "📖 Explore features:"
echo "   $INSTALL_DIR/$BINARY_NAME --colors    # View 90+ color palette"
echo "   $INSTALL_DIR/$BINARY_NAME theme list  # Theme management"
echo "   $ROOT_DIR/bin/ux.sh                   # Feature demonstration"
