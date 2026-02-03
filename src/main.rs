//! # chezmoi-files
//!
//! A command-line utility that generates colorized tree visualizations of file paths.
//! It reads file paths from stdin, filters them based on configurable rules, and outputs
//! a hierarchical tree structure with syntax-highlighted file names.

use chezmoi_files::{ColorScheme, TreeDepth, TreeNode, TreeParams, TreeTrunk, config};
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
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    /// Disable colorized output
    #[arg(long, global = true)]
    no_color: bool,

    /// Show statistics (file and directory counts)
    #[arg(long, short, global = true)]
    stats: bool,

    /// Sort order: name, type, or none
    #[arg(long, value_name = "ORDER", default_value = "none", global = true)]
    sort: SortOrder,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum SortOrder {
    /// No sorting (order from input)
    None,
    /// Sort alphabetically by name
    Name,
    /// Sort by type (directories first, then by extension)
    Type,
}

#[derive(Parser, Debug)]
enum Command {
    /// Show configuration information
    Config {
        /// Output the default configuration
        #[arg(long)]
        default: bool,

        /// Initialize configuration file with defaults
        #[arg(long)]
        init: bool,
    },
}

/// Statistics about the tree structure.
#[derive(Default, Debug)]
struct TreeStats {
    files: usize,
    directories: usize,
    excluded: usize,
}

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
    let args = Args::parse();

    if let Some(ref command) = args.command {
        handle_command(command);
        return;
    }

    if io::stdin().is_terminal() {
        eprintln!("No input provided. Please pipe data into the program.");
        return;
    }

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let current_dir_str = current_dir
        .to_str()
        .expect("Failed to convert PathBuf to string");

    let config = config::Config::new();
    let color_enabled = !args.no_color && config.colors.enabled;
    let color_scheme = ColorScheme::from_config(
        color_enabled,
        config.colors.folder.clone(),
        config.colors.default_file.clone(),
        config.colors.extensions.clone(),
    );

    let mut root = TreeNode::new();
    root.is_leaf = false;
    let mut stats = TreeStats::default();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let Ok(path) = line else {
            eprintln!("Error reading line: {}", line.unwrap_err());
            continue;
        };

        match process_path(&path, current_dir_str, &config) {
            PathResult::Included(relative_path) => {
                root.add_path(relative_path.split('/').filter(|p| !p.is_empty()));
            }
            PathResult::Excluded => {
                stats.excluded += 1;
            }
            PathResult::Empty => {}
        }
    }

    // Apply sorting if requested
    if !matches!(args.sort, SortOrder::None) {
        sort_tree(&mut root, args.sort);
    }

    // Count files and directories
    count_tree(&root, &mut stats);

    let mut trunk = TreeTrunk::default();
    println!(".");
    print_tree(&root, &mut trunk, TreeDepth::root().deeper(), &color_scheme);

    if args.stats {
        println!();
        println!(
            "Files: {}, Directories: {}, Excluded: {}",
            stats.files, stats.directories, stats.excluded
        );
    }
}

/// Handles subcommands.
fn handle_command(command: &Command) {
    match command {
        Command::Config { default, init } => {
            if *init {
                initialize_config();
            } else if *default {
                print_default_config();
            } else {
                show_config_info();
            }
        }
    }
}

/// Shows information about the current configuration.
fn show_config_info() {
    let config_path = config::Config::config_path();
    println!("Configuration file: {}", config_path.display());

    if config_path.exists() {
        println!("\nCurrent configuration:");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            println!("{content}");
        } else {
            eprintln!("Error reading configuration file");
        }
    } else {
        println!("\nConfiguration file does not exist.");
        println!("Using default configuration.");
        println!("\nRun 'chezmoi-files config --init' to create a configuration file.");
    }
}

/// Prints the default configuration.
fn print_default_config() {
    println!("{}", config::Config::default_config_toml());
}

/// Initializes the configuration file with default values.
fn initialize_config() {
    let config_path = config::Config::config_path();

    if config_path.exists() {
        eprintln!(
            "Configuration file already exists at: {}",
            config_path.display()
        );
        eprintln!("Remove it first or edit it manually.");
        return;
    }

    if let Some(parent) = config_path.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        eprintln!("Error creating config directory: {e}");
        return;
    }

    let default_config = config::Config::default_config_toml();
    if let Err(e) = std::fs::write(&config_path, default_config) {
        eprintln!("Error writing configuration file: {e}");
        return;
    }

    println!("Configuration file created at: {}", config_path.display());
}

/// Result of processing a path.
enum PathResult {
    /// Path should be included in the tree.
    Included(String),
    /// Path was excluded by filters.
    Excluded,
    /// Path was empty or invalid.
    Empty,
}

/// Processes a path by filtering and normalizing it.
fn process_path(path: &str, current_dir: &str, config: &config::Config) -> PathResult {
    let trimmed_path = path.trim_end_matches('/');

    if trimmed_path.is_empty() {
        return PathResult::Empty;
    }

    if should_exclude(trimmed_path, config) {
        return PathResult::Excluded;
    }

    let relative_path = trimmed_path
        .strip_prefix(current_dir)
        .unwrap_or(trimmed_path);
    PathResult::Included(relative_path.trim_start_matches('/').to_owned())
}

/// Determines if a path should be excluded based on configuration.
///
/// A path is excluded if it matches any exclusion pattern and doesn't match any inclusion pattern.
fn should_exclude(path: &str, config: &config::Config) -> bool {
    let is_excluded = config.is_excluded(path);
    let is_included = config.is_included(path);

    is_excluded && !is_included
}

/// Sorts the tree recursively based on the specified sort order.
fn sort_tree(node: &mut TreeNode, sort_order: SortOrder) {
    match sort_order {
        SortOrder::None => {}
        SortOrder::Name => {
            node.children.sort_by(|k1, _, k2, _| k1.cmp(k2));
        }
        SortOrder::Type => {
            node.children.sort_by(|k1, v1, k2, v2| {
                // Directories before files
                match (v1.is_leaf, v2.is_leaf) {
                    (false, true) => std::cmp::Ordering::Less,
                    (true, false) => std::cmp::Ordering::Greater,
                    _ => {
                        // Same type, sort by extension then name
                        let ext1 = k1.rsplit('.').next().unwrap_or(k1);
                        let ext2 = k2.rsplit('.').next().unwrap_or(k2);
                        match ext1.cmp(ext2) {
                            std::cmp::Ordering::Equal => k1.cmp(k2),
                            other => other,
                        }
                    }
                }
            });
        }
    }

    // Recursively sort children
    for (_, child) in &mut node.children {
        sort_tree(child, sort_order);
    }
}

/// Counts files and directories in the tree.
fn count_tree(node: &TreeNode, stats: &mut TreeStats) {
    for (_, child) in &node.children {
        if child.is_leaf {
            stats.files += 1;
        } else {
            stats.directories += 1;
            count_tree(child, stats);
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_result_included() {
        let result = PathResult::Included("test/path".to_string());
        match result {
            PathResult::Included(path) => assert_eq!(path, "test/path"),
            _ => panic!("Expected Included variant"),
        }
    }

    #[test]
    fn test_path_result_excluded() {
        let result = PathResult::Excluded;
        assert!(matches!(result, PathResult::Excluded));
    }

    #[test]
    fn test_path_result_empty() {
        let result = PathResult::Empty;
        assert!(matches!(result, PathResult::Empty));
    }

    #[test]
    fn test_tree_stats_default() {
        let stats = TreeStats::default();
        assert_eq!(stats.files, 0);
        assert_eq!(stats.directories, 0);
        assert_eq!(stats.excluded, 0);
    }

    #[test]
    fn test_process_path_empty() {
        let config = config::Config::default();
        let result = process_path("", "/current/dir", &config);
        assert!(matches!(result, PathResult::Empty));
    }

    #[test]
    fn test_process_path_trailing_slash() {
        let config = config::Config::default();
        let result = process_path("test/path/", "/current/dir", &config);
        match result {
            PathResult::Included(path) => assert_eq!(path, "test/path"),
            _ => panic!("Expected Included variant"),
        }
    }

    #[test]
    fn test_process_path_excluded() {
        let config = config::Config::default();
        let result = process_path("path/DS_Store", "/current/dir", &config);
        assert!(matches!(result, PathResult::Excluded));
    }

    #[test]
    fn test_process_path_strip_prefix() {
        let config = config::Config::default();
        let result = process_path("/current/dir/src/main.rs", "/current/dir", &config);
        match result {
            PathResult::Included(path) => assert_eq!(path, "src/main.rs"),
            _ => panic!("Expected Included variant"),
        }
    }

    #[test]
    fn test_should_exclude_default() {
        let config = config::Config::default();
        assert!(should_exclude("DS_Store", &config));
        assert!(should_exclude("path/to/DS_Store", &config));
        assert!(!should_exclude("regular_file.txt", &config));
    }

    #[test]
    fn test_should_exclude_with_inclusion() {
        let mut config = config::Config::default();
        config
            .included_files
            .files
            .push("important.txt".to_string());
        config.excluded_files.files.push("*.txt".to_string());

        // Should not be excluded if included
        assert!(!should_exclude("important.txt", &config));
        // Should be excluded if not in inclusion list
        assert!(should_exclude("other.txt", &config));
    }

    #[test]
    fn test_sort_tree_none() {
        let mut root = TreeNode::new();
        root.add_path(vec!["c.txt"]);
        root.add_path(vec!["a.txt"]);
        root.add_path(vec!["b.txt"]);

        sort_tree(&mut root, SortOrder::None);

        let keys: Vec<_> = root.children.keys().collect();
        // Order should remain as inserted
        assert_eq!(keys, vec!["c.txt", "a.txt", "b.txt"]);
    }

    #[test]
    fn test_sort_tree_name() {
        let mut root = TreeNode::new();
        root.add_path(vec!["c.txt"]);
        root.add_path(vec!["a.txt"]);
        root.add_path(vec!["b.txt"]);

        sort_tree(&mut root, SortOrder::Name);

        let keys: Vec<_> = root.children.keys().collect();
        assert_eq!(keys, vec!["a.txt", "b.txt", "c.txt"]);
    }

    #[test]
    fn test_sort_tree_type() {
        let mut root = TreeNode::new();
        root.add_path(vec!["file.txt"]);
        root.add_path(vec!["dir", "nested.txt"]);
        root.add_path(vec!["file.rs"]);

        sort_tree(&mut root, SortOrder::Type);

        let keys: Vec<_> = root.children.keys().collect();
        // Directory should come before files
        assert_eq!(keys[0], "dir");
    }

    #[test]
    fn test_sort_tree_type_by_extension() {
        let mut root = TreeNode::new();
        root.add_path(vec!["file.txt"]);
        root.add_path(vec!["file.rs"]);
        root.add_path(vec!["file.md"]);

        sort_tree(&mut root, SortOrder::Type);

        let keys: Vec<_> = root.children.keys().collect();
        // Should be sorted by extension
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_count_tree_empty() {
        let root = TreeNode::new();
        let mut stats = TreeStats::default();
        count_tree(&root, &mut stats);

        assert_eq!(stats.files, 0);
        assert_eq!(stats.directories, 0);
    }

    #[test]
    fn test_count_tree_files_only() {
        let mut root = TreeNode::new();
        root.add_path(vec!["a.txt"]);
        root.add_path(vec!["b.txt"]);
        root.add_path(vec!["c.txt"]);

        let mut stats = TreeStats::default();
        count_tree(&root, &mut stats);

        assert_eq!(stats.files, 3);
        assert_eq!(stats.directories, 0);
    }

    #[test]
    fn test_count_tree_with_directories() {
        let mut root = TreeNode::new();
        root.add_path(vec!["src", "main.rs"]);
        root.add_path(vec!["src", "lib.rs"]);
        root.add_path(vec!["tests", "test.rs"]);

        let mut stats = TreeStats::default();
        count_tree(&root, &mut stats);

        assert_eq!(stats.files, 3);
        assert_eq!(stats.directories, 2);
    }

    #[test]
    fn test_count_tree_nested() {
        let mut root = TreeNode::new();
        root.add_path(vec!["a", "b", "c", "file.txt"]);

        let mut stats = TreeStats::default();
        count_tree(&root, &mut stats);

        assert_eq!(stats.files, 1);
        assert_eq!(stats.directories, 3);
    }

    #[test]
    fn test_print_tree_basic() {
        let mut root = TreeNode::new();
        root.is_leaf = false;
        root.add_path(vec!["test.txt"]);

        let mut trunk = TreeTrunk::default();
        let color_scheme = ColorScheme::with_colors(false);

        // This should not panic
        print_tree(&root, &mut trunk, TreeDepth::root().deeper(), &color_scheme);
    }

    #[test]
    fn test_print_tree_nested() {
        let mut root = TreeNode::new();
        root.is_leaf = false;
        root.add_path(vec!["src", "main.rs"]);
        root.add_path(vec!["src", "lib.rs"]);

        let mut trunk = TreeTrunk::default();
        let color_scheme = ColorScheme::with_colors(false);

        // This should not panic
        print_tree(&root, &mut trunk, TreeDepth::root().deeper(), &color_scheme);
    }
}
