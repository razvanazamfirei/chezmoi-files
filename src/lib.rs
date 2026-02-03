//! # chezmoi-files
//!
//! A command-line utility and library for generating colorized tree visualizations of file paths.
//!
//! This crate provides functionality for:
//! - Building hierarchical tree structures from file paths
//! - Filtering paths using glob patterns
//! - Colorizing output based on file types
//! - Configuring exclusion and inclusion rules
//!
//! ## Usage as a Binary
//!
//! ```bash
//! # Install from crates.io
//! cargo install chezmoi-files
//!
//! # Use with chezmoi
//! chezmoi managed | chezmoi-files
//!
//! # Use with any file list
//! find . -type f | chezmoi-files --sort name --stats
//! ```
//!
//! ## Usage as a Library
//!
//! ```rust
//! use chezmoi_files::{TreeNode, ColorScheme, Config};
//!
//! // Create a tree structure
//! let mut root = TreeNode::new();
//! root.add_path(vec!["src", "main.rs"]);
//! root.add_path(vec!["src", "lib.rs"]);
//!
//! // Load configuration
//! let config = Config::default();
//!
//! // Create color scheme
//! let color_scheme = ColorScheme::new();
//! ```
//!
//! ## Features
//!
//! - **Glob Pattern Filtering**: Advanced pattern matching with wildcards
//!   (`*`, `?`, `[abc]`, `[a-z]`)
//! - **Customizable Colors**: Configure colors for folders, files, and specific extensions
//! - **Multiple Sorting Options**: Sort by name, type, or keep original order
//! - **Statistics**: Display counts of files, directories, and excluded items
//! - **Fast**: Optimized Rust implementation with minimal overhead
//!
//! ## Configuration
//!
//! Configuration is loaded from `~/.config/chezmoi/chezmoi-files.toml`:
//!
//! ```toml
//! [excluded-files]
//! files = [
//!     "DS_Store",
//!     "*.tmp",
//!     "cache/*",
//! ]
//!
//! [included-files]
//! files = []
//!
//! [colors]
//! enabled = true
//! folder = "white"
//! default-file = "blue"
//!
//! [colors.extensions]
//! ".rs" = "red"
//! ".py" = "green"
//! ```

// Re-export main modules
pub mod color;
pub mod config;
pub mod tree;

// Re-export commonly used types
pub use color::ColorScheme;
pub use config::{ColorConfig, Config, FileList};
pub use tree::{TreeDepth, TreeNode, TreeParams, TreePart, TreeTrunk};
