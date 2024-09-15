#!/bin/bash

# Determine OS and architecture
OS="$(uname)"
ARCH="$(uname -m)"

if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then
        BINARY_URL="https://github.com/ryanwashacked/dirtree/releases/latest/download/dirtree-macos-arm"
    else
        BINARY_URL="https://github.com/ryanwashacked/dirtree/releases/latest/download/dirtree-macos-intel"
    fi
elif [ "$OS" = "Linux" ]; then
    BINARY_URL="https://github.com/ryanwashacked/dirtree/releases/latest/download/dirtree-linux"
else
    echo "Unsupported operating system: $OS"
    exit 1
fi

# Download binary
curl -L $BINARY_URL -o dirtree

# Make binary executable
chmod +x dirtree

# Move to a directory in PATH
sudo mv dirtree /usr/local/bin/

echo "dirtree has been installed successfully!"