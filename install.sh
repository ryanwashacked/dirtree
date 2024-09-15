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
elif [ "$OS" = "MINGW64_NT-10.0" ] || [ "$OS" = "MSYS_NT-10.0" ]; then
    BINARY_URL="https://github.com/ryanwashacked/dirtree/releases/latest/download/dirtree-windows.exe"
else
    echo "Unsupported operating system: $OS"
    exit 1
fi

# Download binary
if [ "$OS" = "MINGW64_NT-10.0" ] || [ "$OS" = "MSYS_NT-10.0" ]; then
    curl -L $BINARY_URL -o dirtree.exe
else
    curl -L $BINARY_URL -o dirtree
fi

# Make binary executable (not needed for Windows)
if [ "$OS" != "MINGW64_NT-10.0" ] && [ "$OS" != "MSYS_NT-10.0" ]; then
    chmod +x dirtree
fi

# Move to a directory in PATH
if [ "$OS" = "MINGW64_NT-10.0" ] || [ "$OS" = "MSYS_NT-10.0" ]; then
    echo "Please move dirtree.exe to a directory in your PATH manually."
else
    sudo mv dirtree /usr/local/bin/
fi

echo "dirtree has been installed successfully!"