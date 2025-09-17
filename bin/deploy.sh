#!/bin/bash
set -e

# Configuration
LIB_DIR="$HOME/.local/lib/odx/boxylib"
BIN_DIR="$HOME/.local/bin/odx"
BINARY_NAME="boxy"

lib_file="$LIB_DIR/$BINARY_NAME"
bin_file="$BIN_DIR/$BINARY_NAME"

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

echo "📦 Building boxy lib at $LIB_DIR..."
mkdir -p "$BIN_DIR" "$LIB_DIR"

if [ -f "$lib_file" ]; then 
	echo "📦 Removing previous boxy lib"
	rm "$lib_file"
	rm "$bin_file"
	#ls "$lib_file";
	#ls "$bin_file";
fi

if ! cp "$ROOT_DIR/target/release/${DEPLOYABLE}" "$lib_file"; then
    echo "❌ Failed to copy binary to $lib_file"
    exit 1
fi

if ! chmod +x "$lib_file"; then
	echo "❌ Failed to make binary executable"
	exit 1
fi

if ! ln -s "$lib_file" "$bin_file"; then
	echo "❌ Failed to make link boxy lib to odx bin"
	exit 1
fi

# Verify deployment
if [ ! -x "$bin_file" ]; then
    echo "❌ Binary is not executable at $bin_file"
    exit 1
fi

# Test the binary
echo "🧪 Testing binary..."
if ! echo "Test" | "$bin_file" > /dev/null 2>&1; then
    echo "❌ Binary test failed!"
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════════════╗"
echo "║          DEPLOYMENT SUCCESSFUL!                ║"
echo "╚════════════════════════════════════════════════╝"
echo "  Deployed: boxy v$VERSION                       "
echo "  Location: $bin_file                            "
echo ""
echo "💡 To use in your bash scripts:"
echo "   boxy_print() {"
echo "       echo \"\$1\" | \"$bin_file\""
echo "   }"
echo ""
echo "🎨 Quick test of boxy v$VERSION theme system:"

echo "Deploy successful!" | ${BINARY_NAME} --theme success --header "🚀 Boxy v$VERSION"

echo ""
echo "📖 Explore features:"
echo "   $INSTALL_DIR/$BINARY_NAME --colors    # View 90+ color palette"
echo "   $INSTALL_DIR/$BINARY_NAME theme list  # Theme management"
echo "   $ROOT_DIR/bin/ux.sh                   # Feature demonstration"
