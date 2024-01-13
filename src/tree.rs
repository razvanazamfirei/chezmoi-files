use indexmap::IndexMap;

/// A **tree part** is a single character in the tree structure. It can be
/// either an edge, a line, a corner, or a blank space.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TreePart {
    /// Rightmost column, *not* the last in the directory.
    Edge,

    /// Not the rightmost column, and the directory has not finished yet.
    Line,

    /// Rightmost column, and the last in the directory.
    Corner,

    /// Not the rightmost column, and the directory *has* finished.
    Blank,
}

impl TreePart {
    /// Turn this tree part into ASCII-licious box drawing characters!
    /// (Warning: not actually ASCII)
    pub fn ascii_art(self) -> &'static str {
        #[rustfmt::skip]
        return match self {
            Self::Edge    => "├──",
            Self::Line    => "│   ",
            Self::Corner  => "└──",
            Self::Blank   => "    ",
        };
    }
}

/// A **tree trunk** builds up arrays of tree parts over multiple depths.
#[derive(Debug, Default)]
pub struct TreeTrunk {
    /// A stack tracks which tree characters should be printed. It’s
    /// necessary to maintain information about the previously-printed
    /// lines, as the output will change based on any previous entries.
    stack: Vec<TreePart>,

    /// A tuple for the last ‘depth’ and ‘last’ parameters that are passed in.
    last_params: Option<TreeParams>,
}

impl TreeTrunk {
    /// Calculates the tree parts for an entry at the given depth and
    /// last-ness. The depth is used to determine where in the stack the tree
    /// part should be inserted, and the last-ness is used to determine which
    /// type of tree part to insert.
    ///
    /// This takes a `&mut self` because the results of each file are stored
    /// and used in future rows.
    pub fn new_row(&mut self, params: TreeParams) -> &[TreePart] {
        // If this isn’t our first iteration, then update the tree parts thus
        // far to account for there being another row after it.
        if let Some(last) = self.last_params {
            self.stack[last.depth.0] = if last.last {
                TreePart::Blank
            } else {
                TreePart::Line
            };
        }

        // Make sure the stack has enough space, then add or modify another
        // part into it.
        self.stack.resize(params.depth.0 + 1, TreePart::Edge);
        self.stack[params.depth.0] = if params.last {
            TreePart::Corner
        } else {
            TreePart::Edge
        };

        self.last_params = Some(params);

        // Return the tree parts as a slice of the stack.
        //
        // Ignore the first element here to prevent a ‘zeroth level’ from
        // appearing before the very first directory. This level would
        // join unrelated directories without connecting to anything:
        //
        //     with [0..]        with [1..]
        //     ==========        ==========
        //      ├── folder        folder
        //      │  └── file       └── file
        //      └── folder        folder
        //         └── file       └──file
        //
        &self.stack[1..]
    }
}

#[derive(Debug, Copy, Clone)]
/// A structure representing the parameters of a tree.
///
/// # Fields
///
/// * `depth` - A `TreeDepth` that represents how many directories deep into the tree structure this is.
/// Directories on top have depth 0.
/// * `last` - A boolean flag that indicates whether this is the last entry in the directory.
pub struct TreeParams {
    /// How many directories deep into the tree structure this is. Directories
    /// on top have depth 0.
    depth: TreeDepth,

    /// Whether this is the last entry in the directory.
    last: bool,
}

impl TreeParams {
    /// Create a new set of tree parameters.
    pub fn new(depth: TreeDepth, last: bool) -> Self {
        Self { depth, last }
    }
}

#[derive(Debug, Copy, Clone)]
/// A structure representing the depth of a node in a tree.
///
/// This structure is used to represent the depth of a node in a tree.
/// The depth of a node is the number of edges from the node to the tree's root node.
/// A root node will have a depth of 0.
///
/// # Fields
///
/// * `0` - A `usize` that represents the depth of the node in the tree.
pub struct TreeDepth(pub usize);
impl TreeDepth {
    /// Create a new tree depth.
    pub fn root() -> Self {
        Self(0)
    }
    /// Increase the depth by one.
    pub fn deeper(self) -> Self {
        Self(self.0 + 1)
    }
}

/// A structure representing an iterator with a current depth.
///
/// This structure is used to represent an iterator that keeps track of the current depth in a tree.
/// It is generic over the type of the inner iterator `I`.
///
/// # Fields
///
/// * `current_depth` - A `TreeDepth` that represents the current depth in the tree.
/// * `inner` - The inner iterator of type `I`.
pub struct Iter<I> {
    current_depth: TreeDepth,
    inner: I,
}

impl<I, T> Iterator for Iter<I>
where
    I: ExactSizeIterator + Iterator<Item = T>,
{
    type Item = (TreeParams, T);

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.inner.next()?;

        // TODO: use exact_size_is_empty API soon
        let params = TreeParams::new(self.current_depth, self.inner.len() == 0);
        Some((params, t))
    }
}

/// A structure representing a node in a tree.
///
/// This structure is used to represent a node in a tree. Each node has a collection of children,
/// which are also nodes.
/// The `IndexMap` ensures that the children are ordered in the order they were inserted.
///
/// # Fields
///
/// * `children` - An `IndexMap` where the keys are `String` and the values are `TreeNode`.
/// This represents the children of the node.
/// * `is_leaf` - A boolean flag that indicates whether the node is a leaf node
/// (i.e. it has no children).
pub struct TreeNode {
    pub children: IndexMap<String, TreeNode>,
    pub is_leaf: bool,
}

impl TreeNode {
    /// Creates a new `TreeNode`.
    pub fn new() -> TreeNode {
        TreeNode {
            children: IndexMap::new(),
            is_leaf: true,
        }
    }

    pub fn add_path<I>(&mut self, parts: I)
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
