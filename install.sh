#!/usr/bin/env bash
set -e

BINARY="lcars-ops"
INSTALL_DIR="/usr/bin"

echo "Building $BINARY (release)..."
cargo build --release

echo "Installing $BINARY to $INSTALL_DIR..."
sudo install -Dm755 "target/release/$BINARY" "$INSTALL_DIR/$BINARY"

echo "Done. Run with: $BINARY"
