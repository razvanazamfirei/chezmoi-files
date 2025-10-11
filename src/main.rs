//! # chezmoi-files
//!
//! A command-line utility that generates colorized tree visualizations of file paths.
//! It reads file paths from stdin, filters them based on configurable rules, and outputs
//! a hierarchical tree structure with syntax-highlighted file names.

mod color;
mod config;
mod tree;

use crate::color::ColorScheme;
use crate::tree::{TreeDepth, TreeNode, TreeParams, TreeTrunk};
use clap::Parser;
use std::env;
use std::io::{self, BufRead, IsTerminal};

/// A command-line utility that generates colorized tree visualizations of file paths.
///
/// Reads file paths from stdin, filters them based on configurable rules, and outputs
/// a hierarchical tree structure with syntax-highlighted file names.
#[derive(Parser, Debug)]
#[command(name = "chezmoi-files")]
#[command(version)]
#[command(about, long_about = None)]
struct Args {}

/// The main function of the program.
///
/// This function is the entry point of the program. It reads lines from the standard input and
/// processes each line to build and display a tree visualization.
///
/// # Panics
///
/// Panics if it cannot get the current working directory.
///
/// # Example
///
/// ```bash
/// echo "path/to/file" | cargo run
/// ```
fn main() {
    // Parse command-line arguments
    let _args = Args::parse();

    // Check if there is any input provided to the program
    if io::stdin().is_terminal() {
        eprintln!("No input provided. Please pipe data into the program.");
        return;
    }

    let stdin = io::stdin();
    let handle = stdin.lock();

    let pwd = env::current_dir().expect("Failed to get current directory");
    let pwd_str = pwd.to_str().expect("Failed to convert PathBuf to string");
    let color_scheme = ColorScheme::new();
    let config = config::Config::new();
    let excluded_files = &config.excluded_files;
    let included_files = &config.included_files;
    let mut root = TreeNode::new();
    root.is_leaf = false;

    // Read lines from the standard input and process each line
    for line in handle.lines() {
        let path = match line {
            Ok(path) => {
                let trimmed_path = path.trim_end_matches('/');
                if trimmed_path.is_empty()
                    || (excluded_files
                        .files
                        .iter()
                        .any(|excluded| trimmed_path.contains(excluded.as_str()))
                        && !included_files
                            .files
                            .iter()
                            .any(|included| trimmed_path.contains(included.as_str())))
                {
                    continue;
                }
                // Strip the prefix of the current directory from the line and trim leading slashes
                let relative_path = trimmed_path.strip_prefix(pwd_str).unwrap_or(trimmed_path);
                relative_path.trim_start_matches('/').to_owned()
            }
            Err(error) => {
                eprintln!("Error reading line: {error}");
                continue;
            }
        };

        root.add_path(path.split('/').filter(|p| !p.is_empty()));
    }

    let mut trunk = TreeTrunk::default();
    println!(".");
    print_tree(&root, &mut trunk, TreeDepth::root().deeper(), &color_scheme);
}

/// Prints a tree structure.
///
/// This function prints a tree structure with the specified root node, trunk, depth,
/// and color scheme using a depth-first traversal.
///
/// # Arguments
///
/// * `node` - A reference to the `TreeNode` that is currently being processed.
/// * `trunk` - A mutable reference to the `TreeTrunk` that is used to store the tree structure.
/// * `depth` - The current depth of the tree.
/// * `color_scheme` - A reference to the `ColorScheme` that is used to colorize the output.
///
/// # Example
///
/// ```no_run
/// use chezmoi_files::{TreeNode, TreeTrunk, TreeDepth, ColorScheme};
///
/// let node = TreeNode::new();
/// let mut trunk = TreeTrunk::default();
/// let depth = TreeDepth::root().deeper();
/// let color_scheme = ColorScheme::new();
/// print_tree(&node, &mut trunk, depth, &color_scheme);
/// ```
fn print_tree(
    node: &TreeNode,
    trunk: &mut TreeTrunk,
    depth: TreeDepth,
    color_scheme: &ColorScheme,
) {
    let children = &node.children;
    let last_key = children.keys().last();

    for (name, subtree) in children {
        let is_last = Some(name) == last_key;
        let params = TreeParams::new(depth, is_last);
        let parts = trunk.new_row(params);

        let prefix: String = parts.iter().map(|part| part.ascii_art()).collect();
        color_scheme.print_with_color(&prefix, name);

        if !subtree.is_leaf {
            print_tree(subtree, trunk, depth.deeper(), color_scheme);
        }
    }
}
