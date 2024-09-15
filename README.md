# Directory Structure Generator

## Overview

This Rust-based tool generates a visual representation of a directory structure and automatically updates the README.md file of a project with this structure. It respects `.gitignore` rules and provides options to control the depth of subdirectory expansion.

## Features

- Generate a tree-like representation of directory structures
- Automatically update README.md files with the generated structure
- Respect `.gitignore` rules to exclude ignored files and directories
- Customizable depth for subdirectory expansion
- Use of intuitive icons for different file types

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:
   ```
   git clone https://github.com/yourusername/directory-structure-generator.git
   cd directory-structure-generator
   ```

3. Build the project:
   ```
   cargo build --release
   ```

The executable will be available in `target/release/directory-structure-generator`.

## Usage

Run the tool using cargo:

```
cargo run -- [OPTIONS] [DIR]
```

Or, if you've built the release version:

```
./target/release/directory-structure-generator [OPTIONS] [DIR]
```

### Arguments

- `[DIR]`: Optional. The target directory to analyze. If not provided, the current directory is used.

### Options

- `-d, --depth <DEPTH>`: Number of subdirectory levels to expand. Default is 0 (expand all levels).
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### Examples

1. Generate full directory structure for the current directory:
   ```
   cargo run
   ```

2. Generate structure for a specific directory, expanding only 2 levels:
   ```
   cargo run -- --depth 2 /path/to/directory
   ```

3. Show only the root directory and its immediate subdirectories:
   ```
   cargo run -- --depth 1
   ```

## Output

The tool will update (or create if it doesn't exist) the README.md file in the target directory. It will add or update a "Directory Structure" section with the generated tree-like representation.

## File Icons

The generator uses the following icons for different file types:

- 📁 : Directory
- 🔧 : YAML files (.yml, .yaml)
- 📜 : Shell scripts (.sh)
- 🔑 : PEM files (.pem)
- 📝 : Markdown files (.md)
- 📄 : Text files (.txt)
- 📊 : SQL files (.sql)
- 📄 : Other file types

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.