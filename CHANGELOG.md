# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2026-02-02

### Added

### New Features

#### 1. Glob Pattern Support
- Full glob-style pattern matching with wildcards: `*`, `?`, `[abc]`, `[a-z]`
- Examples: `*.tmp`, `fish_variables*`, `test_?.rs`, `cache/*`
- Backward compatible: patterns without wildcards still use substring matching

#### 2. Configurable Colors
- Customize colors for folders and files in config file
- Support for color names: black, red, green, yellow, blue, magenta, cyan, white
- Custom ANSI codes supported for advanced users
- Per-extension color customization via `[colors.extensions]`

#### 3. Command-Line Options
- `--no-color`: Disable colorized output
- `--stats` / `-s`: Show statistics (file count, directory count, excluded count)
- `--sort <ORDER>`: Sort output (name, type, or none)
  - `name`: Alphabetical sorting
  - `type`: Directories first, then by extension

#### 4. Configuration Subcommand
- `config`: Show current configuration location and contents
- `config --default`: Output the default configuration
- `config --init`: Initialize configuration file with defaults

#### 5. Comprehensive Test Suite
- 12 unit tests for pattern matching, colors, and configuration
- 5 integration tests for CLI functionality
- Full test coverage for new features

### Improvements

#### Code Quality
- More idiomatic Rust code with modern patterns
- Better error handling and user feedback
- Extracted helper functions for improved readability
- All clippy warnings resolved
- Clean separation of concerns

#### Configuration
- Moved from `$CHEZMOI_FILES/config.toml` to `~/.config/chezmoi/chezmoi-files.toml`
- Standard XDG config directory location
- Better default exclusion patterns with glob support
- Enhanced config documentation with examples

#### Documentation
- Comprehensive README.md with:
  - Installation instructions
  - Usage examples
  - Configuration documentation
  - Pattern matching guide
  - Color customization guide
  - Testing information
- Example configuration file with detailed comments
- Inline code documentation for all public APIs

### Updated Default Exclusions
- `DS_Store`
- `fish_variables*` (with wildcard)
- `.rubocop.yml`
- `.ruff_cache`
- `yazi.toml-*` (with wildcard)
- `.zcompcache`
- `.zcompdump`
- `.zsh_history`
- `plugins/fish`
- `plugins/zsh`

### Technical Changes
- Added `glob` crate for pattern matching
- Refactored config module for better pattern matching
- Enhanced color module with dynamic configuration
- Improved main module with sorting and statistics
- Better CLI argument handling with clap

### Performance
- Efficient glob pattern matching
- Optimized tree traversal
- Release build with LTO and single codegen unit

## Previous Versions

See git history for earlier changes.
