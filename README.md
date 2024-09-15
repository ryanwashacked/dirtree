# Dirtree

A tool for generating and updating directory structure representations in README files.

## Demo
![Video Demo](dirtree.gif)

## Installation

### macOS (Intel and ARM) and Linux

You can install dirtree with the following command:

```bash
curl -sSL https://raw.githubusercontent.com/ryanwashacked/dirtree/main/install.sh | bash
```

This will download the appropriate binary for your system (including ARM-based Macs) and install it in `/usr/local/bin`.

### Windows

1. Download the latest Windows binary (`dirtree-windows.exe`) from the [releases page](https://github.com/ryanwashacked/dirtree/releases/latest).
2. Rename the downloaded file to `dirtree.exe`.
3. Move the `dirtree.exe` file to a directory that's in your system's PATH.

Alternatively, if you're using Git Bash or a similar Unix-like environment on Windows, you can use the install script:

```bash
curl -sSL https://raw.githubusercontent.com/ryanwashacked/dirtree/main/install.sh | bash
```

Note: When using the script on Windows, you'll need to manually move the `dirtree.exe` file to a directory in your PATH.

### Manual Installation

If you prefer to install manually:

1. Download the appropriate binary for your system from the [latest release](https://github.com/ryanwashacked/dirtree/releases/latest):
   - macOS Intel: `dirtree-macos-intel`
   - macOS ARM (M1, M2, etc.): `dirtree-macos-arm`
   - Linux (x86_64): `dirtree-linux`
   - Windows: `dirtree-windows.exe`
2. For macOS and Linux:
   - Make the binary executable: `chmod +x dirtree-*`
   - Move the binary to a directory in your PATH and rename it to `dirtree`, e.g., `sudo mv dirtree-* /usr/local/bin/dirtree`
3. For Windows:
   - Rename the binary to `dirtree.exe`
   - Move the binary to a directory in your PATH

## Usage

```bash
dirtree [OPTIONS] [DIR]
```

Options:
- `-d, --depth <DEPTH>`: Number of subdirectory levels to expand (0 means no limit)
- `-v, --verbose`: Enable verbose output

Example:
```bash
dirtree -d 2 ~/projects/my-awesome-project
```

This will generate a directory tree for `~/projects/my-awesome-project` with a depth of 2 levels and update the README.md file in that directory.

## Directory Structure

```
📁 directory_structure_generator
   📁 .git
   📁 .github
   📁 src
   📄 .gitignore
   📄 Cargo.toml
   📝 README.md
   📜 install.sh
```