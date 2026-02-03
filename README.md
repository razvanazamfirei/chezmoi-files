# chezmoi-files

[![Crates.io](https://img.shields.io/crates/v/chezmoi-files.svg)](https://crates.io/crates/chezmoi-files)
[![Documentation](https://docs.rs/chezmoi-files/badge.svg)](https://docs.rs/chezmoi-files)
[![License](https://img.shields.io/crates/l/chezmoi-files.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.92%2B-blue.svg)](https://www.rust-lang.org)

A command-line utility and Rust library that generates colorized tree visualizations of file paths. It reads file paths
from stdin, filters them based on configurable glob patterns, and outputs a hierarchical tree structure with
syntax-highlighted file names.

Perfect for use with [chezmoi](https://www.chezmoi.io/) to visualize your dotfiles, or with any tool that outputs file
paths. Can also be used as a library in your own Rust projects.

## Features

- **Glob Pattern Filtering**: Advanced pattern matching with wildcards (`*`, `?`, `[abc]`, `[a-z]`)
- **Customizable Colors**: Configure colors for folders, files, and specific extensions
- **Multiple Sorting Options**: Sort by name, type, or keep original order
- **Statistics**: Display counts of files, directories, and excluded items
- **Fast**: Optimized Rust implementation with minimal overhead
- **Configurable**: Simple TOML configuration file
- **Well-tested**: 89.61% code coverage with 83 tests

## Installation

### From crates.io

```bash
cargo install chezmoi-files
```

### From source

```bash
git clone https://github.com/razvanazamfirei/chezmoi-files.git
cd chezmoi-files
cargo install --path .
```

### Pre-built binaries

Download pre-built binaries from the [releases page](https://github.com/razvanazamfirei/chezmoi-files/releases).

## Usage

### As a Command-Line Tool

Pipe file paths into the program:

```bash
# Basic usage
find . -type f | chezmoi-files

# With chezmoi
chezmoi managed | chezmoi-files

# From a file list
cat files.txt | chezmoi-files
```

The program expects one file path per line on stdin. It will:

1. Filter paths based on exclusion patterns
2. Strip the current directory prefix to create relative paths
3. Build a tree structure
4. Display it with syntax-highlighted file names

### Command-Line Options

```bash
# Disable colorized output
find . -type f | chezmoi-files --no-color

# Show statistics (file and directory counts)
chezmoi managed | chezmoi-files --stats

# Sort output by name
find . -type f | chezmoi-files --sort name

# Sort output by type (directories first, then by extension)
find . -type f | chezmoi-files --sort type

# Combine options
chezmoi managed | chezmoi-files --stats --sort name --no-color
```

### Configuration Commands

```bash
# Show current configuration location and contents
chezmoi-files config

# Output the default configuration
chezmoi-files config --default

# Initialize configuration file with defaults
chezmoi-files config --init
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
chezmoi-files = "0.7"
```

Use in your code:

```rust
use chezmoi_files::{TreeNode, ColorScheme, Config};

// Create a tree structure
let mut root = TreeNode::new();
root.add_path(vec!["src", "main.rs"]);
root.add_path(vec!["src", "lib.rs"]);
root.add_path(vec!["tests", "test.rs"]);

// Load configuration
let config = Config::default ();

// Create color scheme
let color_scheme = ColorScheme::new();

// Check if paths should be excluded
if config.is_excluded("DS_Store") {
println ! ("DS_Store files are excluded");
}
```

See the [API documentation](https://docs.rs/chezmoi-files) for more details.

## Configuration

The program reads configuration from:

```
~/.config/chezmoi/chezmoi-files.toml
```

An example configuration file is provided as `chezmoi-files.toml.example`. Copy it to the config location and customize
as needed.

### Configuration Format

The config file uses TOML format with two sections:

```toml
[excluded-files]
files = [
    "DS_Store",
    "fish_variables",
    ".rubocop.yml",
    ".ruff_cache",
    "yazi.toml-",
    ".zcompcache",
    ".zcompdump",
    ".zsh_history",
    "plugins/fish",
    "plugins/zsh",
]

[included-files]
files = []
```

### Pattern Matching

Patterns support glob-style wildcards:

- `*` - Matches any sequence of characters
- `?` - Matches any single character
- `[abc]` - Matches any character in the set
- `[a-z]` - Matches any character in the range

Examples:

- `*.tmp` - Matches any file ending in `.tmp`
- `cache/*` - Matches any file in a cache directory
- `test_?.rs` - Matches `test_1.rs`, `test_a.rs`, etc.
- `fish_variables*` - Matches `fish_variables`, `fish_variables.bak`, etc.

**Exclusion Logic:**

- Paths matching exclusion patterns are filtered out
- Paths matching inclusion patterns override exclusions (whitelist)
- Patterns without wildcards use substring matching for backward compatibility

### Default Exclusions

If no config file exists, these patterns are excluded by default:

- `DS_Store`
- `fish_variables`
- `.rubocop.yml`
- `.ruff_cache`
- `yazi.toml-`
- `.zcompcache`
- `.zcompdump`
- `.zsh_history`
- `plugins/fish`
- `plugins/zsh`

## Color Scheme

### Default Colors

Files are colorized based on their extension:

- **Folders**: White
- **Shell scripts** (.sh, .bash, .zsh, .fish, .nu): Green
- **Config files** (.toml, .yaml, .yml, .json, .xml, .ini, .conf): Yellow
- **Documentation** (.md, .txt, .rst): Cyan
- **Source code** (.rs, .py, .js, .ts, .go, .c, .cpp, .java, .jl): Red
- **Plists** (.plist, .sublime): Magenta
- **Default**: Blue

### Customizing Colors

You can customize colors in the configuration file:

```toml
[colors]
enabled = true
folder = "white"
default-file = "blue"

[colors.extensions]
".rs" = "red"
".py" = "green"
".md" = "cyan"
```

Available color names: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`

You can also use custom ANSI codes for more control.

## Examples

### Basic directory tree

```bash
echo -e "src/main.rs\nsrc/lib.rs\ntests/test.rs" | chezmoi-files
```

Output:

```
.
├── src
│   ├── main.rs
│   └── lib.rs
└── tests
    └── test.rs
```

### With chezmoi

```bash
chezmoi managed | chezmoi-files
```

This displays all files managed by chezmoi in a tree structure, excluding patterns from your config file.

## Requirements

- Rust 1.92.0 or later
- Input must be piped (the program will exit if stdin is a terminal)

## Testing

The project has comprehensive test coverage with **83 tests** achieving **89.61% overall coverage**:

```bash
# Run all tests
cargo test

# Generate coverage report
cargo llvm-cov --all-features --workspace --html
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development

```bash
# Clone the repository
git clone https://github.com/razvanazamfirei/chezmoi-files.git
cd chezmoi-files

# Run tests
cargo test

# Run with coverage
cargo llvm-cov --all-features --workspace --html

# Run clippy
cargo clippy --all-targets --all-features
```

## Similar Projects

- [tree](https://linux.die.net/man/1/tree) - The classic Unix tree command
- [eza](https://github.com/eza-community/eza) - Modern replacement for ls with tree view
- [broot](https://github.com/Canop/broot) - Interactive tree view

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

## Acknowledgments

- Tree rendering algorithm derived from [eza](https://github.com/eza-community/eza) (MIT License)
- Built with Rust's excellent ecosystem
