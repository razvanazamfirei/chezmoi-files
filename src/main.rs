/// This module defines the `ColorScheme` struct, which is used to colorize the output.
mod color;

/// This module encapsulates the tree structure. Part of the code in this module is derived from
/// the `eza` crate, which is licensed under the MIT License. See source.
mod tree;

use crate::color::ColorScheme;
use crate::tree::{TreeDepth, TreeNode, TreeParams, TreeTrunk};
use std::env;
use std::io::{self, BufRead, IsTerminal};

#[deny(missing_docs)]
/// The main function of the program.
///
/// This function is the entry point of the program. It reads lines from the standard input and
/// processes each line.
///
/// # Example
///
/// ```
/// echo "path/to/file" | cargo run
/// ```
fn main() {
    // Check if there is any input provided to the program
    if io::stdin().is_terminal() {
        println!("No input provided. Please pipe data into the program.");
        return;
    }
    let stdin = io::stdin();
    let handle = stdin.lock();

    let pwd = env::current_dir().expect("Failed to get current directory");
    let pwd_str = pwd.to_str().expect("Failed to convert PathBuf to string");
    let color_scheme = ColorScheme::new();

    let mut root = TreeNode::new();
    root.is_leaf = false;

    // Read lines from the standard input and process each line
    for line in handle.lines() {
        let path = match line {
            Ok(path) => {
                let trimmed_path = path.trim_end_matches('/');
                if trimmed_path.is_empty()
                    || trimmed_path.contains("DS_Store")
                    || trimmed_path.contains("plugins/fish")
                    || trimmed_path.contains("plugins/zsh")
                {
                    continue; // Skip empty lines and lines containing excluded substrings
                }
                // Strip the prefix of the current directory from the line and trim leading slashes
                let relative_path = trimmed_path.strip_prefix(pwd_str).unwrap_or(trimmed_path);
                relative_path.trim_start_matches('/').to_owned()
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
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
/// This function is used to print a tree structure with the specified root node, trunk, depth,
/// and color scheme.
///
/// # Arguments
///
/// * `node` - A reference to the TreeNode that is currently being processed.
/// * `trunk` - A mutable reference to the TreeTrunk that is used to store the tree structure.
/// * `depth` - The current depth of the tree.
/// * `color_scheme` - A reference to the ColorScheme that is used to colorize the output.
///
/// # Example
///
/// ```
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
    for (name, subtree) in &node.children {
        let is_last = name == node.children.keys().last().unwrap();
        let params = TreeParams::new(depth, is_last);
        let parts = trunk.new_row(params);

        let prefix: String = parts.iter().map(|part| part.ascii_art()).collect();
        color_scheme.print_with_color(&prefix, name);

        if !subtree.is_leaf {
            print_tree(subtree, trunk, depth.deeper(), color_scheme);
        }
    }
}
