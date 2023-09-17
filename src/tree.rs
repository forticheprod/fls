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
