#!/bin/bash

APP_NAME="rust_rss"
BUILD_DIR="target/release"
INSTALL_DIR="/usr/local/bin"

# Build the release binary
echo "Building $APP_NAME..."
cargo build --release || { echo "Build failed!"; exit 1; }

# Move the binary (requires sudo)
echo "Installing $APP_NAME to $INSTALL_DIR..."
sudo cp "$BUILD_DIR/$APP_NAME" "$INSTALL_DIR/" || { echo "Failed to copy binary!"; exit 1; }

# Reload shell
exec "$SHELL"

echo "'$APP_NAME' installed to $INSTALL_DIR. Run it with '$APP_NAME'."

