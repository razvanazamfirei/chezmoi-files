mod tree;

use crate::tree::{TreeDepth, TreeParams, TreeTrunk};
use indexmap::IndexMap;
use std::env;
use std::io::{self, BufRead};

struct TreeNode {
    children: IndexMap<String, TreeNode>,
    is_leaf: bool,
}

impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            children: IndexMap::new(),
            is_leaf: true,
        }
    }

    fn add_path<I>(&mut self, parts: I)
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut current = self;
        for part in parts {
            current.is_leaf = false;
            let part_str = part.as_ref().to_string();
            current = current
                .children
                .entry(part_str)
                .or_insert_with(TreeNode::new);
        }
    }
}

fn print_tree(node: &TreeNode, trunk: &mut TreeTrunk, depth: TreeDepth) {
    let mut children = node.children.iter().peekable();
    while let Some((name, subtree)) = children.next() {
        let is_last = children.peek().is_none();
        let params = TreeParams::new(depth, is_last);
        let parts = trunk.new_row(params);

        let prefix: String = parts.iter().map(|part| part.ascii_art()).collect();
        println!("{}{}", prefix, name);

        if !subtree.is_leaf {
            print_tree(subtree, trunk, depth.deeper());
        }
    }
}

fn main() {
    let pwd = env::current_dir().expect("Failed to get current directory");
    let pwd_str = pwd.to_str().expect("Failed to convert PathBuf to string");

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut root = TreeNode::new();
    root.is_leaf = false; // Root is not a leaf

    for line in handle.lines() {
        let path = match line {
            Ok(path) => {
                let trimmed_path = path.trim_end_matches('/');
                if trimmed_path.is_empty()
                    || trimmed_path.contains("DS_Store")
                    || trimmed_path.contains("plugins/fish")
                    || trimmed_path.contains("plugins/zsh")
                {
                    continue; // Skip lines containing the excluded substrings
                }
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
    print_tree(&root, &mut trunk, TreeDepth::root().deeper());
}
