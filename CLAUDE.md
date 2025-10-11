# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`chezmoi-files` is a command-line utility written in Rust that generates colorized tree visualizations of file paths.
It reads file paths from stdin, filters them based on configurable rules, and outputs a hierarchical tree structure with
syntax-highlighted file names.

The tool is designed to work with chezmoi configuration files and uses a custom configuration system via the
`CHEZMOI_FILES` environment variable.

## Build and Run Commands

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the program (requires piped input)
echo "path/to/file" | cargo run

# Test with the provided test file
cat test.txt | cargo run

# Run from binary
cat test.txt | ./target/release/chezmoi-files
```

## Configuration

The program looks for a `config.toml` file at `$CHEZMOI_FILES/config.toml`. If not found, it uses default values.

Configuration structure:

```toml
[excluded-files]
files = ["DS_Store", "plugins/fish", "plugins/zsh"]

[included-files]
files = []
```

- **Excluded files**: Paths containing these strings are filtered out
- **Included files**: Paths matching these strings override exclusions (whitelist)
- Default exclusions: `DS_Store`, `plugins/fish`, `plugins/zsh`

## Architecture

### Core Module Structure

1. **main.rs** - Entry point and orchestration
    - Reads stdin line-by-line
    - Filters paths based on config (excluded/included files)
    - Strips current directory prefix to create relative paths
    - Builds tree structure via `TreeNode::add_path()`
    - Renders tree with `print_tree()` using recursive traversal

2. **tree.rs** - Tree structure and ASCII rendering (derived from `eza` crate, MIT licensed)
    - `TreeNode`: Hierarchical structure using `IndexMap` for ordered children
    - `TreeTrunk`: State machine for tracking vertical line drawing across depths
    - `TreePart`: Enum for box-drawing characters (Edge, Line, Corner, Blank)
    - `TreeDepth`: Wrapper for depth tracking in recursive traversal

   Key insight: The trunk stack maintains parent-child relationship rendering state as the tree is printed depth-first.

3. **color.rs** - Syntax highlighting
    - `ColorScheme`: Maps file extensions to ANSI color codes
    - Folders are detected by absence of `.` in name and colored white
    - Extension-based coloring: shells (green), configs (yellow), docs (cyan), code (red), plists (magenta)
    - Default file color: blue

4. **config.rs** - Configuration loading
    - Reads TOML from `$CHEZMOI_FILES/config.toml`
    - Graceful fallback to defaults on missing file or parse errors
    - Uses serde for deserialization

### Data Flow

```
stdin → filter by config → strip pwd prefix → split by '/' → TreeNode::add_path() → print_tree() with ColorScheme
```

### Tree Rendering Algorithm

The tree rendering uses a depth-first traversal with stateful trunk management:

1. `TreeTrunk` maintains a stack of `TreePart` elements representing vertical lines at each depth
2. For each node, determine if it's the last child (`is_last`)
3. Generate ASCII art prefix by converting trunk stack to strings
4. Recursively render children, updating trunk state for proper line continuation

## Important Implementation Details

- Input must be piped; exits with message if stdin is a terminal (main.rs:27-30)
- Paths are normalized: trailing slashes removed, empty components filtered
- The program expects paths relative to current directory or absolute paths
- Uses `IndexMap` for deterministic ordering of tree children (important for `is_last` calculation)
- Filtering logic: excluded unless explicitly included (main.rs:49-56)
- The `TreeTrunk::stack` is reused across depth levels, with elements resized/updated per row

## Dependencies

- **indexmap** (2.11.4): Ordered hash map for tree children
- **toml** (0.9.8): Configuration parsing
- **serde** (1.0.228): Deserialization with derive feature

## Modernization (v0.5.0)

This codebase uses the latest Rust standards:

- **Rust Edition 2024**: Takes advantage of the latest language features
- **Modern linting**: Comprehensive clippy lints (all, pedantic, nursery, cargo)
- **Safety**: `unsafe_code = "forbid"` ensures memory safety
- **Const functions**: Functions that can be evaluated at compile time use `const fn`
- **Default implementations**: All types implement `Default` where appropriate
- **Documentation**: Complete rustdoc coverage with examples
- **Release optimizations**: LTO and single codegen unit for maximum performance
