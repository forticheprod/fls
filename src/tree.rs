extern crate ptree;
use ptree::{print_tree, TreeBuilder};
use std::borrow::Cow;

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
}

fn build_children(dad: children: Vec<Node>) {
    for child in children {}
}
fn walk_tree(root: Node) {
    let dad = TreeBuilder::new(root.name);
    build_children(dad, root.children)
}

pub fn build_tree_from_paths(paths: Vec<String>) -> Node {
    let mut root = Node {
        name: String::from("/"), // Root node
        children: Vec::new(),
    };

    for path in paths {
        let mut current_node = &mut root;
        for component in path.split('/') {
            if component.is_empty() {
                continue; // Skip empty components (e.g., leading slash)
            }

            // Check if the component is already a child of the current node
            if let Some(child) = current_node
                .children
                .iter_mut()
                .find(|c| c.name == component)
            {
                current_node = child; // Move to the existing child
            } else {
                // Create a new child node if not found
                let new_child = Node {
                    name: String::from(component),
                    children: Vec::new(),
                };
                current_node.children.push(new_child);
                current_node = current_node.children.last_mut().unwrap();
            }
        }
    }

    root
}

fn build_tree(root: Node) -> TreeBuilder {
    Node("");
    ptree::print_tree(&dir).expect("Unable to print directory tree");
}
