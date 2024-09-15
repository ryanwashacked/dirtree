# Dirtree

A tool for generating and updating directory structure representations in README files.

## Installation

### macOS (Intel and ARM) and Linux

You can install dirtree with the following command:

```bash
curl -sSL https://raw.githubusercontent.com/ryanwashacked/dirtree/main/install.sh | bash
```

This will download the appropriate binary for your system (including ARM-based Macs) and install it in `/usr/local/bin`.

### Manual Installation

If you prefer to install manually:

1. Download the appropriate binary for your system from the [latest release](https://github.com/yourusername/dirtree/releases/latest):
   - macOS Intel: `dirtree-macos-intel`
   - macOS ARM (M1, M2, etc.): `dirtree-macos-arm`
   - Linux (x86_64): `dirtree-linux`
2. Make the binary executable: `chmod +x dirtree-*`
3. Move the binary to a directory in your PATH and rename it to `dirtree`, e.g., `sudo mv dirtree-* /usr/local/bin/dirtree`

## Usage

```bash
dirtree [OPTIONS] [DIR]
```

Options:
- `-d, --depth <DEPTH>`: Number of subdirectory levels to expand (0 means no limit)

Example:
```bash
dirtree -d 2 ~/projects/my-awesome-project
```

This will generate a directory tree for `~/projects/my-awesome-project` with a depth of 2 levels and update the README.md file in that directory.## Directory Structure

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
