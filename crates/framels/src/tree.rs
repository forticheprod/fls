use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct TreeNode {
    path: PathBuf,
    children: BTreeMap<OsString, TreeNode>,
}

impl TreeNode {
    fn new(path: PathBuf) -> TreeNode {
        TreeNode {
            path,
            children: BTreeMap::new(),
        }
    }
}
/// Builds a tree of `TreeNode` objects from a slice of `PathBuf` objects.
///
/// # Arguments
///
/// * `paths` - A slice of `PathBuf` objects representing the paths to include in the tree.
///
/// # Returns
///
/// A `TreeNode` object representing the root of the tree.
///
/// # Example
///
/// ```rust
/// use std::path::PathBuf;
/// use tree::TreeNode;
///
/// let paths = vec![
///     PathBuf::from("/foo/bar/baz"),
///     PathBuf::from("/foo/bar/qux"),
///     PathBuf::from("/foo/quux"),
/// ];
///
/// let root_node = build_tree(&paths);
///
/// assert_eq!(root_node.children.len(), 1);
/// assert_eq!(root_node.children.keys().next().unwrap(), "foo");
/// assert_eq!(root_node.children.values().next().unwrap().children.len(), 2);
/// ```
fn build_tree(paths: &[PathBuf]) -> TreeNode {
    let mut root_node = TreeNode::new(PathBuf::new());

    for path in paths {
        let mut current_node = &mut root_node;

        for component in path.iter() {
            current_node = current_node
                .children
                .entry(component.to_os_string())
                .or_insert(TreeNode::new(Path::new(component).to_path_buf()));
        }
    }

    root_node
}

fn print_tree(tree: &TreeNode, indent: usize) {
    println!(
        "{:indent$}┗ {}",
        "",
        tree.path.display(),
        indent = indent * 4
    );
    for child in tree.children.values() {
        print_tree(child, indent + 1);
    }
}

pub fn run_tree(paths: Vec<PathBuf>) {
    let tree = build_tree(&paths);
    print_tree(&tree, 0);
}
